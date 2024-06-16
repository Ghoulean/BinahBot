# annotations

Contains heuristics and manual mappings for annotations and disambiguations.

**Annotations** are *hidden* labels that assist querying for game objects. e.g. Searching "Yan" against the English index will bring up "얀샋ㄷ요무's Page" (Distorted Yan's key page). Page -> annotation mapping is be many-to-one.

**Disambiguations** are *displayed* labels that differentiate game objects with similar (or same) names. e.g. Searching "Prepared Mind" should bring up "Prepared Mind (Lulu)" and "Prepared Mind (Mars)" rather than simply two identical-looking entries of "Prepared Mind". Page -> disambiguations is be many-to-one in general, but one-to-one within a disambiguation pool.
