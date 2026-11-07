use std::fmt::Display;

use crate::domain::types::{Map, ThrowType, UtilityKind};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Lineup {
    id: String,
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
            id: String::new(),
            title,
            map,
            kind,
            throw_type,
            notes: String::new(),
            tags: vec![],
            images: vec![],
        }
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
