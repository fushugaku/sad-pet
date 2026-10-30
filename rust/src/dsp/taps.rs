//! The stage taps at audio rate. A Chain share ramps, so that leaving a stage
//! out of the chain crossfades to its input; each send ramps per bus, so that
//! choosing another bus crossfades between the two. The advanced mode grain
//! groups have taps of their own after the stages'.
use crate::{
    Stereo,
    bus::MAX_BUSES,
    grain_groups::GROUPS,
    preset::{FilterSlot, MAX_FILTER_SLOTS},
    taps::{STAGES, Stage, Tap, Taps},
};

/// The stages, then the grain groups.
const SLOT_OFFSET: usize = STAGES + GROUPS;
const SOURCES: usize = SLOT_OFFSET + MAX_FILTER_SLOTS;

/// About 10 ms at 48 kHz from one end of a share to the other.
const STEP: f32 = 1.0 / 512.0;

#[derive(Clone)]
pub(crate) struct Runtime {
    settings: Taps,
    /// The grain groups' outputs; all at their defaults outside advanced mode.
    groups: [Tap; GROUPS],
    filters: [Tap; MAX_FILTER_SLOTS],
    filter_ids: [u32; MAX_FILTER_SLOTS],
    /// Bit n set for each Bus n that exists to take a send.
    open: u16,
    chain: [f32; SOURCES],
    chain_targets: [f32; SOURCES],
    gains: [[f32; MAX_BUSES]; SOURCES],
    targets: [[f32; MAX_BUSES]; SOURCES],
    /// Every share is at its target: a frame costs one compare.
    settled: bool,
    /// Bit s set while source s has a send gain above zero.
    sending: u32,
    /// Some send has a target above zero.
    targeted: bool,
    /// Bit b set for each Bus b + 1 that a section is routed to, and for
    /// each one a send still fades out of.
    routed: u8,
    fading: u8,
    /// What the taps sent into each bus in this frame.
    out: [Stereo; MAX_BUSES],
    filled: bool,
    /// The first settings apply at once: a loaded preset starts where it is.
    configured: bool,
}
impl Runtime {
    pub(crate) fn new() -> Self {
        Self {
            settings: Taps::default(),
            groups: [Tap::default(); GROUPS],
            filters: [Tap::default(); MAX_FILTER_SLOTS],
            filter_ids: [0; MAX_FILTER_SLOTS],
            open: 0,
            chain: [1.0; SOURCES],
            chain_targets: [1.0; SOURCES],
            gains: [[0.0; MAX_BUSES]; SOURCES],
            targets: [[0.0; MAX_BUSES]; SOURCES],
            settled: true,
            sending: 0,
            targeted: false,
            routed: 0,
            fading: 0,
            out: [[0.0; 2]; MAX_BUSES],
            filled: false,
            configured: false,
        }
    }
    /// Takes the modulated settings once per control block.
    pub(crate) fn configure(&mut self, settings: &Taps, groups: [Tap; GROUPS]) {
        if *settings != self.settings || groups != self.groups {
            self.settings = *settings;
            self.groups = groups;
            self.retarget();
        }
        if !self.configured {
            self.configured = true;
            self.chain = self.chain_targets;
        }
    }
    /// Keep a filter's ramp history with its ID when the list is reordered.
    pub(crate) fn configure_filters(&mut self, slots: &[FilterSlot]) {
        let filters = std::array::from_fn(|i| slots.get(i).map_or(Tap::default(), |s| s.output));
        let ids = std::array::from_fn(|i| slots.get(i).map_or(0, |s| s.id));
        if ids != self.filter_ids {
            let chain = self.chain;
            let gains = self.gains;
            for (i, &id) in ids.iter().enumerate() {
                let previous = self.filter_ids.iter().position(|old| id != 0 && *old == id);
                self.chain[SLOT_OFFSET + i] =
                    previous.map_or(filters[i].chain, |j| chain[SLOT_OFFSET + j]);
                self.gains[SLOT_OFFSET + i] =
                    previous.map_or([0.0; MAX_BUSES], |j| gains[SLOT_OFFSET + j]);
            }
        }
        if filters != self.filters || ids != self.filter_ids {
            self.filters = filters;
            self.filter_ids = ids;
            self.retarget();
        }
    }
    /// Without its bus, a send goes nowhere; the chain is unaffected.
    pub(crate) fn set_open_buses(&mut self, open: u16) {
        if open != self.open {
            self.open = open;
            self.retarget();
        }
    }
    fn retarget(&mut self) {
        self.targeted = false;
        self.routed = 0;
        let taps = Stage::ALL
            .map(|stage| *self.settings.get(stage))
            .into_iter()
            .chain(self.groups)
            .chain(self.filters);
        for (s, tap) in taps.enumerate() {
            self.chain_targets[s] = tap.chain;
            let open = tap.bus > 0 && self.open & 1 << tap.bus != 0;
            if open {
                self.routed |= 1 << (tap.bus - 1);
            }
            for (b, target) in self.targets[s].iter_mut().enumerate() {
                *target = if open && b + 1 == tap.bus as usize {
                    tap.send
                } else {
                    0.0
                };
                self.targeted |= *target > 0.0;
            }
        }
        self.settled = false;
    }
    /// Ramps every share one frame towards its target.
    #[inline]
    pub(crate) fn begin_frame(&mut self) {
        if self.filled {
            self.out = [[0.0; 2]; MAX_BUSES];
            self.filled = false;
        }
        if self.settled {
            return;
        }
        let mut settled = true;
        let mut sending = 0;
        let mut fading = 0;
        for s in 0..SOURCES {
            settled &= ramp(&mut self.chain[s], self.chain_targets[s]);
            for (b, (gain, &target)) in self.gains[s].iter_mut().zip(&self.targets[s]).enumerate() {
                settled &= ramp(gain, target);
                if *gain != 0.0 {
                    sending |= 1 << s;
                    fading |= 1 << b;
                }
            }
        }
        self.settled = settled;
        self.sending = sending;
        self.fading = fading;
    }
    /// Some send is running or about to: the stages must be rendered.
    pub(crate) fn active(&self) -> bool {
        self.targeted || self.sending != 0
    }
    /// The buses the next frames may send to (bit b for Bus b + 1).
    pub(crate) fn buses(&self) -> u8 {
        self.routed | self.fading
    }
    #[inline]
    pub(crate) fn chain(&self, stage: Stage) -> f32 {
        self.chain[stage as usize]
    }
    /// The stage's input and output blended as its Chain share says.
    #[inline]
    pub(crate) fn mix(&self, stage: Stage, input: Stereo, output: Stereo) -> Stereo {
        self.mix_source(stage as usize, input, output)
    }
    #[inline]
    fn mix_source(&self, source: usize, input: Stereo, output: Stereo) -> Stereo {
        let chain = self.chain[source];
        if chain == 1.0 {
            output
        } else if chain == 0.0 {
            input
        } else {
            std::array::from_fn(|c| input[c] + (output[c] - input[c]) * chain)
        }
    }
    /// The stage sends this frame: worth summing its output for.
    #[inline]
    pub(crate) fn sends(&self, stage: Stage) -> bool {
        self.sending & 1 << stage as u8 != 0
    }
    #[inline]
    pub(crate) fn send(&mut self, stage: Stage, frame: Stereo) {
        self.send_source(stage as usize, frame);
    }
    /// The grain groups' Chain shares.
    #[inline]
    pub(crate) fn group_chains(&self) -> [f32; GROUPS] {
        std::array::from_fn(|g| self.chain[STAGES + g])
    }
    /// Bit g set for each grain group that sends this frame.
    #[inline]
    pub(crate) fn group_sends(&self) -> u8 {
        ((self.sending >> STAGES) & ((1 << GROUPS) - 1)) as u8
    }
    #[inline]
    pub(crate) fn send_group(&mut self, group: usize, frame: Stereo) {
        self.send_source(STAGES + group, frame);
    }
    #[inline]
    pub(crate) fn slot_sends(&self, slot: usize) -> bool {
        self.sending & (1 << (SLOT_OFFSET + slot)) != 0
    }
    #[inline]
    pub(crate) fn send_slot(&mut self, slot: usize, frame: Stereo) {
        self.send_source(SLOT_OFFSET + slot, frame);
    }
    #[inline]
    pub(crate) fn mix_slot(&self, slot: usize, input: Stereo, output: Stereo) -> Stereo {
        self.mix_source(SLOT_OFFSET + slot, input, output)
    }
    #[inline]
    fn send_source(&mut self, source: usize, frame: Stereo) {
        for (out, &gain) in self.out.iter_mut().zip(&self.gains[source]) {
            if gain != 0.0 {
                out[0] += frame[0] * gain;
                out[1] += frame[1] * gain;
                self.filled = true;
            }
        }
    }
    /// Scales this frame's sends by the timbre's volume, or silences them.
    #[inline]
    pub(crate) fn finish(&mut self, gain: f32, silent: bool) {
        if !self.filled {
            return;
        }
        for out in &mut self.out {
            for x in out {
                let value = if silent { 0.0 } else { *x * gain };
                *x = if value.is_finite() { value } else { 0.0 };
            }
        }
    }
    /// This frame's input of each bus, or None when no tap sent anything.
    pub(crate) fn output(&self) -> Option<&[Stereo; MAX_BUSES]> {
        self.filled.then_some(&self.out)
    }
}

