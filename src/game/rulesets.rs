#[derive(Debug, Copy, Clone, PartialEq)]
pub enum FlushPrecedence {
    Suit,
    Rank
}

#[derive(Debug, Copy, Clone)]
pub struct Ruleset {
    pub reversals_enabled: bool,
    pub flush_precedence: FlushPrecedence
}

