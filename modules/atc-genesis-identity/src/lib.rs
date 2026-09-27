//! Data-driven species, culture, lineage and character identity domain.
//!
//! This crate intentionally contains no moral/alignment field on species.
//! Gameplay systems consume validated IDs rather than branching on concrete species.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

const MAGIC: &[u8] = b"GNS-ID-1";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SpeciesDefinition {
    pub id: String,
    pub display_name: String,
    pub parent_species: Option<String>,
    pub primordial_force: String,
    pub habitat_profiles: Vec<String>,
    pub physical_profile: Vec<String>,
    pub cultural_profiles: Vec<String>,
    pub gameplay_traits: Vec<String>,
    pub abilities: Vec<String>,
    pub transformations: Vec<String>,
    pub resource_affinities: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CultureDefinition {
    pub id: String,
    pub species_id: String,
    pub values: Vec<String>,
    pub traditions: Vec<String>,
    pub settlement_profile: Vec<String>,
    pub economy_profile: Vec<String>,
    pub diplomatic_profile: Vec<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LineageOwnerType {
    Clan,
    Bloodline,
    Tribe,
    Lineage,
}

impl LineageOwnerType {
    fn tag(self) -> u8 {
        match self {
            Self::Clan => 0,
            Self::Bloodline => 1,
            Self::Tribe => 2,
            Self::Lineage => 3,
        }
    }

    fn from_tag(tag: u8) -> Result<Self, ValidationError> {
        match tag {
            0 => Ok(Self::Clan),
            1 => Ok(Self::Bloodline),
            2 => Ok(Self::Tribe),
            3 => Ok(Self::Lineage),
            _ => Err(ValidationError::MalformedSerialization("invalid lineage owner type")),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LineageDefinition {
    pub id: String,
    pub owner_type: LineageOwnerType,
    pub parent_id: Option<String>,
    pub heritage_traits: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CharacterIdentity {
    pub species_id: String,
    pub subspecies_id: Option<String>,
    pub culture_id: String,
    pub lineage_id: String,
    pub heritage_id: Option<String>,
    pub class_id: String,
    pub ability_ids: Vec<String>,
    pub transformation_ids: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IdentityRegistry {
    pub species: BTreeMap<String, SpeciesDefinition>,
    pub cultures: BTreeMap<String, CultureDefinition>,
    pub lineages: BTreeMap<String, LineageDefinition>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ValidationError {
    EmptyId(&'static str),
    InvalidId(String),
    DuplicateId(String),
    UnknownSpecies(String),
    UnknownCulture(String),
    UnknownLineage(String),
    CultureSpeciesMismatch { culture: String, species: String },
    LineageCultureMismatch { lineage: String, culture: String },
    EmptyField(&'static str),
    MalformedSerialization(&'static str),
    TrailingBytes,
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for ValidationError {}

impl SpeciesDefinition {
    pub fn validate(&self) -> Result<(), ValidationError> {
        validate_id(&self.id, "species")?;
        if self.display_name.trim().is_empty() {
            return Err(ValidationError::EmptyField("display_name"));
        }
        if self.primordial_force.trim().is_empty() {
            return Err(ValidationError::EmptyField("primordial_force"));
        }
        validate_optional_id(&self.parent_species)?;
        validate_list(&self.habitat_profiles, "habitat_profiles")?;
        validate_list(&self.physical_profile, "physical_profile")?;
        validate_list(&self.cultural_profiles, "cultural_profiles")?;
        validate_list(&self.gameplay_traits, "gameplay_traits")?;
        validate_list(&self.abilities, "abilities")?;
        validate_list(&self.transformations, "transformations")?;
        validate_list(&self.resource_affinities, "resource_affinities")?;
        Ok(())
    }

    pub fn canonical_serialize(&self) -> Vec<u8> {
        let mut out = MAGIC.to_vec();
        put_string(&mut out, "species");
        put_string(&mut out, &self.id);
        put_string(&mut out, &self.display_name);
        put_opt_string(&mut out, self.parent_species.as_deref());
        put_string(&mut out, &self.primordial_force);
        put_sorted_strings(&mut out, &self.habitat_profiles);
        put_sorted_strings(&mut out, &self.physical_profile);
        put_sorted_strings(&mut out, &self.cultural_profiles);
        put_sorted_strings(&mut out, &self.gameplay_traits);
        put_sorted_strings(&mut out, &self.abilities);
        put_sorted_strings(&mut out, &self.transformations);
        put_sorted_strings(&mut out, &self.resource_affinities);
        out
    }

    pub fn canonical_deserialize(bytes: &[u8]) -> Result<Self, ValidationError> {
        let mut p = Parser::new(bytes);
        p.magic()?;
        p.expect("species")?;
        let value = Self {
            id: p.string()?,
            display_name: p.string()?,
            parent_species: p.opt_string()?,
            primordial_force: p.string()?,
            habitat_profiles: p.strings()?,
            physical_profile: p.strings()?,
            cultural_profiles: p.strings()?,
            gameplay_traits: p.strings()?,
            abilities: p.strings()?,
            transformations: p.strings()?,
            resource_affinities: p.strings()?,
        };
        p.finish()?;
        value.validate()?;
        Ok(value)
    }
}

impl CultureDefinition {
    pub fn validate(&self) -> Result<(), ValidationError> {
        validate_id(&self.id, "culture")?;
        validate_id(&self.species_id, "species")?;
        validate_list(&self.values, "values")?;
        validate_list(&self.traditions, "traditions")?;
        validate_list(&self.settlement_profile, "settlement_profile")?;
        validate_list(&self.economy_profile, "economy_profile")?;
        validate_list(&self.diplomatic_profile, "diplomatic_profile")?;
        Ok(())
    }

    pub fn canonical_serialize(&self) -> Vec<u8> {
        let mut out = MAGIC.to_vec();
        put_string(&mut out, "culture");
        put_string(&mut out, &self.id);
        put_string(&mut out, &self.species_id);
        put_sorted_strings(&mut out, &self.values);
        put_sorted_strings(&mut out, &self.traditions);
        put_sorted_strings(&mut out, &self.settlement_profile);
        put_sorted_strings(&mut out, &self.economy_profile);
        put_sorted_strings(&mut out, &self.diplomatic_profile);
        out
    }

    pub fn canonical_deserialize(bytes: &[u8]) -> Result<Self, ValidationError> {
        let mut p = Parser::new(bytes);
        p.magic()?;
        p.expect("culture")?;
        let value = Self {
            id: p.string()?,
            species_id: p.string()?,
            values: p.strings()?,
            traditions: p.strings()?,
            settlement_profile: p.strings()?,
            economy_profile: p.strings()?,
            diplomatic_profile: p.strings()?,
        };
        p.finish()?;
        value.validate()?;
        Ok(value)
    }
}

impl LineageDefinition {
    pub fn validate(&self) -> Result<(), ValidationError> {
        validate_id(&self.id, "lineage")?;
        validate_optional_id(self.parent_id.as_ref())?;
        validate_list(&self.heritage_traits, "heritage_traits")
    }

    pub fn canonical_serialize(&self) -> Vec<u8> {
        let mut out = MAGIC.to_vec();
        put_string(&mut out, "lineage");
        put_string(&mut out, &self.id);
        out.push(self.owner_type.tag());
        put_opt_string(&mut out, self.parent_id.as_deref());
        put_sorted_strings(&mut out, &self.heritage_traits);
        out
    }

    pub fn canonical_deserialize(bytes: &[u8]) -> Result<Self, ValidationError> {
        let mut p = Parser::new(bytes);
        p.magic()?;
        p.expect("lineage")?;
        let value = Self {
            id: p.string()?,
            owner_type: LineageOwnerType::from_tag(p.byte()?)?,
            parent_id: p.opt_string()?,
            heritage_traits: p.strings()?,
        };
        p.finish()?;
        value.validate()?;
        Ok(value)
    }
}

impl CharacterIdentity {
    pub fn validate(&self, registry: &IdentityRegistry) -> Result<(), ValidationError> {
        validate_id(&self.species_id, "species")?;
        validate_id(&self.culture_id, "culture")?;
        validate_id(&self.lineage_id, "lineage")?;
        validate_id(&self.class_id, "class")?;
        validate_optional_id(self.subspecies_id.as_ref())?;
        validate_optional_id(self.heritage_id.as_ref())?;
        validate_list(&self.ability_ids, "ability_ids")?;
        validate_list(&self.transformation_ids, "transformation_ids")?;

        let species = registry.species.get(&self.species_id)
            .ok_or_else(|| ValidationError::UnknownSpecies(self.species_id.clone()))?;
        let culture = registry.cultures.get(&self.culture_id)
            .ok_or_else(|| ValidationError::UnknownCulture(self.culture_id.clone()))?;
        registry.lineages.get(&self.lineage_id)
            .ok_or_else(|| ValidationError::UnknownLineage(self.lineage_id.clone()))?;

        if culture.species_id != species.id {
            return Err(ValidationError::CultureSpeciesMismatch {
                culture: culture.id.clone(),
                species: species.id.clone(),
            });
        }

        Ok(())
    }

    pub fn canonical_serialize(&self) -> Vec<u8> {
        let mut out = MAGIC.to_vec();
        put_string(&mut out, "character");
        put_string(&mut out, &self.species_id);
        put_opt_string(&mut out, self.subspecies_id.as_deref());
        put_string(&mut out, &self.culture_id);
        put_string(&mut out, &self.lineage_id);
        put_opt_string(&mut out, self.heritage_id.as_deref());
        put_string(&mut out, &self.class_id);
        put_sorted_strings(&mut out, &self.ability_ids);
        put_sorted_strings(&mut out, &self.transformation_ids);
        out
    }

    pub fn canonical_deserialize(bytes: &[u8]) -> Result<Self, ValidationError> {
        let mut p = Parser::new(bytes);
        p.magic()?;
        p.expect("character")?;
        let value = Self {
            species_id: p.string()?,
            subspecies_id: p.opt_string()?,
            culture_id: p.string()?,
            lineage_id: p.string()?,
            heritage_id: p.opt_string()?,
            class_id: p.string()?,
            ability_ids: p.strings()?,
            transformation_ids: p.strings()?,
        };
        p.finish()?;
        Ok(value)
    }
}

impl IdentityRegistry {
    pub fn validate(&self) -> Result<(), ValidationError> {
        for (id, species) in &self.species {
            if id != &species.id {
                return Err(ValidationError::DuplicateId(id.clone()));
            }
            species.validate()?;
            if let Some(parent) = &species.parent_species {
                if !self.species.contains_key(parent) {
                    return Err(ValidationError::UnknownSpecies(parent.clone()));
                }
            }
        }
        for (id, culture) in &self.cultures {
            if id != &culture.id {
                return Err(ValidationError::DuplicateId(id.clone()));
            }
            culture.validate()?;
            if !self.species.contains_key(&culture.species_id) {
                return Err(ValidationError::UnknownSpecies(culture.species_id.clone()));
            }
        }
        for (id, lineage) in &self.lineages {
            if id != &lineage.id {
                return Err(ValidationError::DuplicateId(id.clone()));
            }
            lineage.validate()?;
        }
        Ok(())
    }
}

fn validate_id(value: &str, field: &'static str) -> Result<(), ValidationError> {
    if value.is_empty() {
        return Err(ValidationError::EmptyId(field));
    }
    if !value.bytes().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-' || b == b'_') {
        return Err(ValidationError::InvalidId(value.to_owned()));
    }
    Ok(())
}

fn validate_optional_id(value: Option<&String>) -> Result<(), ValidationError> {
    if let Some(value) = value {
        validate_id(value, "optional_id")?;
    }
    Ok(())
}

fn validate_list(values: &[String], field: &'static str) -> Result<(), ValidationError> {
    if values.iter().any(|v| v.trim().is_empty()) {
        return Err(ValidationError::EmptyField(field));
    }
    let mut unique = BTreeSet::new();
    if values.iter().any(|value| !unique.insert(value)) {
        return Err(ValidationError::DuplicateId(field.to_owned()));
    }
    Ok(())
}

fn put_string(out: &mut Vec<u8>, value: &str) {
    let bytes = value.as_bytes();
    out.extend_from_slice(&(bytes.len() as u32).to_be_bytes());
    out.extend_from_slice(bytes);
}

fn put_opt_string(out: &mut Vec<u8>, value: Option<&str>) {
    match value {
        Some(value) => {
            out.push(1);
            put_string(out, value);
        }
        None => out.push(0),
    }
}

fn put_sorted_strings(out: &mut Vec<u8>, values: &[String]) {
    let mut sorted = values.to_vec();
    sorted.sort();
    out.extend_from_slice(&(sorted.len() as u32).to_be_bytes());
    for value in sorted {
        put_string(out, &value);
    }
}

struct Parser<'a> {
    bytes: &'a [u8],
    offset: usize,
}

impl<'a> Parser<'a> {
    fn new(bytes: &'a [u8]) -> Self { Self { bytes, offset: 0 } }

    fn magic(&mut self) -> Result<(), ValidationError> {
        if self.take(MAGIC.len())? != MAGIC {
            return Err(ValidationError::MalformedSerialization("invalid magic"));
        }
        Ok(())
    }

    fn expect(&mut self, expected: &str) -> Result<(), ValidationError> {
        if self.string()? == expected { Ok(()) }
        else { Err(ValidationError::MalformedSerialization("unexpected record type")) }
    }

    fn byte(&mut self) -> Result<u8, ValidationError> {
        let b = *self.bytes.get(self.offset)
            .ok_or(ValidationError::MalformedSerialization("unexpected eof"))?;
        self.offset += 1;
        Ok(b)
    }

    fn string(&mut self) -> Result<String, ValidationError> {
        let len = u32::from_be_bytes(self.take(4)?.try_into().unwrap()) as usize;
        let bytes = self.take(len)?;
        String::from_utf8(bytes.to_vec())
            .map_err(|_| ValidationError::MalformedSerialization("invalid utf8"))
    }

    fn opt_string(&mut self) -> Result<Option<String>, ValidationError> {
        match self.byte()? {
            0 => Ok(None),
            1 => Ok(Some(self.string()?)),
            _ => Err(ValidationError::MalformedSerialization("invalid option tag")),
        }
    }

    fn strings(&mut self) -> Result<Vec<String>, ValidationError> {
        let len = u32::from_be_bytes(self.take(4)?.try_into().unwrap()) as usize;
        let mut values = Vec::with_capacity(len);
        for _ in 0..len { values.push(self.string()?); }
        if values.windows(2).any(|w| w[0] > w[1]) {
            return Err(ValidationError::MalformedSerialization("non-canonical list order"));
        }
        Ok(values)
    }

    fn take(&mut self, len: usize) -> Result<&'a [u8], ValidationError> {
        let end = self.offset.checked_add(len)
            .ok_or(ValidationError::MalformedSerialization("length overflow"))?;
        let value = self.bytes.get(self.offset..end)
            .ok_or(ValidationError::MalformedSerialization("unexpected eof"))?;
        self.offset = end;
        Ok(value)
    }

    fn finish(&self) -> Result<(), ValidationError> {
        if self.offset == self.bytes.len() { Ok(()) }
        else { Err(ValidationError::TrailingBytes) }
    }
}

/// Canonical six-species baseline. Adding a seventh species only adds data here;
/// consumers are keyed by IDs and do not match on this list.
pub fn baseline_species() -> Vec<SpeciesDefinition> {
    [
        ("forest-elf", "Forest Elves", "life"),
        ("desert-elf", "Desert Elves", "sun"),
        ("dwarf", "Dwarves", "earth"),
        ("fairy", "Fairies", "arcana"),
        ("vampire", "Vampires", "death"),
        ("werewolf", "Werewolves", "life"),
    ].into_iter().map(|(id, name, force)| SpeciesDefinition {
        id: id.into(),
        display_name: name.into(),
        parent_species: None,
        primordial_force: force.into(),
        habitat_profiles: vec!["world".into()],
        physical_profile: vec!["species-defined".into()],
        cultural_profiles: vec!["species-defined".into()],
        gameplay_traits: vec!["species-defined".into()],
        abilities: vec!["species-defined".into()],
        transformations: Vec::new(),
        resource_affinities: vec!["species-defined".into()],
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn registry() -> IdentityRegistry {
        let species = baseline_species().into_iter().map(|s| (s.id.clone(), s)).collect();
        let culture = CultureDefinition {
            id: "moon-grove".into(),
            species_id: "forest-elf".into(),
            values: vec!["balance".into()],
            traditions: vec!["moon-watch".into()],
            settlement_profile: vec!["living-grove".into()],
            economy_profile: vec!["herbs".into()],
            diplomatic_profile: vec!["grove-council".into()],
        };
        let lineage = LineageDefinition {
            id: "ancient-lineage".into(),
            owner_type: LineageOwnerType::Lineage,
            parent_id: None,
            heritage_traits: vec!["old-blood".into()],
        };
        IdentityRegistry {
            species,
            cultures: [(culture.id.clone(), culture)].into_iter().collect(),
            lineages: [(lineage.id.clone(), lineage)].into_iter().collect(),
        }
    }

    #[test]
    fn all_six_baseline_species_use_one_schema() {
        let species = baseline_species();
        assert_eq!(species.len(), 6);
        assert!(species.iter().all(|s| s.validate().is_ok()));
    }

    #[test]
    fn canonical_serialization_round_trips() {
        let original = baseline_species()[0].clone();
        let encoded = original.canonical_serialize();
        let decoded = SpeciesDefinition::canonical_deserialize(&encoded).unwrap();
        assert_eq!(decoded, original);
        assert_eq!(encoded, decoded.canonical_serialize());
    }

    #[test]
    fn canonical_serialization_is_order_independent_for_sets() {
        let mut a = baseline_species()[0].clone();
        let mut b = a.clone();
        a.abilities = vec!["z".into(), "a".into()];
        b.abilities = vec!["a".into(), "z".into()];
        assert_eq!(a.canonical_serialize(), b.canonical_serialize());
    }

    #[test]
    fn character_identity_resolves_hierarchy() {
        let r = registry();
        let identity = CharacterIdentity {
            species_id: "forest-elf".into(),
            subspecies_id: None,
            culture_id: "moon-grove".into(),
            lineage_id: "ancient-lineage".into(),
            heritage_id: Some("ancient-lineage".into()),
            class_id: "druid".into(),
            ability_ids: vec!["plant-magic".into()],
            transformation_ids: vec![],
        };
        assert!(identity.validate(&r).is_ok());
    }

    #[test]
    fn negative_unknown_species_is_rejected() {
        let mut r = registry();
        let identity = CharacterIdentity {
            species_id: "dragon".into(),
            subspecies_id: None,
            culture_id: "moon-grove".into(),
            lineage_id: "ancient-lineage".into(),
            heritage_id: None,
            class_id: "ranger".into(),
            ability_ids: vec![],
            transformation_ids: vec![],
        };
        assert_eq!(
            identity.validate(&r),
            Err(ValidationError::UnknownSpecies("dragon".into()))
        );
        r.species.clear();
    }

    #[test]
    fn negative_culture_species_mismatch_is_rejected() {
        let mut r = registry();
        let mut culture = r.cultures.remove("moon-grove").unwrap();
        culture.species_id = "dwarf".into();
        r.cultures.insert(culture.id.clone(), culture);
        let identity = CharacterIdentity {
            species_id: "forest-elf".into(),
            subspecies_id: None,
            culture_id: "moon-grove".into(),
            lineage_id: "ancient-lineage".into(),
            heritage_id: None,
            class_id: "druid".into(),
            ability_ids: vec![],
            transformation_ids: vec![],
        };
        assert!(matches!(
            identity.validate(&r),
            Err(ValidationError::CultureSpeciesMismatch { .. })
        ));
    }

    #[test]
    fn negative_malformed_serialization_is_rejected() {
        let mut bytes = baseline_species()[0].canonical_serialize();
        bytes.pop();
        assert!(matches!(
            SpeciesDefinition::canonical_deserialize(&bytes),
            Err(ValidationError::MalformedSerialization(_)) | Err(ValidationError::TrailingBytes)
        ));
    }

    #[test]
    fn species_has_no_moral_alignment_field() {
        let species = baseline_species()[0].canonical_serialize();
        assert!(!String::from_utf8_lossy(&species).contains("alignment"));
    }
}
