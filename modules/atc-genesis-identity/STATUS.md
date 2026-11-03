# Status — atc-genesis-identity

Status: IMPLEMENTED_PENDING_CI

This crate provides the first machine-checkable domain layer for species, culture, lineage and character identity.

Implemented:
- SpeciesDefinition
- CultureDefinition
- LineageDefinition
- CharacterIdentity
- IdentityRegistry validation
- deterministic canonical serialization/deserialization
- six baseline species data
- positive and negative unit tests

Runtime Character/World/Combat/Economy integration is intentionally not claimed by this change.
