#[derive(Debug)]
pub struct KeyPageLocale<'a> {
    pub text_id: &'a str,
    pub name: &'a str,
    pub description: &'a [&'a str]
}
