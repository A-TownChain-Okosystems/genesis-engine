#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DirectionalLight {
    pub direction: [f32; 3],
    pub color: [f32; 3],
    pub intensity: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PointLight {
    pub position: [f32; 3],
    pub color: [f32; 3],
    pub intensity: f32,
    pub range: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SpotLight {
    pub position: [f32; 3],
    pub direction: [f32; 3],
    pub color: [f32; 3],
    pub intensity: f32,
    pub range: f32,
    pub inner_cos: f32,
    pub outer_cos: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct LightId(pub u128);

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LightKind {
    Directional(DirectionalLight),
    Point(PointLight),
    Spot(SpotLight),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Light { pub id: LightId, pub kind: LightKind }

#[derive(Default, Debug)]
pub struct LightRegistry { lights: Vec<Light> }

impl LightRegistry {
    pub fn insert(&mut self, light: Light) -> Result<(), LightingError> {
        validate_light(&light.kind)?;
        if light.id.0 == 0 { return Err(LightingError::InvalidId); }
        if self.lights.iter().any(|existing| existing.id == light.id) { return Err(LightingError::DuplicateId); }
        self.lights.push(light);
        self.lights.sort_by_key(|light| light.id);
        Ok(())
    }

    pub fn get(&self, id: LightId) -> Option<&Light> { self.lights.iter().find(|light| light.id == id) }
    pub fn iter(&self) -> impl Iterator<Item = &Light> { self.lights.iter() }
    pub fn len(&self) -> usize { self.lights.len() }
    pub fn is_empty(&self) -> bool { self.lights.is_empty() }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LightingError {
    InvalidId,
    DuplicateId,
    NonFinite,
    NonPositiveIntensity,
    NonPositiveRange,
    InvalidDirection,
    InvalidSpotAngles,
}

fn finite3(v: &[f32; 3]) -> bool { v.iter().all(|value| value.is_finite()) }
fn direction_valid(v: &[f32; 3]) -> bool { finite3(v) && v.iter().map(|x| x * x).sum::<f32>() > f32::EPSILON }

fn validate_light(kind: &LightKind) -> Result<(), LightingError> {
    match kind {
        LightKind::Directional(light) => {
            if !finite3(&light.color) || !light.intensity.is_finite() { return Err(LightingError::NonFinite); }
            if light.intensity < 0.0 { return Err(LightingError::NonPositiveIntensity); }
            if !direction_valid(&light.direction) { return Err(LightingError::InvalidDirection); }
        }
        LightKind::Point(light) => {
            if !finite3(&light.position) || !finite3(&light.color) || !light.intensity.is_finite() || !light.range.is_finite() { return Err(LightingError::NonFinite); }
            if light.intensity < 0.0 { return Err(LightingError::NonPositiveIntensity); }
            if light.range <= 0.0 { return Err(LightingError::NonPositiveRange); }
        }
        LightKind::Spot(light) => {
            if !finite3(&light.position) || !finite3(&light.direction) || !finite3(&light.color) || !light.intensity.is_finite() || !light.range.is_finite() || !light.inner_cos.is_finite() || !light.outer_cos.is_finite() { return Err(LightingError::NonFinite); }
            if light.intensity < 0.0 { return Err(LightingError::NonPositiveIntensity); }
            if light.range <= 0.0 { return Err(LightingError::NonPositiveRange); }
            if !direction_valid(&light.direction) { return Err(LightingError::InvalidDirection); }
            if !(-1.0..=1.0).contains(&light.outer_cos) || !(-1.0..=1.0).contains(&light.inner_cos) || light.inner_cos < light.outer_cos { return Err(LightingError::InvalidSpotAngles); }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    fn point(id: u128) -> Light { Light { id:LightId(id), kind:LightKind::Point(PointLight{position:[0.0;3],color:[1.0;3],intensity:1.0,range:10.0}) } }
    #[test] fn lights_are_sorted_by_id() { let mut r=LightRegistry::default();r.insert(point(3)).unwrap();r.insert(point(1)).unwrap();assert_eq!(r.iter().next().unwrap().id,LightId(1)); }
    #[test] fn duplicate_light_is_rejected() { let mut r=LightRegistry::default();r.insert(point(1)).unwrap();assert_eq!(r.insert(point(1)),Err(LightingError::DuplicateId)); }
    #[test] fn invalid_range_is_rejected() { let mut l=point(1);if let LightKind::Point(ref mut p)=l.kind{p.range=0.0;}let mut r=LightRegistry::default();assert_eq!(r.insert(l),Err(LightingError::NonPositiveRange)); }
}
