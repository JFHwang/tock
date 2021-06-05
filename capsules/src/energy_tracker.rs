use core::cell::Cell;
use kernel::common::cells::TakeCell;
use kernel::hil::energy_tracker::{
    Energy, PowerModel, PowerState, Query, Track, MAX_COMPONENT_NUM,
};
use kernel::hil::time::{Alarm, Frequency, Ticks};
use kernel::{Grant, ProcessId};

pub static mut ENERGY_STATES: [EnergyState; MAX_COMPONENT_NUM] =
    [EnergyState::default(); MAX_COMPONENT_NUM];

#[derive(Clone, Copy)]
pub struct EnergyState {
    energy_consumed: Energy,
    power_state: PowerState,
    start_time_in_ms: u64,
}

// This is a trick to use ::default() with static
impl EnergyState {
    pub const fn default() -> Self {
        Self {
            energy_consumed: 0.0,
            power_state: PowerState::None,
            start_time_in_ms: 0,
        }
    }
}

impl Default for EnergyState {
    fn default() -> Self {
        Self::default()
    }
}

pub struct App {
    total_energy_consumed: Energy,
    energy_states: [EnergyState; MAX_COMPONENT_NUM],
}

impl Default for App {
    fn default() -> Self {
        Self {
            total_energy_consumed: 0.0,
            energy_states: [EnergyState::default(); MAX_COMPONENT_NUM],
        }
    }
}

pub struct EnergyTracker<'a, A: Alarm<'a>> {
    alarm: &'a A,
    grants: Grant<App>,
    power_model: &'a dyn PowerModel,
    energy_states: TakeCell<'static, [EnergyState]>,
    component_num: Cell<usize>,
}

impl<'a, A: Alarm<'a>> EnergyTracker<'a, A> {
    pub fn new(
        alarm: &'a A,
        grants: Grant<App>,
        power_model: &'a dyn PowerModel,
        energy_states: &'static mut [EnergyState],
        component_num: usize,
    ) -> Self {
        Self {
            alarm,
            grants,
            power_model,
            energy_states: TakeCell::new(energy_states),
            component_num: Cell::new(component_num),
        }
    }

    fn now_in_ms(&self) -> u64 {
        let freq_in_hz = <A::Frequency>::frequency();
        let now_in_count = self.alarm.now().into_u32();
        (now_in_count as u64) * 1000 / (freq_in_hz as u64)
    }

    fn update_energy_state(
        &self,
        energy_state: &mut EnergyState,
        component_id: usize,
        power_state: PowerState,
        time_in_ms: u64,
    ) {
        let power = self
            .power_model
            .get_power(component_id, energy_state.power_state);
        let duration = (time_in_ms - energy_state.start_time_in_ms) as f32;
        energy_state.energy_consumed += power * duration;
        energy_state.power_state = power_state;
        energy_state.start_time_in_ms = time_in_ms;
    }
}

impl<'a, A: Alarm<'a>> Track for EnergyTracker<'a, A> {
    fn set_power_state(&self, app_id: ProcessId, component_id: usize, power_state: PowerState) {
        let now_in_ms = self.now_in_ms();

        // Update global energy states
        self.energy_states.map(|energy_states| {
            self.update_energy_state(
                &mut energy_states[component_id],
                component_id,
                power_state,
                now_in_ms,
            )
        });

        // Update app-specific energy states
        self.grants.each(|grant_app_id, app| {
            if grant_app_id == app_id {
                // For the app that sets the new power state, update its component energy state directly.
                self.update_energy_state(
                    &mut app.energy_states[component_id],
                    component_id,
                    power_state,
                    now_in_ms,
                )
            } else if app.energy_states[component_id].power_state != power_state {
                // For the app that doesn't set the new power state,
                // if the new power state is not the same,
                // regard this app as not using this component any more.
                app.energy_states[component_id].power_state = PowerState::None;
            }
        });
    }
}

impl<'a, A: Alarm<'a>> Query for EnergyTracker<'a, A> {
    fn freeze_all(&self) {
        let now_in_ms = self.now_in_ms();

        // Update global energy states
        for cid in 0..self.component_num.get() {
            self.energy_states.map(|energy_states| {
                let power_state = energy_states[cid].power_state;
                self.update_energy_state(&mut energy_states[cid], cid, power_state, now_in_ms)
            });
        }

        // Update app-specific energy states
        self.grants.each(|_, app| {
            app.total_energy_consumed = 0.0;
            for cid in 0..self.component_num.get() {
                let power_state = app.energy_states[cid].power_state;
                self.update_energy_state(&mut app.energy_states[cid], cid, power_state, now_in_ms);
                app.total_energy_consumed += app.energy_states[cid].energy_consumed;
            }
        });
    }

    fn query_app_energy_consumption(&self, app_id: ProcessId) -> Energy {
        self.grants
            .enter(app_id, |app| app.total_energy_consumed)
            .unwrap_or(0.0)
    }

    fn query_component_energy_consumption(&self, cid: usize) -> Energy {
        let mut energy_consumed: Energy = 0.0;
        self.energy_states.map(|energy_states| {
            energy_consumed = energy_states[cid].energy_consumed;
        });
        energy_consumed
    }

    fn query_component_num(&self) -> usize {
        self.component_num.get()
    }

    fn query_total_energy_consumption(&self) -> Energy {
        let mut total_energy_consumed: Energy = 0.0;
        self.energy_states.map(|energy_states| {
            for cid in 0..self.component_num.get() {
                total_energy_consumed += energy_states[cid].energy_consumed;
            }
        });
        total_energy_consumed
    }
}
