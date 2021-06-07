use kernel::hil::energy_tracker::{Power, PowerModel, PowerState};

pub const COMPONENT_NUM: usize = 4;
pub const LED_COMPONENT_IDS: &'static [usize] = &[0, 1, 2, 3];
pub const POWER_MODEL: &'static BoardPowerModel = &BoardPowerModel {};

pub struct BoardPowerModel {}

impl PowerModel for BoardPowerModel {
    fn get_power(&self, component_id: usize, power_state: PowerState) -> Power {
        match component_id {
            // Led 0
            0 => match power_state {
                PowerState::LedOn => 12.19,
                _ => 0.0,
            },
            // Led 1
            1 => match power_state {
                PowerState::LedOn => 12.19,
                _ => 0.0,
            },
            // Led 2
            2 => match power_state {
                PowerState::LedOn => 12.19,
                _ => 0.0,
            },
            // Led 3
            3 => match power_state {
                PowerState::LedOn => 12.19,
                _ => 0.0,
            },
            // This shouldn't happen
            _ => 0.0,
        }
    }
}
