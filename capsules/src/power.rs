//use kernel::hil::time::Ticks;
use kernel::power;
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

pub struct Power {
//pub struct Power<'a> {
    //grants: Grant<App>,
    total_time: [u32; 1], //power::PeripheralStates::StateCount],
    current_time: [u32; 1], // power::PeripheralStates::StateCount],
    current_mode: [u32; 1], // power::PeripheralStates::StateCount],
}
/*
#[derive(Default)]
pub struct App {
    total_time: [u32; PeripheralStates::StateCount],
}
*/


//pub struct AccelerateDriver<'a, A: Accelerate

impl<'a> Power {
    pub fn new() -> Power {
        Power {
            total_time:   [0; 1], // power::PeripheralStates::StateCount],
            current_time: [0; 1], // power::PeripheralStates::StateCount],
            current_mode: [0; 1], // power::PeripheralStates::StateCount],
        }
    }
}

impl<'a> power::PowerState for Power {
    fn track_on(&self, state: usize) {
//        self.total_time[state] = 1;
//        let cur_time = self.now().into_u32;
//        debug!("Peripheral {} is on at time {}", state, cur_time);
        debug!("Peripheral {} is on", state);
    }

    fn track_off(&self, state: usize) {
//        self.total_time[state] = 0;
//        let cur_time = self.now().into_u32;
//        debug!("Peripheral {} is off at time {}", state, cur_time);
        debug!("Peripheral {} is off", state);
    }

    /*
    fn power_switch(&self, offstate: u32, onstate: u32);
    fn power_init(&self);
    fn power_setval(&self, state: u32, val: u32);
    fn power_gettime(&self, state: u32) -> u32;
    fn power_flush(&self);
    */
    
}


