

pub enum PeripheralStates {
    LED,
    StateCount,
}

#[allow(unused_variables)]
pub trait StateTracker {
    fn track_on(&self, state: usize, component_id: usize, pid: usize);
    fn track_off(&self, state: usize, component_id: usize, pid: usize);
}

