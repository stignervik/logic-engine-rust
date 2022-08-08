#![allow(dead_code)]

use crate::unit_util::states::States;

pub struct Unit {
    id: i64,
    name: String,
    states: States,
}

impl Unit {
    pub fn new(id: i64, name: String) -> Self {
        let states = States::new();
        Self { id, name, states }
    }
    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn states(&self) -> &States {
        &self.states
    }

    pub fn has_alarm(&self) -> bool {
      self.states.has_alarm()
    }
}

pub fn description() {
    println!("This is a unit")
}

#[cfg(test)]
#[path = "unit_test.rs"]
mod unit_test;
