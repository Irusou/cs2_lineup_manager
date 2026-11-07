use std::fmt::Display;

use crate::domain::types::{Map, ThrowType, UtilityKind};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Lineup {
    id: Uuid,
    title: String,
    map: Map,
    kind: UtilityKind,
    throw_type: ThrowType,
    notes: String,
    tags: Vec<String>,
    images: Vec<String>,
}

impl Lineup {
    pub fn new(title: String, map: Map, kind: UtilityKind, throw_type: ThrowType) -> Self {
        Lineup {
            id: Uuid::new_v4(),
            title,
            map,
            kind,
            throw_type,
            notes: String::new(),
            tags: vec![],
            images: vec![],
        }
    }

    pub fn as_label(&self) -> String {
        format!("{}: {}\n{}", self.map, self.kind, self.title)
    }
}

impl Display for Lineup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {}\n{} - {}",
            self.map, self.kind, self.title, self.throw_type
        )
    }
}
