use kernel::debug;
use kernel::hil::energy_tracker::{PowerState, PowerStateTracker, MAX_COMPONENT_NUM};
use kernel::{Grant, ProcessId};

pub struct EnergyTracker {
    grants: Grant<App>,
}

pub struct App {
    total_energy_consumed: f32,
    power_state_records: [PowerStateRecord; MAX_COMPONENT_NUM],
}

#[derive(Clone, Copy)]
pub struct PowerStateRecord {
    power_state: PowerState,
    start_time: f32, // TODO: change to a time-specific type
}

impl EnergyTracker {
    pub fn new(grants: Grant<App>) -> Self {
        Self { grants }
    }
}

impl PowerStateTracker for EnergyTracker {
    fn set_power_state(&self, component_id: usize, app_id: ProcessId, state: PowerState) {
        debug!(
            "App {} sets component {}'s power state to be {}",
            app_id.id(),
            component_id,
            state,
        );
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            total_energy_consumed: 0.0,
            power_state_records: [PowerStateRecord::default(); MAX_COMPONENT_NUM],
        }
    }
}

impl Default for PowerStateRecord {
    fn default() -> Self {
        Self {
            power_state: PowerState::None,
            start_time: 0.0,
        }
    }
}
