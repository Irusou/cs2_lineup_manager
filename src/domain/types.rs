use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Map {
    Mirage,
    Dust2,
    Nuke,
    Ancient,
    Anubis,
    Inferno,
    Cache,
    Overpass,
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Mirage => write!(f, "Mirage"),
            Self::Dust2 => write!(f, "Dust2"),
            Self::Nuke => write!(f, "Nuke"),
            Self::Ancient => write!(f, "Ancient"),
            Self::Anubis => write!(f, "Anubis"),
            Self::Inferno => write!(f, "Inferno"),
            Self::Cache => write!(f, "Cache"),
            Self::Overpass => write!(f, "Overpass"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum UtilityKind {
    Flash,
    HeGrenade,
    Smoke,
    Molotov,
    Incendiary,
    Decoy,
}

impl std::fmt::Display for UtilityKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Flash => write!(f, "Flashbang"),
            Self::HeGrenade => write!(f, "Grenade"),
            Self::Smoke => write!(f, "Smoke"),
            Self::Molotov => write!(f, "Molotov"),
            Self::Incendiary => write!(f, "Incendiary"),
            Self::Decoy => write!(f, "Decoy"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Side {
    T,
    CT,
}

impl std::fmt::Display for Side {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::T => write!(f, "Terrorist"),
            Self::CT => write!(f, "Counter Terrorist"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ThrowType {
    Stand,
    Jumpthrow,
    WJumpthrow,
    Crouch,
    CrouchJumpthrow,
}

impl std::fmt::Display for ThrowType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Stand => write!(f, "Stand throw"),
            Self::Jumpthrow => write!(f, "Jumpthrow"),
            Self::WJumpthrow => write!(f, "W + Jumpthrow"),
            Self::Crouch => write!(f, "Crouch throw"),
            Self::CrouchJumpthrow => write!(f, "Crouch + Jumpthrow"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Role {
    Position, // where to stand
    Aim,      // where to look
    Outcome,  // where it lands/explodes
    Other,
}

impl std::fmt::Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Position => write!(f, "Position"),
            Self::Aim => write!(f, "Aim"),
            Self::Outcome => write!(f, "Outcome"),
            Self::Other => write!(f, "Other"),
        }
    }
}
