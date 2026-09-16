#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WaterSample { pub height: f32, pub velocity: f32, pub foam: f32 }

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WaterCell3d { pub density: f32, pub pressure: f32, pub velocity: [f32; 3], pub volume_fraction: f32 }

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WaterConfig {
    pub width: usize,
    pub depth: usize,
    pub cell_size: f32,
    pub damping: f32,
    pub wave_speed: f32,
    pub height: usize,
    pub fluid_density: f32,
}

impl Default for WaterConfig {
    fn default() -> Self {
        Self { width: 64, depth: 64, cell_size: 1.0, damping: 0.04, wave_speed: 1.0, height: 16, fluid_density: 1000.0 }
    }
}

impl WaterConfig {
    pub fn validate(&self) -> bool {
        self.width > 0 && self.depth > 0 && self.height > 0
            && self.cell_size.is_finite() && self.cell_size > 0.0
            && self.damping.is_finite() && (0.0..=1.0).contains(&self.damping)
            && self.wave_speed.is_finite() && self.wave_speed >= 0.0
            && self.fluid_density.is_finite() && self.fluid_density > 0.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WaterError { InvalidConfig, InvalidDelta, InvalidCell, NonFiniteHeight, NonFiniteVelocity }

#[derive(Clone, Debug)]
pub struct WaterSurface { config: WaterConfig, samples: Vec<WaterSample> }

impl WaterSurface {
    pub fn new(config: WaterConfig) -> Result<Self, WaterError> {
        if !config.validate() { return Err(WaterError::InvalidConfig); }
        Ok(Self { samples: vec![WaterSample { height: 0.0, velocity: 0.0, foam: 0.0 }; config.width * config.depth], config })
    }
    pub fn config(&self) -> WaterConfig { self.config }
    fn index(&self, x: usize, z: usize) -> Result<usize, WaterError> {
        if x >= self.config.width || z >= self.config.depth { Err(WaterError::InvalidCell) } else { Ok(z * self.config.width + x) }
    }
    pub fn sample(&self, x: usize, z: usize) -> Result<WaterSample, WaterError> { Ok(self.samples[self.index(x, z)?]) }
    pub fn set_height(&mut self, x: usize, z: usize, height: f32) -> Result<(), WaterError> {
        if !height.is_finite() { return Err(WaterError::NonFiniteHeight); }
        let i = self.index(x, z)?; self.samples[i].height = height; Ok(())
    }
    pub fn disturb(&mut self, x: usize, z: usize, height_delta: f32) -> Result<(), WaterError> {
        if !height_delta.is_finite() { return Err(WaterError::NonFiniteHeight); }
        let i = self.index(x, z)?; self.samples[i].height += height_delta; Ok(())
    }
    pub fn step(&mut self, delta: f32) -> Result<(), WaterError> {
        if !delta.is_finite() || delta < 0.0 { return Err(WaterError::InvalidDelta); }
        if delta == 0.0 { return Ok(()); }
        let dt = delta.min(0.25);
        let old = self.samples.clone();
        let c = self.config.wave_speed * self.config.wave_speed;
        for z in 0..self.config.depth { for x in 0..self.config.width {
            let i = z * self.config.width + x; let center = old[i].height; let mut lap = 0.0; let mut count = 0.0;
            if x > 0 { lap += old[i - 1].height - center; count += 1.0; }
            if x + 1 < self.config.width { lap += old[i + 1].height - center; count += 1.0; }
            if z > 0 { lap += old[i - self.config.width].height - center; count += 1.0; }
            if z + 1 < self.config.depth { lap += old[i + self.config.width].height - center; count += 1.0; }
            if count > 0.0 { self.samples[i].velocity += (lap / count) * c * dt; }
            self.samples[i].velocity *= 1.0 - (self.config.damping * dt).min(1.0);
            self.samples[i].height += self.samples[i].velocity * dt;
            self.samples[i].foam = (self.samples[i].foam * 0.98 + self.samples[i].velocity.abs() * 0.02).clamp(0.0, 1.0);
        }}
        Ok(())
    }
    pub fn min_max_height(&self) -> (f32, f32) { self.samples.iter().fold((f32::INFINITY, f32::NEG_INFINITY), |(min, max), s| (min.min(s.height), max.max(s.height))) }
    pub fn samples(&self) -> &[WaterSample] { &self.samples }
}

#[derive(Clone, Debug)]
pub struct WaterVolume3d { config: WaterConfig, cells: Vec<WaterCell3d> }

impl WaterVolume3d {
    pub fn new(config: WaterConfig) -> Result<Self, WaterError> {
        if !config.validate() { return Err(WaterError::InvalidConfig); }
        let cell = WaterCell3d { density: config.fluid_density, pressure: 0.0, velocity: [0.0; 3], volume_fraction: 1.0 };
        Ok(Self { cells: vec![cell; config.width * config.height * config.depth], config })
    }
    pub fn config(&self) -> WaterConfig { self.config }
    fn index(&self, x: usize, y: usize, z: usize) -> Result<usize, WaterError> {
        if x >= self.config.width || y >= self.config.height || z >= self.config.depth { Err(WaterError::InvalidCell) } else { Ok((y * self.config.depth + z) * self.config.width + x) }
    }
    pub fn cell(&self, x: usize, y: usize, z: usize) -> Result<WaterCell3d, WaterError> { Ok(self.cells[self.index(x, y, z)?]) }
    pub fn set_velocity(&mut self, x: usize, y: usize, z: usize, velocity: [f32; 3]) -> Result<(), WaterError> {
        if !velocity.iter().all(|v| v.is_finite()) { return Err(WaterError::NonFiniteVelocity); }
        let i = self.index(x, y, z)?; self.cells[i].velocity = velocity; Ok(())
    }
    pub fn add_velocity(&mut self, x: usize, y: usize, z: usize, delta: [f32; 3]) -> Result<(), WaterError> {
        if !delta.iter().all(|v| v.is_finite()) { return Err(WaterError::NonFiniteVelocity); }
        let i = self.index(x, y, z)?;
        for axis in 0..3 { self.cells[i].velocity[axis] += delta[axis]; }
        Ok(())
    }
    pub fn step(&mut self, delta: f32, gravity: [f32; 3]) -> Result<(), WaterError> {
        if !delta.is_finite() || delta < 0.0 || !gravity.iter().all(|v| v.is_finite()) { return Err(WaterError::InvalidDelta); }
        let dt = delta.min(0.1);
        if dt == 0.0 { return Ok(()); }
        let old = self.cells.clone();
        for y in 0..self.config.height { for z in 0..self.config.depth { for x in 0..self.config.width {
            let i = (y * self.config.depth + z) * self.config.width + x;
            let mut v = old[i].velocity;
            for axis in 0..3 { v[axis] += gravity[axis] * dt; }
            self.cells[i].velocity = v;
        }}}
        Ok(())
    }
    pub fn world_position(&self, x: usize, y: usize, z: usize) -> Result<[f32; 3], WaterError> {
        self.index(x, y, z)?;
        Ok([x as f32 * self.config.cell_size, y as f32 * self.config.cell_size, z as f32 * self.config.cell_size])
    }
    pub fn cells(&self) -> &[WaterCell3d] { &self.cells }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WaterBody { pub surface_y: f32, pub density: f32, pub buoyancy_scale: f32 }
impl Default for WaterBody { fn default() -> Self { Self { surface_y: 0.0, density: 1000.0, buoyancy_scale: 1.0 } } }
impl WaterBody { pub fn buoyancy(&self, submerged_fraction: f32, gravity: f32) -> f32 { self.density * self.buoyancy_scale * submerged_fraction.clamp(0.0, 1.0) * gravity } }

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn disturbance_propagates() { let mut w=WaterSurface::new(WaterConfig{width:5,depth:5,..Default::default()}).unwrap(); w.disturb(2,2,1.0).unwrap(); let before=w.sample(2,2).unwrap().height; w.step(1.0/60.0).unwrap(); assert!(w.sample(2,2).unwrap().height<before); assert!(w.sample(1,2).unwrap().height!=0.0||w.sample(2,1).unwrap().height!=0.0); }
    #[test] fn deterministic_step() { let c=WaterConfig{width:4,depth:4,..Default::default()}; let mut a=WaterSurface::new(c).unwrap(); let mut b=WaterSurface::new(c).unwrap(); a.disturb(1,1,2.0).unwrap(); b.disturb(1,1,2.0).unwrap(); for _ in 0..10 { a.step(1.0/60.0).unwrap(); b.step(1.0/60.0).unwrap(); } assert_eq!(a.samples(),b.samples()); }
    #[test] fn volume_is_three_dimensional() { let c=WaterConfig{width:2,height:3,depth:4,..Default::default()}; let w=WaterVolume3d::new(c).unwrap(); assert_eq!(w.cells().len(),24); assert_eq!(w.world_position(1,2,3).unwrap(),[1.0,2.0,3.0]); }
    #[test] fn volume_gravity_is_deterministic() { let c=WaterConfig{width:2,height:2,depth:2,..Default::default()}; let mut a=WaterVolume3d::new(c).unwrap(); let mut b=WaterVolume3d::new(c).unwrap(); a.step(1.0/60.0,[0.0,-9.81,0.0]).unwrap(); b.step(1.0/60.0,[0.0,-9.81,0.0]).unwrap(); assert_eq!(a.cells(),b.cells()); assert!(a.cell(0,0,0).unwrap().velocity[1] < 0.0); }
    #[test] fn invalid_velocity_rejected() { let mut w=WaterVolume3d::new(WaterConfig::default()).unwrap(); assert_eq!(w.set_velocity(0,0,0,[f32::NAN,0.0,0.0]),Err(WaterError::NonFiniteVelocity)); }
    #[test] fn buoyancy_is_clamped() { let b=WaterBody::default(); assert_eq!(b.buoyancy(2.0,9.81),9810.0); }
}
