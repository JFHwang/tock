//use kernel::hil::time::Ticks;
use kernel::common::cells::TakeCell;
use kernel::state_tracker;
use kernel::debug;

///// Syscall driver number.
//use crate::driver;
//pub const DRIVER_NUM: usize = driver::NUM::Accel as usize;

/*
enum PeripheralStates {
    LED,
    StateCount,
}
*/

pub static mut TOTAL_TIME: [u32; 4] = [0; 4]; // state_tracker::PeripheralStates::StateCount],
pub static mut CURRENT_TIME: [u32; 4] = [0; 4]; // state_tracker::PeripheralStates::StateCount],
pub static mut CURRENT_MODE: [u32; 4] = [0; 4]; // state_tracker::PeripheralStates::StateCount],

pub struct EnergyTracker {
//pub struct EnergyTracker<'a> {
    total_time: TakeCell<'static, [u32]>,
    current_time: TakeCell<'static, [u32]>,
    current_mode: TakeCell<'static, [u32]>,
//    tt_in_progress: Cell<bool>,
//    ct_in_progress: Cell<bool>,
//    cm_in_progress: Cell<bool>,
    //grants: Grant<App>,
}
/*
#[derive(Default)]
pub struct App {
    total_time: [u32; PeripheralStates::StateCount],
}
*/


//pub struct AccelerateDriver<'a, A: Accelerate

impl<'a> EnergyTracker {
    pub fn new(
        tt: &'static mut [u32],
        ct: &'static mut [u32],
        cm: &'static mut [u32],
    ) -> EnergyTracker {
        EnergyTracker {
            total_time: TakeCell::new(tt),
            current_time: TakeCell::new(ct),
            current_mode: TakeCell::new(cm),
//            tt_in_progress: Cell::new(false),
//            ct_in_progress: Cell::new(false),
//            cm_in_progress: Cell::new(false),
        }
    }
}

impl<'a> state_tracker::StateTracker for EnergyTracker {

    fn track_on(&self, state: usize, component_id: usize, pid: usize) {
//        let cur_time = self.now().into_u32;
//        debug!("Peripheral {} is on at time {}", state, cur_time);

        self.current_mode.map(|buffer| {
            if buffer[state] == 1 {
                debug!("LED: {}, pid: {} stays {}.", component_id, pid, buffer[state]);
            } else {
                buffer[state] = 1;
                debug!("LED: {}, pid: {} switched to {}.", component_id, pid, buffer[state]);
            }
        });
    }


    fn track_off(&self, state: usize, component_id: usize, pid: usize) {
//        let cur_time = self.now().into_u32;
//        debug!("Peripheral {} is off at time {}", state, cur_time);

        self.current_mode.map(|buffer| {
            if buffer[state] == 0 {
                debug!("LED: {}, pid: {} stays {}.", component_id, pid, buffer[state]);
            } else {
                buffer[state] = 0;
                debug!("LED: {}, pid: {} switched to {}.", component_id, pid, buffer[state]);
            }
        });
    }

    /*
    fn energy_tracker_switch(&self, offstate: u32, onstate: u32);
    fn energy_tracker_init(&self);
    fn energy_tracker_setval(&self, state: u32, val: u32);
    fn energy_tracker_gettime(&self, state: u32) -> u32;
    fn energy_tracker_flush(&self);
    */
    
}


