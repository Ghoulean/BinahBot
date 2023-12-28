use std::fmt;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Locale {
    Korean,
    English,
    Japanese,
    Chinese,
    TraditionalChinese
}

impl fmt::Display for Locale {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Locale::Korean => write!(f, "Korean"),
            Locale::English => write!(f, "English"),
            Locale::Japanese => write!(f, "Japanese"),
            Locale::Chinese => write!(f, "Chinese"),
            Locale::TraditionalChinese => write!(f, "TraditionalChinese")
        }
    }
}
