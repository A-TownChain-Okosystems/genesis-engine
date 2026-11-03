//! Deterministic detail analysis and level-of-detail selection.
//!
//! Details AI is the micro-detail intelligence layer for Genesis Engine.
//! This module deliberately contains deterministic decision logic only: model
//! inference can be integrated above it without making rendering decisions
//! nondeterministic.

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum DetailLevel {
    D0 = 0,
    D1 = 1,
    D2 = 2,
    D3 = 3,
    D4 = 4,
    D5 = 5,
}

impl DetailLevel {
    pub const fn clamp(self, max: Self) -> Self {
        if (self as u8) > (max as u8) { max } else { self }
    }

    pub const fn from_score(score: u8) -> Self {
        match score {
            0..=16 => Self::D0,
            17..=33 => Self::D1,
            34..=50 => Self::D2,
            51..=67 => Self::D3,
            68..=84 => Self::D4,
            _ => Self::D5,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DetailContext {
    pub distance: f32,
    pub visible: bool,
    pub camera_focused: bool,
    pub object_importance: f32,
    pub gameplay_importance: f32,
    pub narrative_importance: f32,
    pub material_complexity: f32,
    pub performance_budget: f32,
    pub max_level: DetailLevel,
}

impl Default for DetailContext {
    fn default() -> Self {
        Self {
            distance: 10.0,
            visible: true,
            camera_focused: false,
            object_importance: 0.5,
            gameplay_importance: 0.0,
            narrative_importance: 0.0,
            material_complexity: 0.5,
            performance_budget: 1.0,
            max_level: DetailLevel::D5,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DetailScore {
    pub visibility: f32,
    pub focus: f32,
    pub object: f32,
    pub gameplay: f32,
    pub narrative: f32,
    pub material: f32,
    pub distance: f32,
    pub performance: f32,
    pub total: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DetailDecision {
    pub level: DetailLevel,
    pub score: u8,
    pub generate_microdetail: bool,
    pub use_displacement: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DetailIssue {
    pub missing_microdetail: bool,
    pub excessive_distance_detail: bool,
    pub invalid_context: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DetailQualityReport {
    pub issues: DetailIssue,
    pub recommended_level: DetailLevel,
    pub pass: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct DetailsAi;

impl DetailsAi {
    pub fn score(&self, context: DetailContext) -> DetailScore {
        let distance = sanitize_non_negative(context.distance);
        let distance_factor = 1.0 / (1.0 + distance / 25.0);
        let budget = clamp01(context.performance_budget);

        let visibility = if context.visible { 1.0 } else { 0.0 };
        let focus = bool_factor(context.camera_focused);
        let object = clamp01(context.object_importance);
        let gameplay = clamp01(context.gameplay_importance);
        let narrative = clamp01(context.narrative_importance);
        let material = clamp01(context.material_complexity);

        // Weights sum to 1.0. Performance budget reduces expensive detail,
        // but never changes the semantic importance inputs.
        let weighted = visibility * 0.12
            + focus * 0.18
            + object * 0.16
            + gameplay * 0.14
            + narrative * 0.10
            + material * 0.08
            + distance_factor * 0.14
            + budget * 0.08;

        DetailScore {
            visibility,
            focus,
            object,
            gameplay,
            narrative,
            material,
            distance: distance_factor,
            performance: budget,
            total: weighted,
        }
    }

    pub fn decide(&self, context: DetailContext) -> DetailDecision {
        let score = self.score(context);
        let score_u8 = (score.total.clamp(0.0, 1.0) * 100.0).round() as u8;
        let level = DetailLevel::from_score(score_u8).clamp(context.max_level);

        DetailDecision {
            level,
            score: score_u8,
            generate_microdetail: level >= DetailLevel::D4,
            use_displacement: level >= DetailLevel::D5 && context.material_complexity >= 0.6,
        }
    }

    pub fn quality(
        &self,
        context: DetailContext,
        actual_level: DetailLevel,
        has_microdetail: bool,
    ) -> DetailQualityReport {
        let decision = self.decide(context);
        let missing_microdetail =
            decision.level >= DetailLevel::D4 && !has_microdetail;
        let excessive_distance_detail =
            context.distance > 100.0 && actual_level >= DetailLevel::D4;
        let invalid_context = !context.distance.is_finite()
            || !context.object_importance.is_finite()
            || !context.gameplay_importance.is_finite()
            || !context.narrative_importance.is_finite()
            || !context.material_complexity.is_finite()
            || !context.performance_budget.is_finite();

        let issues = DetailIssue {
            missing_microdetail,
            excessive_distance_detail,
            invalid_context,
        };

        DetailQualityReport {
            issues,
            recommended_level: decision.level,
            pass: !missing_microdetail && !excessive_distance_detail && !invalid_context,
        }
    }
}

fn clamp01(value: f32) -> f32 {
    if !value.is_finite() { 0.0 } else { value.clamp(0.0, 1.0) }
}

fn sanitize_non_negative(value: f32) -> f32 {
    if value.is_finite() && value >= 0.0 { value } else { 0.0 }
}

fn bool_factor(value: bool) -> f32 {
    if value { 1.0 } else { 0.0 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detail_levels_are_ordered() {
        assert!(DetailLevel::D5 > DetailLevel::D4);
        assert_eq!(DetailLevel::from_score(0), DetailLevel::D0);
        assert_eq!(DetailLevel::from_score(100), DetailLevel::D5);
    }

    #[test]
    fn focused_important_object_gets_high_detail() {
        let ai = DetailsAi;
        let decision = ai.decide(DetailContext {
            camera_focused: true,
            object_importance: 1.0,
            gameplay_importance: 1.0,
            narrative_importance: 1.0,
            material_complexity: 1.0,
            ..Default::default()
        });
        assert!(decision.level >= DetailLevel::D4);
        assert!(decision.generate_microdetail);
    }

    #[test]
    fn distant_object_is_not_forced_to_microdetail() {
        let ai = DetailsAi;
        let decision = ai.decide(DetailContext {
            distance: 500.0,
            camera_focused: false,
            object_importance: 0.0,
            gameplay_importance: 0.0,
            narrative_importance: 0.0,
            material_complexity: 0.0,
            ..Default::default()
        });
        assert!(decision.level <= DetailLevel::D2);
        assert!(!decision.generate_microdetail);
    }

    #[test]
    fn max_level_is_respected() {
        let ai = DetailsAi;
        let decision = ai.decide(DetailContext {
            camera_focused: true,
            object_importance: 1.0,
            gameplay_importance: 1.0,
            narrative_importance: 1.0,
            material_complexity: 1.0,
            max_level: DetailLevel::D2,
            ..Default::default()
        });
        assert_eq!(decision.level, DetailLevel::D2);
    }

    #[test]
    fn quality_gate_detects_missing_microdetail() {
        let ai = DetailsAi;
        let context = DetailContext {
            camera_focused: true,
            object_importance: 1.0,
            gameplay_importance: 1.0,
            narrative_importance: 1.0,
            material_complexity: 1.0,
            ..Default::default()
        };
        let report = ai.quality(context, DetailLevel::D3, false);
        assert!(!report.pass);
        assert!(report.issues.missing_microdetail);
    }

    #[test]
    fn invalid_float_input_is_rejected_by_quality_gate() {
        let ai = DetailsAi;
        let context = DetailContext {
            distance: f32::NAN,
            ..Default::default()
        };
        let report = ai.quality(context, DetailLevel::D0, true);
        assert!(!report.pass);
        assert!(report.issues.invalid_context);
    }

    #[test]
    fn scoring_is_deterministic() {
        let ai = DetailsAi;
        let context = DetailContext {
            distance: 12.5,
            visible: true,
            camera_focused: true,
            object_importance: 0.8,
            gameplay_importance: 0.4,
            narrative_importance: 0.2,
            material_complexity: 0.9,
            performance_budget: 0.7,
            ..Default::default()
        };
        assert_eq!(ai.score(context), ai.score(context));
        assert_eq!(ai.decide(context), ai.decide(context));
    }
}
