use kernel::hil::energy_tracker::{
    Energy, PowerModel, PowerState, Query, Track, MAX_COMPONENT_NUM,
};
use kernel::common::cells::TakeCell;
use kernel::hil::time::{Alarm, Frequency, Ticks};
use kernel::{Grant, ProcessId};

pub static mut TOTAL_TIME_TABLE: [Energy; MAX_COMPONENT_NUM] = [0.0; MAX_COMPONENT_NUM]; 
pub static mut POWER_STATE_TABLE: [PowerStateRecord; MAX_COMPONENT_NUM] = [PowerStateRecord {power_state: PowerState::None, start_time_in_ms: 0}; MAX_COMPONENT_NUM]; 

pub struct EnergyTracker<'a, A: Alarm<'a>> {
    alarm: &'a A,
    grants: Grant<App>,
    power_model: &'a dyn PowerModel,
    total_energy_consumed: TakeCell<'static, [Energy]>,
    power_state_records: TakeCell<'static, [PowerStateRecord]>,
}

pub struct App {
    total_energy_consumed_freeze: Energy,
    total_energy_consumed: Energy,
    power_state_records: [PowerStateRecord; MAX_COMPONENT_NUM],
}

#[derive(Clone, Copy)]
pub struct PowerStateRecord {
    power_state: PowerState,
    start_time_in_ms: u64,
}

impl<'a, A: Alarm<'a>> EnergyTracker<'a, A> {
    pub fn new(alarm: &'a A, 
               grants: Grant<App>, 
               power_model: &'a dyn PowerModel, 
               total_energy_consumed: &'static mut [Energy],
               power_state_records: &'static mut [PowerStateRecord],) -> Self {
        Self {
            alarm,
            grants,
            power_model,
            total_energy_consumed: TakeCell::new(total_energy_consumed),
            power_state_records: TakeCell::new(power_state_records),
        }
    }

    fn now_in_ms(&self) -> u64 {
        let freq_in_hz = <A::Frequency>::frequency();
        let now_in_count = self.alarm.now().into_u32();
        (now_in_count as u64) * 1000 / (freq_in_hz as u64)
    }
}

impl<'a, A: Alarm<'a>> Track for EnergyTracker<'a, A> {
    fn set_power_state(&self, component_id: usize, app_id: ProcessId, power_state: PowerState) {
        let now_in_ms = self.now_in_ms();

        // Perform updates on global power table
        self.total_energy_consumed.map(|energy_table| {
            self.power_state_records.map(|state_table| {
                let power = self.power_model.get_power(
                    component_id,
                    state_table[component_id].power_state,
                );
                let time = (now_in_ms - state_table[component_id].start_time_in_ms) as f32;
                energy_table[component_id] += power * time;
                state_table[component_id].start_time_in_ms = now_in_ms;
                state_table[component_id].power_state = power_state;
            });
        });

        // Perform updates on app-specific tables
        self.grants.each(|grant_app_id, app| {
            // Update consumed energy
            let power = self.power_model.get_power(
                component_id,
                app.power_state_records[component_id].power_state,
            );

            let time = (now_in_ms - app.power_state_records[component_id].start_time_in_ms) as f32;
            app.total_energy_consumed += power * time;
            app.power_state_records[component_id].start_time_in_ms = now_in_ms;

            //  Update power state records
            if grant_app_id == app_id {
                // For the app that sets the new power state, update the power state directly.
                app.power_state_records[component_id].power_state = power_state;
            } else if app.power_state_records[component_id].power_state != power_state {
                // For the app that doesn't set the new power state,
                // if the new power state is not the same,
                // regard this app as not using this component any more.
                app.power_state_records[component_id].power_state = PowerState::None;
            }
        });
    }
}

impl<'a, A: Alarm<'a>> Query for EnergyTracker<'a, A> {
    fn query_total_energy_consumption(&self) -> Energy {
        let mut sum : Energy = 0.0;
        self.total_energy_consumed.map(|table| {
            for component_id in 0..MAX_COMPONENT_NUM {
                sum += table[component_id];
            }
        });
        sum
    }

    fn query_peripheral_energy_consumption(&self, component_id: usize) -> Energy {
        let mut value : Energy = 0.0;
        self.total_energy_consumed.map(|table| {
            value = table[component_id];
        });
        value
    }

    fn query_app_energy_consumption(&self, app_id: ProcessId) -> Energy {
        self.grants
            .enter(app_id, |app| app.total_energy_consumed_freeze)
            .unwrap_or(0.0)
    }

    fn freeze_all(&self) {
        let now_in_ms = self.now_in_ms();
        self.grants.each(|_, app| {
            for component_id in 0..MAX_COMPONENT_NUM {
                // Update consumed energy
                let power = self.power_model.get_power(
                    component_id,
                    app.power_state_records[component_id].power_state,
                );
                let time =
                    (now_in_ms - app.power_state_records[component_id].start_time_in_ms) as f32;
                app.total_energy_consumed += power * time;
                app.power_state_records[component_id].start_time_in_ms = now_in_ms;
            }
            app.total_energy_consumed_freeze = app.total_energy_consumed;
        });
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            total_energy_consumed_freeze: 0.0,
            total_energy_consumed: 0.0,
            power_state_records: [PowerStateRecord::default(); MAX_COMPONENT_NUM],
        }
    }
}

impl Default for PowerStateRecord {
    fn default() -> Self {
        Self {
            power_state: PowerState::None,
            start_time_in_ms: 0,
        }
    }
}
