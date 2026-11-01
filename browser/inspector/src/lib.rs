//! Browser admission and JSON projection. Existing COURT methods own queries.
use court_core::*;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use wasm_bindgen::prelude::*;

pub const MAX_BYTES: usize = 524_288;
pub const MAX_ITEMS: usize = 256;

macro_rules! mapped_enum {
    ($name:ident => $native:ident { $($variant:ident),+ $(,)? }) => {
        #[derive(Debug, Clone, Copy, Deserialize, Serialize)]
        enum $name { $($variant),+ }
        impl From<$name> for $native {
            fn from(value: $name) -> Self { match value { $($name::$variant => Self::$variant),+ } }
        }
    };
}
mapped_enum!(Surface => CourtSurfaceKind { Terminal, Browser, Native2d, AuthoredScene });
mapped_enum!(Role => CourtSceneRole { Surface, Zone, Actor, Prop, Hud, Text, Media, Control, Boundary });
mapped_enum!(SourceClass => CourtProvenanceClass { ProductAuthored, OcwDerived, MetadataOnly, LocalCache, ExternalBoundary });

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Provenance {
    class: SourceClass,
    source_id: Option<String>,
}
impl From<Provenance> for CourtProvenance {
    fn from(v: Provenance) -> Self {
        Self {
            class: v.class.into(),
            source_id: v.source_id,
        }
    }
}
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Intent {
    product_owner: String,
    audience: String,
    design_thesis: String,
    non_goals: Vec<String>,
}
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Experience {
    id: String,
    title: String,
    surface: Surface,
    intent: Intent,
    provenance: Provenance,
}
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Metadata {
    experience_id: String,
    experience_version: String,
    surface: Surface,
    scene_contract_version: String,
}
#[derive(Deserialize, Serialize)]
#[serde(tag = "kind", deny_unknown_fields)]
enum Availability {
    Legal {},
    Unavailable { reason: String },
    GuidedIllegal { guidance: String },
    Destructive { warning: String },
    Diagnostic { note: String },
}
impl From<Availability> for CourtActionAvailability {
    fn from(v: Availability) -> Self {
        match v {
            Availability::Legal {} => Self::Legal,
            Availability::Unavailable { reason } => Self::Unavailable { reason },
            Availability::GuidedIllegal { guidance } => Self::GuidedIllegal { guidance },
            Availability::Destructive { warning } => Self::Destructive { warning },
            Availability::Diagnostic { note } => Self::Diagnostic { note },
        }
    }
}
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Action {
    id: String,
    label: String,
    command: String,
    availability: Availability,
}
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Feature {
    feature: String,
    fallback: String,
}
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Node {
    id: String,
    label: String,
    player_read_label: String,
    product_meaning: String,
    role: Role,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    provenance: Option<Provenance>,
    unsupported_features: Vec<Feature>,
}
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Input {
    metadata: Metadata,
    experience: Experience,
    state_label: String,
    actions: Vec<Action>,
    scene: Vec<Node>,
}
impl Input {
    fn into_snapshot(self) -> CourtSnapshot {
        CourtSnapshot {
            metadata: CourtSnapshotMetadata {
                experience_id: self.metadata.experience_id,
                experience_version: self.metadata.experience_version,
                surface: self.metadata.surface.into(),
                scene_contract_version: self.metadata.scene_contract_version,
            },
            experience: CourtExperience {
                id: self.experience.id,
                title: self.experience.title,
                surface: self.experience.surface.into(),
                intent: CourtExperienceIntent {
                    product_owner: self.experience.intent.product_owner,
                    audience: self.experience.intent.audience,
                    design_thesis: self.experience.intent.design_thesis,
                    non_goals: self.experience.intent.non_goals,
                },
                provenance: self.experience.provenance.into(),
            },
            state_label: self.state_label,
            actions: self
                .actions
                .into_iter()
                .map(|a| CourtAction {
                    id: a.id,
                    label: a.label,
                    command: a.command,
                    availability: a.availability.into(),
                })
                .collect(),
            scene: self
                .scene
                .into_iter()
                .map(|n| CourtSceneNode {
                    id: n.id,
                    label: n.label,
                    player_read_label: n.player_read_label,
                    product_meaning: n.product_meaning,
                    role: n.role.into(),
                    x: n.x,
                    y: n.y,
                    width: n.width,
                    height: n.height,
                    provenance: n.provenance.map(Into::into),
                    unsupported_features: n
                        .unsupported_features
                        .into_iter()
                        .map(|f| CourtUnsupportedFeatureHint {
                            feature: f.feature,
                            fallback: f.fallback,
                        })
                        .collect(),
                })
                .collect(),
        }
    }
}

