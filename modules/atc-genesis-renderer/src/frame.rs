use atc_genesis_platform::FrameId;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FrameTiming {
    pub fixed_delta_seconds: f32,
    pub max_frame_delta_seconds: f32,
}

impl Default for FrameTiming {
    fn default() -> Self { Self { fixed_delta_seconds: 1.0 / 60.0, max_frame_delta_seconds: 0.25 } }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FrameSchedule {
    pub frame: FrameId,
    pub fixed_steps: u32,
    pub interpolation_alpha: f32,
    pub delta_seconds: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FrameScheduler {
    timing: FrameTiming,
    accumulator: f32,
    next_frame: u64,
}

impl FrameScheduler {
    pub fn new(timing: FrameTiming) -> Result<Self, FrameScheduleError> {
        validate_timing(timing)?;
        Ok(Self { timing, accumulator: 0.0, next_frame: 0 })
    }

    pub fn timing(&self) -> FrameTiming { self.timing }

    pub fn advance(&mut self, delta_seconds: f32) -> Result<FrameSchedule, FrameScheduleError> {
        if !delta_seconds.is_finite() { return Err(FrameScheduleError::NonFiniteDelta); }
        if delta_seconds < 0.0 { return Err(FrameScheduleError::NegativeDelta); }
        let delta = delta_seconds.min(self.timing.max_frame_delta_seconds);
        self.accumulator += delta;
        let mut fixed_steps = 0;
        while self.accumulator >= self.timing.fixed_delta_seconds {
            self.accumulator -= self.timing.fixed_delta_seconds;
            fixed_steps += 1;
        }
        let alpha = (self.accumulator / self.timing.fixed_delta_seconds).clamp(0.0, 1.0);
        let frame = FrameId(self.next_frame);
        self.next_frame = self.next_frame.checked_add(1).ok_or(FrameScheduleError::FrameOverflow)?;
        Ok(FrameSchedule { frame, fixed_steps, interpolation_alpha: alpha, delta_seconds: delta })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FrameScheduleError { InvalidTiming, NonFiniteDelta, NegativeDelta, FrameOverflow }

fn validate_timing(timing: FrameTiming) -> Result<(), FrameScheduleError> {
    if !timing.fixed_delta_seconds.is_finite() || !timing.max_frame_delta_seconds.is_finite() || timing.fixed_delta_seconds <= 0.0 || timing.max_frame_delta_seconds < timing.fixed_delta_seconds { return Err(FrameScheduleError::InvalidTiming); }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn fixed_step_accumulates_deterministically() { let mut s=FrameScheduler::new(FrameTiming::default()).unwrap(); let a=s.advance(1.0/60.0).unwrap(); assert_eq!(a.fixed_steps,1); }
    #[test] fn large_delta_is_clamped() { let mut s=FrameScheduler::new(FrameTiming::default()).unwrap(); let a=s.advance(10.0).unwrap(); assert_eq!(a.delta_seconds,0.25); }
    #[test] fn negative_delta_is_rejected() { let mut s=FrameScheduler::new(FrameTiming::default()).unwrap(); assert_eq!(s.advance(-0.1),Err(FrameScheduleError::NegativeDelta)); }
}
