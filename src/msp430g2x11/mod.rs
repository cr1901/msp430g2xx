#![doc = "Peripheral access API for MSP430G2X11 microcontrollers (generated using svd2rust v0.34.0 (74eee32 2024-11-11))\n\nYou can find an overview of the generated API [here].\n\nAPI features to be included in the [next]
svd2rust release can be generated by cloning the svd2rust [repository], checking out the above commit, and running `cargo doc --open`.\n\n[here]: https://docs.rs/svd2rust/0.34.0/svd2rust/#peripheral-api\n[next]: https://github.com/rust-embedded/svd2rust/blob/master/CHANGELOG.md#unreleased\n[repository]: https://github.com/rust-embedded/svd2rust"]
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
use core::marker::PhantomData;
use core::ops::Deref;
#[cfg(feature = "rt")]
pub use msp430_rt::interrupt;
#[cfg(feature = "rt")]
extern "msp430-interrupt" {
    fn PORT1();
    fn PORT2();
    fn TIMERA1();
    fn TIMERA0();
    fn WDT();
    fn COMPARATORA();
    fn NMI();
}
#[doc(hidden)]
#[repr(C)]
pub union Vector {
    _handler: unsafe extern "msp430-interrupt" fn(),
    _reserved: u16,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
#[used]
pub static __INTERRUPTS: [Vector; 15] = [
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: PORT1 },
    Vector { _handler: PORT2 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: TIMERA1 },
    Vector { _handler: TIMERA0 },
    Vector { _handler: WDT },
    Vector {
        _handler: COMPARATORA,
    },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: NMI },
];
#[doc = r"Enumeration of all the interrupts. This enum is seldom used in application or library crates. It is present primarily for documenting the device's implemented interrupts."]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    #[doc = "2 - 0xFFE4 Port 1"]
    PORT1 = 2,
    #[doc = "3 - 0xFFE6 Port 2"]
    PORT2 = 3,
    #[doc = "8 - 0xFFF0 Timer A CC1-2, TA"]
    TIMERA1 = 8,
    #[doc = "9 - 0xFFF2 Timer A CC0"]
    TIMERA0 = 9,
    #[doc = "10 - 0xFFF4 Watchdog Timer"]
    WDT = 10,
    #[doc = "11 - 0xFFF6 Comparator A"]
    COMPARATORA = 11,
    #[doc = "14 - 0xFFFC Non-maskable"]
    NMI = 14,
}
#[doc = "Special Function"]
pub struct SpecialFunction {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SpecialFunction {}
impl SpecialFunction {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const special_function::RegisterBlock = 0 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const special_function::RegisterBlock {
        Self::PTR
    }
    #[doc = r" Steal an instance of this peripheral"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Ensure that the new instance of the peripheral cannot be used in a way"]
    #[doc = r" that may race with any existing instances, for example by only"]
    #[doc = r" accessing read-only or write-only registers, or by consuming the"]
    #[doc = r" original peripheral and using critical sections to coordinate"]
    #[doc = r" access between multiple new instances."]
    #[doc = r""]
    #[doc = r" Additionally, other software such as HALs may rely on only one"]
    #[doc = r" peripheral instance existing to ensure memory safety; ensure"]
    #[doc = r" no stolen instances are passed to such software."]
    pub unsafe fn steal() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}
impl Deref for SpecialFunction {
    type Target = special_function::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SpecialFunction {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SpecialFunction").finish()
    }
}
#[doc = "Special Function"]
pub mod special_function;
#[doc = "Port 1/2"]
pub struct Port1_2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for Port1_2 {}
impl Port1_2 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const port_1_2::RegisterBlock = 0x20 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const port_1_2::RegisterBlock {
        Self::PTR
    }
    #[doc = r" Steal an instance of this peripheral"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Ensure that the new instance of the peripheral cannot be used in a way"]
    #[doc = r" that may race with any existing instances, for example by only"]
    #[doc = r" accessing read-only or write-only registers, or by consuming the"]
    #[doc = r" original peripheral and using critical sections to coordinate"]
    #[doc = r" access between multiple new instances."]
    #[doc = r""]
    #[doc = r" Additionally, other software such as HALs may rely on only one"]
    #[doc = r" peripheral instance existing to ensure memory safety; ensure"]
    #[doc = r" no stolen instances are passed to such software."]
    pub unsafe fn steal() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}
