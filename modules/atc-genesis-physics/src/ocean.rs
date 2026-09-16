use core::f32::consts::PI;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OceanConfig {
    pub resolution: usize,
    pub patch_size: f32,
    pub wind: [f32; 2],
    pub amplitude: f32,
    pub gravity: f32,
}
impl Default for OceanConfig { fn default() -> Self { Self { resolution: 32, patch_size: 512.0, wind: [20.0, 5.0], amplitude: 1.0, gravity: 9.81 } } }
#[derive(Clone, Copy, Debug, PartialEq, Eq)] pub enum OceanError { InvalidConfig, NonPowerOfTwo }
#[derive(Clone, Copy, Debug, Default, PartialEq)] struct Complex { re: f32, im: f32 }
impl Complex { fn new(re:f32,im:f32)->Self{Self{re,im}} fn add(self,o:Self)->Self{Self::new(self.re+o.re,self.im+o.im)} fn mul(self,o:Self)->Self{Self::new(self.re*o.re-self.im*o.im,self.re*o.im+self.im*o.re)} fn scale(self,s:f32)->Self{Self::new(self.re*s,self.im*s)} }

#[derive(Clone, Debug)] pub struct FftOcean { config: OceanConfig, spectrum: Vec<Complex>, heights: Vec<f32>, displacements: Vec<[f32;3]>, seed: u64 }
impl OceanConfig {
    pub fn validate(&self)->bool { self.resolution >= 2 && self.resolution.is_power_of_two() && self.patch_size.is_finite() && self.patch_size>0.0 && self.wind.iter().all(|v|v.is_finite()) && self.amplitude.is_finite() && self.amplitude>=0.0 && self.gravity.is_finite() && self.gravity>0.0 }
}
impl FftOcean {
    pub fn new(config:OceanConfig)->Result<Self,OceanError>{ if !config.validate(){return Err(if config.resolution>=2&&!config.resolution.is_power_of_two(){OceanError::NonPowerOfTwo}else{OceanError::InvalidConfig})} let n=config.resolution*config.resolution; Ok(Self{config,spectrum:vec![Complex::default();n],heights:vec![0.0;n],displacements:vec![[0.0;3];n],seed:0}) }
    pub fn config(&self)->OceanConfig{self.config}
    pub fn initialize(&mut self,seed:u64){ self.seed=seed; let n=self.config.resolution as isize; for z in 0..n { for x in 0..n { let kx=wave_number(x,n,self.config.patch_size); let kz=wave_number(z,n,self.config.patch_size); let p=phillips([kx,kz],self.config.wind,self.config.amplitude,self.config.gravity); let r=gaussian(seed,z as usize*n as usize+x as usize); let r2=gaussian(seed^0x9e3779b97f4a7c15,(z as usize*n as usize+x as usize).wrapping_add(1)); let i=z as usize*n as usize+x as usize; let a=(p*0.5).sqrt(); self.spectrum[i]=Complex::new(r*a,r2*a); } } }
    pub fn update(&mut self,time:f32)->Result<(),OceanError>{ if !time.is_finite(){return Err(OceanError::InvalidConfig)} let n=self.config.resolution; let mut h=vec![Complex::default();n*n]; for z in 0..n { for x in 0..n { let i=z*n+x; let mx=(n-x)%n; let mz=(n-z)%n; let kx=wave_number(x as isize,n as isize,self.config.patch_size); let kz=wave_number(z as isize,n as isize,self.config.patch_size); let k=(kx*kx+kz*kz).sqrt(); let w=(self.config.gravity*k).sqrt(); let phase=Complex::new((w*time).cos(),(w*time).sin()); let a=self.spectrum[i].mul(phase); let b=self.spectrum[mz*n+mx].mul(Complex::new((w*time).cos(),-(w*time).sin())); h[i]=a.add(b.scale(1.0)); } } fft2d(&mut h,n,true); for i in 0..n*n { self.heights[i]=h[i].re; } let inv_n=1.0/n as f32; for z in 0..n { for x in 0..n { let i=z*n+x; let kx=wave_number(x as isize,n as isize,self.config.patch_size); let kz=wave_number(z as isize,n as isize,self.config.patch_size); let k=(kx*kx+kz*kz).sqrt().max(1e-6); let q=self.config.amplitude.min(1.0)*0.35; self.displacements[i]=[-kx/k*self.heights[i]*q,0.0,-kz/k*self.heights[i]*q]; self.heights[i]*=inv_n; } } Ok(()) }
    pub fn height(&self,x:usize,z:usize)->Option<f32>{if x<self.config.resolution&&z<self.config.resolution{Some(self.heights[z*self.config.resolution+x])}else{None}}
    pub fn heights(&self)->&[f32]{&self.heights}
    pub fn displacements(&self)->&[[f32;3]]{&self.displacements}
}
fn wave_number(i:isize,n:isize,patch:f32)->f32{ let j=if i<=n/2{i}else{i-n}; 2.0*PI*j as f32/patch }
fn phillips(k:[f32;2],wind:[f32;2],a:f32,g:f32)->f32{let k2=k[0]*k[0]+k[1]*k[1];if k2<1e-12{return 0.0}let kw=(k[0]*wind[0]+k[1]*wind[1]).max(0.0);let w2=wind[0]*wind[0]+wind[1]*wind[1];let l=w2/g; a*(-1.0/(k2*l*l)).exp()*kw*kw/(k2*k2)*(-k2*0.001).exp()}
fn gaussian(seed:u64,i:usize)->f32{let a=splitmix(seed.wrapping_add(i as u64));let b=splitmix(seed.rotate_left(17).wrapping_add(i as u64).wrapping_add(1)).max(1e-7);(-2.0*b.ln()).sqrt()*(2.0*PI*a).cos()}
fn splitmix(mut x:u64)->f32{x=x.wrapping_add(0x9e3779b97f4a7c15);let mut z=x;z=(z^(z>>30)).wrapping_mul(0xbf58476d1ce4e5b9);z=(z^(z>>27)).wrapping_mul(0x94d049bb133111eb);z^=z>>31;(z as f64/u64::MAX as f64) as f32}
fn fft1d(a:&mut [Complex],inverse:bool){let n=a.len();let mut j=0;for i in 1..n{let mut bit=n>>1;while j&bit!=0{j^=bit;bit>>=1}j^=bit;if i<j{a.swap(i,j)}}let mut len=2;while len<=n{let ang=2.0*PI/len as f32*if inverse{-1.0}else{1.0};let wlen=Complex::new(ang.cos(),ang.sin());for i in (0..n).step_by(len){let mut w=Complex::new(1.0,0.0);for j in 0..len/2{let u=a[i+j];let v=a[i+j+len/2].mul(w);a[i+j]=u.add(v);a[i+j+len/2]=Complex::new(u.re-v.re,u.im-v.im);w=w.mul(wlen)}}len<<=1}if inverse{let s=1.0/n as f32;for v in a{*v=v.scale(s)}}}
fn fft2d(a:&mut [Complex],n:usize,inverse:bool){for z in 0..n{fft1d(&mut a[z*n..(z+1)*n],inverse)}let mut col=vec![Complex::default();n];for x in 0..n{for z in 0..n{col[z]=a[z*n+x]}fft1d(&mut col,inverse);for z in 0..n{a[z*n+x]=col[z]}}}
#[cfg(test)]mod tests{use super::*;#[test]fn deterministic(){let c=OceanConfig{resolution:8,..Default::default()};let mut a=FftOcean::new(c).unwrap();let mut b=FftOcean::new(c).unwrap();a.initialize(7);b.initialize(7);a.update(1.0).unwrap();b.update(1.0).unwrap();assert_eq!(a.heights,b.heights)}#[test]fn fft_roundtrip(){let mut a=(0..8).map(|x|Complex::new(x as f32,0.0)).collect::<Vec<_>>();let original=a.clone();fft1d(&mut a,false);fft1d(&mut a,true);for(i,v)in a.iter().enumerate(){assert!((v.re-original[i].re).abs()<1e-4)}}}