fn evaluate(text: &str) -> Result<Value, String> {
    if text.len() > MAX_BYTES {
        return Err(format!(
            "Input exceeds the adapter limit of {MAX_BYTES} UTF-8 bytes."
        ));
    }
    let input: Input = serde_json::from_str(text).map_err(|e| format!("Snapshot JSON: {e}"))?;
    if input.actions.len() > MAX_ITEMS || input.scene.len() > MAX_ITEMS {
        return Err(format!(
            "Adapter admits at most {MAX_ITEMS} actions and {MAX_ITEMS} scene nodes."
        ));
    }
    // Preserve admitted fields in the report; provenance is caller supplied, never fetched.
    let admitted = serde_json::to_value(&input).map_err(|e| e.to_string())?;
    let snapshot = input.into_snapshot();
    let roles = [
        Role::Surface,
        Role::Zone,
        Role::Actor,
        Role::Prop,
        Role::Hud,
        Role::Text,
        Role::Media,
        Role::Control,
        Role::Boundary,
    ];
    let present_roles: Vec<_> = roles
        .into_iter()
        .filter(|r| snapshot.has_scene_role((*r).into()))
        .collect();
    let actions: Vec<_> = snapshot
        .actions
        .iter()
        .map(|a| {
            json!({
                "id": a.id, "label": a.label, "command": a.command,
                "player_available": a.is_player_available(),
            })
        })
        .collect();
    let features: Vec<_> = snapshot
        .unsupported_scene_features()
        .map(|f| json!({"feature": f.feature, "fallback": f.fallback}))
        .collect();
    let mut observations = Vec::new();
    if snapshot.metadata.experience_id != snapshot.experience.id {
        observations.push("Declared metadata and experience IDs differ.");
    }
    if snapshot.metadata.surface != snapshot.experience.surface {
        observations.push("Declared metadata and experience surfaces differ.");
    }
    if snapshot.scene.iter().any(|n| n.width < 0 || n.height < 0) {
        observations.push(
            "Some scene extents are negative; the existing native query methods still retain them.",
        );
    }
    Ok(json!({
        "ok": true,
        "scope": "Snapshot queries only. No actions executed, renderer compatibility certified, provenance verified, or product/assessment success established.",
        "input_bytes": text.len(), "snapshot": admitted,
        "available_commands": snapshot.available_commands().collect::<Vec<_>>(),
        "actions": actions, "present_roles": present_roles,
        "unsupported_features": features, "adapter_observations": observations,
    }))
}

/// Exact same JSON entry point for the native CLI and worker WASM.
#[wasm_bindgen]
pub fn inspect_snapshot(text: &str) -> String {
    let value = evaluate(text).unwrap_or_else(|message| json!({"ok": false, "error": message}));
    serde_json::to_string_pretty(&value).expect("JSON values serialize")
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("../fixtures/snapshot.json");
    fn report(text: &str) -> Value {
        serde_json::from_str(&inspect_snapshot(text)).unwrap()
    }
    #[test]
    fn all_availability_states_use_native_order_and_semantics() {
        let r = report(EXAMPLE);
        assert_eq!(r["ok"], true);
        assert_eq!(r["available_commands"], json!(["inspect", "reset"]));
        assert_eq!(
            r["actions"]
                .as_array()
                .unwrap()
                .iter()
                .map(|v| v["player_available"].as_bool().unwrap())
                .collect::<Vec<_>>(),
            vec![true, false, false, true, false]
        );
        assert_eq!(r["present_roles"], json!(["Surface", "Control"]));
        assert_eq!(r["unsupported_features"][0]["fallback"], "Text label");
    }
    #[test]
    fn own_inputs_keep_duplicates_order_and_negative_extents() {
        let mut v: Value = serde_json::from_str(EXAMPLE).unwrap();
        v["actions"][1] = v["actions"][0].clone();
        v["scene"][0]["width"] = json!(-1);
        v["metadata"]["experience_id"] = json!("different");
        let r = report(&v.to_string());
        assert_eq!(
            r["available_commands"],
            json!(["inspect", "inspect", "reset"])
        );
        assert_eq!(r["snapshot"]["scene"][0]["width"], -1);
        assert_eq!(r["adapter_observations"].as_array().unwrap().len(), 2);
    }
    #[test]
    fn strict_admission_rejects_unknown_fields_enums_and_numeric_overflow() {
        for (key, value) in [
            ("extra", json!(true)),
            ("width", json!(2147483648i64)),
            ("role", json!("Camera")),
        ] {
            let mut v: Value = serde_json::from_str(EXAMPLE).unwrap();
            v["scene"][0][key] = value;
            assert_eq!(report(&v.to_string())["ok"], false);
        }
        let mut v: Value = serde_json::from_str(EXAMPLE).unwrap();
        v["actions"][0]["availability"]["warning"] = json!("unexpected");
        assert_eq!(report(&v.to_string())["ok"], false);
    }
    #[test]
    fn resource_limits_are_adapter_errors_not_core_policy() {
        assert_eq!(report(&" ".repeat(MAX_BYTES + 1))["ok"], false);
        let mut v: Value = serde_json::from_str(EXAMPLE).unwrap();
        v["actions"] = json!(vec![v["actions"][0].clone(); MAX_ITEMS + 1]);
        assert_eq!(report(&v.to_string())["ok"], false);
        assert_eq!(report("")["ok"], false);
    }
    #[test]
    fn empty_snapshot_and_inert_source_strings_are_retained() {
        let mut v: Value = serde_json::from_str(EXAMPLE).unwrap();
        v["actions"] = json!([]);
        v["scene"] = json!([]);
        v["experience"]["provenance"]["source_id"] = json!("javascript:alert(1)");
        let r = report(&v.to_string());
        assert_eq!(r["ok"], true);
        assert_eq!(r["available_commands"], json!([]));
        assert_eq!(r["present_roles"], json!([]));
        assert_eq!(
            r["snapshot"]["experience"]["provenance"]["source_id"],
            "javascript:alert(1)"
        );
    }
}
