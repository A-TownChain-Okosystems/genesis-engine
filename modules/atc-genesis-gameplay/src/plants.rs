//! Deterministic plant-growth foundation for farming, survival, simulation and world systems.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlantStage { Seed, Sprout, Vegetative, Flowering, Fruiting, Mature, Withered }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PlantDefinition { pub id: &'static str, pub growth_ticks: u32, pub water_per_tick: u16, pub light_min: u8, pub light_max: u8, pub harvest_yield: u32 }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PlantEnvironment { pub water: u16, pub light: u8, pub temperature: i16 }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PlantInstance { pub species: &'static str, pub stage: PlantStage, pub age_ticks: u32, pub health: u16, pub yield_bonus: u16 }

impl PlantInstance {
    pub fn new(species: &'static str) -> Self { Self { species, stage: PlantStage::Seed, age_ticks: 0, health: 1000, yield_bonus: 0 } }
    pub fn tick(&mut self, def: PlantDefinition, env: PlantEnvironment) -> u32 {
        if self.stage == PlantStage::Withered || self.species != def.id { return 0; }
        let water_ok = env.water >= def.water_per_tick;
        let light_ok = env.light >= def.light_min && env.light <= def.light_max;
        if !water_ok || !light_ok { self.health = self.health.saturating_sub(25); } else { self.health = (self.health + 5).min(1000); self.age_ticks = self.age_ticks.saturating_add(1); }
        if self.health == 0 { self.stage = PlantStage::Withered; return 0; }
        let step = (def.growth_ticks.max(1) / 6).max(1);
        self.stage = match self.age_ticks { 0 => PlantStage::Seed, x if x < step => PlantStage::Sprout, x if x < step*2 => PlantStage::Vegetative, x if x < step*3 => PlantStage::Flowering, x if x < step*4 => PlantStage::Fruiting, _ => PlantStage::Mature };
        0
    }
    pub fn harvest(&mut self, def: PlantDefinition) -> u32 { if self.species != def.id || self.stage != PlantStage::Mature { return 0; } self.age_ticks=0; self.stage=PlantStage::Seed; def.harvest_yield + (def.harvest_yield * self.yield_bonus as u32 / 1000) }
}

#[derive(Default, Debug, PartialEq, Eq)]
pub struct CropPlot { pub plants: Vec<PlantInstance>, pub capacity: usize }
impl CropPlot { pub fn new(capacity: usize) -> Self { Self { plants: Vec::new(), capacity } } pub fn plant(&mut self, species: &'static str) -> bool { if self.plants.len() >= self.capacity { return false; } self.plants.push(PlantInstance::new(species)); true } }

pub const WHEAT: PlantDefinition = PlantDefinition { id:"wheat", growth_ticks:60, water_per_tick:10, light_min:20, light_max:100, harvest_yield:3 };
pub const CORN: PlantDefinition = PlantDefinition { id:"corn", growth_ticks:90, water_per_tick:15, light_min:30, light_max:100, harvest_yield:4 };
pub const HERB: PlantDefinition = PlantDefinition { id:"herb", growth_ticks:30, water_per_tick:5, light_min:10, light_max:80, harvest_yield:2 };

#[cfg(test)]
mod tests { use super::*;
#[test] fn plant_growth_is_environment_gated(){let mut p=PlantInstance::new("wheat"); p.tick(WHEAT,PlantEnvironment{water:0,light:50,temperature:20}); assert_eq!(p.age_ticks,0); p.tick(WHEAT,PlantEnvironment{water:10,light:50,temperature:20}); assert_eq!(p.age_ticks,1);}
#[test] fn plot_capacity_is_enforced(){let mut p=CropPlot::new(1);assert!(p.plant("wheat"));assert!(!p.plant("corn"));}
#[test] fn harvest_is_deterministic(){let mut p=PlantInstance::new("wheat");p.stage=PlantStage::Mature;assert_eq!(p.harvest(WHEAT),3);assert_eq!(p.stage,PlantStage::Seed);}
#[test] fn wrong_definition_does_not_mutate(){let mut p=PlantInstance::new("wheat");p.tick(CORN,PlantEnvironment{water:20,light:50,temperature:20});assert_eq!(p.age_ticks,0);}
}
