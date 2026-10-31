use crate::envelope::{POINTS, Wave, curve};

#[derive(Clone, Copy, Debug)]
pub(super) struct Runtime {
    pub enabled: bool,
    pub held: bool,
    pub active: bool,
    pub level: f32,
    settings: Wave,
    sample_rate: f32,
    bpm: f32,
    point: usize,
    progress: f32,
    anchor: f32,
    origin: f32,
    target: f32,
    edit_slew: bool,
}
impl Default for Runtime {
    fn default() -> Self {
        Self {
            enabled: false,
            held: false,
            active: false,
            level: 0.0,
            settings: Wave::default(),
            sample_rate: 48000.0,
            bpm: 120.0,
            point: 0,
            progress: 0.0,
            anchor: 0.0,
            origin: 0.0,
            target: 0.0,
            edit_slew: false,
        }
    }
}
impl Runtime {
    pub fn configure(&mut self, settings: Wave, sr: f32, bpm: f32) {
        self.settings = settings;
        self.sample_rate = sr.max(1.0);
        self.bpm = bpm.clamp(1.0, 999.0);
        self.enabled = settings.enabled;
        if self.active && self.held && self.point > settings.loop_end as usize {
            self.begin(settings.loop_end as usize);
        }
        let target = if self.point < POINTS {
            settings.levels[self.point]
        } else {
            0.0
        };
        if target != self.target && self.active {
            // Endpoint automation near the end still gets a 20 ms ramp. Keep
            // the current sounded level, rather than jumping to the edited point.
            let duration = if self.point < POINTS {
                settings.times[self.point] * if settings.beats { 60.0 / self.bpm } else { 1.0 }
            } else {
                0.005
            };
            self.progress = self
                .progress
                .min((1.0 - 0.020 / duration.max(0.020)).max(0.0));
            self.edit_slew = true;
            self.origin = self.level;
            self.anchor = self.progress;
            self.target = target;
        } else if !self.active {
            self.target = target;
        }
    }
    fn begin(&mut self, point: usize) {
        self.edit_slew = false;
        self.point = point;
        self.progress = 0.0;
        self.anchor = 0.0;
        self.origin = self.level;
        self.target = if point < POINTS {
            self.settings.levels[point]
        } else {
            0.0
        };
    }
    pub fn adopt(&mut self, level: f32, held: bool) {
        self.level = level;
        self.held = held;
        self.active = true;
        self.begin(if held {
            0
        } else {
            self.settings.loop_end as usize + 1
        });
    }
    pub fn on(&mut self) {
        self.adopt(self.level, true);
    }
    pub fn off(&mut self) {
        if self.active {
            self.held = false;
            self.begin(self.settings.loop_end as usize + 1);
        }
    }
    pub fn kill(&mut self) {
        self.active = false;
        self.held = false;
        self.level = 0.0;
    }
    pub fn next(&mut self) -> f32 {
        if !self.active {
            return 0.0;
        }
        // A held non-looping envelope sustains at loop_end. Release always
        // starts from the actual sounded level, including mid-segment note-off.
        if self.held
            && self.point == self.settings.loop_end as usize
            && self.progress >= 1.0
            && !self.settings.loop_on
        {
            return self.level;
        }
        let seconds = if self.point < POINTS {
            self.settings.times[self.point]
                * if self.settings.beats {
                    60.0 / self.bpm
                } else {
                    1.0
                }
        } else {
            0.005
        }; // finite, click-free zero tail if the last editable point is nonzero
        let seconds = if self.edit_slew {
            seconds.max(0.020)
        } else {
            seconds
        };
        self.progress = (self.progress + 1.0 / (seconds * self.sample_rate).max(1.0)).min(1.0);
        let x =
            ((self.progress - self.anchor) / (1.0 - self.anchor).max(f32::EPSILON)).clamp(0.0, 1.0);
        let shape = if self.point < POINTS {
            self.settings.curves[self.point]
        } else {
            0.0
        };
        self.level = self.origin + (self.target - self.origin) * curve(x, shape);
        if self.progress >= 1.0 {
            self.level = self.target;
            if self.held && self.point >= self.settings.loop_end as usize {
                if self.settings.loop_on {
                    self.begin(self.settings.loop_start as usize);
                }
            } else if self.point + 1 < POINTS {
                self.begin(self.point + 1);
            } else if self.level > 0.0 && self.point < POINTS {
                self.begin(POINTS);
            } else {
                self.kill();
            }
        }
        self.level
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn env() -> Runtime {
        let mut r = Runtime::default();
        r.configure(
            Wave {
                enabled: true,
                times: [0.1; 6],
                levels: [1.0, 0.0, 1.0, 0.0, 0.8, 0.0],
                loop_on: true,
                loop_start: 1,
                loop_end: 2,
                ..Wave::default()
            },
            1000.0,
            120.0,
        );
        r.on();
        r
    }
    #[test]
    fn independent_loops_release_and_live_tempo() {
        let mut a = env();
        for _ in 0..50 {
            a.next();
        }
        let mut b = env();
        assert!((a.level - 0.5).abs() < 0.001);
        assert_eq!(b.level, 0.0);
        a.off();
        let previous = a.level;
        let first = a.next();
        assert!((first - previous).abs() < 0.02);
        for _ in 0..1000 {
            a.next();
            b.next();
        }
        assert!(!a.active);
        assert!(b.active);
        let mut w = b.settings;
        w.beats = true;
        let before = b.level;
        b.configure(w, 1000.0, 60.0);
        assert_eq!(before, b.level);
        w.levels[b.point] = 0.7;
        b.configure(w, 1000.0, 90.0);
        assert!(
            (b.next() - before).abs() <= 0.051,
            "live endpoint edits must slew for at least 20 ms"
        );
    }
    #[test]
    fn zero_times_terminate_and_nonzero_last_point_has_a_tail() {
        let mut r = Runtime::default();
        r.configure(
            Wave {
                enabled: true,
                times: [0.0; 6],
                levels: [1.0; 6],
                ..Wave::default()
            },
            1000.0,
            120.0,
        );
        r.on();
        r.off();
        for _ in 0..20 {
            r.next();
        }
        assert!(!r.active);
        assert_eq!(r.level, 0.0);
    }
}
