
#[derive(Debug)]
pub struct EncyclopediaInfoLocalization<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub code: &'a str,
    pub selection_text: Option<&'a str>,
    pub managerial_guidances: &'a [&'a str],
    pub story: &'a [&'a str],
}

#[derive(Debug)]
pub struct BreachingEntityLocalization<'a> {
    pub id: &'a str,
    pub name: Option<&'a str>,
    pub code: Option<&'a str>,
}
