use std::collections::HashMap;
use std::error::Error;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use futures::future::select_ok;
use futures::FutureExt;
use index_analyzer::generate_inverse_index;

use crate::ddb::get_deck_search_cache;
use crate::ddb::list_decks;
use crate::ddb::put_deck_search_cache_pages;
use crate::models::binahbot::BinahBotEnvironment;
use crate::models::deck::DeckMetadata;
use crate::models::deck::DeckSearchCacheEntry;
use crate::models::deck::DeckSearchParams;

pub const CACHE_PAGE_SIZE: usize = 10;
pub const CACHE_PAGE_COUNT: usize = 5;
const CACHE_TTL_SECONDS: u64 = 15 * 60;

type SearchResult = Result<DeckSearchCacheEntry, Box<dyn Error + Send + Sync>>;

pub async fn try_get_search_cache(
    search_params: &DeckSearchParams,
    display_page: usize,
    env: &BinahBotEnvironment,
) -> SearchResult {
    check_display_page(display_page)?;
    let entry = get_deck_search_cache(
        env.ddb_client.as_ref().expect("no ddb client"),
        &env.ddb_interaction_ttl_table_name,
        search_params,
        display_page,
    )
    .await?;

    if entry.ttl <= now() {
        return Err("deck search cache entry expired".into());
    }

    tracing::info!(
        "Deck search cache hit: search_params={:?}, display_page={}",
        search_params,
        display_page
    );
    Ok(entry)
}

pub async fn search(
    search_params: &DeckSearchParams,
    display_page: usize,
    env: &BinahBotEnvironment,
) -> SearchResult {
    check_display_page(display_page)?;
    let ddb_client = env.ddb_client.as_ref().expect("no ddb client");

    let decks = list_decks(
        ddb_client,
        &env.ddb_table_name,
        search_params.author.as_deref(),
        search_params.keypage.as_deref(),
    )
    .await?;
    let mut cache_pages = build_cache_pages(
        &search_params.query,
        decks,
        display_page,
        now() + CACHE_TTL_SECONDS,
    );

    if let Err(e) = put_deck_search_cache_pages(
        ddb_client,
        &env.ddb_interaction_ttl_table_name,
        search_params,
        &cache_pages,
    )
    .await
    {
        tracing::error!("couldn't write deck search cache: {:?}", e);
    }

    let index = cache_pages
        .iter()
        .position(|(x, _)| *x == display_page)
        .expect("requested display page is always built");
    Ok(cache_pages.swap_remove(index).1)
}

pub async fn search_race(
    search_params: &DeckSearchParams,
    display_page: usize,
    env: &BinahBotEnvironment,
) -> SearchResult {
    select_ok([
        try_get_search_cache(search_params, display_page, env).boxed(),
        search(search_params, display_page, env).boxed(),
    ])
    .await
    .map(|(entry, _)| entry)
}

pub async fn search_prefer_cache(
    search_params: &DeckSearchParams,
    display_page: usize,
    env: &BinahBotEnvironment,
) -> SearchResult {
    match try_get_search_cache(search_params, display_page, env).await {
        Ok(x) => Ok(x),
        Err(e) => {
            tracing::info!("Deck search cache miss: {:?}", e);
            search(search_params, display_page, env).await
        }
    }
}

fn check_display_page(display_page: usize) -> Result<(), Box<dyn Error + Send + Sync>> {
    if display_page >= CACHE_PAGE_COUNT {
        return Err(format!(
            "display page {} is past the last display page",
            display_page
        )
        .into());
    }
    Ok(())
}

fn build_cache_pages(
    query: &str,
    decks: Vec<DeckMetadata>,
    display_page: usize,
    ttl: u64,
) -> Vec<(usize, DeckSearchCacheEntry)> {
    let ranked = rank(query, decks);

    (0..CACHE_PAGE_COUNT)
        .map(|i| {
            let start = (i * CACHE_PAGE_SIZE).min(ranked.len());
            let end = (start + CACHE_PAGE_SIZE).min(ranked.len());
            (
                i,
                DeckSearchCacheEntry {
                    decks: ranked[start..end].to_vec(),
                    result_count: ranked.len(),
                    ttl,
                },
            )
        })
        .filter(|(i, entry)| *i == display_page || !entry.decks.is_empty())
        .collect()
}

fn rank(query: &str, decks: Vec<DeckMetadata>) -> Vec<DeckMetadata> {
    let tiebreak = |a: &DeckMetadata, b: &DeckMetadata| {
        a.name
            .to_lowercase()
            .cmp(&b.name.to_lowercase())
            .then_with(|| a.name.cmp(&b.name))
            .then_with(|| a.author_id.cmp(&b.author_id))
    };

    if query.is_empty() {
        let mut decks = decks;
        decks.sort_by(tiebreak);
        return decks;
    }

    let map: HashMap<usize, &String> = decks
        .iter()
        .enumerate()
        .map(|(i, x)| (i, &x.name))
        .collect();
    let inverse_index = generate_inverse_index(&map);
    let mut scored = index_analyzer::query_with_scores(query, &inverse_index)
        .into_iter()
        .map(|(i, score)| (*i, score))
        .collect::<Vec<_>>();
    scored.sort_by(|a, b| {
        b.1.cmp(&a.1)
            .then_with(|| tiebreak(&decks[a.0], &decks[b.0]))
    });

    scored.into_iter().map(|(i, _)| decks[i].clone()).collect()
}

fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("couldn't calculate epoch time")
        .as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn deck(name: &str, author_id: &str) -> DeckMetadata {
        DeckMetadata {
            name: name.to_string(),
            author_id: author_id.to_string(),
            author_name: format!("author {}", author_id),
        }
    }

    fn names(decks: &[DeckMetadata]) -> Vec<&str> {
        decks.iter().map(|x| x.name.as_str()).collect()
    }

    fn sample_decks() -> Vec<DeckMetadata> {
        vec![
            deck("Turbo Nikolai", "1"),
            deck("Bore Kali to death with this one neat trick!", "2"),
            deck("Op", "3"),
            deck("Crimson Claws", "4"),
            deck("TurboNikolai", "1"),
        ]
    }

    #[test]
    fn sanity_rank() {
        let ranked = rank("turbo nikolai", sample_decks());
        assert_eq!(Some(&"Turbo Nikolai"), names(&ranked).first());
        assert_eq!(Some(&"TurboNikolai"), names(&ranked).get(1));

        let ranked = rank("one neat trick", sample_decks());
        assert_eq!(
            Some(&"Bore Kali to death with this one neat trick!"),
            names(&ranked).first()
        );

        let ranked = rank("cccccccc", sample_decks());
        assert_eq!(vec!["Crimson Claws"], names(&ranked));
    }

    #[test]
    fn empty_query_returns_all_decks_alphabetically() {
        let ranked = rank("", sample_decks());
        assert_eq!(
            vec![
                "Bore Kali to death with this one neat trick!",
                "Crimson Claws",
                "Op",
                "Turbo Nikolai",
                "TurboNikolai"
            ],
            names(&ranked)
        );
    }

    #[test]
    fn ties_are_ordered_deterministically() {
        let decks = (0..20)
            .map(|i| deck("Same Name", &format!("{:02}", 19 - i)))
            .collect::<Vec<_>>();
        let expected = (0..20).map(|i| format!("{:02}", i)).collect::<Vec<_>>();

        for _ in 0..10 {
            let ranked = rank("same name", decks.clone());
            assert_eq!(
                expected,
                ranked
                    .iter()
                    .map(|x| x.author_id.clone())
                    .collect::<Vec<_>>()
            );
        }
    }

    #[test]
    fn cache_pages_hold_top_results() {
        let decks = (0..120)
            .map(|i| deck(&format!("Deck {:03}", i), "1"))
            .collect::<Vec<_>>();

        let cache_pages = build_cache_pages("", decks.clone(), 0, 0);

        assert_eq!(
            (0..CACHE_PAGE_COUNT).collect::<Vec<_>>(),
            cache_pages.iter().map(|(i, _)| *i).collect::<Vec<_>>()
        );
        assert!(cache_pages
            .iter()
            .all(|(_, x)| x.decks.len() == CACHE_PAGE_SIZE));
        assert!(cache_pages.iter().all(|(_, x)| x.result_count == 120));
        assert_eq!(
            decks[..CACHE_PAGE_COUNT * CACHE_PAGE_SIZE].to_vec(),
            cache_pages
                .into_iter()
                .flat_map(|(_, x)| x.decks)
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn cache_pages_skip_empty_pages() {
        let decks = (0..23)
            .map(|i| deck(&format!("Deck {:03}", i), "1"))
            .collect::<Vec<_>>();

        let cache_pages = build_cache_pages("", decks.clone(), 0, 0);
        assert_eq!(
            vec![0, 1, 2],
            cache_pages.iter().map(|(i, _)| *i).collect::<Vec<_>>()
        );
        assert_eq!(3, cache_pages[2].1.decks.len());

        let cache_pages = build_cache_pages("", decks, 4, 0);
        assert_eq!(
            vec![0, 1, 2, 4],
            cache_pages.iter().map(|(i, _)| *i).collect::<Vec<_>>()
        );
        assert!(cache_pages[3].1.decks.is_empty());
    }

    #[test]
    fn requested_display_page_is_built_even_if_empty() {
        let cache_pages = build_cache_pages("", Vec::new(), 0, 0);
        assert_eq!(1, cache_pages.len());
        assert_eq!(0, cache_pages[0].0);
        assert!(cache_pages[0].1.decks.is_empty());
        assert_eq!(0, cache_pages[0].1.result_count);
    }

    #[test]
    fn display_pages_past_the_last_display_page_are_rejected() {
        assert!(check_display_page(CACHE_PAGE_COUNT - 1).is_ok());
        assert!(check_display_page(CACHE_PAGE_COUNT).is_err());
    }
}
