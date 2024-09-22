mod filters;

use std::cmp::min;
use std::collections::HashMap;

use crate::filters::filter;

const LEFT_PAD: char = '^';
const RIGHT_PAD: char = '|';
const N_NGRAM: usize = 3;

type Token = String;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Ngram(pub String);

type Frequency = i32;

pub fn analyze<S: AsRef<str> + ?Sized>(text: &S) -> HashMap<Ngram, Frequency> {
    let mut filtered_tokens = filter(tokenize(text.as_ref()));

    if filtered_tokens.is_empty() {
        filtered_tokens.push("".to_string());
    }

    let padded_str = filtered_tokens.iter().map(pad).collect::<Vec<_>>().join("");

    let vec = generate_ngrams(&padded_str);

    vec.iter()
        .filter(|x| !x.ends_with(LEFT_PAD))
        .map(|x| {
            if !x.contains(LEFT_PAD) {
                sort_ngram(x)
            } else {
                x.to_string()
            }
        })
        .map(|x| Ngram(x.to_string()))
        .fold(HashMap::new(), |mut map, x| {
            map.entry(x).and_modify(|y| *y += 1).or_insert(1);
            map
        })
}

pub fn generate_inverse_index<
    T: core::cmp::Eq + std::hash::Hash + core::clone::Clone,
    S: AsRef<str>,
>(
    map: &HashMap<T, S>,
) -> HashMap<Ngram, HashMap<T, Frequency>> {
    let index = generate_index(map);
    invert_index(&index)
}

fn generate_index<T: core::cmp::Eq + std::hash::Hash + core::clone::Clone, S: AsRef<str>>(
    map: &HashMap<T, S>,
) -> HashMap<T, HashMap<Ngram, Frequency>> {
    map.iter().map(|(k, v)| (k.clone(), analyze(v))).collect()
}
fn invert_index<T: core::cmp::Eq + std::hash::Hash + core::clone::Clone>(
    index: &HashMap<T, HashMap<Ngram, Frequency>>,
) -> HashMap<Ngram, HashMap<T, Frequency>> {
    let mut ret_val = HashMap::new();

    for (t, ngram_map) in index.iter() {
        for (ngram, frequency) in ngram_map.iter() {
            let entry = ret_val.entry(ngram.clone()).or_insert_with(HashMap::new);
            entry.insert(t.clone(), frequency.to_owned());
        }
    }

    ret_val
}

pub fn query<'a, T: core::cmp::Eq + std::hash::Hash, S: AsRef<str> + ?Sized>(
    query: &'a S,
    inverse_index: &'a HashMap<Ngram, HashMap<T, Frequency>>,
) -> Vec<&'a T> {
    let ngrams = analyze(query);

    let mut scorekeeper = HashMap::new();

    ngrams.iter().for_each(|(ngram, freq1)| {
        if let Some(map) = inverse_index.get(&ngram) {
            map.into_iter().for_each(|(t, freq2)| {
                scorekeeper
                    .entry(t)
                    .and_modify(|x: &mut i32| *x += min(*freq1, *freq2))
                    .or_insert(min(*freq1, *freq2));
            });
        }
    });

    let mut vec: Vec<_> = scorekeeper.iter().collect();
    vec.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());

    vec.into_iter().map(|(t, _)| *t).collect()
}

fn tokenize(txt: &str) -> Vec<Token> {
    txt.split(' ').map(String::from).collect()
}

fn pad(token: &Token) -> Token {
    format!("{}{}{}{}", LEFT_PAD, LEFT_PAD, token, RIGHT_PAD)
}

fn generate_ngrams(padded_str: &str) -> Vec<String> {
    let mut vec = Vec::new();
    let count = padded_str.char_indices().count();

    for i in 0..count - N_NGRAM + 1 {
        let mut iter = padded_str.char_indices();
        let start = iter.nth(i).map(|x| x.0).unwrap();
        let end = iter
            .nth(N_NGRAM - 1)
            .map(|x| x.0)
            .unwrap_or(padded_str.len());
        vec.push(String::from(&padded_str[start..end]));
    }

    vec
}

