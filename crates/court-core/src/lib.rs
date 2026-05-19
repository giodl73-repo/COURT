//! Product-neutral contracts for scalable experiences.
//!
//! COURT defines the portable state/action/snapshot/scene boundary that lets
//! products scale across terminal, browser, native, and authored-scene surfaces.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CourtExperience {
    pub id: String,
    pub title: String,
    pub surface: CourtSurfaceKind,
    pub intent: CourtExperienceIntent,
    pub provenance: CourtProvenance,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CourtExperienceIntent {
    pub product_owner: String,
    pub audience: String,
    pub design_thesis: String,
    pub non_goals: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CourtSurfaceKind {
    Terminal,
    Browser,
    Native2d,
    AuthoredScene,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CourtProvenance {
    pub class: CourtProvenanceClass,
    pub source_id: Option<String>,
}

impl CourtProvenance {
    pub fn product_authored(source_id: impl Into<String>) -> Self {
        Self {
            class: CourtProvenanceClass::ProductAuthored,
            source_id: Some(source_id.into()),
        }
    }

    pub fn external_boundary(source_id: impl Into<String>) -> Self {
        Self {
            class: CourtProvenanceClass::ExternalBoundary,
            source_id: Some(source_id.into()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CourtProvenanceClass {
    ProductAuthored,
    OcwDerived,
    MetadataOnly,
    LocalCache,
    ExternalBoundary,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CourtAction {
    pub id: String,
    pub label: String,
    pub command: String,
    pub availability: CourtActionAvailability,
}

impl CourtAction {
    pub fn is_player_available(&self) -> bool {
        self.availability.is_player_available()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CourtActionAvailability {
    Legal,
    Unavailable { reason: String },
    GuidedIllegal { guidance: String },
    Destructive { warning: String },
    Diagnostic { note: String },
}

impl CourtActionAvailability {
    pub fn is_player_available(&self) -> bool {
        matches!(self, Self::Legal | Self::Destructive { .. })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CourtSceneNode {
    pub id: String,
    pub label: String,
    pub player_read_label: String,
    pub product_meaning: String,
    pub role: CourtSceneRole,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub provenance: Option<CourtProvenance>,
    pub unsupported_features: Vec<CourtUnsupportedFeatureHint>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CourtUnsupportedFeatureHint {
    pub feature: String,
    pub fallback: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CourtSceneRole {
    Surface,
    Zone,
    Actor,
    Prop,
    Hud,
    Text,
    Media,
    Control,
    Boundary,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CourtSnapshot {
    pub metadata: CourtSnapshotMetadata,
    pub experience: CourtExperience,
    pub state_label: String,
    pub actions: Vec<CourtAction>,
    pub scene: Vec<CourtSceneNode>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CourtSnapshotMetadata {
    pub experience_id: String,
    pub experience_version: String,
    pub surface: CourtSurfaceKind,
    pub scene_contract_version: String,
}

impl CourtSnapshot {
    pub fn available_commands(&self) -> impl Iterator<Item = &str> {
        self.actions
            .iter()
            .filter(|action| action.is_player_available())
            .map(|action| action.command.as_str())
    }

    pub fn has_scene_role(&self, role: CourtSceneRole) -> bool {
        self.scene.iter().any(|node| node.role == role)
    }

    pub fn unsupported_scene_features(&self) -> impl Iterator<Item = &CourtUnsupportedFeatureHint> {
        self.scene
            .iter()
            .flat_map(|node| node.unsupported_features.iter())
    }
}

pub trait CourtHost {
    fn snapshot(&self) -> CourtSnapshot;
    fn apply_action(&mut self, action: &CourtAction) -> Result<CourtSnapshot, CourtHostError>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CourtHostError {
    UnknownAction(String),
    RejectedAction { action: String, reason: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CourtValidationPacket {
    pub experience_id: String,
    pub prototype_revisions: Vec<CourtPrototypeRevision>,
    pub playtest_sessions: Vec<CourtPlaytestSession>,
    pub critique_findings: Vec<CourtCritiqueFinding>,
    pub focus_test_findings: Vec<CourtFocusTestFinding>,
    pub assessment_targets: Vec<CourtAssessmentTarget>,
    pub postmortem_notes: Vec<CourtPostmortemNote>,
}

impl CourtValidationPacket {
    pub fn has_assessment_claim(&self, claim: CourtAssessmentClaim) -> bool {
        self.assessment_targets
            .iter()
            .any(|target| target.claim == claim)
    }

    pub fn finding_count(&self) -> usize {
        self.critique_findings.len() + self.focus_test_findings.len()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CourtPrototypeRevision {
    pub experience_id: String,
    pub revision_id: String,
    pub design_thesis: String,
    pub changed_areas: Vec<String>,
    pub non_goals: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CourtPlaytestSession {
    pub session_id: String,
    pub audience: String,
    pub build_revision: String,
    pub script_ref: String,
    pub observed_blockers: Vec<String>,
    pub completion_outcome: CourtCompletionOutcome,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CourtCompletionOutcome {
    Completed,
    Partial,
    Blocked,
    NotRun,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CourtCritiqueFinding {
    pub reviewer_role: String,
    pub finding_id: String,
    pub source: CourtFindingSource,
    pub severity: CourtFindingSeverity,
    pub recommendation: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CourtFindingSource {
    Experience(String),
    Action(String),
    SceneNode(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CourtFindingSeverity {
    Info,
    Warning,
    Blocking,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CourtFocusTestFinding {
    pub prompt_ref: String,
    pub action_ref: Option<String>,
    pub observed_comprehension: String,
    pub follow_up_change: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CourtAssessmentTarget {
    pub claim: CourtAssessmentClaim,
    pub evidence_needed: String,
    pub pass_fail_rule: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CourtAssessmentClaim {
    Learning,
    Impact,
    Simulation,
    Comprehension,
    EntertainmentOnly,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CourtPostmortemNote {
    pub release_id: String,
    pub worked: String,
    pub failed: String,
    pub next_design_constraint: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snapshot_exposes_actions_and_scene_roles() {
        let snapshot = CourtSnapshot {
            metadata: CourtSnapshotMetadata {
                experience_id: "demo".to_string(),
                experience_version: "0.1.0".to_string(),
                surface: CourtSurfaceKind::Native2d,
                scene_contract_version: "court.scene.v1".to_string(),
            },
            experience: CourtExperience {
                id: "demo".to_string(),
                title: "Demo".to_string(),
                surface: CourtSurfaceKind::Native2d,
                intent: CourtExperienceIntent {
                    product_owner: "COURT".to_string(),
                    audience: "Framework reviewers".to_string(),
                    design_thesis: "One contract can describe a playable experience.".to_string(),
                    non_goals: vec!["Do not own product rules.".to_string()],
                },
                provenance: CourtProvenance::product_authored("court:demo"),
            },
            state_label: "ready".to_string(),
            actions: vec![
                CourtAction {
                    id: "start".to_string(),
                    label: "Start".to_string(),
                    command: "start".to_string(),
                    availability: CourtActionAvailability::Legal,
                },
                CourtAction {
                    id: "force".to_string(),
                    label: "Force hidden diagnostic".to_string(),
                    command: "force".to_string(),
                    availability: CourtActionAvailability::Diagnostic {
                        note: "Test harness only.".to_string(),
                    },
                },
                CourtAction {
                    id: "locked".to_string(),
                    label: "Open locked door".to_string(),
                    command: "open locked door".to_string(),
                    availability: CourtActionAvailability::GuidedIllegal {
                        guidance: "Find the key first.".to_string(),
                    },
                },
            ],
            scene: vec![CourtSceneNode {
                id: "court".to_string(),
                label: "Court surface".to_string(),
                player_read_label: "A playable court surface".to_string(),
                product_meaning: "The neutral play area for framework validation.".to_string(),
                role: CourtSceneRole::Surface,
                x: 0,
                y: 0,
                width: 12,
                height: 8,
                provenance: Some(CourtProvenance::product_authored("court:demo:surface")),
                unsupported_features: vec![CourtUnsupportedFeatureHint {
                    feature: "surface-texture".to_string(),
                    fallback: "Use flat court color.".to_string(),
                }],
            }],
        };

        assert_eq!(
            snapshot.available_commands().collect::<Vec<_>>(),
            vec!["start"]
        );
        assert_eq!(
            snapshot.experience.intent.non_goals,
            vec!["Do not own product rules."]
        );
        assert_eq!(
            snapshot.experience.provenance.class,
            CourtProvenanceClass::ProductAuthored
        );
        assert_eq!(snapshot.metadata.experience_id, snapshot.experience.id);
        assert_eq!(snapshot.metadata.scene_contract_version, "court.scene.v1");
        assert_eq!(
            snapshot.scene[0].player_read_label,
            "A playable court surface"
        );
        assert_eq!(
            snapshot.scene[0].product_meaning,
            "The neutral play area for framework validation."
        );
        assert_eq!(snapshot.unsupported_scene_features().count(), 1);
        assert!(snapshot.has_scene_role(CourtSceneRole::Surface));
        assert!(!snapshot.has_scene_role(CourtSceneRole::Actor));
    }

    #[test]
    fn action_availability_names_contract_shape_without_policy() {
        assert!(CourtActionAvailability::Legal.is_player_available());
        assert!(CourtActionAvailability::Destructive {
            warning: "This cannot be undone.".to_string()
        }
        .is_player_available());
        assert!(!CourtActionAvailability::Unavailable {
            reason: "Not enough focus.".to_string()
        }
        .is_player_available());
        assert!(!CourtActionAvailability::GuidedIllegal {
            guidance: "Try a legal move first.".to_string()
        }
        .is_player_available());
        assert!(!CourtActionAvailability::Diagnostic {
            note: "Harness only.".to_string()
        }
        .is_player_available());
    }

    #[test]
    fn validation_packet_references_evidence_without_private_details() {
        let packet = CourtValidationPacket {
            experience_id: "demo".to_string(),
            prototype_revisions: vec![CourtPrototypeRevision {
                experience_id: "demo".to_string(),
                revision_id: "rev-001".to_string(),
                design_thesis: "Players should understand the first move.".to_string(),
                changed_areas: vec!["opening-action-copy".to_string()],
                non_goals: vec!["No renderer migration.".to_string()],
            }],
            playtest_sessions: vec![CourtPlaytestSession {
                session_id: "playtest-001".to_string(),
                audience: "first-time players".to_string(),
                build_revision: "rev-001".to_string(),
                script_ref: "product-owned-script-001".to_string(),
                observed_blockers: vec!["opening action unclear".to_string()],
                completion_outcome: CourtCompletionOutcome::Partial,
            }],
            critique_findings: vec![CourtCritiqueFinding {
                reviewer_role: "Game Design Methods Reviewer".to_string(),
                finding_id: "finding-001".to_string(),
                source: CourtFindingSource::Action("start".to_string()),
                severity: CourtFindingSeverity::Warning,
                recommendation: "Add clearer action guidance.".to_string(),
            }],
            focus_test_findings: vec![CourtFocusTestFinding {
                prompt_ref: "prompt-001".to_string(),
                action_ref: Some("start".to_string()),
                observed_comprehension: "Player understood the goal after hint copy.".to_string(),
                follow_up_change: "Keep hint copy in product-owned UI.".to_string(),
            }],
            assessment_targets: vec![CourtAssessmentTarget {
                claim: CourtAssessmentClaim::Comprehension,
                evidence_needed: "Player can name the next goal from the action list.".to_string(),
                pass_fail_rule: "Pass when no blocking confusion remains.".to_string(),
            }],
            postmortem_notes: vec![CourtPostmortemNote {
                release_id: "release-001".to_string(),
                worked: "Action labels were readable.".to_string(),
                failed: "Initial objective was too implicit.".to_string(),
                next_design_constraint: "Keep objective text near available actions.".to_string(),
            }],
        };

        assert!(packet.has_assessment_claim(CourtAssessmentClaim::Comprehension));
        assert!(!packet.has_assessment_claim(CourtAssessmentClaim::Learning));
        assert_eq!(packet.finding_count(), 2);
        assert_eq!(
            packet.playtest_sessions[0].script_ref,
            "product-owned-script-001"
        );
    }
}
