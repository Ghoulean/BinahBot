use ruina_common::game_objects::battle_symbol::BattleSymbol;
use ruina_reparser::get_battle_symbol_locales_by_internal_name;

use super::tagger::PageType;
use super::tagger::Tag;
use super::tagger::Tagger;
use super::tagger::TypedId;

const DEFAULT_BATTLE_SYMBOL_TAGS: [&str; 2] = ["battle", "symbol"];

impl Tagger for BattleSymbol<'_> {
    fn get_typed_id(&self) -> TypedId {
        TypedId(PageType::BattleSymbolId, String::from(self.internal_name))
    }

    fn generate_tags(&self) -> Vec<Tag> {
        let default_tags = DEFAULT_BATTLE_SYMBOL_TAGS.to_vec();
        get_battle_symbol_locales_by_internal_name(self.internal_name)
            .values()
            .flat_map(|battle_symbol_locale| {
                vec![battle_symbol_locale.prefix, battle_symbol_locale.postfix]
            })
            .chain(default_tags)
            .map(String::from)
            .map(Tag)
            .collect()
    }
}
