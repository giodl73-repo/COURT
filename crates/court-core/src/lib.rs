//! Product-neutral contracts for scalable experiences.
//!
//! COURT defines the portable state/action/snapshot/scene boundary that lets
//! products scale across terminal, browser, native, and authored-scene surfaces.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CourtExperience {
    pub id: String,
    pub title: String,
    pub surface: CourtSurfaceKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CourtSurfaceKind {
    Terminal,
    Browser,
    Native2d,
    AuthoredScene,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CourtAction {
    pub id: String,
    pub label: String,
    pub command: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CourtSceneNode {
    pub id: String,
    pub label: String,
    pub role: CourtSceneRole,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CourtSceneRole {
    Surface,
    Zone,
    Actor,
    Prop,
    Hud,
    Text,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CourtSnapshot {
    pub experience: CourtExperience,
    pub state_label: String,
    pub actions: Vec<CourtAction>,
    pub scene: Vec<CourtSceneNode>,
}

impl CourtSnapshot {
    pub fn available_commands(&self) -> impl Iterator<Item = &str> {
        self.actions.iter().map(|action| action.command.as_str())
    }

    pub fn has_scene_role(&self, role: CourtSceneRole) -> bool {
        self.scene.iter().any(|node| node.role == role)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snapshot_exposes_actions_and_scene_roles() {
        let snapshot = CourtSnapshot {
            experience: CourtExperience {
                id: "demo".to_string(),
                title: "Demo".to_string(),
                surface: CourtSurfaceKind::Native2d,
            },
            state_label: "ready".to_string(),
            actions: vec![CourtAction {
                id: "start".to_string(),
                label: "Start".to_string(),
                command: "start".to_string(),
            }],
            scene: vec![CourtSceneNode {
                id: "court".to_string(),
                label: "Court surface".to_string(),
                role: CourtSceneRole::Surface,
                x: 0,
                y: 0,
                width: 12,
                height: 8,
            }],
        };

        assert_eq!(
            snapshot.available_commands().collect::<Vec<_>>(),
            vec!["start"]
        );
        assert!(snapshot.has_scene_role(CourtSceneRole::Surface));
        assert!(!snapshot.has_scene_role(CourtSceneRole::Actor));
    }
}
