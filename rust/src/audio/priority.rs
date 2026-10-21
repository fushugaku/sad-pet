//! Real-time scheduling for the DSP worker. At normal priority it can be
//! descheduled long enough to drain the output ring under load, even when its
//! average CPU use is low. Rendering itself is unchanged.

/// Time-constraint policy sized to one render block: the thread asks for half
/// of each block's real time and may be preempted beyond that.
#[cfg(target_os = "macos")]
pub(super) fn promote(block_frames: usize, sample_rate: u32) -> Result<(), String> {
    #[repr(C)]
    struct Timebase {
        numer: u32,
        denom: u32,
    }
    #[repr(C)]
    struct TimeConstraint {
        period: u32,
        computation: u32,
        constraint: u32,
        preemptible: u32,
    }
    const THREAD_TIME_CONSTRAINT_POLICY: u32 = 2;
    unsafe extern "C" {
        fn mach_timebase_info(info: *mut Timebase) -> i32;
        fn pthread_self() -> usize;
        fn pthread_mach_thread_np(thread: usize) -> u32;
        fn thread_policy_set(
            thread: u32,
            flavor: u32,
            policy: *const TimeConstraint,
            count: u32,
        ) -> i32;
    }
    let mut timebase = Timebase { numer: 0, denom: 0 };
    // SAFETY: fills the struct it is given and reads nothing else.
    if unsafe { mach_timebase_info(&mut timebase) } != 0 || timebase.numer == 0 {
        return Err("No Mach timebase".into());
    }
    let ticks = |seconds: f64| {
        (seconds * 1e9 * timebase.denom as f64 / timebase.numer as f64).round() as u32
    };
    let period = block_frames as f64 / sample_rate as f64;
    let policy = TimeConstraint {
        period: ticks(period),
        computation: ticks(period * 0.5),
        constraint: ticks(period),
        preemptible: 1,
    };
    let count = (size_of::<TimeConstraint>() / size_of::<u32>()) as u32;
    // SAFETY: the calling thread's own port, a policy of the flavor's size, and
    // pthread_mach_thread_np does not transfer a port right to deallocate.
    let result = unsafe {
        thread_policy_set(
            pthread_mach_thread_np(pthread_self()),
            THREAD_TIME_CONSTRAINT_POLICY,
            &policy,
            count,
        )
    };
    if result == 0 {
        Ok(())
    } else {
        Err(format!("Real-time DSP priority unavailable ({result})"))
    }
}

/// Other platforms keep the default priority.
#[cfg(not(target_os = "macos"))]
pub(super) fn promote(_block_frames: usize, _sample_rate: u32) -> Result<(), String> {
    Ok(())
}

#[cfg(all(test, target_os = "macos"))]
mod tests {
    #[test]
    fn promotes_a_render_thread() {
        let promoted = std::thread::spawn(|| super::promote(128, 48_000));
        assert_eq!(promoted.join().unwrap(), Ok(()));
    }
}
