//! Deterministic, fixed-point physics core.
//!
//! The simulation path in this module intentionally contains no floating-point
//! arithmetic. Floating-point conversion is only exposed at API boundaries.
//! This makes replay, lockstep and cross-platform verification possible.

use atc_genesis_platform::EntityId;

pub const FIXED_SCALE: i64 = 1_000_000;
pub const DEFAULT_FIXED_DT: Fixed = Fixed::from_raw(16_666);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Fixed(i64);

impl Fixed {
    pub const ZERO: Self = Self(0);
    pub const ONE: Self = Self(FIXED_SCALE);
    pub const fn from_raw(raw: i64) -> Self { Self(raw) }
    pub const fn raw(self) -> i64 { self.0 }

    pub fn from_ratio(numerator: i64, denominator: i64) -> Self {
        assert!(denominator != 0, "fixed-point denominator must not be zero");
        let value = (numerator as i128 * FIXED_SCALE as i128) / denominator as i128;
        Self(clamp_i128(value))
    }

    pub fn from_f32(value: f32) -> Self {
        if !value.is_finite() {
            return Self::ZERO;
        }
        let scaled = (value as f64 * FIXED_SCALE as f64).round();
        if scaled >= i64::MAX as f64 {
            Self(i64::MAX)
        } else if scaled <= i64::MIN as f64 {
            Self(i64::MIN)
        } else {
            Self(scaled as i64)
        }
    }

    pub fn to_f32(self) -> f32 {
        self.0 as f32 / FIXED_SCALE as f32
    }

    pub fn abs(self) -> Self {
        Self(self.0.saturating_abs())
    }

    pub fn min(self, other: Self) -> Self { Self(self.0.min(other.0)) }
    pub fn max(self, other: Self) -> Self { Self(self.0.max(other.0)) }

    pub fn checked_add(self, other: Self) -> Option<Self> {
        self.0.checked_add(other.0).map(Self)
    }

    pub fn saturating_add(self, other: Self) -> Self {
        Self(self.0.saturating_add(other.0))
    }

    pub fn saturating_sub(self, other: Self) -> Self {
        Self(self.0.saturating_sub(other.0))
    }

    pub fn mul(self, other: Self) -> Self {
        Self(clamp_i128(self.0 as i128 * other.0 as i128 / FIXED_SCALE as i128))
    }

    pub fn div(self, other: Self) -> Self {
        assert!(other.0 != 0, "fixed-point division by zero");
        Self(clamp_i128(self.0 as i128 * FIXED_SCALE as i128 / other.0 as i128))
    }
}

fn clamp_i128(value: i128) -> i64 {
    value.clamp(i64::MIN as i128, i64::MAX as i128) as i64
}

impl std::ops::Add for Fixed {
    type Output = Self;
    fn add(self, rhs: Self) -> Self { self.saturating_add(rhs) }
}
impl std::ops::Sub for Fixed {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self { self.saturating_sub(rhs) }
}
impl std::ops::Neg for Fixed {
    type Output = Self;
    fn neg(self) -> Self { Self(self.0.saturating_neg()) }
}
impl std::ops::Mul for Fixed {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self { self.mul(rhs) }
}
impl std::ops::Div for Fixed {
    type Output = Self;
    fn div(self, rhs: Self) -> Self { self.div(rhs) }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct FixedVec3 {
    pub x: Fixed,
    pub y: Fixed,
    pub z: Fixed,
}

impl FixedVec3 {
    pub const ZERO: Self = Self { x: Fixed::ZERO, y: Fixed::ZERO, z: Fixed::ZERO };

    pub const fn new(x: Fixed, y: Fixed, z: Fixed) -> Self { Self { x, y, z } }

    pub fn from_f32(v: [f32; 3]) -> Self {
        Self::new(Fixed::from_f32(v[0]), Fixed::from_f32(v[1]), Fixed::from_f32(v[2]))
    }

    pub fn to_f32(self) -> [f32; 3] {
        [self.x.to_f32(), self.y.to_f32(), self.z.to_f32()]
    }

    pub fn dot(self, rhs: Self) -> Fixed {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn scale(self, scalar: Fixed) -> Self {
        Self::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }

    pub fn div(self, scalar: Fixed) -> Self {
        Self::new(self.x / scalar, self.y / scalar, self.z / scalar)
    }

    pub fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }

    pub fn sub(self, rhs: Self) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl std::ops::Add for FixedVec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self { self.add(rhs) }
}
impl std::ops::Sub for FixedVec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self { self.sub(rhs) }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FixedAabb {
    pub min: FixedVec3,
    pub max: FixedVec3,
}

impl FixedAabb {
    pub fn contains(&self, point: FixedVec3) -> bool {
        point.x >= self.min.x && point.x <= self.max.x
            && point.y >= self.min.y && point.y <= self.max.y
            && point.z >= self.min.z && point.z <= self.max.z
    }