impl Deref for Port1_2 {
    type Target = port_1_2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for Port1_2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Port1_2").finish()
    }
}
#[doc = "Port 1/2"]
pub mod port_1_2;
#[doc = "System Clock"]
pub struct SystemClock {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SystemClock {}
impl SystemClock {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const system_clock::RegisterBlock = 0x52 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const system_clock::RegisterBlock {
        Self::PTR
    }
    #[doc = r" Steal an instance of this peripheral"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Ensure that the new instance of the peripheral cannot be used in a way"]
    #[doc = r" that may race with any existing instances, for example by only"]
    #[doc = r" accessing read-only or write-only registers, or by consuming the"]
    #[doc = r" original peripheral and using critical sections to coordinate"]
    #[doc = r" access between multiple new instances."]
    #[doc = r""]
    #[doc = r" Additionally, other software such as HALs may rely on only one"]
    #[doc = r" peripheral instance existing to ensure memory safety; ensure"]
    #[doc = r" no stolen instances are passed to such software."]
    pub unsafe fn steal() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}
impl Deref for SystemClock {
    type Target = system_clock::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SystemClock {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SystemClock").finish()
    }
}
#[doc = "System Clock"]
pub mod system_clock;
#[doc = "Comparator A"]
pub struct ComparatorA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ComparatorA {}
impl ComparatorA {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const comparator_a::RegisterBlock = 0x58 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const comparator_a::RegisterBlock {
        Self::PTR
    }
    #[doc = r" Steal an instance of this peripheral"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Ensure that the new instance of the peripheral cannot be used in a way"]
    #[doc = r" that may race with any existing instances, for example by only"]
    #[doc = r" accessing read-only or write-only registers, or by consuming the"]
    #[doc = r" original peripheral and using critical sections to coordinate"]
    #[doc = r" access between multiple new instances."]
    #[doc = r""]
    #[doc = r" Additionally, other software such as HALs may rely on only one"]
    #[doc = r" peripheral instance existing to ensure memory safety; ensure"]
    #[doc = r" no stolen instances are passed to such software."]
    pub unsafe fn steal() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}
impl Deref for ComparatorA {
    type Target = comparator_a::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for ComparatorA {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ComparatorA").finish()
    }
}
#[doc = "Comparator A"]
pub mod comparator_a;
#[doc = "Calibration Data"]
pub struct CalibrationData {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CalibrationData {}
impl CalibrationData {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const calibration_data::RegisterBlock = 0x10fe as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const calibration_data::RegisterBlock {
        Self::PTR
    }
    #[doc = r" Steal an instance of this peripheral"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Ensure that the new instance of the peripheral cannot be used in a way"]
    #[doc = r" that may race with any existing instances, for example by only"]
    #[doc = r" accessing read-only or write-only registers, or by consuming the"]
    #[doc = r" original peripheral and using critical sections to coordinate"]
    #[doc = r" access between multiple new instances."]
    #[doc = r""]
    #[doc = r" Additionally, other software such as HALs may rely on only one"]
    #[doc = r" peripheral instance existing to ensure memory safety; ensure"]
    #[doc = r" no stolen instances are passed to such software."]
    pub unsafe fn steal() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}
impl Deref for CalibrationData {
    type Target = calibration_data::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for CalibrationData {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CalibrationData").finish()
    }
}
#[doc = "Calibration Data"]
pub mod calibration_data;
#[doc = "Watchdog Timer"]
pub struct WatchdogTimer {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WatchdogTimer {}
impl WatchdogTimer {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const watchdog_timer::RegisterBlock = 0x0120 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const watchdog_timer::RegisterBlock {
        Self::PTR
    }
    #[doc = r" Steal an instance of this peripheral"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Ensure that the new instance of the peripheral cannot be used in a way"]
    #[doc = r" that may race with any existing instances, for example by only"]
    #[doc = r" accessing read-only or write-only registers, or by consuming the"]
    #[doc = r" original peripheral and using critical sections to coordinate"]
    #[doc = r" access between multiple new instances."]
    #[doc = r""]
    #[doc = r" Additionally, other software such as HALs may rely on only one"]
    #[doc = r" peripheral instance existing to ensure memory safety; ensure"]
    #[doc = r" no stolen instances are passed to such software."]
    pub unsafe fn steal() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}
impl Deref for WatchdogTimer {
    type Target = watchdog_timer::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for WatchdogTimer {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WatchdogTimer").finish()
    }
}
#[doc = "Watchdog Timer"]
pub mod watchdog_timer;
#[doc = "Flash"]
pub struct Flash {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for Flash {}
impl Flash {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const flash::RegisterBlock = 0x0128 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const flash::RegisterBlock {
        Self::PTR
    }
    #[doc = r" Steal an instance of this peripheral"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Ensure that the new instance of the peripheral cannot be used in a way"]
    #[doc = r" that may race with any existing instances, for example by only"]
    #[doc = r" accessing read-only or write-only registers, or by consuming the"]
    #[doc = r" original peripheral and using critical sections to coordinate"]
    #[doc = r" access between multiple new instances."]
    #[doc = r""]
    #[doc = r" Additionally, other software such as HALs may rely on only one"]
    #[doc = r" peripheral instance existing to ensure memory safety; ensure"]
    #[doc = r" no stolen instances are passed to such software."]
    pub unsafe fn steal() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}
impl Deref for Flash {
    type Target = flash::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for Flash {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flash").finish()
    }
}
#[doc = "Flash"]
pub mod flash;
#[doc = "Timer A2"]
pub struct TimerA2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TimerA2 {}
impl TimerA2 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const timer_a2::RegisterBlock = 0x012e as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer_a2::RegisterBlock {
        Self::PTR
    }
    #[doc = r" Steal an instance of this peripheral"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Ensure that the new instance of the peripheral cannot be used in a way"]
    #[doc = r" that may race with any existing instances, for example by only"]
    #[doc = r" accessing read-only or write-only registers, or by consuming the"]
    #[doc = r" original peripheral and using critical sections to coordinate"]
    #[doc = r" access between multiple new instances."]
    #[doc = r""]
    #[doc = r" Additionally, other software such as HALs may rely on only one"]
    #[doc = r" peripheral instance existing to ensure memory safety; ensure"]
    #[doc = r" no stolen instances are passed to such software."]
    pub unsafe fn steal() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}
impl Deref for TimerA2 {
    type Target = timer_a2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for TimerA2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TimerA2").finish()
    }
}
#[doc = "Timer A2"]
pub mod timer_a2;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r" All the peripherals."]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "SPECIAL_FUNCTION"]
    pub special_function: SpecialFunction,
    #[doc = "PORT_1_2"]
    pub port_1_2: Port1_2,
    #[doc = "SYSTEM_CLOCK"]
    pub system_clock: SystemClock,
    #[doc = "COMPARATOR_A"]
    pub comparator_a: ComparatorA,
    #[doc = "CALIBRATION_DATA"]
    pub calibration_data: CalibrationData,
    #[doc = "WATCHDOG_TIMER"]
    pub watchdog_timer: WatchdogTimer,
    #[doc = "FLASH"]
    pub flash: Flash,
    #[doc = "TIMER_A2"]
    pub timer_a2: TimerA2,
}
impl Peripherals {
    #[doc = r" Returns all the peripherals *once*."]
    #[cfg(feature = "critical-section")]
    #[inline]
    pub fn take() -> Option<Self> {
        critical_section::with(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                return None;
            }
            Some(unsafe { Peripherals::steal() })
        })
    }
    #[doc = r" Unchecked version of `Peripherals::take`."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Each of the returned peripherals must be used at most once."]
    #[inline]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            special_function: SpecialFunction::steal(),
            port_1_2: Port1_2::steal(),
            system_clock: SystemClock::steal(),
            comparator_a: ComparatorA::steal(),
            calibration_data: CalibrationData::steal(),
            watchdog_timer: WatchdogTimer::steal(),
            flash: Flash::steal(),
            timer_a2: TimerA2::steal(),
        }
    }
}
