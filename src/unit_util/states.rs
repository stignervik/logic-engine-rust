// An attribute to hide warnings for unused code.
#![allow(dead_code)]

// #[derive(Debug)]
pub struct States {
    alarm: AlarmState,
    fault: FaultState,
    disabled: DisabledState,
    input: InputState,
    output: OutputState,
}

impl States {
    pub fn new() -> Self {
        Self {
            alarm: AlarmState::Off,
            fault: FaultState::Off,
            disabled: DisabledState::Enabled,
            input: InputState::Off,
            output: OutputState::Off,
        }
    }

    pub fn alarm_state(&self) -> &AlarmState {
      &self.alarm
    }

    pub fn has_alarm(&self) -> bool {
        match self.alarm {
            AlarmState::Alarm => true,
            AlarmState::Off => false,
        }
    }
}
pub enum AlarmState {
    Alarm,
    Off,
}
pub enum FaultState {
    Fault,
    Off,
}
pub enum DisabledState {
    Enabled,
    Disabled,
}
pub enum OutputState {
    On,
    Off,
}

pub enum InputState {
    On,
    Off,
}
