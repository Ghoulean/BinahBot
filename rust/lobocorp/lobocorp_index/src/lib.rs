use std::cmp::min;
use std::collections::HashMap;

use index_analyzer::analyze;

include!(concat!(env!("OUT_DIR"), "/out.rs"));

pub fn query(query: &str) -> Vec<u32> {
    let ngrams = analyze(query);

    let mut scorekeeper = HashMap::new();

    ngrams.iter().for_each(|(ngram, freq1)| {
        if let Some(map) = INVERSE_INDEX.get(&ngram.0) {
            map.into_iter().for_each(|(id, freq2)| {
                scorekeeper
                    .entry(id)
                    .and_modify(|x: &mut i32| *x += min(*freq1, *freq2))
                    .or_insert(min(*freq1, *freq2));
            });
        }
    });

    let mut vec = scorekeeper.iter().collect::<Vec<_>>();
    vec.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());

    vec.iter().map(|(id, _)| ***id).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanity_query() {
        let return_padding = 2;

        let punishing_bird_id: u32 = 100020;

        let punishing_bird_query = query("punishing bird");
        let punishing_bird_query_position = punishing_bird_query
            .iter()
            .position(|x| *x == punishing_bird_id)
            .expect("couldn't find punishing bird");

        assert!(punishing_bird_query_position <= 0 + return_padding);
    }
}
