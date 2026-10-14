#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DriveMode { Park, Drive, Reverse }

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VehicleSpec {
    pub mass_kg: f32,
    pub max_speed_mps: f32,
    pub max_reverse_speed_mps: f32,
    pub max_accel_mps2: f32,
    pub max_brake_mps2: f32,
    pub max_steer_rad: f32,
    pub wheelbase_m: f32,
    pub drag: f32,
    pub rolling_resistance: f32,
}

impl Default for VehicleSpec {
    fn default() -> Self {
        Self { mass_kg: 1500.0, max_speed_mps: 55.0, max_reverse_speed_mps: 8.0,
            max_accel_mps2: 5.0, max_brake_mps2: 10.0, max_steer_rad: 0.60,
            wheelbase_m: 2.8, drag: 0.015, rolling_resistance: 0.015 }
    }
}

impl VehicleSpec {
    pub fn validate(&self) -> bool {
        self.mass_kg.is_finite() && self.mass_kg > 0.0 &&
        self.max_speed_mps.is_finite() && self.max_speed_mps >= 0.0 &&
        self.max_reverse_speed_mps.is_finite() && self.max_reverse_speed_mps >= 0.0 &&
        self.max_accel_mps2.is_finite() && self.max_accel_mps2 >= 0.0 &&
        self.max_brake_mps2.is_finite() && self.max_brake_mps2 >= 0.0 &&
        self.max_steer_rad.is_finite() && self.max_steer_rad >= 0.0 &&
        self.wheelbase_m.is_finite() && self.wheelbase_m > 0.0 &&
        self.drag.is_finite() && self.drag >= 0.0 &&
        self.rolling_resistance.is_finite() && self.rolling_resistance >= 0.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VehicleState {
    pub position: [f32; 2],
    pub heading_rad: f32,
    pub speed_mps: f32,
    pub steering_rad: f32,
    pub throttle: f32,
    pub brake: f32,
    pub mode: DriveMode,
}

impl Default for VehicleState {
    fn default() -> Self {
        Self { position: [0.0, 0.0], heading_rad: 0.0, speed_mps: 0.0,
            steering_rad: 0.0, throttle: 0.0, brake: 0.0, mode: DriveMode::Park }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VehicleCommand {
    pub throttle: f32,
    pub brake: f32,
    pub steering: f32,
    pub mode: DriveMode,
}

impl Default for VehicleCommand {
    fn default() -> Self {
        Self { throttle: 0.0, brake: 0.0, steering: 0.0, mode: DriveMode::Park }
    }
}

impl VehicleCommand {
    pub fn clamped(self) -> Self {
        Self { throttle: self.throttle.clamp(0.0,1.0), brake: self.brake.clamp(0.0,1.0),
            steering: self.steering.clamp(-1.0,1.0), mode: self.mode }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Waypoint { pub position: [f32;2], pub target_speed_mps: f32 }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DrivingStyle { Defensive, Normal, Sport }

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VehiclePerception {
    pub waypoint: Option<Waypoint>,
    pub obstacle_distance_m: Option<f32>,
    pub obstacle_lateral_m: f32,
    pub speed_limit_mps: Option<f32>,
}

impl Default for VehiclePerception {
    fn default() -> Self {
        Self { waypoint: None, obstacle_distance_m: None, obstacle_lateral_m: 0.0, speed_limit_mps: None }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VehicleAgent {
    pub style: DrivingStyle,
    pub reaction_time_s: f32,
    pub obstacle_clearance_m: f32,
}

impl Default for VehicleAgent {
    fn default() -> Self {
        Self { style: DrivingStyle::Normal, reaction_time_s: 0.6, obstacle_clearance_m: 3.0 }
    }
}

impl VehicleAgent {
    pub fn decide(&self, state: VehicleState, perception: VehiclePerception) -> VehicleCommand {
        let Some(waypoint) = perception.waypoint else {
            return VehicleCommand { brake: 1.0, mode: DriveMode::Drive, ..Default::default() };
        };
        let dx = waypoint.position[0] - state.position[0];
        let dy = waypoint.position[1] - state.position[1];
        let distance = (dx*dx + dy*dy).sqrt();
        let forward = [state.heading_rad.cos(), state.heading_rad.sin()];
        let cross = forward[0]*dy - forward[1]*dx;
        let heading_error = normalize_angle(dy.atan2(dx) - state.heading_rad);

        let mut target_speed = waypoint.target_speed_mps.max(0.0);
        if let Some(limit) = perception.speed_limit_mps { target_speed = target_speed.min(limit.max(0.0)); }

        if let Some(obstacle) = perception.obstacle_distance_m {
            if obstacle >= 0.0 && perception.obstacle_lateral_m.abs() < 2.5 {
                let stopping = state.speed_mps.max(0.0) * self.reaction_time_s
                    + state.speed_mps.max(0.0).powi(2) / 16.0;
                if obstacle <= stopping + self.obstacle_clearance_m { target_speed = 0.0; }
                else if obstacle <= stopping * 1.5 + self.obstacle_clearance_m { target_speed *= 0.5; }
            }
        }

        let speed_error = target_speed - state.speed_mps;
        let distance_gain = (distance / 10.0).clamp(0.0,1.0);
        let mut command = VehicleCommand {
            throttle: if speed_error > 0.05 { (speed_error/5.0).clamp(0.0,1.0) * distance_gain.max(0.25) } else { 0.0 },
            brake: if speed_error < -0.05 { (-speed_error/8.0).clamp(0.0,1.0) } else { 0.0 },
            steering: (heading_error*1.5 + cross*0.08).clamp(-1.0,1.0),
            mode: DriveMode::Drive,
        };
        match self.style {
            DrivingStyle::Defensive => { command.throttle *= 0.85; command.steering *= 0.85; }
            DrivingStyle::Sport => command.throttle = (command.throttle*1.15).clamp(0.0,1.0),
            DrivingStyle::Normal => {}
        }
        if distance < 1.0 { command.throttle=0.0; command.brake=1.0; }
        command.clamped()
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VehicleSimulator { pub spec: VehicleSpec, pub state: VehicleState }

impl VehicleSimulator {
    pub fn new(spec: VehicleSpec) -> Self {
        assert!(spec.validate(), "invalid VehicleSpec");
        Self { spec, state: VehicleState::default() }
    }

    pub fn step(&mut self, command: VehicleCommand, dt: f32) {
        let dt = dt.clamp(0.0,0.25);
        if dt == 0.0 { return; }
        let command = command.clamped();
        self.state.mode=command.mode;
        self.state.throttle=command.throttle;
        self.state.brake=command.brake;
        self.state.steering_rad=command.steering*self.spec.max_steer_rad;
        let drive_sign=match command.mode { DriveMode::Reverse=>-1.0, DriveMode::Park=>0.0, DriveMode::Drive=>1.0 };
        let speed_abs=self.state.speed_mps.abs();
        let engine=command.throttle*self.spec.max_accel_mps2*drive_sign;
        let brake=command.brake*self.spec.max_brake_mps2*self.state.speed_mps.signum();
        let drag=(self.spec.rolling_resistance+self.spec.drag*speed_abs)*self.state.speed_mps.signum();
        self.state.speed_mps += (engine-brake-drag)*dt;
        let max_speed=match command.mode { DriveMode::Reverse=>self.spec.max_reverse_speed_mps, DriveMode::Drive=>self.spec.max_speed_mps, DriveMode::Park=>0.0 };
        self.state.speed_mps=self.state.speed_mps.clamp(-self.spec.max_reverse_speed_mps,max_speed);
        if command.mode==DriveMode::Park || (command.brake>0.99 && speed_abs<0.1) { self.state.speed_mps=0.0; }
        let yaw=if self.spec.wheelbase_m>f32::EPSILON { self.state.speed_mps/self.spec.wheelbase_m*self.state.steering_rad.tan() } else { 0.0 };
        self.state.heading_rad=normalize_angle(self.state.heading_rad+yaw*dt);
        self.state.position[0]+=self.state.speed_mps*self.state.heading_rad.cos()*dt;
        self.state.position[1]+=self.state.speed_mps*self.state.heading_rad.sin()*dt;
    }
}

fn normalize_angle(mut angle:f32)->f32 {
    let pi=std::f32::consts::PI;
    while angle>pi { angle-=2.0*pi; }
    while angle< -pi { angle+=2.0*pi; }
    angle
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn spec_valid(){ assert!(VehicleSpec::default().validate()); }
    #[test] fn command_clamps(){ let c=VehicleCommand{throttle:2.0,brake:-1.0,steering:-3.0,mode:DriveMode::Drive}.clamped(); assert_eq!((c.throttle,c.brake,c.steering),(1.0,0.0,-1.0)); }
    #[test] fn accelerates(){ let mut s=VehicleSimulator::new(VehicleSpec::default()); s.step(VehicleCommand{throttle:1.0,mode:DriveMode::Drive,..Default::default()},0.1); assert!(s.state.speed_mps>0.0&&s.state.position[0]>0.0); }
    #[test] fn brakes(){ let mut s=VehicleSimulator::new(VehicleSpec::default()); s.state.speed_mps=10.0; for _ in 0..120 { s.step(VehicleCommand{brake:1.0,mode:DriveMode::Drive,..Default::default()},1.0/60.0); } assert!(s.state.speed_mps.abs()<0.2); }
    #[test] fn obstacle_braking(){ let a=VehicleAgent::default(); let c=a.decide(VehicleState{speed_mps:10.0,..Default::default()},VehiclePerception{waypoint:Some(Waypoint{position:[100.0,0.0],target_speed_mps:20.0}),obstacle_distance_m:Some(3.0),..Default::default()}); assert_eq!((c.throttle,c.brake),(0.0,1.0)); }
    #[test] fn deterministic(){ let a=VehicleAgent::default(); let p=VehiclePerception{waypoint:Some(Waypoint{position:[20.0,4.0],target_speed_mps:12.0}),speed_limit_mps:Some(10.0),..Default::default()}; let s=VehicleState::default(); assert_eq!(a.decide(s,p),a.decide(s,p)); }
    #[test] fn reverse(){ let mut s=VehicleSimulator::new(VehicleSpec::default()); s.step(VehicleCommand{throttle:1.0,mode:DriveMode::Reverse,..Default::default()},0.1); assert!(s.state.position[0]<0.0&&s.state.speed_mps<0.0); }
}
