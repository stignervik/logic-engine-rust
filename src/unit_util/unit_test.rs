#[cfg(test)]
mod unit_test {
    // use crate::unit_util::states::AlarmState;

    use super::super::{*};
    #[test]
    fn build_unit() {
        let unit = Unit::new(1, String::from("Unit1"));
        println!("Unitid: {} Name: {}", unit.id(), unit.name());
        assert_eq!(unit.name, "Unit1");
        assert_eq!(unit.has_alarm(), false)
    }

    #[test]
    fn it_works_too() {
        assert_eq!(2 + 2, 4);
    }
}
