//! Product-neutral contracts for scalable experiences.
//!
//! COURT defines the portable state/action/snapshot/scene boundary that lets
//! products scale across terminal, browser, native, and authored-scene surfaces.

use rune_core::{ContractRegistration, DescriptorCollectionDocument, RuneContract};
use rune_derive::RuneContract as DeriveRuneContract;

pub const RUNE_COLLECTION_ID: &str = "court.experience_contracts";
pub const RUNE_COLLECTION_VERSION: &str = "v0";

#[derive(Debug, Clone, PartialEq, Eq, DeriveRuneContract)]
#[rune(
    id = "court.experience",
    version = "v0",
    kind = "entity",
    requirement = "RUNE-REQ-076",
    invariant(id = "court.experience.id.present", text = "id is not empty"),
    extension(
        namespace = "court.experience",
        name = "adoption_lane",
        value = "second_games_spike"
    )
)]
pub struct CourtExperience {
    #[rune_field(
        required = true,
        sensitivity = "public",
        example = "demo",
        stability = "stable"
    )]
    pub id: String,
    #[rune_field(
        required = true,
        sensitivity = "public",
        example = "Demo",
        stability = "stable"
    )]
    pub title: String,
    #[rune_field(
        required = true,
        sensitivity = "public",
        example = "Native2d",
        stability = "stable"
    )]
    pub surface: CourtSurfaceKind,
    #[rune_field(
        required = true,
        sensitivity = "public",
        example = "CourtExperienceIntent",
        stability = "stable"
    )]
    pub intent: CourtExperienceIntent,
    #[rune_field(
        required = true,
        sensitivity = "public",
        example = "product-authored",
        stability = "stable"
    )]
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

#[derive(Debug, Clone, PartialEq, Eq, DeriveRuneContract)]
#[rune(
    id = "court.action",
    version = "v0",
    kind = "command",
    requirement = "RUNE-REQ-076",
    invariant(id = "court.action.id.present", text = "id is not empty"),
    invariant(id = "court.action.command.present", text = "command is not empty")
)]
pub struct CourtAction {
    #[rune_field(
        required = true,
        sensitivity = "public",
        example = "inspect-door",
        stability = "stable"
    )]
    pub id: String,
    #[rune_field(
        required = true,
        sensitivity = "public",
        example = "Inspect door",
        stability = "stable"
    )]
    pub label: String,
    #[rune_field(
        required = true,
        sensitivity = "public",
        example = "inspect door",
        stability = "stable",
        alias = "input"
    )]
    pub command: String,
    #[rune_field(
        required = true,
        sensitivity = "public",
        example = "Legal",
        stability = "stable"
    )]
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

#[derive(Debug, Clone, PartialEq, Eq, DeriveRuneContract)]
#[rune(
    id = "court.scene_node",
    version = "v0",
    kind = "entity",
    requirement = "RUNE-REQ-076",
    invariant(id = "court.scene_node.id.present", text = "id is not empty")
)]
pub struct CourtSceneNode {
    #[rune_field(
        required = true,
        sensitivity = "public",
        example = "door",
        stability = "stable"
    )]
    pub id: String,
    #[rune_field(
        required = true,
        sensitivity = "public",
        example = "Door",
        stability = "stable"
    )]
    pub label: String,
    #[rune_field(
        required = true,
        sensitivity = "public",
        example = "A locked door",
        stability = "stable"
    )]
    pub player_read_label: String,
    #[rune_field(
        required = true,
        sensitivity = "public",
        example = "blocks progression",
        stability = "stable"
    )]
    pub product_meaning: String,
    #[rune_field(
        required = true,
        sensitivity = "public",
        example = "Prop",
        stability = "stable"
    )]
    pub role: CourtSceneRole,
    #[rune_field(
        required = true,
        unit = "scene-unit",
        sensitivity = "public",
        example = "0",
        stability = "stable"
    )]
    pub x: i32,
    #[rune_field(
        required = true,
        unit = "scene-unit",
        sensitivity = "public",
        example = "0",
        stability = "stable"
    )]
    pub y: i32,
    #[rune_field(
        required = true,
        unit = "scene-unit",
        min = "0",
        sensitivity = "public",
        example = "10",
        stability = "stable"
    )]
    pub width: i32,
    #[rune_field(
        required = true,
        unit = "scene-unit",
        min = "0",
        sensitivity = "public",
        example = "10",
        stability = "stable"
    )]
    pub height: i32,
    #[rune_field(
        required = false,
        sensitivity = "public",
        example = "none",
        stability = "stable"
    )]
    pub provenance: Option<CourtProvenance>,
    #[rune_field(
        required = true,
        sensitivity = "public",
        example = "[]",
        stability = "stable"
    )]
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

