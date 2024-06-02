use unicode_normalization::char::is_combining_mark;
use unicode_normalization::UnicodeNormalization;

use crate::Token;

fn punctuation_filter(token: &Token) -> Token {
    let str = &token.0.nfd().filter(|x| !is_combining_mark(*x)).collect::<String>();
    Token(str.to_lowercase()
        .replace("’s", "")
        .replace("'s", "")
        .replace(&['(', ')', ',', '\"', '.', ';', ':', '\'', '?', '!', '’', '~', '…', '♣', '◆'][..], "")
        .replace("<color=red>♥</color>", "")
        .replace(&['-'][..], " ")
    )
}

fn stopword_filter(token: &Token) -> bool {
    // todo: something less hardcoded and more config
    match token.0.as_str() {
        "a" | "the" | "of" => false,
        _ => true,
    }
}

pub fn filter(tokens: Vec<Token>) -> Vec<Token> {
    tokens.iter().map(
        punctuation_filter
    ).filter(
        stopword_filter
    ).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn punctuation_filter_sanity() {
        let input = Token("hello?".to_string());
        let expected = Token("hello".to_string());

        assert_eq!(expected, punctuation_filter(&input));
    }

    #[test]
    fn punctuation_filter_accents() {
        let input = Token("ChīWěn".to_string());
        let expected = Token("chiwen".to_string());

        assert_eq!(expected, punctuation_filter(&input));
    }

    #[test]
    fn stopword_filter_sanity() {
        assert!(stopword_filter(&Token("another".to_string())));
    }

    #[test]
    fn sanity() {
        let input = vec![
            Token("Trim".to_string()),
            Token("The".to_string()),
            Token("Ingredients".to_string())
        ];
        let expected = vec![
            Token("trim".to_string()),
            Token("ingredients".to_string())
        ];
        let expected = vec![Token("trim".to_string()), Token("ingredi".to_string())];
        assert_eq!(expected, filter(input));
    }
}
