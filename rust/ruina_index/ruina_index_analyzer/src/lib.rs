mod filters;
mod tokenizer;

use crate::filters::filter;
use crate::tokenizer::tokenize;
pub use crate::tokenizer::Token;

pub fn analyze(text: &str) -> Vec<Token> {
    filter(tokenize(text))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanity() {
        let input = "Eternally Lit Lamp";
        let expected = vec![
            Token("etern".to_string()),
            Token("lit".to_string()),
            Token("lamp".to_string()),
        ];
        assert_eq!(expected, analyze(input));
    }

    #[test]
    fn sanity_degraded_pillar() {
        let input = "degraded pillar";
        let expected = vec![Token("degrad".to_string()), Token("pillar".to_string())];
        assert_eq!(expected, analyze(input));
    }
}
