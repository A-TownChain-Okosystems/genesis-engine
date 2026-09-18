pub mod lifecycle;

use atc_genesis_animation::{AnimationClip, AnimationClipId, AnimationPlayer, BoneId};
use atc_genesis_ecs::{World, WorldEcsBridge};
use atc_genesis_gameplay::{GameplayRuntime, MovementConfig};
use atc_genesis_input::{InputEvent, InputState};
use atc_genesis_network::security::{PacketGuard, PacketRejectReason, SessionId};
use atc_genesis_network::{
    NetworkEntity, ReplicatedState, ReplicationHeader, ReplicationTransport, Replicator, Tick,
};
use atc_genesis_physics::{PhysicsConfig, PhysicsSimulation, WorldPhysicsBridge};
use atc_genesis_platform::{AudioRuntime, EntityId, FrameId, Renderer, Transform};
use atc_genesis_world::{WorldChunk, WorldChunkId, WorldStreamer};
use std::collections::HashMap;

pub struct GenesisRuntime<
    R,
    A = atc_genesis_audio::NullAudioRuntime,
    N = atc_genesis_network::LoopbackTransport,
> {
    pub world: WorldStreamer,
    pub ecs: World,
    pub physics: PhysicsSimulation,
    pub renderer: R,
    pub audio: A,
    pub input: InputState,
    pub gameplay: GameplayRuntime,
    pub network: Replicator<N>,
    network_tick: Tick,
    animations: HashMap<EntityId, AnimationBinding>,
    clips: HashMap<AnimationClipId, AnimationClip>,
    ecs_bridge: WorldEcsBridge,
    physics_bridge: WorldPhysicsBridge,
    frame: u64,
}

struct AnimationBinding {
    player: AnimationPlayer,
    clip: AnimationClipId,
    last_root_translation: [f32; 3],
}

impl<R> GenesisRuntime<R> {
    pub fn new(renderer: R, physics_config: PhysicsConfig) -> Self {
        Self::with_audio_and_network(
            renderer,
            atc_genesis_audio::NullAudioRuntime::default(),
            Replicator::new(atc_genesis_network::LoopbackTransport::default()),
            physics_config,
        )
    }
}

impl<R, A, N: ReplicationTransport> GenesisRuntime<R, A, N> {
    pub fn with_audio_and_network(
        renderer: R,
        audio: A,
        network: Replicator<N>,
        physics_config: PhysicsConfig,
    ) -> Self {
        Self {
            world: WorldStreamer::default(),
            ecs: World::new(),
            physics: PhysicsSimulation::new(physics_config),
            renderer,
            audio,
            input: InputState::default(),
            gameplay: GameplayRuntime::default(),
            network,
            network_tick: Tick(0),
            animations: HashMap::new(),
            clips: HashMap::new(),
            ecs_bridge: WorldEcsBridge::new(),
            physics_bridge: WorldPhysicsBridge::new(),
            frame: 0,
        }
    }

