use core::fmt;
use core::fmt::{Display, Formatter};

use crate::process::ProcessId;

pub const MAX_COMPONENT_NUM: usize = 10;

pub type Energy = f32;
pub type Power = f32;

#[derive(Clone, Copy, PartialEq)]
pub enum PowerState {
    None,
    LedOn,
}

impl Display for PowerState {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        let display_str = match *self {
            PowerState::None => "None",
            PowerState::LedOn => "LedOn",
        };
        write!(fmt, "{}", display_str)
    }
}

pub trait PowerModel {
    fn get_power(&self, component_id: usize, state: PowerState) -> Power;
}

pub trait Track {
    fn set_power_state(&self, app_id: ProcessId, component_id: usize, power_state: PowerState);
}

pub trait Query {
    fn debug_on(&self);
    fn debug_off(&self);
    fn freeze_all(&self);
    fn query_app_energy_consumption(&self, app_id: ProcessId) -> Energy;
    fn query_component_energy_consumption(&self, component_id: usize) -> Energy;
    fn query_component_num(&self) -> usize;
    fn query_total_energy_consumption(&self) -> Energy;
}
