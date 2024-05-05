#[derive(Debug)]
pub struct AbnoPageLocale<'a> {
    pub internal_name: &'a str,
    pub floor: &'a str,
    pub abnormality: &'a str,
    pub card_name: &'a str,
    pub description: &'a str,
    pub flavor_text: &'a str,
    pub dialogues: &'a [&'a str],
}
