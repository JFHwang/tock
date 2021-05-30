use kernel::hil::energy_tracker::{Energy, PowerState, Query, Track, MAX_COMPONENT_NUM};
use kernel::hil::time::Alarm;
use kernel::{Grant, ProcessId};

pub struct EnergyTracker<'a, A: Alarm<'a>> {
    alarm: &'a A,
    grants: Grant<App>,
}

pub struct App {
    total_energy_consumed_freeze: Energy,
    total_energy_consumed: Energy,
    power_state_records: [PowerStateRecord; MAX_COMPONENT_NUM],
}

#[derive(Clone, Copy)]
pub struct PowerStateRecord {
    power_state: PowerState,
    start_time: f32, // TODO: change to a time-specific type
}

impl<'a, A: Alarm<'a>> EnergyTracker<'a, A> {
    pub fn new(alarm: &'a A, grants: Grant<App>) -> Self {
        Self { alarm, grants }
    }
}

impl<'a, A: Alarm<'a>> Track for EnergyTracker<'a, A> {
    fn set_power_state(&self, component_id: usize, app_id: ProcessId, power_state: PowerState) {
        self.grants.each(|_, app| {
            app.power_state_records[component_id].power_state = power_state;
        });
    }
}

impl<'a, A: Alarm<'a>> Query for EnergyTracker<'a, A> {
    fn query_app_energy_consumption(&self, app_id: ProcessId) -> Energy {
        self.grants
            .enter(app_id, |app| app.total_energy_consumed_freeze)
            .unwrap_or(Energy::default())
    }

    fn freeze(&self, app_id: ProcessId) {}

    fn freeze_all(&self) {}
}

impl Default for App {
    fn default() -> Self {
        Self {
            total_energy_consumed_freeze: Energy::default(),
            total_energy_consumed: Energy::default(),
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
