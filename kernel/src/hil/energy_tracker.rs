use core::fmt;
use core::fmt::{Display, Formatter};

use crate::process::ProcessId;

pub const MAX_COMPONENT_NUM: usize = 10;

pub type Energy = f32;
pub type Power = f32;

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum PowerState {
    None,
    CpuOff,
    CpuOn,
    LedOff,
    LedOn,
}

impl Display for PowerState {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        let display_str = match *self {
            PowerState::None => "None",
            PowerState::CpuOff => "CpuOff",
            PowerState::CpuOn => "CpuOn",
            PowerState::LedOff => "LedOff",
            PowerState::LedOn => "LedOn",
        };
        write!(fmt, "{}", display_str)
    }
}

pub trait PowerModel {
    fn get_power(&self, component_id: usize, state: PowerState) -> Power;
}

pub trait Track {
    fn set_power_state(&self, component_id: usize, app_id: ProcessId, power_state: PowerState);
}

pub trait Query {
    fn query_total_energy_consumption(&self) -> Energy;
    fn query_peripheral_energy_consumption(&self, component_id: usize) -> Energy;
    fn query_app_energy_consumption(&self, app_id: ProcessId) -> Energy;
    fn freeze_all(&self);
}