    pub fn point_resolution(&self, point: FixedVec3) -> Option<(FixedVec3, Fixed)> {
        if !self.contains(point) {
            return None;
        }
        let distances = [
            (point.x - self.min.x, FixedVec3::new(-Fixed::ONE, Fixed::ZERO, Fixed::ZERO)),
            (self.max.x - point.x, FixedVec3::new(Fixed::ONE, Fixed::ZERO, Fixed::ZERO)),
            (point.y - self.min.y, FixedVec3::new(Fixed::ZERO, -Fixed::ONE, Fixed::ZERO)),
            (self.max.y - point.y, FixedVec3::new(Fixed::ZERO, Fixed::ONE, Fixed::ZERO)),
            (point.z - self.min.z, FixedVec3::new(Fixed::ZERO, Fixed::ZERO, -Fixed::ONE)),
            (self.max.z - point.z, FixedVec3::new(Fixed::ZERO, Fixed::ZERO, Fixed::ONE)),
        ];
        let mut best = 0usize;
        for index in 1..distances.len() {
            if distances[index].0 < distances[best].0 {
                best = index;
            }
        }
        Some((distances[best].1, distances[best].0))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FixedCollider {
    pub entity: EntityId,
    pub bounds: FixedAabb,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FixedRigidBody {
    pub entity: EntityId,
    pub position: FixedVec3,
    pub velocity: FixedVec3,
    pub force: FixedVec3,
    pub mass: Fixed,
    pub dynamic: bool,
    pub restitution: Fixed,
}

impl FixedRigidBody {
    pub fn dynamic(entity: EntityId, position: FixedVec3, mass: Fixed) -> Self {
        assert!(mass > Fixed::ZERO, "dynamic body mass must be positive");
        Self {
            entity,
            position,
            velocity: FixedVec3::ZERO,
            force: FixedVec3::ZERO,
            mass,
            dynamic: true,
            restitution: Fixed::ZERO,
        }
    }

    pub fn clear_force(&mut self) {
        self.force = FixedVec3::ZERO;
    }

    pub fn apply_force(&mut self, force: FixedVec3) {
        self.force = self.force + force;
    }

    pub fn momentum(&self) -> FixedVec3 {
        self.velocity.scale(self.mass)
    }

    pub fn kinetic_energy(&self) -> Fixed {
        let speed_squared = self.velocity.dot(self.velocity);
        self.mass * speed_squared / Fixed::from_raw(2 * FIXED_SCALE)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FixedPhysicsConfig {
    pub fixed_dt: Fixed,
    pub gravity: FixedVec3,
    pub max_collision_iterations: u32,
}

impl Default for FixedPhysicsConfig {
    fn default() -> Self {
        Self {
            fixed_dt: DEFAULT_FIXED_DT,
            gravity: FixedVec3::new(
                Fixed::ZERO,
                Fixed::from_ratio(-981, 100),
                Fixed::ZERO,
            ),
            max_collision_iterations: 4,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DeterministicPhysics {
    pub config: FixedPhysicsConfig,
    bodies: Vec<FixedRigidBody>,
    colliders: Vec<FixedCollider>,
    tick: u64,
}

impl DeterministicPhysics {
    pub fn new(config: FixedPhysicsConfig) -> Self {
        assert!(config.fixed_dt > Fixed::ZERO, "fixed dt must be positive");
        Self { config, ..Self::default() }
    }

    pub fn tick(&self) -> u64 { self.tick }
    pub fn bodies(&self) -> &[FixedRigidBody] { &self.bodies }

    pub fn add_body(&mut self, body: FixedRigidBody) {
        self.bodies.push(body);
        self.bodies.sort_by_key(|body| body.entity.0);
    }

    pub fn remove_body(&mut self, entity: EntityId) -> bool {
        if let Some(index) = self.bodies.iter().position(|body| body.entity == entity) {
            self.bodies.remove(index);
            true
        } else {
            false
        }
    }

    pub fn add_collider(&mut self, collider: FixedCollider) {
        self.colliders.push(collider);
        self.colliders.sort_by_key(|collider| collider.entity.0);
    }

    pub fn remove_collider(&mut self, entity: EntityId) -> bool {
        if let Some(index) = self.colliders.iter().position(|c| c.entity == entity) {
            self.colliders.remove(index);
            true
        } else {
            false
        }
    }

    fn resolve_collisions(&self, body: &mut FixedRigidBody) {
        for _ in 0..self.config.max_collision_iterations {
            let Some((_, normal, penetration)) = self.colliders.iter()
                .filter_map(|collider| collider.bounds.point_resolution(body.position)
                    .map(|(normal, penetration)| (collider.entity, normal, penetration)))
                .min_by(|a, b| a.2.raw().cmp(&b.2.raw()).then_with(|| a.0.0.cmp(&b.0.0)))
            else { break };

            body.position = body.position + normal.scale(penetration + Fixed::from_raw(1));
            let inward = body.velocity.dot(normal);
            if inward < Fixed::ZERO {
                body.velocity = body.velocity - normal.scale(
                    (Fixed::ONE + body.restitution).mul(inward)
                );
            }
        }
    }

    pub fn step_fixed(&mut self) {
        let dt = self.config.fixed_dt;
        let gravity = self.config.gravity;
        let iterations = self.config.max_collision_iterations;
        for body in &mut self.bodies {
            if !body.dynamic { continue; }
            let acceleration = gravity + body.force.div(body.mass);
            body.velocity = body.velocity + acceleration.scale(dt);
            body.position = body.position + body.velocity.scale(dt);
            body.clear_force();

            // Deterministic collision resolution against static AABBs.
            for _ in 0..iterations {
                let collision = self.colliders.iter()
                    .filter_map(|collider| collider.bounds.point_resolution(body.position)
                        .map(|(normal, penetration)| (collider.entity, normal, penetration)))
                    .min_by(|a, b| a.2.raw().cmp(&b.2.raw()).then_with(|| a.0.0.cmp(&b.0.0)));
                let Some((_, normal, penetration)) = collision else { break };
                body.position = body.position + normal.scale(penetration + Fixed::from_raw(1));
                let inward = body.velocity.dot(normal);
                if inward < Fixed::ZERO {
                    body.velocity = body.velocity - normal.scale((Fixed::ONE + body.restitution).mul(inward));
                }
            }
        }
        self.tick = self.tick.saturating_add(1);
    }

    pub fn simulate_ticks(&self, ticks: u32) -> Vec<FixedRigidBody> {
        let mut clone = self.clone();
        for _ in 0..ticks { clone.step_fixed(); }
        clone.bodies
    }

    pub fn query_aabb(&self, bounds: FixedAabb) -> Vec<EntityId> {
        self.colliders.iter()
            .filter(|collider| {
                collider.bounds.min.x <= bounds.max.x && collider.bounds.max.x >= bounds.min.x
                    && collider.bounds.min.y <= bounds.max.y && collider.bounds.max.y >= bounds.min.y
                    && collider.bounds.min.z <= bounds.max.z && collider.bounds.max.z >= bounds.min.z
            })
            .map(|collider| collider.entity)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixed(value: i64) -> Fixed { Fixed::from_ratio(value, 1) }

    #[test]
    fn fixed_arithmetic_is_exact_for_common_values() {
        assert_eq!(fixed(2) * fixed(3), fixed(6));
        assert_eq!(fixed(9) / fixed(3), fixed(3));
        assert_eq!(Fixed::from_ratio(1, 2), Fixed::from_raw(500_000));
    }

    #[test]
    fn identical_inputs_produce_identical_state() {
        let config = FixedPhysicsConfig::default();
        let mut a = DeterministicPhysics::new(config);
        let mut b = DeterministicPhysics::new(config);
        let body = FixedRigidBody::dynamic(EntityId(7), FixedVec3::new(Fixed::ZERO, fixed(10), Fixed::ZERO), fixed(1));
        a.add_body(body.clone());
        b.add_body(body);
        for _ in 0..120 {
            a.step_fixed();
            b.step_fixed();
        }
        assert_eq!(a, b);
        assert_eq!(a.tick(), 120);
    }

    #[test]
    fn force_is_applied_and_cleared_each_tick() {
        let mut sim = DeterministicPhysics::new(FixedPhysicsConfig {
            gravity: FixedVec3::ZERO,
            ..Default::default()
        });
        let id = EntityId(1);
        sim.add_body(FixedRigidBody::dynamic(id, FixedVec3::ZERO, fixed(2)));
        sim.bodies[0].apply_force(FixedVec3::new(fixed(2), Fixed::ZERO, Fixed::ZERO));
        sim.step_fixed();
        assert!(sim.bodies[0].velocity.x > Fixed::ZERO);
        assert_eq!(sim.bodies[0].force, FixedVec3::ZERO);
    }

    #[test]
    fn collision_resolution_is_entity_deterministic() {
        let mut sim = DeterministicPhysics::new(FixedPhysicsConfig {
            gravity: FixedVec3::ZERO,
            ..Default::default()
        });
        let bounds = FixedAabb {
            min: FixedVec3::new(fixed(-1), fixed(-1), fixed(-1)),
            max: FixedVec3::new(fixed(1), fixed(1), fixed(1)),
        };
        sim.add_collider(FixedCollider { entity: EntityId(2), bounds });
        sim.add_collider(FixedCollider { entity: EntityId(1), bounds });
        sim.add_body(FixedRigidBody::dynamic(EntityId(10), FixedVec3::ZERO, fixed(1)));
        sim.step_fixed();
        assert_eq!(sim.bodies[0].position.x.abs(), fixed(1) + Fixed::from_raw(1));
    }

    #[test]
    fn prediction_does_not_mutate_source_state() {
        let mut sim = DeterministicPhysics::new(FixedPhysicsConfig::default());
        sim.add_body(FixedRigidBody::dynamic(EntityId(1), FixedVec3::ZERO, fixed(1)));
        let before = sim.clone();
        let predicted = sim.simulate_ticks(60);
        assert_ne!(predicted[0].position, FixedVec3::ZERO);
        assert_eq!(sim, before);
    }
}
