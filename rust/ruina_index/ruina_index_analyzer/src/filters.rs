use rust_stemmers::Algorithm;
use rust_stemmers::Stemmer;

use crate::tokenizer::Token;

fn punctuation_filter(token: &Token) -> Token {
    Token(
        token
            .0
            .to_lowercase()
            .replace("’s", "")
            .replace(
                &[
                    '(', ')', ',', '\"', '.', ';', ':', '\'', '?', '!', '’', '~', '…', '♣', '◆',
                ][..],
                "",
            )
            .replace("<color=red>♥</color>", "")
            .replace(&['-'][..], " "),
    )
}

fn stemmer_filter(token: Token) -> Token {
    // todo: lazy static
    let en_stemmer: Stemmer = Stemmer::create(Algorithm::English);
    Token(String::from(en_stemmer.stem(&token.0)))
}

fn stopword_filter(token: &Token) -> bool {
    // todo: something less hardcoded and more config
    match token.0.as_str() {
        "a" | "the" | "of" => false,
        _ => true,
    }
}

pub fn filter(tokens: Vec<Token>) -> Vec<Token> {
    tokens
        .iter()
        .map(punctuation_filter)
        .filter(stopword_filter)
        .map(stemmer_filter)
        .collect()
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
    fn stemmer_filter_sanity() {
        let input = Token("senses".to_string());
        let expected = Token("sens".to_string());

        assert_eq!(expected, stemmer_filter(input));
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
            Token("Ingredients".to_string()),
        ];
        let expected = vec![Token("trim".to_string()), Token("ingredi".to_string())];
        assert_eq!(expected, filter(input));
    }
}
