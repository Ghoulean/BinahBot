use std::fmt;
use std::str::FromStr;

use ruina_common::game_objects::common::PageType;

// todo: figure out how to combine these two
// or just stick with String instead of str
#[derive(Clone, Eq, Hash, PartialEq, Debug)]
pub struct TypedId<'a>(pub PageType, pub &'a str);

#[derive(Clone, Eq, Hash, PartialEq, Debug)]
pub struct ParsedTypedId(pub PageType, pub String);

impl fmt::Display for TypedId<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{}{}", self.0, self.1))
    }
}

impl fmt::Display for ParsedTypedId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{}{}", self.0, self.1))
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

impl From<TypedId<'_>> for ParsedTypedId {
    fn from(value: TypedId) -> Self {
        ParsedTypedId(value.0, value.1.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanity_typed_id_display() {
        let under_test = TypedId(PageType::AbnoPage, "123");
        let format = format!("{}", under_test);
        assert_eq!(format, "a#123");
    }

    #[test]
    fn sanity_parsed_typed_id_fromstr() {
        let under_test = "a#123";
        let parsed_typed_id = ParsedTypedId::from_str(under_test).expect("should not fail");
        assert_eq!(parsed_typed_id.0, PageType::AbnoPage);
        assert_eq!(parsed_typed_id.1, "123");
    }
}
