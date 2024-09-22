use unicode_normalization::char::is_combining_mark;
use unicode_normalization::UnicodeNormalization;

type Token = String;

fn punctuation_filter(token: &Token) -> Token {
    token
        .nfd()
        .filter(|x| !is_combining_mark(*x))
        .collect::<Token>()
        .to_lowercase()
        .replace("’s", "")
        .replace("'s", "")
        .replace(
            &[
                '(', ')', ',', '\"', '.', ';', ':', '\'', '?', '!', '’', '~', '…', '♣', '◆',
            ][..],
            "",
        )
        .replace("<color=red>♥</color>", "♥")
        .replace('Ⅰ', "I")
        .replace('Ⅱ', "II")
        .replace('Ⅲ', "III")
        .replace(&['-', '/'][..], " ")
}

fn stopword_filter(token: &Token) -> bool {
    // todo: something less hardcoded and more config
    !matches!(token.as_str(), "a" | "the" | "of")
}

pub fn filter(tokens: Vec<Token>) -> Vec<Token> {
    tokens
        .iter()
        .map(punctuation_filter)
        .filter(stopword_filter)
        .filter(|x| !x.is_empty())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn punctuation_filter_sanity() {
        let input = "hello?".to_string();
        let expected = "hello".to_string();

        assert_eq!(expected, punctuation_filter(&input));
    }

    #[test]
    fn punctuation_filter_accents() {
        let input = "ChīWěn".to_string();
        let expected = "chiwen".to_string();

        assert_eq!(expected, punctuation_filter(&input));
    }

    #[test]
    fn stopword_filter_sanity() {
        assert!(stopword_filter(&"another".to_string()));
    }

    #[test]
    fn sanity() {
        let input = vec![
            "Trim".to_string(),
            "The".to_string(),
            "Ingredients".to_string(),
        ];
        let expected = vec!["trim".to_string(), "ingredients".to_string()];
        assert_eq!(expected, filter(input));
    }
}
