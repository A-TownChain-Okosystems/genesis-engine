#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OceanConfig { pub resolution: usize, pub patch_size: f32, pub wind: [f32; 2], pub amplitude: f32, pub gravity: f32 }
impl Default for OceanConfig { fn default()->Self{Self{resolution:32,patch_size:512.0,wind:[20.0,5.0],amplitude:1.0,gravity:9.81}} }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OceanError { InvalidConfig }
#[derive(Clone, Debug)]
pub struct FftOcean { config: OceanConfig, spectrum: Vec<[f32;2]>, heights: Vec<f32> }
impl OceanConfig { pub fn validate(&self)->bool{self.resolution>1&&self.patch_size.is_finite()&&self.patch_size>0.0&&self.wind.iter().all(|v|v.is_finite())&&self.amplitude.is_finite()&&self.amplitude>=0.0&&self.gravity.is_finite()&&self.gravity>0.0} }
impl FftOcean {
 pub fn new(config:OceanConfig)->Result<Self,OceanError>{if !config.validate(){return Err(OceanError::InvalidConfig)}let n=config.resolution*config.resolution;Ok(Self{config,spectrum:vec![[0.0;2];n],heights:vec![0.0;n]})}
 pub fn config(&self)->OceanConfig{self.config}
 pub fn initialize(&mut self,seed:u64){for z in 0..self.config.resolution{for x in 0..self.config.resolution{let i=z*self.config.resolution+x;let v=splitmix(seed.wrapping_add(i as u64));self.spectrum[i]=[(v*2.0-1.0)*self.config.amplitude,(splitmix(v.to_bits())*2.0-1.0)*self.config.amplitude];}}}
 pub fn update(&mut self,time:f32)->Result<(),OceanError>{if !time.is_finite(){return Err(OceanError::InvalidConfig)}let n=self.config.resolution;for z in 0..n{for x in 0..n{let i=z*n+x;let kx=(x as f32-n as f32*0.5)/self.config.patch_size*core::f32::consts::TAU;let kz=(z as f32-n as f32*0.5)/self.config.patch_size*core::f32::consts::TAU;let k=(kx*kx+kz*kz).sqrt();let phase=(self.config.gravity*k).sqrt()*time;let s=self.spectrum[i];self.heights[i]=s[0]*phase.cos()-s[1]*phase.sin();}}Ok(())}
 pub fn height(&self,x:usize,z:usize)->Option<f32>{if x<self.config.resolution&&z<self.config.resolution{Some(self.heights[z*self.config.resolution+x])}else{None}}
 pub fn heights(&self)->&[f32]{&self.heights}
}
fn splitmix(mut x:u64)->f32{x=x.wrapping_add(0x9e3779b97f4a7c15);let mut z=x;z=(z^(z>>30)).wrapping_mul(0xbf58476d1ce4e5b9);z=(z^(z>>27)).wrapping_mul(0x94d049bb133111eb);z^=z>>31;(z as f64/u64::MAX as f64) as f32}
#[cfg(test)]mod tests{use super::*;#[test]fn deterministic_initialization(){let c=OceanConfig::default();let mut a=FftOcean::new(c).unwrap();let mut b=FftOcean::new(c).unwrap();a.initialize(7);b.initialize(7);assert_eq!(a.spectrum,b.spectrum);}#[test]fn update_is_deterministic(){let mut a=FftOcean::new(OceanConfig::default()).unwrap();let mut b=FftOcean::new(OceanConfig::default()).unwrap();a.initialize(1);b.initialize(1);a.update(2.0).unwrap();b.update(2.0).unwrap();assert_eq!(a.heights,b.heights);}}