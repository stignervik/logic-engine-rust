use crate::unit_util::units::Units;

mod unit_util;

fn main() {
    let units: Units = unit_util::units::Units::new();
    units.description();
    units.count();
    unit_util::unit::description();
    let unit = unit_util::unit::Unit::new(1, String::from("Unit1"));
    println!("UnitId: {} Name: {}", unit.id(), unit.name());
    unit.states();
    // unit_util::units::description();
}
