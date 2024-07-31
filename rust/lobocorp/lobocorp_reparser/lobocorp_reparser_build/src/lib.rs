use list::load_encyclopedia_list;

mod abnormalities;
mod equipment;
mod list;
mod localization_index;
mod path;
mod xml;

pub fn build() -> String {
    let abno_list = load_encyclopedia_list();
    format!("{:?}", abno_list)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanity() {
        let _does_not_crash = build();
    }
}