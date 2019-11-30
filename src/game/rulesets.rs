use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum FlushPrecedence {
    Suit,
    Rank
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Ruleset {
    pub reversals_enabled: bool,
    pub flush_precedence: FlushPrecedence
}

