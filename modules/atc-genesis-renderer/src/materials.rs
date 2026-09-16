use atc_genesis_platform::AssetId;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PbrMaterial {
    pub base_color: [f32; 4],
    pub metallic: f32,
    pub roughness: f32,
    pub emissive: [f32; 3],
    pub normal_scale: f32,
}

impl Default for PbrMaterial {
    fn default() -> Self { Self { base_color: [1.0, 1.0, 1.0, 1.0], metallic: 0.0, roughness: 0.5, emissive: [0.0; 3], normal_scale: 1.0 } }
}

impl PbrMaterial {
    pub fn validate(&self) -> Result<(), MaterialError> {
        if !self.base_color.iter().chain(self.emissive.iter()).chain(std::iter::once(&self.metallic)).chain(std::iter::once(&self.roughness)).chain(std::iter::once(&self.normal_scale)).all(|v| v.is_finite()) { return Err(MaterialError::NonFinite); }
        if !(0.0..=1.0).contains(&self.metallic) { return Err(MaterialError::MetallicOutOfRange); }
        if !(0.0..=1.0).contains(&self.roughness) { return Err(MaterialError::RoughnessOutOfRange); }
        if self.normal_scale < 0.0 { return Err(MaterialError::NegativeNormalScale); }
        if self.base_color.iter().any(|v| *v < 0.0) { return Err(MaterialError::NegativeBaseColor); }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MaterialError { NonFinite, MetallicOutOfRange, RoughnessOutOfRange, NegativeNormalScale, NegativeBaseColor }

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct MaterialId(pub AssetId);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MaterialInstance { pub material: MaterialId }

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn default_material_validates() { assert!(PbrMaterial::default().validate().is_ok()); }
    #[test] fn rejects_invalid_roughness() { let m=PbrMaterial { roughness:1.1,..Default::default() }; assert_eq!(m.validate(),Err(MaterialError::RoughnessOutOfRange)); }
    #[test] fn rejects_non_finite_input() { let m=PbrMaterial { metallic:f32::NAN,..Default::default() }; assert_eq!(m.validate(),Err(MaterialError::NonFinite)); }
}
