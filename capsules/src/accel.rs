use kernel::hil::time::{self, Alarm, Ticks};
use kernel::debug;

///// Syscall driver number.
//use crate::driver;
//pub const DRIVER_NUM: usize = driver::NUM::Accel as usize;

pub struct Accelerate<'a, A: Alarm<'a> + 'a> {
    alarm: &'a A
}

//pub struct AccelerateDriver<'a, A: Accelerate


impl<'a, A: Alarm<'a>> Accelerate<'a, A> {
    pub fn new(alarm: &'a A) -> Accelerate<'a, A> {
        Accelerate {
            alarm: alarm
        }
    }

    pub fn start(&self) {
        let delay = A::ticks_from_ms(10);
        self.alarm.set_alarm(self.alarm.now().wrapping_add(A::ticks_from_seconds(1)), delay);
    }
}

impl<'a, A: Alarm<'a>> time::AlarmClient for Accelerate<'a, A> {
    fn alarm(&self) {
        self.start();
        debug!("Hi world");
    }
}



