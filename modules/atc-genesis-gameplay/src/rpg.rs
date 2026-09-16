//! Deterministic RPG and narrative foundations: inventory, equipment, skills, quests and dialogue.

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ItemStack { pub item_id: String, pub quantity: u32 }
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Inventory { pub items: Vec<ItemStack> }
impl Inventory {
    pub fn add(&mut self, item_id: impl Into<String>, quantity: u32) -> bool { if quantity == 0 { return false; } let id=item_id.into(); if let Some(s)=self.items.iter_mut().find(|s|s.item_id==id){s.quantity=s.quantity.saturating_add(quantity);} else {self.items.push(ItemStack{item_id:id,quantity});self.items.sort_by(|a,b|a.item_id.cmp(&b.item_id));} true }
    pub fn remove(&mut self, item_id: &str, quantity: u32) -> bool { if quantity==0{return false;} if let Some(s)=self.items.iter_mut().find(|s|s.item_id==item_id && s.quantity>=quantity){s.quantity-=quantity;if s.quantity==0{self.items.retain(|x|x.quantity>0);}return true;} false }
    pub fn count(&self,item_id:&str)->u32{self.items.iter().find(|s|s.item_id==item_id).map(|s|s.quantity).unwrap_or(0)}
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EquipmentSlot { pub slot: String, pub item_id: Option<String> }
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Equipment { pub slots: Vec<EquipmentSlot> }
impl Equipment { pub fn equip(&mut self,slot:impl Into<String>,item_id:impl Into<String>){let slot=slot.into();let item_id=item_id.into();if let Some(s)=self.slots.iter_mut().find(|s|s.slot==slot){s.item_id=Some(item_id);}else{self.slots.push(EquipmentSlot{slot,item_id:Some(item_id)});self.slots.sort_by(|a,b|a.slot.cmp(&b.slot));}} pub fn unequip(&mut self,slot:&str)->bool{if let Some(s)=self.slots.iter_mut().find(|s|s.slot==slot){s.item_id=None;true}else{false}} }

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Stat { pub id:String, pub base:i32, pub bonus:i32 }
impl Stat { pub fn value(&self)->i32{self.base.saturating_add(self.bonus)} }
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SkillTree { pub unlocked: Vec<String> }
impl SkillTree { pub fn unlock(&mut self,id:impl Into<String>)->bool{let id=id.into();if id.is_empty()||self.unlocked.contains(&id){return false;}self.unlocked.push(id);self.unlocked.sort();true} pub fn has(&self,id:&str)->bool{self.unlocked.iter().any(|x|x==id)} }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum QuestState { Locked, Active, Completed, Failed }
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Quest { pub id:u32, pub state:QuestState, pub objectives_total:u32, pub objectives_done:u32 }
impl Quest { pub fn activate(&mut self)->bool{if self.state==QuestState::Locked{self.state=QuestState::Active;true}else{false}} pub fn progress(&mut self,count:u32){if self.state==QuestState::Active{self.objectives_done=self.objectives_done.saturating_add(count).min(self.objectives_total);if self.objectives_done>=self.objectives_total{self.state=QuestState::Completed;}}} }

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DialogueChoice { pub id:u32, pub target:u32 }
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DialogueNode { pub id:u32, pub text_key:String, pub choices:Vec<DialogueChoice> }
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DialogueGraph { pub nodes:Vec<DialogueNode> }
impl DialogueGraph { pub fn add(&mut self,node:DialogueNode)->bool{if self.nodes.iter().any(|n|n.id==node.id){return false;}self.nodes.push(node);self.nodes.sort_by_key(|n|n.id);true} pub fn node(&self,id:u32)->Option<&DialogueNode>{self.nodes.iter().find(|n|n.id==id)} pub fn choice_target(&self,node:u32,choice:u32)->Option<u32>{self.node(node)?.choices.iter().find(|c|c.id==choice).map(|c|c.target).filter(|target|self.node(*target).is_some())} }

#[cfg(test)]
mod tests { use super::*;
#[test]fn inventory_is_deterministic(){let mut i=Inventory::default();assert!(i.add("z",2));assert!(i.add("a",1));assert_eq!(i.items[0].item_id,"a");assert!(i.remove("a",1));assert_eq!(i.count("a"),0);}
#[test]fn quest_completes_at_objective_limit(){let mut q=Quest{id:1,state:QuestState::Locked,objectives_total:2,objectives_done:0};assert!(q.activate());q.progress(3);assert_eq!(q.state,QuestState::Completed);}
#[test]fn dialogue_rejects_missing_target(){let mut g=DialogueGraph::default();g.add(DialogueNode{id:1,text_key:"start".into(),choices:vec![DialogueChoice{id:1,target:2}]});assert_eq!(g.choice_target(1,1),None);}
}
