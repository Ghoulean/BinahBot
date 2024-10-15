#[derive(Debug)]
pub struct EncyclopediaInfoLocalization<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub other_names: &'a [&'a str],
    pub code: &'a str,
    pub selection_text: Option<&'a str>,
    pub managerial_guidances: &'a [&'a str],
    pub story: &'a [&'a str],
    pub breaching_entity_localizations: &'a [BreachingEntityLocalization<'a>],
    pub narration_map: &'a [(NarrationType<'a>, &'a str)],
}

#[derive(Debug)]
pub struct BreachingEntityLocalization<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub code: Option<&'a str>,
}

#[derive(Debug)]
pub enum NarrationType<'a> {
    Move,
    Start,
    Death,
    Panic,
    Finish,
    Mid(i32),
    Special(&'a str)
}
