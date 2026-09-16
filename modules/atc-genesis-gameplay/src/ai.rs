//! Deterministic AI foundations: grid navigation, A* pathfinding, behavior trees and utility scoring.

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GridNode { pub x: i32, pub y: i32 }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GridBounds { pub min: GridNode, pub max: GridNode }
impl GridBounds { pub fn contains(&self, n: GridNode) -> bool { n.x >= self.min.x && n.x <= self.max.x && n.y >= self.min.y && n.y <= self.max.y } }

pub trait GridCost { fn walkable(&self, node: GridNode) -> bool; fn cost(&self, from: GridNode, to: GridNode) -> u32; }

fn manhattan(a: GridNode, b: GridNode) -> u32 { a.x.abs_diff(b.x) + a.y.abs_diff(b.y) }
fn neighbors(n: GridNode) -> [GridNode; 4] { [GridNode { x: n.x + 1, y: n.y }, GridNode { x: n.x - 1, y: n.y }, GridNode { x: n.x, y: n.y + 1 }, GridNode { x: n.x, y: n.y - 1 }] }

/// Deterministic A*: ties are resolved by node coordinates, then insertion order is irrelevant.
pub fn astar<G: GridCost>(grid: &G, bounds: GridBounds, start: GridNode, goal: GridNode) -> Option<Vec<GridNode>> {
    if !bounds.contains(start) || !bounds.contains(goal) || !grid.walkable(start) || !grid.walkable(goal) { return None; }
    let mut open: Vec<(u32, u32, GridNode)> = vec![(manhattan(start, goal), 0, start)];
    let mut came_from: Vec<(GridNode, GridNode)> = Vec::new();
    let mut g_score: Vec<(GridNode, u32)> = vec![(start, 0)];
    let mut closed: Vec<GridNode> = Vec::new();
    while !open.is_empty() {
        open.sort_by_key(|e| (e.0, e.1, e.2));
        let (_, g, current) = open.remove(0);
        if closed.contains(&current) { continue; }
        if current == goal {
            let mut path = vec![current];
            let mut cursor = current;
            while let Some((child, parent)) = came_from.iter().find(|(child, _)| *child == cursor) { path.push(*parent); cursor = *parent; if cursor == start { break; } }
            path.reverse(); return Some(path);
        }
        closed.push(current);
        for next in neighbors(current) {
            if !bounds.contains(next) || !grid.walkable(next) || closed.contains(&next) { continue; }
            let tentative = g.saturating_add(grid.cost(current, next).max(1));
            let known = g_score.iter().find(|(n, _)| *n == next).map(|(_, v)| *v);
            if known.map_or(true, |v| tentative < v) {
                if let Some(entry) = g_score.iter_mut().find(|(n, _)| *n == next) { entry.1 = tentative; } else { g_score.push((next, tentative)); }
                came_from.retain(|(n, _)| *n != next); came_from.push((next, current));
                open.push((tentative.saturating_add(manhattan(next, goal)), tentative, next));
            }
        }
    }
    None
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BehaviorStatus { Success, Failure, Running }

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BehaviorNode { Sequence(Vec<BehaviorNode>), Selector(Vec<BehaviorNode>), Leaf(BehaviorStatus) }
impl BehaviorNode {
    pub fn tick(&self) -> BehaviorStatus {
        match self {
            Self::Leaf(s) => *s,
            Self::Sequence(nodes) => { for node in nodes { match node.tick() { BehaviorStatus::Success => {}, BehaviorStatus::Failure => return BehaviorStatus::Failure, BehaviorStatus::Running => return BehaviorStatus::Running } } BehaviorStatus::Success }
            Self::Selector(nodes) => { for node in nodes { match node.tick() { BehaviorStatus::Failure => {}, BehaviorStatus::Success => return BehaviorStatus::Success, BehaviorStatus::Running => return BehaviorStatus::Running } } BehaviorStatus::Failure }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UtilityOption { pub id: u32, pub score: f32 }
pub fn choose_utility(options: &[UtilityOption]) -> Option<UtilityOption> {
    options.iter().copied().filter(|o| o.score.is_finite()).max_by(|a,b| a.score.total_cmp(&b.score).then_with(|| b.id.cmp(&a.id)))
}

#[cfg(test)]
mod tests {
    use super::*;
    struct Map { blocked: Vec<GridNode> }
    impl GridCost for Map { fn walkable(&self,n:GridNode)->bool { !self.blocked.contains(&n) } fn cost(&self,_:GridNode,_:GridNode)->u32 { 1 } }
    #[test] fn astar_finds_deterministic_path() { let map=Map{blocked:vec![GridNode{x:1,y:0}]}; let p=astar(&map,GridBounds{min:GridNode{x:0,y:0},max:GridNode{x:2,y:2}},GridNode{x:0,y:0},GridNode{x:2,y:0}).unwrap(); assert_eq!(p.first().copied(),Some(GridNode{x:0,y:0})); assert_eq!(p.last().copied(),Some(GridNode{x:2,y:0})); }
    #[test] fn behavior_sequence_fails_fast() { let b=BehaviorNode::Sequence(vec![BehaviorNode::Leaf(BehaviorStatus::Success),BehaviorNode::Leaf(BehaviorStatus::Failure)]); assert_eq!(b.tick(),BehaviorStatus::Failure); }
    #[test] fn utility_ties_use_lowest_id() { let c=choose_utility(&[UtilityOption{id:2,score:1.0},UtilityOption{id:1,score:1.0}]).unwrap(); assert_eq!(c.id,1); }
}
