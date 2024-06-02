mod filters;

use crate::filters::filter;

const LEFT_PAD: char = '^';
const RIGHT_PAD: char = '|';
const N_NGRAM: usize = 3;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Token(pub String);

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Ngram(pub String);

pub fn analyze(text: &str) -> Vec<Ngram> {
    let str = filter(tokenize(text)).iter()
        .map(|x| pad(x).0)
        .collect::<Vec<_>>()
        .join("");

    let vec = generate_ngrams(&str);

    return vec.iter()
        .filter(|x| !x.ends_with(LEFT_PAD))
        .map(|x| if !x.contains(LEFT_PAD) { sort_ngram(x) } else { x.to_string() })
        .map(|x| Ngram(x.to_string()))
        .collect();
}

fn tokenize(txt: &str) -> Vec<Token> {
    txt.split(" ").map(String::from).map(Token).collect()
}

fn pad(token: &Token) -> Token {
    Token(format!("{}{}{}{}", LEFT_PAD, LEFT_PAD, token.0, RIGHT_PAD))
}

fn generate_ngrams(str: &str) -> Vec<String> {
    let mut vec = Vec::new();
    let count = str.char_indices().count();

    for i in 0..count - N_NGRAM + 1 {
        let mut iter = str.char_indices();
        let start = iter.nth(i).map(|x| x.0).unwrap();
        let end = iter.nth(N_NGRAM - 1).map(|x| x.0).unwrap_or(str.len());
        vec.push(String::from(&str[start..end]));
    }

    vec
}

fn sort_ngram(ngram: &str) -> String {
    let mut chars: Vec<char> = ngram.chars().collect();
    chars.sort_by(|a, b| a.cmp(b));
    String::from_iter(chars)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanity_tokenize() {
        let input = "The Weight of Sin";
        let expected = vec![
            Token("The".to_string()),
            Token("Weight".to_string()),
            Token("of".to_string()),
            Token("Sin".to_string()),
        ];
        assert_eq!(expected, tokenize(input));
    }

    #[test]
    fn sanity_eternally_lit_lamp() {
        let input = "Eternally Lit Lamp"; // ^^eternally|^^lit|^^lamp|
        let expected: Vec<_> = vec![
            "^^e", "^et", "eet", "ert", "enr", "anr", "aln", "all", "lly", "ly|",
            "^^l", "^li", "ilt", "it|", "^^l", "^la", "alm", "amp", "mp|"
        ].iter()
        .map(|x| Ngram(x.to_string()))
        .collect();
        assert_eq!(expected, analyze(input));
    }

    #[test]
    fn sanity_degraded_pillar() {
        let input = "degraded pillar"; // ^^degraded|^^pillar|
        let expected: Vec<_> = vec![
            "^^d", "^de", "deg", "egr", "agr", "adr", "ade", "dde", "de|",
            "^^p", "^pi", "ilp", "ill", "all", "alr", "ar|"
        ].iter()
        .map(|x| Ngram(x.to_string()))
        .collect();
        assert_eq!(expected, analyze(input));
    }

    #[test]
    fn sanity_jp() {
        let input = "回避"; // ^^回避|
        let expected: Vec<_> = vec![
            "^^回", "^回避", "|回避"
        ].iter()
        .map(|x| Ngram(x.to_string()))
        .collect();
        assert_eq!(expected, analyze(input));
    }
}
