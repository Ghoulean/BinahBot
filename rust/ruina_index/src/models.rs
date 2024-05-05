use std::str::FromStr;
use std::fmt;

pub struct Autocomplete<'a> {
    pub base: &'a str,
    pub disambiguator: Option<&'a str>
}

pub struct DisambiguationPage<'a> {
    pub id: &'a str,
    pub typed_ids: &'a [TypedId<'a>],
    pub default: Option<&'a str>
}

#[derive(Clone, Eq, Hash, PartialEq, Debug)]
pub enum PageType {
    AbnoPageId,
    BattleSymbolId,
    CombatPageId,
    KeyPageId,
    PassiveId,
}

#[derive(Clone, Eq, Hash, PartialEq, Debug)]
pub struct TypedId<'a>(pub PageType, pub &'a str);

#[derive(Clone, Eq, Hash, PartialEq, Debug)]
pub struct ParsedTypedId(pub PageType, pub String);

impl fmt::Display for PageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            PageType::AbnoPageId => write!(f, "a#"),
            PageType::BattleSymbolId => write!(f, "b#"),
            PageType::CombatPageId => write!(f, "c#"),
            PageType::KeyPageId => write!(f, "k#"),
            PageType::PassiveId => write!(f, "p#")
        }
    }
}

impl fmt::Display for TypedId<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{}{}", self.0, self.1))
    }
}

impl FromStr for PageType {
    type Err = Box<dyn std::error::Error>;

    fn from_str(pagetype_str: &str) -> Result<Self, Self::Err> {
        match pagetype_str {
            "a#" => Ok(PageType::AbnoPageId),
            "b#" => Ok(PageType::BattleSymbolId),
            "c#" => Ok(PageType::CombatPageId),
            "k#" => Ok(PageType::KeyPageId),
            "p#" => Ok(PageType::PassiveId),
            _ => Err("unrecognized PageType")?
        }
    }
}

impl FromStr for ParsedTypedId {
    type Err = Box<dyn std::error::Error>;

    fn from_str(parsedtypeid_str: &str) -> Result<Self, Self::Err> {
        let pagetype = PageType::from_str(&parsedtypeid_str[..2])?;
        let id = &parsedtypeid_str[2..];
        Ok(ParsedTypedId(pagetype.clone(), String::from(id)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanity_typed_id_display() {
        let under_test = TypedId(PageType::AbnoPageId, "123");
        let format = format!("{}", under_test);
        assert_eq!(format, "a#123");
    }

    #[test]
    fn sanity_parsed_typed_id_fromstr() {
        let under_test = "a#123";
        let parsed_typed_id = ParsedTypedId::from_str(under_test).expect("should not fail");
        assert_eq!(parsed_typed_id.0, PageType::AbnoPageId);
        assert_eq!(parsed_typed_id.1, "123");
    }
}