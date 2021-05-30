//! Components for collections of LEDs.
//!
//! Usage
//! -----
//! ```rust
//! let led = components::led::LedsComponent::new(components::led_component_helper!(
//!     kernel::hil::led::LedLow<'static, sam4l::gpio::GPIOPin>,
//!     LedLow::new(&sam4l::gpio::PORT[LED_RED_PIN]),
//!     LedLow::new(&sam4l::gpio::PORT[LED_GREEN_PIN]),
//!     LedLow::new(&sam4l::gpio::PORT[LED_BLUE_PIN]),
//! ))
//! .finalize(led_component_buf!(kernel::hil::led::LedLow<'static, sam4l::gpio::GPIOPin>));
//! ```

use capsules::led::LedDriver;
use core::mem::MaybeUninit;
use kernel::component::Component;
use kernel::hil::energy_tracker;
use kernel::hil::led::Led;
use kernel::static_init_half;

#[macro_export]
macro_rules! led_component_helper {
    ($Led:ty, $($L:expr),+ $(,)?) => {{
        use kernel::count_expressions;
        use kernel::static_init;
        const NUM_LEDS: usize = count_expressions!($($L),+);

        static_init!(
            [&'static $Led; NUM_LEDS],
            [
                $(
                    static_init!(
                        $Led,
                        $L
                    )
                ),+
            ]
        )
    };};
}

#[macro_export]
macro_rules! led_component_buf {
    ($Led:ty $(,)?) => {{
        use capsules::led::LedDriver;
        use core::mem::MaybeUninit;
        static mut BUF: MaybeUninit<LedDriver<'static, $Led>> = MaybeUninit::uninit();
        &mut BUF
    };};
}

pub struct LedsComponent<L: 'static + Led, ET: 'static + energy_tracker::Track> {
    leds: &'static mut [&'static L],
    component_ids: &'static [usize],
    energy_tracker: &'static ET,
}

impl<L: 'static + Led, ET: 'static + energy_tracker::Track> LedsComponent<L, ET> {
    pub fn new(
        leds: &'static mut [&'static L],
        component_ids: &'static [usize],
        energy_tracker: &'static ET,
    ) -> Self {
        Self {
            leds,
            component_ids,
            energy_tracker,
        }
    }
}

impl<L: 'static + Led, ET: 'static + energy_tracker::Track> Component for LedsComponent<L, ET> {
    type StaticInput = &'static mut MaybeUninit<LedDriver<'static, L>>;
    type Output = &'static LedDriver<'static, L>;

    unsafe fn finalize(self, static_buffer: Self::StaticInput) -> Self::Output {
        static_init_half!(
            static_buffer,
            LedDriver<'static, L>,
            LedDriver::new(self.leds, self.component_ids, self.energy_tracker)
        )
    }
}
