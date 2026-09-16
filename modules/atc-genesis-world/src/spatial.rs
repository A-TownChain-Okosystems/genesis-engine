//! Deterministic spatial partition primitives for world queries.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CellCoord {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bounds {
    pub min: Vec3,
    pub max: Vec3,
}

impl Bounds {
    pub fn contains(&self, point: Vec3) -> bool {
        point.x >= self.min.x && point.x <= self.max.x
            && point.y >= self.min.y && point.y <= self.max.y
            && point.z >= self.min.z && point.z <= self.max.z
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SpatialGrid {
    pub cell_size: f32,
}

impl SpatialGrid {
    pub fn new(cell_size: f32) -> Option<Self> {
        if cell_size.is_finite() && cell_size > 0.0 {
            Some(Self { cell_size })
        } else {
            None
        }
    }

    pub fn cell_for(&self, point: Vec3) -> Option<CellCoord> {
        if [point.x, point.y, point.z].iter().all(|v| v.is_finite()) {
            Some(CellCoord {
                x: (point.x / self.cell_size).floor() as i32,
                y: (point.y / self.cell_size).floor() as i32,
                z: (point.z / self.cell_size).floor() as i32,
            })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_positive_and_negative_coordinates() {
        let grid = SpatialGrid::new(10.0).unwrap();
        assert_eq!(grid.cell_for(Vec3 { x: 15.0, y: -1.0, z: 20.0 }), Some(CellCoord { x: 1, y: -1, z: 2 }));
    }

    #[test]
    fn rejects_invalid_cell_size() {
        assert!(SpatialGrid::new(0.0).is_none());
        assert!(SpatialGrid::new(f32::NAN).is_none());
    }
}
