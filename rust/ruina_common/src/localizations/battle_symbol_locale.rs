pub struct BattleSymbolLocale<'a> {
    pub internal_name: &'a str,
    pub prefix: &'a str,
    pub postfix: &'a str,
    pub description: Option<&'a str>,
    pub acquire_condition: Option<&'a str>,
}