fn sort_ngram(ngram: &str) -> String {
    let mut chars: Vec<char> = ngram.chars().collect();
    chars.sort();
    String::from_iter(chars)
}

#[cfg(test)]
mod tests {
    use crate::invert_index;

    use super::*;

    #[test]
    fn sanity_tokenize() {
        let input = "The Weight of Sin";
        let expected = vec![
            "The".to_string(),
            "Weight".to_string(),
            "of".to_string(),
            "Sin".to_string(),
        ];
        assert_eq!(expected, tokenize(input));
    }

    #[test]
    fn sanity_eternally_lit_lamp() {
        let input = "Eternally Lit Lamp"; // ^^eternally|^^lit|^^lamp|

        // all are Frequency=1 except "^^l"
        let mut expected: HashMap<_, _> = vec![
            "^^e", "^et", "eet", "ert", "enr", "anr", "aln", "all", "lly", "ly|", "^^l", "^li",
            "ilt", "it|", "^^l", "^la", "alm", "amp", "mp|",
        ]
        .iter()
        .map(|x| (Ngram(x.to_string()), 1))
        .collect();

        expected.insert(Ngram("^^l".to_string()), 2);

        assert_eq!(expected, analyze(input));
    }

    #[test]
    fn sanity_degraded_pillar() {
        let input = "degraded pillar"; // ^^degraded|^^pillar|
        let expected: HashMap<_, _> = vec![
            "^^d", "^de", "deg", "egr", "agr", "adr", "ade", "dde", "de|", "^^p", "^pi", "ilp",
            "ill", "all", "alr", "ar|",
        ]
        .iter()
        .map(|x| (Ngram(x.to_string()), 1))
        .collect();
        assert_eq!(expected, analyze(input));
    }

    #[test]
    fn sanity_jp() {
        let input = "回避"; // ^^回避|
        let expected: HashMap<_, _> = ["^^回", "^回避", "|回避"]
            .iter()
            .map(|x| (Ngram(x.to_string()), 1))
            .collect();
        assert_eq!(expected, analyze(input));
    }

    #[test]
    fn analyze_short_str() {
        ["", "b", "bb", "bbb", "a", "of", "the"]
            .iter()
            .for_each(|x| {
                analyze(x);
            });
    }

    #[test]
    fn sanity_generate_index() {
        let map = HashMap::from([("index1", "hello"), ("index2", "hi")]);
        let index = generate_index(&map);
        let index1_map = index.get("index1").expect("couldn't find index1 entry");
        let index2_map = index.get("index2").expect("couldn't find index2 entry");
        assert_eq!(Some(1), index1_map.get(&Ngram("ehl".to_string())).copied());
        assert_eq!(None, index2_map.get(&Ngram("ehl".to_string())).copied());
    }

    #[test]
    fn sanity_invert_index() {
        let index = HashMap::from([("index1", analyze("hello")), ("index2", analyze("hi"))]);
        let inverted_index = invert_index(&index);
        let keys = inverted_index
            .keys()
            .map(|x| x.0.as_str())
            .collect::<Vec<_>>();
        ["^^h", "^he", "ehl", "ell", "llo", "lo|", "^hi", "hi|"]
            .iter()
            .for_each(|x| {
                assert!(keys.contains(&x));
            });
    }

    #[test]
    fn sanity_query() {
        let inverse_index = generate_inverse_index(&HashMap::from([
            ("index1", "hello"),
            ("index2", "goodbye"),
            ("index3", "hi"),
        ]));
        let query = query("hellllllohi", &inverse_index);
        // "goodbye" has zero common ngrams, so is completely left out (intended behavior)
        assert_eq!(2, query.len());
        assert_eq!("index1", *query[0]);
        assert_eq!("index3", *query[1]);
    }
}
