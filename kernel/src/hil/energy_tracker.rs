use core::fmt;
use core::fmt::{Display, Formatter};

use crate::process::ProcessId;

pub const MAX_COMPONENT_NUM: usize = 10;

#[derive(Clone, Copy)]
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
    fn get_power(&self, component_id: usize, state: PowerState) -> f32;
}

pub trait PowerStateTracker {
    fn set_power_state(&self, component_id: usize, app_id: ProcessId, state: PowerState);
}
