use atc_genesis_ecs::World;
use atc_genesis_input::{InputState, Key};
use atc_genesis_platform::EntityId;
pub mod genre; pub mod combat; pub mod interaction; pub mod systems; pub mod presets; pub mod items; pub mod item_systems; pub mod plants; pub mod animals; pub mod technology; pub mod building;
pub use genre::{Capability, Genre, GenreProfile, ALL_GENRES, UNIVERSAL_CAPABILITIES, profile};
pub use combat::{Attack, CombatEvent, Combatant, Damage, DamageType, Health, TeamRelation};
pub use interaction::{Ability, AbilityState, Interaction, InteractionKind};
pub use presets::{AbilityBarPreset, CharacterSlotPreset, EquipmentPreset, InventoryPreset, LoadoutPreset, SkillTreePreset};
pub use items::{DamageType as ItemDamageType, Inventory, ItemDefinition, ItemKind, ItemStack, WeaponDefinition, WeaponInstance, WeaponKind, WeaponLoadout, WeaponFireResult};
pub use item_systems::{Ammunition, ArmorSlot, ArmorStats, LootEntry, LootTable, Recipe, RecipeIngredient, Rarity, WeaponAttachment, WeaponAttachmentState};
pub use plants::{PlantDefinition, PlantEnvironment, PlantInstance, PlantStage, CropPlot, WHEAT, CORN, HERB};
pub use animals::{AnimalDefinition, AnimalEnvironment, AnimalInstance, AnimalKind, AnimalState, Herd, DEER, WOLF, COW};
pub use technology::{TechnologyCategory, TechnologyDefinition, TechnologyTier, TechnologyTree, FurnitureDefinition, FurnitureInstance, FurnitureKind, FurnitureRoom, AGRICULTURE, CONSTRUCTION, ELECTRICITY, COMPUTING, CHAIR, TABLE, BED, CABINET};
pub use building::{ConstructionDefinition, ConstructionInstance, ConstructionSite, ResourceCost, ResourceStock, HOUSE, WORKSHOP};
pub use systems::{BuildCell, BuildGrid, CharacterState, EconomyBalance, FlightState, InputFrame, ModDescriptor, ModRegistry, NarrativeEvent, NarrativeState, NetworkTick, RaceState, ReplicatedTransform, ReplicationBuffer, SpaceState, StrategyOrder, StrategyQueue};

#[derive(Clone, Copy, Debug, PartialEq)] pub struct MovementConfig { pub speed:f32,pub axis_x:Option<u8>,pub axis_z:Option<u8> }
impl Default for MovementConfig{fn default()->Self{Self{speed:5.0,axis_x:None,axis_z:None}}}
#[derive(Clone,Copy,Debug,PartialEq)] pub struct GameplayCommand{pub entity:EntityId,pub movement:[f32;3]}
#[derive(Default)] pub struct GameplayRuntime{pub commands:Vec<GameplayCommand>}
impl GameplayRuntime{pub fn sample_movement(&mut self,input:&InputState,entity:EntityId,config:MovementConfig)->GameplayCommand{let mut x=input.axis(config.axis_x.unwrap_or(255));let mut z=input.axis(config.axis_z.unwrap_or(254));if x==0.0{x=if input.pressed(Key::Right){1.0}else if input.pressed(Key::Left){-1.0}else{0.0}}if z==0.0{z=if input.pressed(Key::Down){1.0}else if input.pressed(Key::Up){-1.0}else{0.0}}x=x.clamp(-1.0,1.0);z=z.clamp(-1.0,1.0);let m=(x*x+z*z).sqrt();if m>1.0{x/=m;z/=m}let c=GameplayCommand{entity,movement:[x*config.speed,0.0,z*config.speed]};self.commands.push(c);c}pub fn apply(&mut self,world:&mut World,dt:f32)->usize{let dt=dt.max(0.0);let commands=std::mem::take(&mut self.commands);let mut n=0;for c in commands{if let Some(mut t)=world.transform(c.entity).copied(){t.translation[0]+=c.movement[0]*dt;t.translation[1]+=c.movement[1]*dt;t.translation[2]+=c.movement[2]*dt;if world.set_transform(c.entity,t){n+=1}}}n}}
