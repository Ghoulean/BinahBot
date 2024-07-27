#[derive(Debug)]
pub struct Defenses {
    red: Resistance,
    white: Resistance,
    black: Resistance,
    pale: Resistance
}

#[derive(Debug, PartialEq, strum_macros::Display)]
pub enum DamageType {
    Red,
    White,
    Black,
    Pale,
}

#[derive(Debug)]
pub struct Resistance(f64);

#[derive(Debug, PartialEq, strum_macros::Display)]
pub enum ResistanceCategories {
    Absorb,
    Immune,
    Resistant,
    Endured,
    Normal,
    Weak,
    Vulnerable
}

#[derive(Debug, PartialEq, strum_macros::Display)]
pub enum RiskLevel {
    Zayin,
    Teth,
    He,
    Waw,
    Aleph,
}

#[derive(Debug)]
pub struct StatBonus(Stat, i32);

#[derive(Debug, PartialEq, strum_macros::Display)]
pub enum Stat {
    Hp, Sanity, MovementSpeed, AttackSpeed, SuccessRate, WorkSpeed,
}

impl From<f64> for ResistanceCategories {
    fn from(value: f64) -> Self {
        if value < 0.0 {
            ResistanceCategories::Absorb
        } else if value == 0.0 {
            ResistanceCategories::Immune
        } else if value < 0.5 {
            ResistanceCategories::Resistant
        } else if value < 1.0 {
            ResistanceCategories::Endured
        } else if value == 1.0 {
            ResistanceCategories::Normal
        } else if value < 2.0 {
            ResistanceCategories::Weak
        } else {
            ResistanceCategories::Vulnerable
        }
    }
}