#[derive(Debug, Clone, PartialEq, Eq, DeriveRuneContract)]
#[rune(
    id = "court.snapshot",
    version = "v0",
    kind = "state",
    requirement = "RUNE-REQ-076",
    invariant(
        id = "court.snapshot.state_label.present",
        text = "state_label is not empty"
    )
)]
pub struct CourtSnapshot {
    #[rune_field(
        required = true,
        sensitivity = "public",
        example = "CourtSnapshotMetadata",
        stability = "stable"
    )]
    pub metadata: CourtSnapshotMetadata,
    #[rune_field(
        required = true,
        sensitivity = "public",
        example = "CourtExperience",
        stability = "stable"
    )]
    pub experience: CourtExperience,
    #[rune_field(
        required = true,
        sensitivity = "public",
        example = "intro",
        stability = "stable"
    )]
    pub state_label: String,
    #[rune_field(
        required = true,
        sensitivity = "public",
        example = "[]",
        stability = "stable"
    )]
    pub actions: Vec<CourtAction>,
    #[rune_field(
        required = true,
        sensitivity = "public",
        example = "[]",
        stability = "stable"
    )]
    pub scene: Vec<CourtSceneNode>,
}

#[derive(Debug, Clone, PartialEq, Eq, DeriveRuneContract)]
#[rune(
    id = "court.snapshot_metadata",
    version = "v0",
    kind = "entity",
    requirement = "RUNE-REQ-076",
    invariant(
        id = "court.snapshot_metadata.experience_id.present",
        text = "experience_id is not empty"
    )
)]
pub struct CourtSnapshotMetadata {
    #[rune_field(
        required = true,
        sensitivity = "public",
        example = "demo",
        stability = "stable"
    )]
    pub experience_id: String,
    #[rune_field(
        required = true,
        sensitivity = "public",
        example = "0.1.0",
        stability = "stable"
    )]
    pub experience_version: String,
    #[rune_field(
        required = true,
        sensitivity = "public",
        example = "Native2d",
        stability = "stable"
    )]
    pub surface: CourtSurfaceKind,
    #[rune_field(
        required = true,
        sensitivity = "public",
        example = "court.scene.v1",
        stability = "stable"
    )]
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

