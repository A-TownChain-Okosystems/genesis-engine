//! Genre-neutral gameplay capability registry.
//! The engine provides reusable building blocks; individual games compose them.

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Genre {
    Action, Adventure, Rpg, Arpg, Mmorpg, Strategy, Rts, Tactics, Simulation,
    Management, CityBuilder, Survival, Horror, Stealth, Shooter, Fps, Tps,
    Racing, Sports, Fighting, Platformer, Metroidvania, Roguelike, Roguelite,
    Sandbox, OpenWorld, ImmersiveSim, Puzzle, Rhythm, Party, Card, Board,
    TurnBased, Narrative, VisualNovel, Moba, BattleRoyale, TowerDefense,
    FlightSim, SpaceSim, VehicleSim, Educational, SeriousGame, Hybrid,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Capability {
    CharacterController, Camera, Input, Interaction, Dialogue, Quest, Inventory,
    Equipment, Crafting, SkillTree, Stats, Combat, Melee, Ranged, Cover,
    Stealth, Navigation, AiAgents, BehaviorTree, UtilityAi, Squad, Factions,
    Economy, Trading, Building, Farming, ResourceManagement, Research,
    Diplomacy, Territory, ProceduralGeneration, WorldStreaming, SaveLoad,
    Multiplayer, Matchmaking, Lobby, Replication, Prediction, Replay,
    VehiclesGround, VehiclesWater, VehiclesAir, VehicleDamage, Traffic,
    FlightModel, Racing, SportsRules, Ballistics, Destruction, Physics,
    Animation, Cinematics, PhotoMode, ReplayCamera, Weather, DayNight,
    Terrain, Water, Ocean, Clouds, Audio, Music, SpatialAudio, Ui,
    Accessibility, Localization, Modding, Scripting, Networking, Esports,
}

pub const ALL_GENRES: &[Genre] = &[
    Genre::Action, Genre::Adventure, Genre::Rpg, Genre::Arpg, Genre::Mmorpg,
    Genre::Strategy, Genre::Rts, Genre::Tactics, Genre::Simulation,
    Genre::Management, Genre::CityBuilder, Genre::Survival, Genre::Horror,
    Genre::Stealth, Genre::Shooter, Genre::Fps, Genre::Tps, Genre::Racing,
    Genre::Sports, Genre::Fighting, Genre::Platformer, Genre::Metroidvania,
    Genre::Roguelike, Genre::Roguelite, Genre::Sandbox, Genre::OpenWorld,
    Genre::ImmersiveSim, Genre::Puzzle, Genre::Rhythm, Genre::Party, Genre::Card,
    Genre::Board, Genre::TurnBased, Genre::Narrative, Genre::VisualNovel,
    Genre::Moba, Genre::BattleRoyale, Genre::TowerDefense, Genre::FlightSim,
    Genre::SpaceSim, Genre::VehicleSim, Genre::Educational, Genre::SeriousGame,
    Genre::Hybrid,
];

pub const UNIVERSAL_CAPABILITIES: &[Capability] = &[
    Capability::CharacterController, Capability::Camera, Capability::Input,
    Capability::Interaction, Capability::Dialogue, Capability::Quest,
    Capability::Inventory, Capability::Equipment, Capability::Crafting,
    Capability::Stats, Capability::Combat, Capability::Navigation,
    Capability::AiAgents, Capability::SaveLoad, Capability::WorldStreaming,
    Capability::ProceduralGeneration, Capability::Physics, Capability::Animation,
    Capability::Cinematics, Capability::Weather, Capability::DayNight,
    Capability::Terrain, Capability::Water, Capability::Ocean, Capability::Clouds,
    Capability::Audio, Capability::Music, Capability::SpatialAudio, Capability::Ui,
    Capability::Accessibility, Capability::Localization, Capability::Modding,
    Capability::Scripting,
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GenreProfile { pub genre: Genre, pub capabilities: &'static [Capability] }

pub const fn profile(genre: Genre) -> GenreProfile {
    GenreProfile { genre, capabilities: UNIVERSAL_CAPABILITIES }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn catalogue_is_nonempty() { assert!(ALL_GENRES.len() >= 40); }
    #[test] fn universal_profile_is_stable() { assert!(profile(Genre::Rpg).capabilities.contains(&Capability::Inventory)); }
}
