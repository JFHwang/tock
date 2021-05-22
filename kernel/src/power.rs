

pub enum PeripheralStates {
    LED,
    StateCount,
}

#[allow(unused_variables)]
pub trait PowerState {
    fn track_on(&self, state: usize);
    fn track_off(&self, state: usize);
}

