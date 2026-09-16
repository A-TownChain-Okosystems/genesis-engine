use atc_genesis_platform::AssetId;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VertexPositionNormalTangentUv {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub tangent: [f32; 4],
    pub uv: [f32; 2],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IndexFormat { U16, U32 }

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Bounds { pub min: [f32; 3], pub max: [f32; 3] }

impl Bounds {
    pub fn validate(&self) -> Result<(), GeometryError> {
        for i in 0..3 {
            if !self.min[i].is_finite() || !self.max[i].is_finite() { return Err(GeometryError::NonFinite); }
            if self.min[i] > self.max[i] { return Err(GeometryError::InvalidBounds); }
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct MeshGeometry {
    pub id: AssetId,
    pub vertices: Vec<VertexPositionNormalTangentUv>,
    pub indices: Vec<u32>,
    pub index_format: IndexFormat,
    pub bounds: Bounds,
}

impl MeshGeometry {
    pub fn validate(&self) -> Result<(), GeometryError> {
        if self.id.0 == 0 { return Err(GeometryError::InvalidAssetId); }
        self.bounds.validate()?;
        if self.vertices.is_empty() { return Err(GeometryError::EmptyVertices); }
        if self.indices.is_empty() { return Err(GeometryError::EmptyIndices); }
        if self.indices.iter().any(|&i| i as usize >= self.vertices.len()) { return Err(GeometryError::IndexOutOfBounds); }
        for v in &self.vertices {
            if v.position.iter().chain(v.normal.iter()).chain(v.tangent.iter()).chain(v.uv.iter()).any(|x| !x.is_finite()) { return Err(GeometryError::NonFinite); }
        }
        if matches!(self.index_format, IndexFormat::U16) && self.indices.iter().any(|&i| i > u16::MAX as u32) { return Err(GeometryError::IndexFormatOverflow); }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GeometryError { InvalidAssetId, EmptyVertices, EmptyIndices, IndexOutOfBounds, IndexFormatOverflow, InvalidBounds, NonFinite }

#[cfg(test)]
mod tests {
    use super::*;
    fn mesh() -> MeshGeometry {
        MeshGeometry { id:AssetId(1), vertices:vec![VertexPositionNormalTangentUv{position:[0.0,0.0,0.0],normal:[0.0,1.0,0.0],tangent:[1.0,0.0,0.0,1.0],uv:[0.0,0.0]}], indices:vec![0], index_format:IndexFormat::U16, bounds:Bounds{min:[0.0;3],max:[0.0;3]} }
    }
    #[test] fn valid_mesh_passes() { assert_eq!(mesh().validate(),Ok(())); }
    #[test] fn invalid_index_is_rejected() { let mut m=mesh();m.indices[0]=2;assert_eq!(m.validate(),Err(GeometryError::IndexOutOfBounds)); }
    #[test] fn inverted_bounds_are_rejected() { let mut m=mesh();m.bounds.min[0]=1.0;m.bounds.max[0]=0.0;assert_eq!(m.validate(),Err(GeometryError::InvalidBounds)); }
}
