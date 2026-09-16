use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RenderResourceId(pub u128);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RenderPassId(pub u128);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResourceAccess { Read, Write, ReadWrite }

impl ResourceAccess {
    fn conflicts(self, other: Self) -> bool {
        matches!((self, other), (Self::Read, Self::Read)) == false
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ResourceUse { pub resource: RenderResourceId, pub access: ResourceAccess }

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RenderPassDesc { pub id: RenderPassId, pub resources: Vec<ResourceUse> }

#[derive(Default)]
pub struct RenderPassGraph { passes: BTreeMap<RenderPassId, RenderPassDesc> }

impl RenderPassGraph {
    pub fn add_pass(&mut self, pass: RenderPassDesc) -> Result<(), RenderGraphError> {
        if self.passes.contains_key(&pass.id) { return Err(RenderGraphError::DuplicatePass); }
        let mut seen = BTreeSet::new();
        for usage in &pass.resources {
            if !seen.insert(usage.resource) { return Err(RenderGraphError::DuplicateResourceUse); }
        }
        self.passes.insert(pass.id, pass);
        Ok(())
    }

    pub fn ordered_passes(&self) -> Result<Vec<RenderPassId>, RenderGraphError> {
        let ids: Vec<_> = self.passes.keys().copied().collect();
        let mut edges: BTreeMap<RenderPassId, BTreeSet<RenderPassId>> = ids.iter().map(|id| (*id, BTreeSet::new())).collect();
        for (i, left_id) in ids.iter().enumerate() {
            for right_id in ids.iter().skip(i + 1) {
                let left = &self.passes[left_id];
                let right = &self.passes[right_id];
                if Self::has_conflict(left, right) {
                    edges.get_mut(left_id).unwrap().insert(*right_id);
                }
            }
        }
        let mut indegree: BTreeMap<RenderPassId, usize> = ids.iter().map(|id| (*id, 0)).collect();
        for targets in edges.values() { for target in targets { *indegree.get_mut(target).unwrap() += 1; } }
        let mut ready: BTreeSet<RenderPassId> = indegree.iter().filter_map(|(id, degree)| (*degree == 0).then_some(*id)).collect();
        let mut ordered = Vec::with_capacity(ids.len());
        while let Some(id) = ready.pop_first() {
            ordered.push(id);
            for target in edges[&id].iter() {
                let degree = indegree.get_mut(target).unwrap();
                *degree -= 1;
                if *degree == 0 { ready.insert(*target); }
            }
        }
        if ordered.len() != ids.len() { return Err(RenderGraphError::CycleDetected); }
        Ok(ordered)
    }

    fn has_conflict(left: &RenderPassDesc, right: &RenderPassDesc) -> bool {
        left.resources.iter().any(|a| right.resources.iter().any(|b| a.resource == b.resource && a.access.conflicts(b.access)))
    }

    pub fn len(&self) -> usize { self.passes.len() }
    pub fn is_empty(&self) -> bool { self.passes.is_empty() }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RenderGraphError { DuplicatePass, DuplicateResourceUse, CycleDetected }

#[cfg(test)]
mod tests {
    use super::*;
    fn pass(id: u128, resource: u128, access: ResourceAccess) -> RenderPassDesc { RenderPassDesc { id: RenderPassId(id), resources: vec![ResourceUse { resource: RenderResourceId(resource), access }] } }

    #[test] fn deterministic_order_for_independent_passes() {
        let mut graph = RenderPassGraph::default();
        graph.add_pass(pass(2, 20, ResourceAccess::Read)).unwrap();
        graph.add_pass(pass(1, 10, ResourceAccess::Read)).unwrap();
        assert_eq!(graph.ordered_passes().unwrap(), vec![RenderPassId(1), RenderPassId(2)]);
    }

    #[test] fn write_then_read_creates_dependency() {
        let mut graph = RenderPassGraph::default();
        graph.add_pass(pass(2, 7, ResourceAccess::Read)).unwrap();
        graph.add_pass(pass(1, 7, ResourceAccess::Write)).unwrap();
        assert_eq!(graph.ordered_passes().unwrap(), vec![RenderPassId(1), RenderPassId(2)]);
    }

    #[test] fn duplicate_resource_use_is_rejected() {
        let mut graph = RenderPassGraph::default();
        let mut p = pass(1, 7, ResourceAccess::Read);
        p.resources.push(ResourceUse { resource: RenderResourceId(7), access: ResourceAccess::Write });
        assert_eq!(graph.add_pass(p), Err(RenderGraphError::DuplicateResourceUse));
    }
}
