use crate::{ChoiceId, StoryError, StoryResult, StoryRuntime};

/// Deterministic runtime command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReplayCommand {
    /// Enter a scene.
    EnterScene(u64),
    /// Choose a narrative edge.
    Choose(ChoiceId),
}

/// Replay log.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ReplayLog {
    /// Ordered commands.
    pub commands: Vec<ReplayCommand>,
}

impl ReplayLog {
    /// Appends a command.
    pub fn push(&mut self, command: ReplayCommand) {
        self.commands.push(command);
    }

    /// Replays commands and verifies the resulting fingerprint.
    pub fn replay(
        &self,
        runtime: &mut StoryRuntime,
        expected: u64,
    ) -> StoryResult<()> {
        for command in &self.commands {
            match command {
                ReplayCommand::EnterScene(id) => {
                    runtime.enter_scene(crate::SceneId::new(*id))?
                }
                ReplayCommand::Choose(choice) => {
                    runtime.choose(*choice)?;
                }
            }
        }
        let actual = state_fingerprint(runtime);
        if actual != expected {
            return Err(StoryError::ReplayMismatch { expected, actual });
        }
        Ok(())
    }
}

/// Stable, non-cryptographic deterministic state fingerprint.
pub fn state_fingerprint(runtime: &StoryRuntime) -> u64 {
    let mut h = 0xcbf29ce484222325_u64;

    fn byte(h: &mut u64, b: u8) {
        *h ^= b as u64;
        *h = h.wrapping_mul(0x100000001b3);
    }

    fn text(h: &mut u64, s: &str) {
        for b in s.as_bytes() {
            byte(h, b);
        }
        byte(h, 0xff);
    }

    fn u64v(h: &mut u64, v: u64) {
        for b in v.to_le_bytes() {
            byte(h, b);
        }
    }

    u64v(&mut h, runtime.state.tick);
    u64v(&mut h, runtime.state.current_scene.unwrap_or(0));

    for (id, c) in &runtime.state.characters {
        u64v(&mut h, id.value());
        text(&mut h, &c.name);
        byte(&mut h, c.alive as u8);
        u64v(&mut h, c.reputation as i64 as u64);
        for (other, rel) in &c.relationships {
            u64v(&mut h, other.value());
            u64v(&mut h, rel.0 as i64 as u64);
        }
    }

    for (k, v) in &runtime.state.flags {
        text(&mut h, k);
        byte(&mut h, *v as u8);
    }
    for (k, v) in &runtime.state.counters {
        text(&mut h, k);
        u64v(&mut h, *v as u64);
    }
    for id in &runtime.state.completed_quests {
        u64v(&mut h, *id);
    }
    for id in &runtime.state.failed_quests {
        u64v(&mut h, *id);
    }

    h
}