    pub fn frame(&self) -> u64 {
        self.frame
    }
    pub fn network_tick(&self) -> Tick {
        self.network_tick
    }
    pub fn add_chunk(&mut self, chunk: WorldChunk) -> Result<(), &'static str> {
        self.world.add_chunk(chunk)
    }
    pub fn spawn(&mut self, transform: Transform) -> EntityId {
        self.ecs.spawn(transform)
    }
    pub fn chunk_entity(&self, id: WorldChunkId) -> Option<EntityId> {
        self.ecs_bridge.entity_for_chunk(id)
    }
    pub fn stream(
        &mut self,
        position: [f32; 3],
        load_distance: f32,
        unload_distance: f32,
        budget: usize,
    ) -> Vec<WorldChunkId> {
        self.world
            .update(position, load_distance, unload_distance, budget)
    }
    pub fn input_event(&mut self, event: InputEvent) {
        self.input.apply(event)
    }
    pub fn queue_movement(
        &mut self,
        entity: EntityId,
        config: MovementConfig,
    ) -> atc_genesis_gameplay::GameplayCommand {
        self.gameplay.sample_movement(&self.input, entity, config)
    }
    pub fn update_gameplay(&mut self, dt: f32) -> usize {
        self.gameplay.apply(&mut self.ecs, dt)
    }
    pub fn register_animation_clip(&mut self, clip: AnimationClip) -> Option<AnimationClip> {
        self.clips.insert(clip.id, clip)
    }

    pub fn play_animation(
        &mut self,
        entity: EntityId,
        clip: AnimationClipId,
        looping: bool,
        speed: f32,
    ) -> bool {
        if self.ecs.transform(entity).is_none() || !self.clips.contains_key(&clip) {
            return false;
        }
        self.animations.insert(
            entity,
            AnimationBinding {
                player: AnimationPlayer {
                    clip: Some(clip),
                    time_seconds: 0.0,
                    speed,
                    looping,
                },
                clip,
                last_root_translation: [0.0; 3],
            },
        );
        true
    }

    pub fn stop_animation(&mut self, entity: EntityId) -> bool {
        self.animations.remove(&entity).is_some()
    }

    pub fn update_animations(&mut self, dt: f32) -> usize {
        let ids: Vec<_> = self.animations.keys().copied().collect();
        let mut updated = 0;
        for entity in ids {
            let Some(binding) = self.animations.get_mut(&entity) else {
                continue;
            };
            let Some(clip) = self.clips.get(&binding.clip) else {
                continue;
            };
            binding.player.update(dt, clip.duration_seconds);
            let Some((_, pose)) = binding
                .player
                .sample(clip)
                .into_iter()
                .find(|(bone, _)| *bone == BoneId(0))
            else {
                continue;
            };
            let delta = [
                pose.translation[0] - binding.last_root_translation[0],
                pose.translation[1] - binding.last_root_translation[1],
                pose.translation[2] - binding.last_root_translation[2],
            ];
            binding.last_root_translation = pose.translation;
            let Some(mut current) = self.ecs.transform(entity).copied() else {
                continue;
            };
            current.translation[0] += delta[0];
            current.translation[1] += delta[1];
            current.translation[2] += delta[2];
            current.rotation_xyzw = pose.rotation_xyzw;
            current.scale = pose.scale;
            if self.ecs.set_transform(entity, current) {
                updated += 1;
            }
        }
        updated
    }

    pub fn publish_entity(&mut self, entity: EntityId) -> Result<u64, String>
    where
        N: ReplicationTransport,
    {
        let transform = self
            .ecs
            .transform(entity)
            .ok_or_else(|| format!("entity {} does not exist", entity.0))?;
        let position_mm = quantize_mm(transform.translation)?;
        let rotation_xyz_microunits = quantize_rotation(transform.rotation_xyzw)?;
        self.network.publish(&ReplicatedState {
            entity: NetworkEntity(entity.0),
            tick: self.network_tick,
            position_mm,
            rotation_xyz_microunits,
        })
    }

    pub fn apply_network_state(&mut self, state: ReplicatedState) -> Result<(), NetworkApplyError> {
        let entity = EntityId(state.entity.0);
        if self.ecs.transform(entity).is_none() {
            return Err(NetworkApplyError::UnknownEntity(entity));
        }
        if !state.has_valid_rotation() {
            return Err(NetworkApplyError::InvalidRotation);
        }
        let current = self
            .ecs
            .transform(entity)
            .copied()
            .ok_or(NetworkApplyError::UnknownEntity(entity))?;
        let mut transform = current;
        transform.translation = dequantize_mm(state.position_mm);
        transform.rotation_xyzw = dequantize_rotation(state.rotation_xyz_microunits);
        if !transform.rotation_xyzw.iter().all(|v| v.is_finite()) {
            return Err(NetworkApplyError::InvalidRotation);
        }
        self.ecs.set_transform(entity, transform);
        Ok(())
    }

    pub fn receive_network_packet(&mut self, bytes: &[u8]) -> Result<(), NetworkReceiveError>
    where
        N: ReplicationTransport,
    {
        let state = self
            .network
            .accept_packet(bytes)
            .ok_or(NetworkReceiveError::Rejected)?;
        self.apply_network_state(state)
            .map_err(NetworkReceiveError::Apply)
    }

    pub fn receive_secure_network_packet(
        &mut self,
        guard: &mut PacketGuard,
        session: SessionId,
        bytes: &[u8],
    ) -> Result<(), SecureNetworkReceiveError> {
        let validated = guard
            .validate(session, bytes)
            .map_err(SecureNetworkReceiveError::Rejected)?;
        self.apply_network_state(validated.state)
            .map_err(SecureNetworkReceiveError::Apply)
    }

    pub fn receive_secure_network_packet_for_entity(
        &mut self,
        guard: &mut PacketGuard,
        session: SessionId,
        expected: NetworkEntity,
        bytes: &[u8],
    ) -> Result<(), SecureNetworkReceiveError> {
        let validated = guard
            .validate_for_entity(session, expected, bytes)
            .map_err(|reason| match reason {
                PacketRejectReason::EntityMismatch => SecureNetworkReceiveError::EntityMismatch {
                    expected,
                    received: ReplicatedState::decode(&bytes[ReplicationHeader::BYTES..])
                        .map(|s| s.entity)
                        .unwrap_or(expected),
                },
                other => SecureNetworkReceiveError::Rejected(other),
            })?;
        self.apply_network_state(validated.state)
            .map_err(SecureNetworkReceiveError::Apply)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NetworkApplyError {
    UnknownEntity(EntityId),
    InvalidRotation,
}

#[derive(Debug, PartialEq, Eq)]
pub enum NetworkReceiveError {
    Rejected,
    Apply(NetworkApplyError),
}

#[derive(Debug, PartialEq, Eq)]
pub enum SecureNetworkReceiveError {
    Rejected(PacketRejectReason),
    EntityMismatch {
        expected: NetworkEntity,
        received: NetworkEntity,
    },
    Apply(NetworkApplyError),
}

impl<R: Renderer, A: AudioRuntime, N: ReplicationTransport> GenesisRuntime<R, A, N> {
    pub fn tick(&mut self, dt: f32) -> u32 {
        self.update_animations(dt);
        self.update_gameplay(dt);
        self.ecs_bridge.sync(&self.world, &mut self.ecs);
        self.physics_bridge.sync(&self.world, &mut self.physics);
        let steps = self.physics.advance(dt);
        self.physics.sync_to_world(&mut self.ecs);
        self.audio.update(dt.max(0.0));
        let frame = FrameId(self.frame);
        self.renderer.begin_frame(frame);
        for (entity, transform) in self.ecs.query_world_transforms() {
            self.renderer.submit(entity, transform);
        }
        self.renderer.end_frame();
        self.network_tick = Tick(self.network_tick.0.saturating_add(1));
        self.frame = self.frame.saturating_add(1);
        steps
    }
}

fn quantize_mm(v: [f32; 3]) -> Result<[i32; 3], String> {
    if !v.iter().all(|value| value.is_finite()) {
        return Err("entity position contains a non-finite value".into());
    }
    Ok(v.map(|x| {
        (x.clamp(i32::MIN as f32 / 1000.0, i32::MAX as f32 / 1000.0) * 1000.0).round() as i32
    }))
}

fn dequantize_mm(v: [i32; 3]) -> [f32; 3] {
    v.map(|x| x as f32 * 0.001)
}

fn quantize_rotation(v: [f32; 4]) -> Result<[i32; 3], String> {
    if !v.iter().all(|value| value.is_finite()) {
        return Err("entity rotation contains a non-finite value".into());
    }
    let norm = v.iter().map(|value| value * value).sum::<f32>();
    if !(norm.is_finite() && norm > 0.0) {
        return Err("entity rotation is not a valid quaternion".into());
    }
    let normalized = v.map(|value| value / norm.sqrt());
    Ok([normalized[0], normalized[1], normalized[2]]
        .map(|x| (x.clamp(-1.0, 1.0) * 1_000_000.0).round() as i32))
}

fn dequantize_rotation(v: [i32; 3]) -> [f32; 4] {
    let mut q = [
        v[0] as f32 / 1_000_000.0,
        v[1] as f32 / 1_000_000.0,
        v[2] as f32 / 1_000_000.0,
        0.0,
    ];
    let w2 = (1.0 - q[0] * q[0] - q[1] * q[1] - q[2] * q[2]).max(0.0);
    q[3] = w2.sqrt();
    q
}

#[cfg(test)]
mod tests {
    use super::*;
    use atc_genesis_audio::NullAudioRuntime;
    use atc_genesis_input::{InputEvent, Key};
    use atc_genesis_network::security::{PacketGuard, PeerId};
    use atc_genesis_platform::EntityId;
    use atc_genesis_renderer::NullRenderer;

    fn runtime() -> GenesisRuntime<NullRenderer> {
        GenesisRuntime::new(
            NullRenderer::default(),
            PhysicsConfig {
                gravity: [0.0; 3],
                ..Default::default()
            },
        )
    }

    #[test]
    fn network_state_is_applied_to_existing_entity() {
        let mut r = runtime();
        let e = r.spawn(Transform::default());
        let s = ReplicatedState {
            entity: NetworkEntity(e.0),
            tick: Tick(1),
            position_mm: [1500, -2000, 3500],
            rotation_xyz_microunits: [0, 0, 0],
        };
        assert!(r.apply_network_state(s).is_ok());
        assert_eq!(r.ecs.transform(e).unwrap().translation, [1.5, -2.0, 3.5]);
    }

    #[test]
    fn invalid_network_rotation_is_rejected_without_mutation() {
        let mut r = runtime();
        let e = r.spawn(Transform::default());
        let before = *r.ecs.transform(e).unwrap();
        let s = ReplicatedState {
            entity: NetworkEntity(e.0),
            tick: Tick(1),
            position_mm: [9000, 0, 0],
            rotation_xyz_microunits: [1_000_001, 0, 0],
        };
        assert_eq!(
            r.apply_network_state(s),
            Err(NetworkApplyError::InvalidRotation)
        );
        assert_eq!(*r.ecs.transform(e).unwrap(), before);
    }

    #[test]
    fn publish_rejects_non_finite_transform() {
        let mut r = runtime();
        let e = r.spawn(Transform {
            translation: [f32::NAN, 0.0, 0.0],
            ..Default::default()
        });
        assert!(r.publish_entity(e).is_err());
    }

    #[test]
    fn publish_normalizes_rotation_before_quantization() {
        let mut r = runtime();
        let e = r.spawn(Transform {
            rotation_xyzw: [0.0, 0.0, 0.0, 2.0],
            ..Default::default()
        });
        r.publish_entity(e).unwrap();
        let packet = r.network.transport.drain().pop().unwrap();
        let (_, state) =
            Replicator::<atc_genesis_network::LoopbackTransport>::decode_packet(&packet).unwrap();
        assert_eq!(state.rotation_xyz_microunits, [0, 0, 0]);
    }

    #[test]
    fn unknown_network_entity_is_rejected() {
        let mut r = runtime();
        let e = EntityId(77);
        let s = ReplicatedState {
            entity: NetworkEntity(e.0),
            tick: Tick(1),
            position_mm: [0; 3],
            rotation_xyz_microunits: [0; 3],
        };
        assert_eq!(
            r.apply_network_state(s),
            Err(NetworkApplyError::UnknownEntity(e))
        );
    }

    #[test]
    fn receive_rejects_replay() {
        let mut r = runtime();
        let e = r.spawn(Transform::default());
        let s = ReplicatedState {
            entity: NetworkEntity(e.0),
            tick: Tick(1),
            position_mm: [1000, 0, 0],
            rotation_xyz_microunits: [0; 3],
        };
        r.network.publish(&s).unwrap();
        let p = r.network.transport.drain().pop().unwrap();
        assert!(r.receive_network_packet(&p).is_ok());
        assert_eq!(
            r.receive_network_packet(&p),
            Err(NetworkReceiveError::Rejected)
        );
    }

    #[test]
    fn secure_receive_enforces_session_replay_and_entity() {
        let mut r = runtime();
        let e = r.spawn(Transform::default());
        let s = ReplicatedState {
            entity: NetworkEntity(e.0),
            tick: Tick(2),
            position_mm: [2000, 0, 0],
            rotation_xyz_microunits: [0; 3],
        };
        r.network.publish(&s).unwrap();
        let p = r.network.transport.drain().pop().unwrap();
        let mut guard = PacketGuard::new(PeerId(5), SessionId(9));
        assert_eq!(
            r.receive_secure_network_packet_for_entity(
                &mut guard,
                SessionId(8),
                NetworkEntity(e.0),
                &p
            ),
            Err(SecureNetworkReceiveError::Rejected(
                PacketRejectReason::SessionMismatch
            ))
        );
        assert_eq!(
            r.receive_secure_network_packet_for_entity(
                &mut guard,
                SessionId(9),
                NetworkEntity(99),
                &p
            ),
            Err(SecureNetworkReceiveError::EntityMismatch {
                expected: NetworkEntity(99),
                received: NetworkEntity(e.0)
            })
        );
        assert!(r
            .receive_secure_network_packet_for_entity(
                &mut guard,
                SessionId(9),
                NetworkEntity(e.0),
                &p
            )
            .is_ok());
        assert_eq!(
            r.receive_secure_network_packet_for_entity(
                &mut guard,
                SessionId(9),
                NetworkEntity(e.0),
                &p
            ),
            Err(SecureNetworkReceiveError::Rejected(
                PacketRejectReason::Replay
            ))
        );
    }

    #[test]
    fn gameplay_pipeline_remains_active() {
        let mut r = runtime();
        let e = r.spawn(Transform::default());
        r.input_event(InputEvent::KeyPressed(Key::Right));
        r.queue_movement(
            e,
            MovementConfig {
                speed: 2.0,
                ..Default::default()
            },
        );
        assert_eq!(r.tick(0.5), 0);
        assert_eq!(r.ecs.transform(e).unwrap().translation[0], 1.0);
    }

    #[test]
    fn explicit_audio_constructor_still_works() {
        let r = GenesisRuntime::with_audio_and_network(
            NullRenderer::default(),
            NullAudioRuntime::default(),
            Replicator::new(atc_genesis_network::LoopbackTransport::default()),
            PhysicsConfig::default(),
        );
        assert_eq!(r.frame(), 0);
    }
}
