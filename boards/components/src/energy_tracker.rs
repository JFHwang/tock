use core::mem::MaybeUninit;

use capsules::energy_tracker::EnergyTracker;
use capsules::virtual_alarm::{MuxAlarm, VirtualMuxAlarm};
use kernel::capabilities;
use kernel::component::Component;
use kernel::create_capability;
use kernel::hil::energy_tracker::PowerModel;
use kernel::hil::time::Alarm;
use kernel::static_init_half;

#[macro_export]
macro_rules! energy_tracker_component_buf {
    ($A:ty $(,)?) => {{
        use capsules::energy_tracker::EnergyTracker;
        use capsules::virtual_alarm::VirtualMuxAlarm;
        use core::mem::MaybeUninit;
        static mut BUF1: MaybeUninit<VirtualMuxAlarm<'static, $A>> = MaybeUninit::uninit();
        static mut BUF2: MaybeUninit<EnergyTracker<'static, VirtualMuxAlarm<'static, $A>>> =
            MaybeUninit::uninit();
        (&mut BUF1, &mut BUF2)
    };};
}

pub struct EnergyTrackerComponent<A: 'static + Alarm<'static>> {
    board_kernel: &'static kernel::Kernel,
    mux_alarm: &'static MuxAlarm<'static, A>,
    power_model: &'static dyn PowerModel,
}

impl<A: 'static + Alarm<'static>> EnergyTrackerComponent<A> {
    pub fn new(
        board_kernel: &'static kernel::Kernel,
        mux_alarm: &'static MuxAlarm<'static, A>,
        power_model: &'static dyn PowerModel,
    ) -> Self {
        Self {
            board_kernel,
            mux_alarm,
            power_model,
        }
    }
}

impl<A: 'static + Alarm<'static>> Component for EnergyTrackerComponent<A> {
    type StaticInput = (
        &'static mut MaybeUninit<VirtualMuxAlarm<'static, A>>,
        &'static mut MaybeUninit<EnergyTracker<'static, VirtualMuxAlarm<'static, A>>>,
    );
    type Output = &'static EnergyTracker<'static, VirtualMuxAlarm<'static, A>>;

    unsafe fn finalize(self, static_buffer: Self::StaticInput) -> Self::Output {
        let grant_cap = create_capability!(capabilities::MemoryAllocationCapability);

        let energy_tracker_virtual_alarm = static_init_half!(
            static_buffer.0,
            VirtualMuxAlarm<'static, A>,
            VirtualMuxAlarm::new(self.mux_alarm)
        );

        let energy_tracker = static_init_half!(
            static_buffer.1,
            EnergyTracker<'static, VirtualMuxAlarm<'static, A>>,
            EnergyTracker::new(
                energy_tracker_virtual_alarm,
                self.board_kernel.create_grant(&grant_cap),
                self.power_model,
                &mut capsules::energy_tracker::ENERGY_STATES,
            )
        );

        energy_tracker
    }
}
