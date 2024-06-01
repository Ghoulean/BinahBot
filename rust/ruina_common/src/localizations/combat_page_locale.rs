#[derive(Debug)]
pub struct CombatPageLocale<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub card_effect: Option<&'a str>,
}
