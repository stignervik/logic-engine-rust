// mod unit;

use crate::unit_util::unit::Unit;

pub struct Units {
    unit_list: Vec<Unit>,
}

impl Units {
    pub fn new() -> Self {
        let v = Vec::new();
        Self { unit_list: v }
    }

    pub fn count(&self) -> usize {
        self.unit_list.len()
    }

    pub fn description(&self) {
        println!("This is units which manage the unit collection.");
    }
}
