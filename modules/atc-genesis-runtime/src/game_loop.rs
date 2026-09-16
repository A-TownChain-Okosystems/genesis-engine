use atc_genesis_platform::{AudioRuntime, PhysicsWorld, Renderer};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Tick(pub u64);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FixedStep {
    pub dt_seconds: f32,
    pub max_steps_per_update: u32,
}

impl Default for FixedStep {
    fn default() -> Self { Self { dt_seconds: 1.0 / 60.0, max_steps_per_update: 8 } }
}

impl FixedStep {
    pub fn validate(&self) -> Result<(), &'static str> {
        if !self.dt_seconds.is_finite() || self.dt_seconds <= 0.0 { return Err("invalid fixed timestep"); }
        if self.max_steps_per_update == 0 { return Err("max_steps_per_update must be non-zero"); }
        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct GameClock { accumulator: f32, tick: Tick }

impl GameClock {
    pub fn tick(&self) -> Tick { self.tick }
    pub fn advance<R, P, A>(&mut self, elapsed_seconds: f32, step: FixedStep, renderer: &mut R, physics: &mut P, audio: &mut A) -> Result<u32, &'static str>
    where R: Renderer, P: PhysicsWorld, A: AudioRuntime {
        step.validate()?;
        if !elapsed_seconds.is_finite() || elapsed_seconds < 0.0 { return Err("invalid elapsed time"); }
        self.accumulator = (self.accumulator + elapsed_seconds).min(step.dt_seconds * step.max_steps_per_update as f32);
        let mut steps = 0;
        while self.accumulator + f32::EPSILON >= step.dt_seconds && steps < step.max_steps_per_update {
            physics.step(step.dt_seconds);
            audio.update(step.dt_seconds);
            self.tick = Tick(self.tick.0.saturating_add(1));
            self.accumulator -= step.dt_seconds;
            steps += 1;
        }
        renderer.begin_frame(atc_genesis_platform::FrameId(self.tick.0));
        renderer.end_frame();
        Ok(steps)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    struct R; impl Renderer for R { fn begin_frame(&mut self,_:atc_genesis_platform::FrameId){} fn submit(&mut self,_:atc_genesis_platform::EntityId,_:atc_genesis_platform::Transform){} fn end_frame(&mut self){} }
    struct P{steps:u32} impl PhysicsWorld for P { fn step(&mut self,_:f32){self.steps+=1} fn raycast(&self,_:[f32;3],_:[f32;3],_:f32)->Option<atc_genesis_platform::EntityId>{None} }
    struct A{updates:u32} impl AudioRuntime for A { fn update(&mut self,_:f32){self.updates+=1} fn set_master_gain(&mut self,_:f32){} }
    #[test] fn fixed_step_is_deterministic(){let mut c=GameClock::default();let mut r=R;let mut p=P{steps:0};let mut a=A{updates:0};assert_eq!(c.advance(1.0/30.0,FixedStep::default(),&mut r,&mut p,&mut a).unwrap(),2);assert_eq!(p.steps,2);assert_eq!(a.updates,2);assert_eq!(c.tick(),Tick(2));}
    #[test] fn rejects_negative_time(){let mut c=GameClock::default();let mut r=R;let mut p=P{steps:0};let mut a=A{updates:0};assert!(c.advance(-1.0,FixedStep::default(),&mut r,&mut p,&mut a).is_err());}
}
