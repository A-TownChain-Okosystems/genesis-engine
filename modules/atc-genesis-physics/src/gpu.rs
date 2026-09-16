#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComputeBackend { Vulkan, Direct3D12, Metal }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ComputeDispatch { pub groups_x:u32, pub groups_y:u32, pub groups_z:u32 }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GpuFluidError { ZeroDispatch, UnsupportedDimension }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GpuFluidPipeline { pub backend:ComputeBackend, pub local_size:[u32;3] }
impl GpuFluidPipeline{pub fn new(backend:ComputeBackend,local_size:[u32;3])->Result<Self,GpuFluidError>{if local_size.iter().any(|v|*v==0){return Err(GpuFluidError::ZeroDispatch)}Ok(Self{backend,local_size})}pub fn dispatch_for(&self,width:u32,height:u32,depth:u32)->Result<ComputeDispatch,GpuFluidError>{if width==0||height==0||depth==0{return Err(GpuFluidError::UnsupportedDimension)}Ok(ComputeDispatch{groups_x:(width+self.local_size[0]-1)/self.local_size[0],groups_y:(height+self.local_size[1]-1)/self.local_size[1],groups_z:(depth+self.local_size[2]-1)/self.local_size[2]})}}
#[cfg(test)]mod tests{use super::*;#[test]fn deterministic_dispatch(){let p=GpuFluidPipeline::new(ComputeBackend::Vulkan,[8,8,4]).unwrap();assert_eq!(p.dispatch_for(17,16,9).unwrap(),ComputeDispatch{groups_x:3,groups_y:2,groups_z:3});}}