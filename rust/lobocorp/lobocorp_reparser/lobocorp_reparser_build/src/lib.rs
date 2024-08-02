use equipment::load_equipment;
use list::load_encyclopedia_list;

mod abnormalities;
mod equipment;
mod list;
mod localization_index;
mod path;
mod xml;

pub fn build() -> String {
    let abno_list = load_encyclopedia_list();
    let equipment_list = load_equipment();

    format!("{:?}, {:?}", abno_list, equipment_list)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanity() {
        let _does_not_crash = build();
    }
}