#[derive(Debug, Clone, PartialEq, Eq, DeriveRuneContract)]
#[rune(
    id = "court.validation_packet",
    version = "v0",
    kind = "artifact",
    requirement = "RUNE-REQ-076",
    invariant(
        id = "court.validation_packet.experience_id.present",
        text = "experience_id is not empty"
    )
)]
pub struct CourtValidationPacket {
    #[rune_field(
        required = true,
        sensitivity = "public",
        example = "demo",
        stability = "stable"
    )]
    pub experience_id: String,
    #[rune_field(
        required = true,
        sensitivity = "public",
        example = "[]",
        stability = "stable"
    )]
    pub prototype_revisions: Vec<CourtPrototypeRevision>,
    #[rune_field(
        required = true,
        sensitivity = "public",
        example = "[]",
        stability = "stable"
    )]
    pub evidence_references: Vec<CourtEvidenceReference>,
    #[rune_field(
        required = true,
        sensitivity = "public",
        example = "[]",
        stability = "stable"
    )]
    pub playtest_sessions: Vec<CourtPlaytestSession>,
    #[rune_field(
        required = true,
        sensitivity = "public",
        example = "[]",
        stability = "stable"
    )]
    pub critique_findings: Vec<CourtCritiqueFinding>,
    #[rune_field(
        required = true,
        sensitivity = "public",
        example = "[]",
        stability = "stable"
    )]
    pub focus_test_findings: Vec<CourtFocusTestFinding>,
    #[rune_field(
        required = true,
        sensitivity = "public",
        example = "[]",
        stability = "stable"
    )]
    pub assessment_targets: Vec<CourtAssessmentTarget>,
    #[rune_field(
        required = true,
        sensitivity = "public",
        example = "[]",
        stability = "stable"
    )]
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

    pub fn has_evidence_reference(&self, owner_repo: &str, artifact_ref: &str) -> bool {
        self.evidence_references.iter().any(|reference| {
            reference.owner_repo == owner_repo && reference.artifact_ref == artifact_ref
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, DeriveRuneContract)]
#[rune(
    id = "court.evidence_reference",
    version = "v0",
    kind = "evidence",
    requirement = "RUNE-REQ-076",
    invariant(
        id = "court.evidence_reference.artifact_ref.present",
        text = "artifact_ref is not empty"
    )
)]
pub struct CourtEvidenceReference {
    #[rune_field(
        required = true,
        sensitivity = "public",
        example = "RALLY",
        stability = "stable"
    )]
    pub owner_repo: String,
    #[rune_field(
        required = true,
        sensitivity = "public",
        example = "docs/rune/simulation_contracts.json",
        stability = "stable"
    )]
    pub artifact_ref: String,
    #[rune_field(
        required = true,
        sensitivity = "public",
        example = "RallyValidation",
        stability = "stable"
    )]
    pub evidence_kind: CourtEvidenceKind,
    #[rune_field(
        required = true,
        sensitivity = "public",
        example = "RUNE contract evidence",
        stability = "stable"
    )]
    pub summary: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CourtEvidenceKind {
    ProductPlaytest,
    MuddlePathTest,
    RallyValidation,
    PersonaHarness,
    ExternalReport,
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

pub const RUNE_CONTRACTS: &[ContractRegistration] = &[
    ContractRegistration {
        name: "CourtExperience",
        descriptor: CourtExperience::descriptor,
    },
    ContractRegistration {
        name: "CourtAction",
        descriptor: CourtAction::descriptor,
    },
    ContractRegistration {
        name: "CourtSceneNode",
        descriptor: CourtSceneNode::descriptor,
    },
    ContractRegistration {
        name: "CourtSnapshot",
        descriptor: CourtSnapshot::descriptor,
    },
    ContractRegistration {
        name: "CourtSnapshotMetadata",
        descriptor: CourtSnapshotMetadata::descriptor,
    },
    ContractRegistration {
        name: "CourtValidationPacket",
        descriptor: CourtValidationPacket::descriptor,
    },
    ContractRegistration {
        name: "CourtEvidenceReference",
        descriptor: CourtEvidenceReference::descriptor,
    },
];

pub fn rune_descriptor_collection() -> Result<DescriptorCollectionDocument, String> {
    DescriptorCollectionDocument::from_registrations(
        RUNE_COLLECTION_ID,
        RUNE_COLLECTION_VERSION,
        RUNE_CONTRACTS,
        "COURT-RUNE-001",
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;

    fn readiness_boundaries() -> Value {
        serde_json::from_str(include_str!(
            "../../../docs/court-readiness-boundaries.v1.json"
        ))
        .expect("readiness boundary manifest must remain valid JSON")
    }

    fn pitfall_boundary<'a>(manifest: &'a Value, pitfall: &str) -> &'a Value {
        manifest["pitfall_boundaries"]
            .as_array()
            .expect("manifest should retain pitfall boundaries")
            .iter()
            .find(|boundary| boundary["pitfall"] == pitfall)
            .unwrap_or_else(|| panic!("missing boundary for {pitfall}"))
    }

    fn array_contains(value: &Value, needle: &str) -> bool {
        value
            .as_array()
            .expect("expected JSON array")
            .iter()
            .any(|entry| entry.as_str() == Some(needle))
    }

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
            evidence_references: vec![CourtEvidenceReference {
                owner_repo: "PRODUCT".to_string(),
                artifact_ref: "tests::opening_path".to_string(),
                evidence_kind: CourtEvidenceKind::MuddlePathTest,
                summary:
                    "Product-owned test proves the first-move path without storing transcript details in COURT."
                        .to_string(),
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
        assert!(packet.has_evidence_reference("PRODUCT", "tests::opening_path"));
    }

    #[test]
    fn contract_proof_does_not_claim_product_readiness() {
        // Checks COURT-PF-01.
        let readme = include_str!("../../../README.md");
        let playtest_contract = include_str!("../../../specs/playtest-validation-contract.md");
        let closeout =
            include_str!("../../../specs/role-reviews/foundation-closeout-2026-05-18.md");
        let manifest = readiness_boundaries();
        let boundary = pitfall_boundary(&manifest, "COURT-PF-01");

        assert!(readme.contains("not for absorbing MUDDLE clients"));
        assert!(playtest_contract.contains("Product repos own player-study details"));
        assert!(closeout.contains("Declaring product migration ready"));
        assert_eq!(boundary["required_owner"], "Experience Assessment Reviewer");
        for blocked in [
            "product readiness",
            "player comprehension proven",
            "player enjoyment proven",
            "learning outcome proven",
            "release quality proven",
            "customer-ready experience",
            "product-owned playtest complete",
        ] {
            assert!(array_contains(&boundary["blocked_claims"], blocked));
        }
        for required in [
            "product-owned playtest script",
            "product-owned player findings",
            "critique disposition",
            "assessment pass/fail rule",
            "release gate owner",
            "product repository acceptance",
        ] {
            assert!(array_contains(
                &boundary["required_product_readiness_evidence"],
                required
            ));
        }
    }

    #[test]
    fn migration_requires_product_need_and_rehearsal() {
        // Checks COURT-PF-05.
        let readme = include_str!("../../../README.md");
        let plan = include_str!("../../../PRODUCT_PLAN.md");
        let compatibility = include_str!("../../../docs/compatibility.md");
        let foundation = include_str!("../../../specs/experience-framework-foundation.md");
        let manifest = readiness_boundaries();
        let boundary = pitfall_boundary(&manifest, "COURT-PF-05");

        assert!(readme.contains("not for absorbing MUDDLE clients"));
        assert!(plan.contains("does not replace MUDDLE"));
        assert!(compatibility.contains("downstream rehearsal"));
        assert!(foundation.contains("product rules"));
        assert_eq!(boundary["required_owner"], "Framework Steward");
        for blocked in [
            "big-bang MUDDLE migration",
            "RALLY report absorption",
            "product rule ownership",
            "runtime behavior ownership",
            "persistence ownership",
            "migration inevitability",
            "downstream adoption approval",
        ] {
            assert!(array_contains(&boundary["blocked_claims"], blocked));
        }
        for gate in [
            "real product fixture",
            "neutral contract expression",
            "adapter rehearsal",
            "existing behavior preservation",
            "concrete product benefit",
            "affected repo acceptance",
        ] {
            assert!(array_contains(&boundary["required_migration_gates"], gate));
        }
    }

    #[test]
    fn rune_contract_registry_preserves_experience_metadata() {
        let collection = rune_descriptor_collection().expect("rune descriptor collection");

        assert_eq!(collection.collection_id, RUNE_COLLECTION_ID);
        assert_eq!(collection.descriptors[0].id, "court.experience");
        assert_eq!(
            collection.descriptors[0].fields[0].metadata.required,
            Some(true)
        );
        assert_eq!(
            collection.descriptors[1].fields[2].metadata.aliases[0],
            "input"
        );
        assert_eq!(
            collection.descriptors[2].fields[5].metadata.unit,
            Some("scene-unit".to_owned())
        );
    }

    #[test]
    fn rune_contract_registry_matches_retained_fixture() {
        let collection = rune_descriptor_collection().expect("rune descriptor collection");
        let actual = serde_json::to_string_pretty(&collection).expect("serialize rune collection");
        let expected = include_str!("../../../docs/rune/experience_contracts.json");

        assert_eq!(normalize_newlines(&actual), normalize_newlines(expected));
    }

    fn normalize_newlines(value: &str) -> String {
        value.replace("\r\n", "\n").trim_end().to_owned()
    }
}