/// Moves `value` one step towards `target`; true once it is there.
#[inline]
fn ramp(value: &mut f32, target: f32) -> bool {
    let delta = target - *value;
    if delta.abs() <= STEP {
        *value = target;
        true
    } else {
        *value += STEP.copysign(delta);
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::taps::Tap;

    fn settled(runtime: &mut Runtime) {
        for _ in 0..1024 {
            runtime.begin_frame();
        }
    }

    #[test]
    fn a_send_needs_its_bus_and_crossfades_to_another() {
        let mut runtime = Runtime::new();
        let mut taps = Taps {
            grains: Tap {
                chain: 1.0,
                send: 0.5,
                bus: 3,
            },
            ..Default::default()
        };
        runtime.configure(&taps, Default::default());
        settled(&mut runtime);
        assert!(!runtime.active(), "Bus 3 does not exist yet");
        assert_eq!(runtime.buses(), 0);
        runtime.set_open_buses(1 << 3 | 1 << 5);
        assert_eq!(runtime.buses(), 1 << 2, "routed to Bus 3");
        runtime.begin_frame();
        assert!(runtime.sends(Stage::Grains) && !runtime.sends(Stage::Filters));
        settled(&mut runtime);
        runtime.begin_frame();
        runtime.send(Stage::Grains, [1.0, -1.0]);
        runtime.finish(0.5, false);
        assert_eq!(runtime.output().unwrap()[2], [0.25, -0.25]);
        taps.grains.bus = 5;
        runtime.configure(&taps, Default::default());
        runtime.begin_frame();
        assert_eq!(runtime.buses(), 1 << 4 | 1 << 2, "Bus 3 still fades out");
        runtime.send(Stage::Grains, [1.0, 1.0]);
        let out = runtime.output().unwrap();
        assert!(out[2][0] > 0.0 && out[2][0] < 0.5 && out[4][0] > 0.0);
        settled(&mut runtime);
        runtime.begin_frame();
        runtime.send(Stage::Grains, [1.0, 1.0]);
        assert_eq!(runtime.output().unwrap()[2], [0.0; 2]);
        runtime.begin_frame();
        assert!(runtime.output().is_none(), "a frame without sends is empty");
    }

    #[test]
    fn chain_shares_blend_input_and_output_and_land_exactly() {
        let mut runtime = Runtime::new();
        let mut taps = Taps::default();
        taps.filters.chain = 0.0;
        runtime.configure(&taps, Default::default());
        assert_eq!(
            runtime.mix(Stage::Filters, [1.0; 2], [3.0; 2]),
            [1.0; 2],
            "a loaded preset starts at its shares"
        );
        taps.filters.chain = 1.0;
        runtime.configure(&taps, Default::default());
        settled(&mut runtime);
        assert_eq!(runtime.mix(Stage::Filters, [1.0; 2], [3.0; 2]), [3.0; 2]);
        taps.filters.chain = 0.0;
        runtime.configure(&taps, Default::default());
        runtime.begin_frame();
        let halfway = runtime.mix(Stage::Filters, [1.0; 2], [3.0; 2]);
        assert!(halfway[0] > 2.9 && halfway[0] < 3.0);
        settled(&mut runtime);
        assert_eq!(runtime.mix(Stage::Filters, [1.0; 2], [3.0; 2]), [1.0; 2]);
        taps.filters.chain = 0.25;
        runtime.configure(&taps, Default::default());
        settled(&mut runtime);
        assert_eq!(runtime.mix(Stage::Filters, [1.0; 2], [3.0; 2]), [1.5; 2]);
    }

    #[test]
    fn reordered_filter_sends_keep_their_routes_without_crossfeeding() {
        let mut runtime = Runtime::new();
        let mut slots = [
            FilterSlot {
                id: 10,
                output: Tap {
                    bus: 1,
                    send: 0.5,
                    chain: 0.25,
                },
                ..Default::default()
            },
            FilterSlot {
                id: 20,
                output: Tap {
                    bus: 2,
                    send: 1.0,
                    chain: 0.75,
                },
                ..Default::default()
            },
        ];
        runtime.configure_filters(&slots);
        runtime.configure(&Taps::default(), Default::default());
        runtime.set_open_buses(1 << 1 | 1 << 2);
        settled(&mut runtime);
        slots.swap(0, 1);
        runtime.configure_filters(&slots);
        runtime.begin_frame();
        runtime.send_slot(0, [1.0; 2]);
        assert_eq!(runtime.output().unwrap()[0], [0.0; 2]);
        assert_eq!(runtime.output().unwrap()[1], [1.0; 2]);
        assert_eq!(runtime.mix_slot(0, [0.0; 2], [1.0; 2]), [0.75; 2]);
    }
}
