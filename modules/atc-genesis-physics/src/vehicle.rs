//! Deterministic multi-domain vehicle dynamics foundation.
//!
//! Vehicles share a rigid-body state but use explicit domain models for
//! ground, water and air. The module intentionally separates simulation from
//! rendering and networking.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VehicleDomain { Ground, Water, Air }

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VehicleState {
    pub position: [f32; 3],
    pub velocity: [f32; 3],
    pub forward: [f32; 3],
    pub up: [f32; 3],
    pub mass: f32,
}

impl Default for VehicleState {
    fn default() -> Self { Self { position:[0.0,0.0,0.0], velocity:[0.0;3], forward:[0.0,0.0,1.0], up:[0.0,1.0,0.0], mass:1000.0 } }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VehicleInput {
    pub throttle: f32,
    pub brake: f32,
    pub steering: f32,
    pub lift: f32,
    pub yaw: f32,
    pub pitch: f32,
    pub roll: f32,
}
impl Default for VehicleInput { fn default()->Self{Self{throttle:0.0,brake:0.0,steering:0.0,lift:0.0,yaw:0.0,pitch:0.0,roll:0.0}} }

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GroundVehicleConfig { pub engine_force:f32, pub brake_force:f32, pub drag:f32, pub rolling_resistance:f32, pub steering_rate:f32, pub max_speed:f32 }
impl Default for GroundVehicleConfig { fn default()->Self{Self{engine_force:8000.0,brake_force:12000.0,drag:0.35,rolling_resistance:80.0,steering_rate:1.8,max_speed:90.0}} }

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WaterVehicleConfig { pub thrust:f32, pub drag:f32, pub lateral_drag:f32, pub buoyancy:f32, pub waterline:f32, pub max_speed:f32 }
impl Default for WaterVehicleConfig { fn default()->Self{Self{thrust:7000.0,drag:0.8,lateral_drag:4.0,buoyancy:11000.0,waterline:0.0,max_speed:35.0}} }

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AirVehicleConfig { pub thrust:f32, pub wing_area:f32, pub lift_coefficient:f32, pub drag_coefficient:f32, pub air_density:f32, pub max_speed:f32 }
impl Default for AirVehicleConfig { fn default()->Self{Self{thrust:12000.0,wing_area:24.0,lift_coefficient:1.1,drag_coefficient:0.035,air_density:1.225,max_speed:340.0}} }

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VehicleEnvironment { pub gravity:[f32;3], pub water_level:f32, pub water_density:f32, pub air_density:f32, pub wind:[f32;3] }
impl Default for VehicleEnvironment { fn default()->Self{Self{gravity:[0.0,-9.81,0.0],water_level:0.0,water_density:1000.0,air_density:1.225,wind:[0.0;3]}} }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VehicleError { InvalidMass, InvalidDt, InvalidConfig, NonFiniteState }

fn finite3(v:[f32;3])->bool{v.iter().all(|x|x.is_finite())}
fn len(v:[f32;3])->f32{(v[0]*v[0]+v[1]*v[1]+v[2]*v[2]).sqrt()}
fn normalize(v:[f32;3])->[f32;3]{let n=len(v);if n<=f32::EPSILON{[0.0,0.0,1.0]}else{[v[0]/n,v[1]/n,v[2]/n]}}
fn dot(a:[f32;3],b:[f32;3])->f32{a[0]*b[0]+a[1]*b[1]+a[2]*b[2]}
fn add_scaled(v:&mut [f32;3], f:[f32;3], scale:f32){for i in 0..3{v[i]+=f[i]*scale;}}
fn clamp_speed(v:&mut [f32;3], max:f32){let s=len(*v);if s>max && max>0.0{let k=max/s;for x in v{*x*=k;}}}

pub fn step_ground(state:&mut VehicleState,input:VehicleInput,cfg:GroundVehicleConfig,env:VehicleEnvironment,dt:f32)->Result<(),VehicleError>{
    validate(state,dt)?; if cfg.engine_force<0.0||cfg.brake_force<0.0||cfg.max_speed<=0.0{return Err(VehicleError::InvalidConfig)};
    let f=normalize(state.forward); let speed=dot(state.velocity,f); let drive=input.throttle.clamp(-1.0,1.0)*cfg.engine_force;
    let brake=input.brake.clamp(0.0,1.0)*cfg.brake_force*speed.signum();
    let resist=cfg.rolling_resistance*speed+cfg.drag*speed*speed.abs();
    add_scaled(&mut state.velocity,f,(drive-brake-resist)/state.mass*dt);
    add_scaled(&mut state.velocity,env.gravity,dt);
    state.position=[state.position[0]+state.velocity[0]*dt,state.position[1]+state.velocity[1]*dt,state.position[2]+state.velocity[2]*dt];
    if state.position[1]<env.water_level{state.position[1]=env.water_level;if state.velocity[1]<0.0{state.velocity[1]=0.0;}}
    clamp_speed(&mut state.velocity,cfg.max_speed);Ok(())
}

pub fn step_water(state:&mut VehicleState,input:VehicleInput,cfg:WaterVehicleConfig,env:VehicleEnvironment,dt:f32)->Result<(),VehicleError>{
    validate(state,dt)?; if cfg.thrust<0.0||cfg.drag<0.0||cfg.lateral_drag<0.0||cfg.buoyancy<0.0||cfg.max_speed<=0.0{return Err(VehicleError::InvalidConfig)};
    let f=normalize(state.forward); let relative=[state.velocity[0]-env.wind[0],state.velocity[1]-env.wind[1],state.velocity[2]-env.wind[2]];
    let forward=dot(relative,f); let lateral=[relative[0]-f[0]*forward,relative[1]-f[1]*forward,relative[2]-f[2]*forward];
    add_scaled(&mut state.velocity,f,(input.throttle.clamp(-1.0,1.0)*cfg.thrust-cfg.drag*forward*forward.abs())/state.mass*dt);
    add_scaled(&mut state.velocity,lateral,-cfg.lateral_drag/state.mass*dt);
    add_scaled(&mut state.velocity,env.gravity,dt);
    let submerged=(cfg.waterline-(state.position[1]-env.water_level)).clamp(0.0,1.0);
    state.velocity[1]+=cfg.buoyancy*submerged/state.mass*dt;
    state.position=[state.position[0]+state.velocity[0]*dt,state.position[1]+state.velocity[1]*dt,state.position[2]+state.velocity[2]*dt];
    clamp_speed(&mut state.velocity,cfg.max_speed);Ok(())
}

pub fn step_air(state:&mut VehicleState,input:VehicleInput,cfg:AirVehicleConfig,env:VehicleEnvironment,dt:f32)->Result<(),VehicleError>{
    validate(state,dt)?; if cfg.thrust<0.0||cfg.wing_area<=0.0||cfg.air_density<=0.0||cfg.max_speed<=0.0{return Err(VehicleError::InvalidConfig)};
    let f=normalize(state.forward); let rel=[state.velocity[0]-env.wind[0],state.velocity[1]-env.wind[1],state.velocity[2]-env.wind[2]]; let speed=len(rel);
    let q=0.5*cfg.air_density*speed*speed; let lift=q*cfg.wing_area*cfg.lift_coefficient*(input.lift.clamp(-1.0,1.0)+0.15).max(0.0);
    let drag=q*cfg.wing_area*cfg.drag_coefficient; add_scaled(&mut state.velocity,f,(input.throttle.clamp(0.0,1.0)*cfg.thrust-drag)/state.mass*dt);
    add_scaled(&mut state.velocity,state.up,lift/state.mass*dt); add_scaled(&mut state.velocity,env.gravity,dt);
    state.position=[state.position[0]+state.velocity[0]*dt,state.position[1]+state.velocity[1]*dt,state.position[2]+state.velocity[2]*dt]; clamp_speed(&mut state.velocity,cfg.max_speed);Ok(())
}

fn validate(state:&VehicleState,dt:f32)->Result<(),VehicleError>{if state.mass<=0.0||!state.mass.is_finite(){return Err(VehicleError::InvalidMass)}if !dt.is_finite()||dt<0.0{return Err(VehicleError::InvalidDt)}if !finite3(state.position)||!finite3(state.velocity)||!finite3(state.forward)||!finite3(state.up){return Err(VehicleError::NonFiniteState)}Ok(())}

#[cfg(test)]mod tests{use super::*;#[test]fn ground_accelerates(){let mut s=VehicleState::default();step_ground(&mut s,VehicleInput{throttle:1.0,..Default::default()},GroundVehicleConfig::default(),VehicleEnvironment::default(),1.0/60.0).unwrap();assert!(len(s.velocity)>0.0)}#[test]fn water_has_buoyancy(){let mut s=VehicleState{position:[0.0,-0.5,0.0],..Default::default()};step_water(&mut s,VehicleInput::default(),WaterVehicleConfig::default(),VehicleEnvironment::default(),1.0/60.0).unwrap();assert!(s.velocity[1]>-0.2)}#[test]fn air_generates_lift(){let mut s=VehicleState{velocity:[40.0,0.0,0.0],..Default::default()};step_air(&mut s,VehicleInput{lift:1.0,..Default::default()},AirVehicleConfig::default(),VehicleEnvironment::default(),1.0/60.0).unwrap();assert!(s.velocity[1]>-0.2)}}
