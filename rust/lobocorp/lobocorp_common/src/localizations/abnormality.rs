
#[derive(Debug)]
pub struct AbnormalityLocalization<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub code: &'a str,
    pub selection_text: &'a str,
    pub managerial_guidances: &'a [&'a str],
    pub story: &'a [&'a str],
}
