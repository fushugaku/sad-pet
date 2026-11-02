//! Persistent mixed oscillator slots. Physical automation addresses never move.
use crate::{resynth, wavetable};
use serde::{Deserialize, Serialize};

pub const SLOTS: usize = 8;
pub const VERSION: u32 = 1;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Kind {
    #[default]
    Radias,
    Wavefolder,
    Pcm,
    Wavetable,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Controls {
    pub enabled: bool,
    pub level: f32,
    pub pan: f32,
    pub transpose: f32,
    pub fine: f32,
    pub native: resynth::Oscillator,
    pub wavetable: wavetable::Controls,
}
impl Default for Controls {
    fn default() -> Self {
        Self {
            enabled: true,
            level: 0.65,
            pan: 0.5,
            transpose: 0.0,
            fine: 0.0,
            native: resynth::Oscillator {
                waveform: resynth::Waveform::Saw,
                ..Default::default()
            },
            wavetable: wavetable::Controls {
                fm_source: 0,
                sync_source: 0,
                ..Default::default()
            },
        }
    }
}
impl Controls {
    pub fn sanitize(&mut self) {
        use crate::preset::finite_clamp;
        finite_clamp(&mut self.level, 0.0, 1.0, 0.65);
        finite_clamp(&mut self.pan, 0.0, 1.0, 0.5);
        finite_clamp(&mut self.transpose, -48.0, 48.0, 0.0);
        finite_clamp(&mut self.fine, -100.0, 100.0, 0.0);
        let cross = self.native.cross_source.min(8);
        self.native.sanitize();
        self.native.cross_source = cross;
        self.wavetable.sanitize_mixed();
    }
    pub fn native(self, kind: Kind) -> resynth::Oscillator {
        let mut n = self.native;
        n.enabled = self.enabled;
        n.level = self.level;
        n.pan = self.pan;
        n.transpose = self.transpose;
        n.fine = self.fine;
        n.model = if kind == Kind::Wavefolder {
            resynth::Model::Wavefolder
        } else {
            resynth::Model::Radias
        };
        if kind == Kind::Pcm {
            n.waveform = resynth::Waveform::Pcm;
        }
        n
    }
    pub fn table(self) -> wavetable::Controls {
        let mut w = self.wavetable;
        w.enabled = self.enabled;
        w.level = self.level;
        w.pan = self.pan;
        w.transpose = self.transpose;
        w.fine = self.fine;
        w
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Slot {
    /// Zero means empty. A new occupant always receives a new identity.
    pub id: u32,
    pub kind: Kind,
    #[serde(flatten)]
    pub controls: Controls,
    pub asset_id: String,
}
impl Default for Slot {
    fn default() -> Self {
        Self {
            id: 0,
            kind: Kind::default(),
            controls: Controls::default(),
            asset_id: wavetable::DEFAULT_ASSET.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub version: u32,
    pub next_id: u32,
    pub slots: [Slot; SLOTS],
    pub order: [u8; SLOTS],
    pub motion: wavetable::motion::Motion,
    /// Explicit participants of the existing four-corner XY performance pad.
    pub xy_slots: [u32; 4],
}
impl Default for Settings {
    fn default() -> Self {
        Self {
            version: VERSION,
            next_id: 2,
            slots: std::array::from_fn(|i| Slot {
                id: u32::from(i == 0),
                ..Default::default()
            }),
            order: std::array::from_fn(|i| i as u8),
            motion: Default::default(),
            xy_slots: [1, 0, 0, 0],
        }
    }
}
impl Settings {
    pub fn validate(&self) -> Result<(), String> {
        if self.version != VERSION {
            return Err(format!(
                "Unsupported oscillator engine version {}",
                self.version
            ));
        }
        let mut ids = Vec::new();
        for slot in &self.slots {
            if slot.id != 0 && ids.contains(&slot.id) {
                return Err("Oscillator identities must be unique".into());
            }
            if slot.id != 0 {
                ids.push(slot.id);
            }
        }
        Ok(())
    }
    pub fn sanitize(&mut self) {
        let mut seen = [false; SLOTS];
        let mut order = [0; SLOTS];
        let mut at = 0;
        for &i in &self.order {
            let i = i as usize;
            if i < SLOTS && !seen[i] {
                seen[i] = true;
                order[at] = i as u8;
                at += 1;
            }
        }
        for (i, present) in seen.iter().enumerate() {
            if !present {
                order[at] = i as u8;
                at += 1;
            }
        }
        self.order = order;
        for slot in &mut self.slots {
            slot.controls.sanitize();
            if slot.kind == Kind::Wavefolder && !slot.controls.native.waveform.analog() {
                slot.controls.native.waveform = resynth::Waveform::Sine;
            }
            if slot.asset_id.is_empty() {
                slot.asset_id = wavetable::DEFAULT_ASSET.into();
            }
        }
        self.motion.sanitize();
        for id in &mut self.xy_slots {
            if !self.slots.iter().any(|s| s.id != 0 && s.id == *id) {
                *id = 0;
            }
        }
        self.next_id = self.next_id.max(1);
    }
    pub fn index(&self, id: u32) -> Option<usize> {
        (id != 0)
            .then(|| self.slots.iter().position(|s| s.id == id))
            .flatten()
    }
    pub fn add(&mut self, kind: Kind) -> Result<u32, String> {
        let index = self
            .slots
            .iter()
            .position(|s| s.id == 0)
            .ok_or("An oscillator bank can hold at most eight oscillators")?;
        let mut id = self.next_id.max(1);
        while self.index(id).is_some() {
            id = id
                .checked_add(1)
                .ok_or("Oscillator identities are exhausted")?;
        }
        self.next_id = id
            .checked_add(1)
            .ok_or("Oscillator identities are exhausted")?;
        self.slots[index] = Slot {
            id,
            kind,
            ..Default::default()
        };
        Ok(id)
    }
    pub fn remove(&mut self, id: u32) -> Result<(), String> {
        let i = self.index(id).ok_or("Choose an oscillator to remove")?;
        self.slots[i].id = 0;
        self.slots[i].controls.enabled = false;
        for slot in &mut self.slots {
            if slot.controls.wavetable.fm_source as usize == i {
                slot.controls.wavetable.fm_mode = 0;
                slot.controls.wavetable.fm_index = 0.0;
            }
            if slot.controls.wavetable.sync_source as usize == i {
                slot.controls.wavetable.sync = false;
            }
            if slot.controls.native.cross_source as usize == i + 1 {
                slot.controls.native.cross_source = 0;
            }
        }
        for xy in &mut self.xy_slots {
            if *xy == id {
                *xy = 0;
            }
        }
        Ok(())
    }
    pub fn move_slot(&mut self, id: u32, direction: i8) -> Result<(), String> {
        let index = self.index(id).ok_or("Choose an oscillator to move")? as u8;
        let at = self.order.iter().position(|&i| i == index).unwrap();
        let next =
            (at as isize + direction.signum() as isize).clamp(0, SLOTS as isize - 1) as usize;
        self.order.swap(at, next);
        Ok(())
    }
    pub fn controls(&self) -> [Controls; SLOTS] {
        std::array::from_fn(|i| {
            let mut c = self.slots[i].controls;
            c.enabled &= self.slots[i].id != 0;
            c
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn identities_and_physical_addresses_survive_reorder_and_reuse() {
        let mut s = Settings::default();
        let b = s.add(Kind::Wavetable).unwrap();
        let c = s.add(Kind::Pcm).unwrap();
        s.move_slot(b, -1).unwrap();
        assert_eq!(s.index(b), Some(1));
        assert_eq!(s.index(c), Some(2));
        s.remove(b).unwrap();
        let d = s.add(Kind::Wavefolder).unwrap();
        assert_ne!(b, d);
        assert_eq!(s.index(d), Some(1));
        assert_eq!(s.index(c), Some(2));
        assert!(s.index(b).is_none());
        assert_eq!(
            serde_json::from_value::<Settings>(serde_json::to_value(&s).unwrap()).unwrap(),
            s
        );
    }
}
