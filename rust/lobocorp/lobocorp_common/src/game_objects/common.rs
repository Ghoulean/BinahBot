#[derive(Debug)]
pub struct Defenses {
    pub red: Resistance,
    pub white: Resistance,
    pub black: Resistance,
    pub pale: Resistance
}

#[derive(Debug)]
pub struct DamageRange(pub i32, pub i32);

#[derive(Debug, PartialEq, strum_macros::Display)]
pub enum DamageType {
    Red,
    White,
    Black,
    Pale,
}

#[derive(Debug)]
pub struct Resistance(pub f64);

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
pub struct StatBonus(pub Stat, pub i32);

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

impl TryFrom<i32> for RiskLevel {
    type Error = String;
    
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(RiskLevel::Zayin),
            2 => Ok(RiskLevel::Teth),
            3 => Ok(RiskLevel::He),
            4 => Ok(RiskLevel::Waw),
            5 => Ok(RiskLevel::Aleph),
            _ => Err("invalid risk level".to_string())
        }
    }
}

impl TryFrom<&str> for DamageType {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "b" => Ok(DamageType::Black),
            "r" => Ok(DamageType::Red),
            "w" => Ok(DamageType::White),
            "p" => Ok(DamageType::Pale),
            _ => Err("invalid damage type".to_string())
        }
    }
}