#[derive(Clone, Copy, Debug, PartialEq, Eq)] pub enum ComputeBackend { Vulkan, Direct3D12, Metal }
#[derive(Clone, Copy, Debug, PartialEq, Eq)] pub struct ComputeDispatch { pub groups_x:u32,pub groups_y:u32,pub groups_z:u32 }
#[derive(Clone, Copy, Debug, PartialEq, Eq)] pub enum GpuFluidError { ZeroDispatch, UnsupportedDimension, InvalidBinding }
#[derive(Clone, Copy, Debug, PartialEq, Eq)] pub struct ComputeBinding { pub slot:u32,pub read_only:bool }
#[derive(Clone, Copy, Debug, PartialEq, Eq)] pub struct GpuFluidPipeline { pub backend:ComputeBackend,pub local_size:[u32;3],pub bindings:[ComputeBinding;4] }
impl GpuFluidPipeline{pub fn new(backend:ComputeBackend,local_size:[u32;3])->Result<Self,GpuFluidError>{if local_size.iter().any(|v|*v==0){return Err(GpuFluidError::ZeroDispatch)}Ok(Self{backend,local_size,bindings:[ComputeBinding{slot:0,read_only:true},ComputeBinding{slot:1,read_only:false},ComputeBinding{slot:2,read_only:false},ComputeBinding{slot:3,read_only:false}]})}pub fn dispatch_for(&self,width:u32,height:u32,depth:u32)->Result<ComputeDispatch,GpuFluidError>{if width==0||height==0||depth==0{return Err(GpuFluidError::UnsupportedDimension)}Ok(ComputeDispatch{groups_x:width.div_ceil(self.local_size[0]),groups_y:height.div_ceil(self.local_size[1]),groups_z:depth.div_ceil(self.local_size[2])})}pub fn shader_entry(&self)->&'static str{match self.backend{ComputeBackend::Vulkan=>"main",ComputeBackend::Direct3D12=>"main",ComputeBackend::Metal=>"main"}}}

/// Backend-neutral kernel contract. Native backend layers must bind storage buffers in this order:
/// 0=input state, 1=output state, 2=obstacle/solid mask, 3=simulation constants.
pub const FLUID_COMPUTE_GLSL: &str = r#"#version 450
layout(local_size_x=8,local_size_y=8,local_size_z=4) in;
layout(set=0,binding=0,std430) readonly buffer Input { vec4 velocity_density[]; } input_state;
layout(set=0,binding=1,std430) buffer Output { vec4 velocity_density[]; } output_state;
layout(set=0,binding=2,std430) readonly buffer Solid { uint solid[]; } obstacle;
layout(set=0,binding=3,std140) uniform Params { uvec3 extent; float dt; vec3 gravity; float cell_size; } params;
void main(){uvec3 p=gl_GlobalInvocationID;if(any(greaterThanEqual(p,params.extent)))return;uint i=(p.z*params.extent.y+p.y)*params.extent.x+p.x;if(obstacle.solid[i]!=0u){output_state.velocity_density[i]=vec4(0.0);return;}vec4 s=input_state.velocity_density[i];s.xyz+=params.gravity*params.dt;output_state.velocity_density[i]=s;}"#;

/// HLSL source used by the Direct3D12 native backend.
pub const FLUID_COMPUTE_HLSL: &str = r#"RWStructuredBuffer<float4> Output : register(u0); StructuredBuffer<float4> Input : register(t0); StructuredBuffer<uint> Solid : register(t1); cbuffer Params : register(b0) { uint3 extent; float dt; float3 gravity; float cell_size; }; [numthreads(8,8,4)] void main(uint3 p : SV_DispatchThreadID) { if(any(p>=extent)) return; uint i=(p.z*extent.y+p.y)*extent.x+p.x; if(Solid[i]!=0) { Output[i]=0; return; } float4 s=Input[i]; s.xyz += gravity*dt; Output[i]=s; }"#;

/// Metal source used by the Metal native backend.
pub const FLUID_COMPUTE_METAL: &str = r#"#include <metal_stdlib>
using namespace metal;
struct Params { uint3 extent; float dt; float3 gravity; float cell_size; };
kernel void fluid_step(device const float4* input [[buffer(0)]], device float4* output [[buffer(1)]], device const uint* solid [[buffer(2)]], constant Params& p [[buffer(3)]], uint3 id [[thread_position_in_grid]]) { if(any(id>=p.extent)) return; uint i=(id.z*p.extent.y+id.y)*p.extent.x+id.x; if(solid[i]!=0){output[i]=float4(0);return;} float4 s=input[i]; s.xyz += p.gravity*p.dt; output[i]=s; }"#;

pub fn shader_source(backend:ComputeBackend)->&'static str{match backend{ComputeBackend::Vulkan=>FLUID_COMPUTE_GLSL,ComputeBackend::Direct3D12=>FLUID_COMPUTE_HLSL,ComputeBackend::Metal=>FLUID_COMPUTE_METAL}}
#[cfg(test)]mod tests{use super::*;#[test]fn dispatch(){let p=GpuFluidPipeline::new(ComputeBackend::Vulkan,[8,8,4]).unwrap();assert_eq!(p.dispatch_for(17,16,9).unwrap(),ComputeDispatch{groups_x:3,groups_y:2,groups_z:3});}#[test]fn native_sources_exist(){assert!(shader_source(ComputeBackend::Vulkan).contains("gl_GlobalInvocationID"));assert!(shader_source(ComputeBackend::Direct3D12).contains("SV_DispatchThreadID"));assert!(shader_source(ComputeBackend::Metal).contains("thread_position_in_grid"));}}