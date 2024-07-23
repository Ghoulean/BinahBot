# ruina_index

ngram index for every page in Library of Ruina. Also provides a matching function

## ruina_index_analyzer

Provides function for splitting a text query into ngrams

## ruina_identifier

Identifies page IDs with their pagetype. Needed because page ids are only unique within the page type (e.g. a combat page and a key page may share the same id) and because a page's ID is not necessarily their "id" field (e.g. abno page id is `internal_name`)

## ruina_index_annotations

Contains heuristics and manual mappings for annotations and disambiguations.

## ruina_index_builder

Precomputes the ngram index
