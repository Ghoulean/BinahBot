#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Token(pub String);

pub fn tokenize(txt: &str) -> Vec<Token> {
    txt.split(' ').map(String::from).map(Token).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanity() {
        let input = "The Weight of Sin";
        let expected = vec![
            Token("The".to_string()),
            Token("Weight".to_string()),
            Token("of".to_string()),
            Token("Sin".to_string()),
        ];
        assert_eq!(expected, tokenize(input));
    }
}
