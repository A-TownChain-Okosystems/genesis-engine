#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FluidCell {
    pub velocity: [f32; 3],
    pub density: f32,
    pub pressure: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FluidConfig {
    pub width: usize,
    pub height: usize,
    pub depth: usize,
    pub cell_size: f32,
    pub density: f32,
    pub viscosity: f32,
    pub pressure_iterations: u32,
}

impl Default for FluidConfig {
    fn default() -> Self { Self { width: 16, height: 16, depth: 16, cell_size: 1.0, density: 1000.0, viscosity: 0.001, pressure_iterations: 8 } }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FluidError { InvalidConfig, InvalidDelta, InvalidCell, NonFinite }

#[derive(Clone, Debug)]
pub struct FluidVolume3d { config: FluidConfig, cells: Vec<FluidCell> }

impl FluidConfig {
    pub fn validate(&self) -> bool {
        self.width > 0 && self.height > 0 && self.depth > 0 && self.cell_size.is_finite() && self.cell_size > 0.0
            && self.density.is_finite() && self.density > 0.0 && self.viscosity.is_finite() && self.viscosity >= 0.0
            && self.pressure_iterations > 0
    }
}

impl FluidVolume3d {
    pub fn new(config: FluidConfig) -> Result<Self, FluidError> {
        if !config.validate() { return Err(FluidError::InvalidConfig); }
        let cell = FluidCell { velocity: [0.0; 3], density: config.density, pressure: 0.0 };
        Ok(Self { cells: vec![cell; config.width * config.height * config.depth], config })
    }
    fn index(&self, x: usize, y: usize, z: usize) -> Result<usize, FluidError> {
        if x >= self.config.width || y >= self.config.height || z >= self.config.depth { Err(FluidError::InvalidCell) }
        else { Ok((y * self.config.depth + z) * self.config.width + x) }
    }
    fn neighbors(&self, x: usize, y: usize, z: usize) -> Vec<(usize, usize, usize)> {
        let mut n = Vec::with_capacity(6);
        if x > 0 { n.push((x - 1, y, z)); } if x + 1 < self.config.width { n.push((x + 1, y, z)); }
        if y > 0 { n.push((x, y - 1, z)); } if y + 1 < self.config.height { n.push((x, y + 1, z)); }
        if z > 0 { n.push((x, y, z - 1)); } if z + 1 < self.config.depth { n.push((x, y, z + 1)); }
        n
    }
    pub fn config(&self) -> FluidConfig { self.config }
    pub fn cell(&self, x: usize, y: usize, z: usize) -> Result<FluidCell, FluidError> { Ok(self.cells[self.index(x, y, z)?]) }
    pub fn set_velocity(&mut self, x: usize, y: usize, z: usize, velocity: [f32; 3]) -> Result<(), FluidError> {
        if !velocity.iter().all(|v| v.is_finite()) { return Err(FluidError::NonFinite); }
        let i = self.index(x, y, z)?; self.cells[i].velocity = velocity; Ok(())
    }
    pub fn add_density(&mut self, x: usize, y: usize, z: usize, amount: f32) -> Result<(), FluidError> {
        if !amount.is_finite() { return Err(FluidError::NonFinite); }
        let i = self.index(x, y, z)?; self.cells[i].density = (self.cells[i].density + amount).max(0.0); Ok(())
    }
    pub fn step(&mut self, delta: f32, gravity: [f32; 3]) -> Result<(), FluidError> {
        if !delta.is_finite() || delta < 0.0 || !gravity.iter().all(|v| v.is_finite()) { return Err(FluidError::InvalidDelta); }
        let dt = delta.min(0.05); if dt == 0.0 { return Ok(()); }
        let old = self.cells.clone();
        for y in 0..self.config.height { for z in 0..self.config.depth { for x in 0..self.config.width {
            let i = (y * self.config.depth + z) * self.config.width + x;
            let mut v = old[i].velocity;
            for a in 0..3 { v[a] += gravity[a] * dt; }
            self.cells[i].velocity = v;
        }}}
        self.project_pressure(dt, &old);
        Ok(())
    }
    fn project_pressure(&mut self, dt: f32, old: &[FluidCell]) {
        let h = self.config.cell_size;
        for _ in 0..self.config.pressure_iterations {
            for y in 0..self.config.height { for z in 0..self.config.depth { for x in 0..self.config.width {
                let i = (y * self.config.depth + z) * self.config.width + x;
                let ns = self.neighbors(x, y, z);
                if ns.is_empty() { continue; }
                let mut divergence = 0.0;
                for &(nx, ny, nz) in &ns {
                    let ni = (ny * self.config.depth + nz) * self.config.width + nx;
                    let axis = if nx != x { 0 } else if ny != y { 1 } else { 2 };
                    let sign = if [nx, ny, nz][axis] > [x, y, z][axis] { 1.0 } else { -1.0 };
                    divergence += sign * self.cells[ni].velocity[axis] / h;
                }
                let target = divergence * self.config.density * h * h / ns.len() as f32;
                self.cells[i].pressure = old[i].pressure - target;
            }}}
        }
        let scale = dt / (self.config.density * h.max(f32::EPSILON));
        for y in 0..self.config.height { for z in 0..self.config.depth { for x in 0..self.config.width {
            let i = (y * self.config.depth + z) * self.config.width + x;
            let p = self.cells[i].pressure;
            let mut gradient = [0.0; 3];
            for &(nx, ny, nz) in &self.neighbors(x, y, z) {
                let ni = (ny * self.config.depth + nz) * self.config.width + nx;
                let dp = self.cells[ni].pressure - p;
                if nx != x { gradient[0] += dp / h; }
                if ny != y { gradient[1] += dp / h; }
                if nz != z { gradient[2] += dp / h; }
            }
            for a in 0..3 { self.cells[i].velocity[a] -= gradient[a] * scale; }
        }}}
    }
    pub fn cells(&self) -> &[FluidCell] { &self.cells }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn volume_has_three_dimensions() { let c=FluidConfig{width:2,height:3,depth:4,..Default::default()}; let f=FluidVolume3d::new(c).unwrap(); assert_eq!(f.cells().len(),24); }
    #[test] fn gravity_changes_velocity() { let mut f=FluidVolume3d::new(FluidConfig{width:2,height:2,depth:2,..Default::default()}).unwrap(); f.step(1.0/60.0,[0.0,-9.81,0.0]).unwrap(); assert!(f.cell(0,0,0).unwrap().velocity[1] < 0.0); }
    #[test] fn deterministic() { let c=FluidConfig{width:3,height:3,depth:3,..Default::default()}; let mut a=FluidVolume3d::new(c).unwrap(); let mut b=FluidVolume3d::new(c).unwrap(); a.set_velocity(1,1,1,[1.0,0.0,0.0]).unwrap(); b.set_velocity(1,1,1,[1.0,0.0,0.0]).unwrap(); a.step(1.0/60.0,[0.0,-9.81,0.0]).unwrap(); b.step(1.0/60.0,[0.0,-9.81,0.0]).unwrap(); assert_eq!(a.cells(),b.cells()); }
}