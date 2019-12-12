#![doc = "Peripheral access API for K1921VK01T microcontrollers (generated using svd2rust v0.16.1)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.16.1/svd2rust/#peripheral-api"]
#![deny(missing_docs)]
#![deny(warnings)]
#![allow(non_camel_case_types)]
#![no_std]
extern crate bare_metal;
extern crate cortex_m;
#[cfg(feature = "rt")]
extern crate cortex_m_rt;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r"Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 3;
#[cfg(feature = "rt")]
extern "C" {}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 0] = [];
#[doc = r"Enumeration of all the interrupts"]
#[derive(Copy, Clone, Debug)]
pub enum Interrupt {}
unsafe impl bare_metal::Nr for Interrupt {
    #[inline]
    fn nr(&self) -> u8 {
        match *self {}
    }
}
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, FPU, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[allow(unused_imports)]
use generic::*;
#[doc = r"Common register and bit access and modify traits"]
pub mod generic;
#[doc = "ADC controller registers"]
pub struct NT_ADC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for NT_ADC {}
impl NT_ADC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const nt_adc::RegisterBlock {
        0x8000_0000 as *const _
    }
}
impl Deref for NT_ADC {
    type Target = nt_adc::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*NT_ADC::ptr() }
    }
}
#[doc = "ADC controller registers"]
pub mod nt_adc;
#[doc = "GPIO controller regisres"]
pub struct NT_GPIOA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for NT_GPIOA {}
impl NT_GPIOA {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const nt_gpioa::RegisterBlock {
        0x8001_0000 as *const _
    }
}
impl Deref for NT_GPIOA {
    type Target = nt_gpioa::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*NT_GPIOA::ptr() }
    }
}
#[doc = "GPIO controller regisres"]
pub mod nt_gpioa;
#[doc = "GPIO controller regisres"]
pub struct NT_GPIOB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for NT_GPIOB {}
impl NT_GPIOB {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const nt_gpioa::RegisterBlock {
        0x8001_1000 as *const _
    }
}
impl Deref for NT_GPIOB {
    type Target = nt_gpioa::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*NT_GPIOB::ptr() }
    }
}
#[doc = "GPIO controller regisres"]
pub struct NT_GPIOC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for NT_GPIOC {}
impl NT_GPIOC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const nt_gpioa::RegisterBlock {
        0x8001_2000 as *const _
    }
}
impl Deref for NT_GPIOC {
    type Target = nt_gpioa::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*NT_GPIOC::ptr() }
    }
}
#[doc = "GPIO controller regisres"]
pub struct NT_GPIOD {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for NT_GPIOD {}
impl NT_GPIOD {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const nt_gpioa::RegisterBlock {
        0x8001_3000 as *const _
    }
}
impl Deref for NT_GPIOD {
    type Target = nt_gpioa::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*NT_GPIOD::ptr() }
    }
}
#[doc = "GPIO controller regisres"]
pub struct NT_GPIOE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for NT_GPIOE {}
impl NT_GPIOE {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const nt_gpioa::RegisterBlock {
        0x8001_4000 as *const _
    }
}
impl Deref for NT_GPIOE {
    type Target = nt_gpioa::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*NT_GPIOE::ptr() }
    }
}
#[doc = "GPIO controller regisres"]
pub struct NT_GPIOF {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for NT_GPIOF {}
impl NT_GPIOF {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const nt_gpioa::RegisterBlock {
        0x8001_5000 as *const _
    }
}
impl Deref for NT_GPIOF {
    type Target = nt_gpioa::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*NT_GPIOF::ptr() }
    }
}
#[doc = "GPIO controller regisres"]
pub struct NT_GPIOG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for NT_GPIOG {}
impl NT_GPIOG {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const nt_gpioa::RegisterBlock {
        0x8001_6000 as *const _
    }
}
impl Deref for NT_GPIOG {
    type Target = nt_gpioa::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*NT_GPIOG::ptr() }
    }
}
#[doc = "GPIO controller regisres"]
pub struct NT_GPIOH {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for NT_GPIOH {}
impl NT_GPIOH {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const nt_gpioa::RegisterBlock {
        0x8001_7000 as *const _
    }
}
impl Deref for NT_GPIOH {
    type Target = nt_gpioa::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*NT_GPIOH::ptr() }
    }
}
#[doc = "Common block registers"]
pub struct NT_COMMON_REG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for NT_COMMON_REG {}
impl NT_COMMON_REG {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const nt_common_reg::RegisterBlock {
        0x8003_0000 as *const _
    }
}
impl Deref for NT_COMMON_REG {
    type Target = nt_common_reg::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*NT_COMMON_REG::ptr() }
    }
}
#[doc = "Common block registers"]
pub mod nt_common_reg;
#[doc = "CAN controller registers"]
pub struct NT_CAN {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for NT_CAN {}
impl NT_CAN {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const nt_can::RegisterBlock {
        0x8007_0000 as *const _
    }
}
impl Deref for NT_CAN {
    type Target = nt_can::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*NT_CAN::ptr() }
    }
}
#[doc = "CAN controller registers"]
pub mod nt_can;
#[doc = "ETHERNET controller registers"]
pub struct NT_ETHERNET {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for NT_ETHERNET {}
impl NT_ETHERNET {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const nt_ethernet::RegisterBlock {
        0x8008_0000 as *const _
    }
}
impl Deref for NT_ETHERNET {
    type Target = nt_ethernet::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*NT_ETHERNET::ptr() }
    }
}
#[doc = "ETHERNET controller registers"]
pub mod nt_ethernet;
#[doc = "USB Host controller registers"]
pub struct NT_USBHOST {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for NT_USBHOST {}
impl NT_USBHOST {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const nt_usbhost::RegisterBlock {
        0x8009_0000 as *const _
    }
}
impl Deref for NT_USBHOST {
    type Target = nt_usbhost::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*NT_USBHOST::ptr() }
    }
}
#[doc = "USB Host controller registers"]
pub mod nt_usbhost;
#[doc = "USB Device controller registers"]
pub struct NT_USBDEVICE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for NT_USBDEVICE {}
impl NT_USBDEVICE {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const nt_usbdevice::RegisterBlock {
        0x8009_0000 as *const _
    }
}
impl Deref for NT_USBDEVICE {
    type Target = nt_usbdevice::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*NT_USBDEVICE::ptr() }
    }
}
#[doc = "USB Device controller registers"]
pub mod nt_usbdevice;
#[doc = "USB OTG controller registers"]
pub struct NT_USBOTG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for NT_USBOTG {}
impl NT_USBOTG {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const nt_usbotg::RegisterBlock {
        0x8009_0704 as *const _
    }
}
impl Deref for NT_USBOTG {
    type Target = nt_usbotg::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*NT_USBOTG::ptr() }
    }
}
#[doc = "USB OTG controller registers"]
pub mod nt_usbotg;
#[doc = "Watchdog controller registers"]
pub struct NT_WDT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for NT_WDT {}
impl NT_WDT {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const nt_wdt::RegisterBlock {
        0xa000_0000 as *const _
    }
}
impl Deref for NT_WDT {
    type Target = nt_wdt::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*NT_WDT::ptr() }
    }
}
#[doc = "Watchdog controller registers"]
pub mod nt_wdt;
#[doc = "I2C controller registers"]
pub struct NT_I2C0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for NT_I2C0 {}
impl NT_I2C0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const nt_i2c0::RegisterBlock {
        0xa000_1000 as *const _
    }
}
impl Deref for NT_I2C0 {
    type Target = nt_i2c0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*NT_I2C0::ptr() }
    }
}
#[doc = "I2C controller registers"]
pub mod nt_i2c0;
#[doc = "I2C controller registers"]
pub struct NT_I2C1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for NT_I2C1 {}
impl NT_I2C1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const nt_i2c0::RegisterBlock {
        0xa000_2000 as *const _
    }
}
impl Deref for NT_I2C1 {
    type Target = nt_i2c0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*NT_I2C1::ptr() }
    }
}
#[doc = "TIMER controller registers"]
pub struct NT_TIMER0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for NT_TIMER0 {}
impl NT_TIMER0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const nt_timer0::RegisterBlock {
        0xa000_3000 as *const _
    }
}
impl Deref for NT_TIMER0 {
    type Target = nt_timer0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*NT_TIMER0::ptr() }
    }
}
#[doc = "TIMER controller registers"]
pub mod nt_timer0;
#[doc = "TIMER controller registers"]
pub struct NT_TIMER1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for NT_TIMER1 {}
impl NT_TIMER1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const nt_timer0::RegisterBlock {
        0xa000_4000 as *const _
    }
}
impl Deref for NT_TIMER1 {
    type Target = nt_timer0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*NT_TIMER1::ptr() }
    }
}
#[doc = "TIMER controller registers"]
pub struct NT_TIMER2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for NT_TIMER2 {}
impl NT_TIMER2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const nt_timer0::RegisterBlock {
        0xa000_5000 as *const _
    }
}
impl Deref for NT_TIMER2 {
    type Target = nt_timer0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*NT_TIMER2::ptr() }
    }
}
#[doc = "DMA controller registers"]
pub struct NT_DMA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for NT_DMA {}
impl NT_DMA {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const nt_dma::RegisterBlock {
        0xa000_6000 as *const _
    }
}
impl Deref for NT_DMA {
    type Target = nt_dma::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*NT_DMA::ptr() }
    }
}
#[doc = "DMA controller registers"]
pub mod nt_dma;
#[doc = "PWM controller registers"]
pub struct NT_PWM0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for NT_PWM0 {}
impl NT_PWM0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const nt_pwm0::RegisterBlock {
        0xa000_b000 as *const _
    }
}
impl Deref for NT_PWM0 {
    type Target = nt_pwm0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*NT_PWM0::ptr() }
    }
}
#[doc = "PWM controller registers"]
pub mod nt_pwm0;
#[doc = "PWM controller registers"]
pub struct NT_PWM1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for NT_PWM1 {}
impl NT_PWM1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const nt_pwm0::RegisterBlock {
        0xa000_c000 as *const _
    }
}
impl Deref for NT_PWM1 {
    type Target = nt_pwm0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*NT_PWM1::ptr() }
    }
}
#[doc = "PWM controller registers"]
pub struct NT_PWM2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for NT_PWM2 {}
impl NT_PWM2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const nt_pwm0::RegisterBlock {
        0xa000_d000 as *const _
    }
}
impl Deref for NT_PWM2 {
    type Target = nt_pwm0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*NT_PWM2::ptr() }
    }
}
#[doc = "PWM controller registers"]
pub struct NT_PWM3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for NT_PWM3 {}
impl NT_PWM3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const nt_pwm0::RegisterBlock {
        0xa000_e000 as *const _
    }
}
impl Deref for NT_PWM3 {
    type Target = nt_pwm0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*NT_PWM3::ptr() }
    }
}
#[doc = "PWM controller registers"]
pub struct NT_PWM4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for NT_PWM4 {}
impl NT_PWM4 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const nt_pwm0::RegisterBlock {
        0xa000_f000 as *const _
    }
}
impl Deref for NT_PWM4 {
    type Target = nt_pwm0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*NT_PWM4::ptr() }
    }
}
#[doc = "PWM controller registers"]
pub struct NT_PWM5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for NT_PWM5 {}
impl NT_PWM5 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const nt_pwm0::RegisterBlock {
        0xa001_0000 as *const _
    }
}
impl Deref for NT_PWM5 {
    type Target = nt_pwm0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*NT_PWM5::ptr() }
    }
}
#[doc = "PWM controller registers"]
pub struct NT_PWM6 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for NT_PWM6 {}
impl NT_PWM6 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const nt_pwm0::RegisterBlock {
        0xa001_1000 as *const _
    }
}
impl Deref for NT_PWM6 {
    type Target = nt_pwm0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*NT_PWM6::ptr() }
    }
}
#[doc = "PWM controller registers"]
pub struct NT_PWM7 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for NT_PWM7 {}
impl NT_PWM7 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const nt_pwm0::RegisterBlock {
        0xa001_2000 as *const _
    }
}
impl Deref for NT_PWM7 {
    type Target = nt_pwm0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*NT_PWM7::ptr() }
    }
}
#[doc = "PWM controller registers"]
pub struct NT_PWM8 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for NT_PWM8 {}
impl NT_PWM8 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const nt_pwm0::RegisterBlock {
        0xa001_3000 as *const _
    }
}
impl Deref for NT_PWM8 {
    type Target = nt_pwm0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*NT_PWM8::ptr() }
    }
}
#[doc = "CAP controller registers"]
pub struct NT_CAP0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for NT_CAP0 {}
impl NT_CAP0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const nt_cap0::RegisterBlock {
        0xa001_4000 as *const _
    }
}
impl Deref for NT_CAP0 {
    type Target = nt_cap0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*NT_CAP0::ptr() }
    }
}
#[doc = "CAP controller registers"]
pub mod nt_cap0;
#[doc = "CAP controller registers"]
pub struct NT_CAP1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for NT_CAP1 {}
impl NT_CAP1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const nt_cap0::RegisterBlock {
        0xa001_5000 as *const _
    }
}
impl Deref for NT_CAP1 {
    type Target = nt_cap0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*NT_CAP1::ptr() }
    }
}
#[doc = "CAP controller registers"]
pub struct NT_CAP2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for NT_CAP2 {}
impl NT_CAP2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const nt_cap0::RegisterBlock {
        0xa001_6000 as *const _
    }
}
impl Deref for NT_CAP2 {
    type Target = nt_cap0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*NT_CAP2::ptr() }
    }
}
#[doc = "CAP controller registers"]
pub struct NT_CAP3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for NT_CAP3 {}
impl NT_CAP3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const nt_cap0::RegisterBlock {
        0xa001_7000 as *const _
    }
}
impl Deref for NT_CAP3 {
    type Target = nt_cap0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*NT_CAP3::ptr() }
    }
}
#[doc = "CAP controller registers"]
pub struct NT_CAP4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for NT_CAP4 {}
impl NT_CAP4 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const nt_cap0::RegisterBlock {
        0xa001_8000 as *const _
    }
}
impl Deref for NT_CAP4 {
    type Target = nt_cap0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*NT_CAP4::ptr() }
    }
}
#[doc = "CAP controller registers"]
pub struct NT_CAP5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for NT_CAP5 {}
impl NT_CAP5 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const nt_cap0::RegisterBlock {
        0xa001_9000 as *const _
    }
}
impl Deref for NT_CAP5 {
    type Target = nt_cap0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*NT_CAP5::ptr() }
    }
}
#[doc = "QEP controller registers"]
pub struct NT_QEP0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for NT_QEP0 {}
impl NT_QEP0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const nt_qep0::RegisterBlock {
        0xa001_a000 as *const _
    }
}
impl Deref for NT_QEP0 {
    type Target = nt_qep0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*NT_QEP0::ptr() }
    }
}
#[doc = "QEP controller registers"]
pub mod nt_qep0;
#[doc = "QEP controller registers"]
pub struct NT_QEP1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for NT_QEP1 {}
impl NT_QEP1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const nt_qep0::RegisterBlock {
        0xa001_b000 as *const _
    }
}
impl Deref for NT_QEP1 {
    type Target = nt_qep0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*NT_QEP1::ptr() }
    }
}
#[doc = "BOOTFLASH controller registers"]
pub struct NT_BOOTFLASH {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for NT_BOOTFLASH {}
impl NT_BOOTFLASH {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const nt_bootflash::RegisterBlock {
        0xa001_c000 as *const _
    }
}
impl Deref for NT_BOOTFLASH {
    type Target = nt_bootflash::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*NT_BOOTFLASH::ptr() }
    }
}
#[doc = "BOOTFLASH controller registers"]
pub mod nt_bootflash;
#[doc = "UART controller registers"]
pub struct NT_UART0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for NT_UART0 {}
impl NT_UART0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const nt_uart0::RegisterBlock {
        0xa000_7000 as *const _
    }
}
impl Deref for NT_UART0 {
    type Target = nt_uart0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*NT_UART0::ptr() }
    }
}
#[doc = "UART controller registers"]
pub mod nt_uart0;
#[doc = "UART controller registers"]
pub struct NT_UART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for NT_UART1 {}
impl NT_UART1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const nt_uart0::RegisterBlock {
        0xa000_8000 as *const _
    }
}
impl Deref for NT_UART1 {
    type Target = nt_uart0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*NT_UART1::ptr() }
    }
}
#[doc = "UART controller registers"]
pub struct NT_UART2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for NT_UART2 {}
impl NT_UART2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const nt_uart0::RegisterBlock {
        0xa000_9000 as *const _
    }
}
impl Deref for NT_UART2 {
    type Target = nt_uart0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*NT_UART2::ptr() }
    }
}
#[doc = "UART controller registers"]
pub struct NT_UART3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for NT_UART3 {}
impl NT_UART3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const nt_uart0::RegisterBlock {
        0xa000_a000 as *const _
    }
}
impl Deref for NT_UART3 {
    type Target = nt_uart0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*NT_UART3::ptr() }
    }
}
#[doc = "CMP controller registers"]
pub struct NT_CMP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for NT_CMP {}
impl NT_CMP {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const nt_cmp::RegisterBlock {
        0xa001_d000 as *const _
    }
}
impl Deref for NT_CMP {
    type Target = nt_cmp::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*NT_CMP::ptr() }
    }
}
#[doc = "CMP controller registers"]
pub mod nt_cmp;
#[doc = "SPI controller registers"]
pub struct NT_SPI0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for NT_SPI0 {}
impl NT_SPI0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const nt_spi0::RegisterBlock {
        0xa001_e000 as *const _
    }
}
impl Deref for NT_SPI0 {
    type Target = nt_spi0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*NT_SPI0::ptr() }
    }
}
#[doc = "SPI controller registers"]
pub mod nt_spi0;
#[doc = "SPI controller registers"]
pub struct NT_SPI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for NT_SPI1 {}
impl NT_SPI1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const nt_spi0::RegisterBlock {
        0xa001_f000 as *const _
    }
}
impl Deref for NT_SPI1 {
    type Target = nt_spi0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*NT_SPI1::ptr() }
    }
}
#[doc = "SPI controller registers"]
pub struct NT_SPI2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for NT_SPI2 {}
impl NT_SPI2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const nt_spi0::RegisterBlock {
        0xa002_0000 as *const _
    }
}
impl Deref for NT_SPI2 {
    type Target = nt_spi0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*NT_SPI2::ptr() }
    }
}
#[doc = "SPI controller registers"]
pub struct NT_SPI3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for NT_SPI3 {}
impl NT_SPI3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const nt_spi0::RegisterBlock {
        0xa002_1000 as *const _
    }
}
impl Deref for NT_SPI3 {
    type Target = nt_spi0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*NT_SPI3::ptr() }
    }
}
#[doc = "USERFLASH controller registers"]
pub struct NT_USERFLASH {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for NT_USERFLASH {}
impl NT_USERFLASH {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const nt_userflash::RegisterBlock {
        0xa002_2000 as *const _
    }
}
impl Deref for NT_USERFLASH {
    type Target = nt_userflash::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*NT_USERFLASH::ptr() }
    }
}
#[doc = "USERFLASH controller registers"]
pub mod nt_userflash;
#[doc = "RTC controller registers"]
pub struct NT_RTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for NT_RTC {}
impl NT_RTC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const nt_rtc::RegisterBlock {
        0xa002_3000 as *const _
    }
}
impl Deref for NT_RTC {
    type Target = nt_rtc::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*NT_RTC::ptr() }
    }
}
#[doc = "RTC controller registers"]
pub mod nt_rtc;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r"All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "NT_ADC"]
    pub NT_ADC: NT_ADC,
    #[doc = "NT_GPIOA"]
    pub NT_GPIOA: NT_GPIOA,
    #[doc = "NT_GPIOB"]
    pub NT_GPIOB: NT_GPIOB,
    #[doc = "NT_GPIOC"]
    pub NT_GPIOC: NT_GPIOC,
    #[doc = "NT_GPIOD"]
    pub NT_GPIOD: NT_GPIOD,
    #[doc = "NT_GPIOE"]
    pub NT_GPIOE: NT_GPIOE,
    #[doc = "NT_GPIOF"]
    pub NT_GPIOF: NT_GPIOF,
    #[doc = "NT_GPIOG"]
    pub NT_GPIOG: NT_GPIOG,
    #[doc = "NT_GPIOH"]
    pub NT_GPIOH: NT_GPIOH,
    #[doc = "NT_COMMON_REG"]
    pub NT_COMMON_REG: NT_COMMON_REG,
    #[doc = "NT_CAN"]
    pub NT_CAN: NT_CAN,
    #[doc = "NT_ETHERNET"]
    pub NT_ETHERNET: NT_ETHERNET,
    #[doc = "NT_USBHOST"]
    pub NT_USBHOST: NT_USBHOST,
    #[doc = "NT_USBDEVICE"]
    pub NT_USBDEVICE: NT_USBDEVICE,
    #[doc = "NT_USBOTG"]
    pub NT_USBOTG: NT_USBOTG,
    #[doc = "NT_WDT"]
    pub NT_WDT: NT_WDT,
    #[doc = "NT_I2C0"]
    pub NT_I2C0: NT_I2C0,
    #[doc = "NT_I2C1"]
    pub NT_I2C1: NT_I2C1,
    #[doc = "NT_TIMER0"]
    pub NT_TIMER0: NT_TIMER0,
    #[doc = "NT_TIMER1"]
    pub NT_TIMER1: NT_TIMER1,
    #[doc = "NT_TIMER2"]
    pub NT_TIMER2: NT_TIMER2,
    #[doc = "NT_DMA"]
    pub NT_DMA: NT_DMA,
    #[doc = "NT_PWM0"]
    pub NT_PWM0: NT_PWM0,
    #[doc = "NT_PWM1"]
    pub NT_PWM1: NT_PWM1,
    #[doc = "NT_PWM2"]
    pub NT_PWM2: NT_PWM2,
    #[doc = "NT_PWM3"]
    pub NT_PWM3: NT_PWM3,
    #[doc = "NT_PWM4"]
    pub NT_PWM4: NT_PWM4,
    #[doc = "NT_PWM5"]
    pub NT_PWM5: NT_PWM5,
    #[doc = "NT_PWM6"]
    pub NT_PWM6: NT_PWM6,
    #[doc = "NT_PWM7"]
    pub NT_PWM7: NT_PWM7,
    #[doc = "NT_PWM8"]
    pub NT_PWM8: NT_PWM8,
    #[doc = "NT_CAP0"]
    pub NT_CAP0: NT_CAP0,
    #[doc = "NT_CAP1"]
    pub NT_CAP1: NT_CAP1,
    #[doc = "NT_CAP2"]
    pub NT_CAP2: NT_CAP2,
    #[doc = "NT_CAP3"]
    pub NT_CAP3: NT_CAP3,
    #[doc = "NT_CAP4"]
    pub NT_CAP4: NT_CAP4,
    #[doc = "NT_CAP5"]
    pub NT_CAP5: NT_CAP5,
    #[doc = "NT_QEP0"]
    pub NT_QEP0: NT_QEP0,
    #[doc = "NT_QEP1"]
    pub NT_QEP1: NT_QEP1,
    #[doc = "NT_BOOTFLASH"]
    pub NT_BOOTFLASH: NT_BOOTFLASH,
    #[doc = "NT_UART0"]
    pub NT_UART0: NT_UART0,
    #[doc = "NT_UART1"]
    pub NT_UART1: NT_UART1,
    #[doc = "NT_UART2"]
    pub NT_UART2: NT_UART2,
    #[doc = "NT_UART3"]
    pub NT_UART3: NT_UART3,
    #[doc = "NT_CMP"]
    pub NT_CMP: NT_CMP,
    #[doc = "NT_SPI0"]
    pub NT_SPI0: NT_SPI0,
    #[doc = "NT_SPI1"]
    pub NT_SPI1: NT_SPI1,
    #[doc = "NT_SPI2"]
    pub NT_SPI2: NT_SPI2,
    #[doc = "NT_SPI3"]
    pub NT_SPI3: NT_SPI3,
    #[doc = "NT_USERFLASH"]
    pub NT_USERFLASH: NT_USERFLASH,
    #[doc = "NT_RTC"]
    pub NT_RTC: NT_RTC,
}
impl Peripherals {
    #[doc = r"Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    #[doc = r"Unchecked version of `Peripherals::take`"]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            NT_ADC: NT_ADC {
                _marker: PhantomData,
            },
            NT_GPIOA: NT_GPIOA {
                _marker: PhantomData,
            },
            NT_GPIOB: NT_GPIOB {
                _marker: PhantomData,
            },
            NT_GPIOC: NT_GPIOC {
                _marker: PhantomData,
            },
            NT_GPIOD: NT_GPIOD {
                _marker: PhantomData,
            },
            NT_GPIOE: NT_GPIOE {
                _marker: PhantomData,
            },
            NT_GPIOF: NT_GPIOF {
                _marker: PhantomData,
            },
            NT_GPIOG: NT_GPIOG {
                _marker: PhantomData,
            },
            NT_GPIOH: NT_GPIOH {
                _marker: PhantomData,
            },
            NT_COMMON_REG: NT_COMMON_REG {
                _marker: PhantomData,
            },
            NT_CAN: NT_CAN {
                _marker: PhantomData,
            },
            NT_ETHERNET: NT_ETHERNET {
                _marker: PhantomData,
            },
            NT_USBHOST: NT_USBHOST {
                _marker: PhantomData,
            },
            NT_USBDEVICE: NT_USBDEVICE {
                _marker: PhantomData,
            },
            NT_USBOTG: NT_USBOTG {
                _marker: PhantomData,
            },
            NT_WDT: NT_WDT {
                _marker: PhantomData,
            },
            NT_I2C0: NT_I2C0 {
                _marker: PhantomData,
            },
            NT_I2C1: NT_I2C1 {
                _marker: PhantomData,
            },
            NT_TIMER0: NT_TIMER0 {
                _marker: PhantomData,
            },
            NT_TIMER1: NT_TIMER1 {
                _marker: PhantomData,
            },
            NT_TIMER2: NT_TIMER2 {
                _marker: PhantomData,
            },
            NT_DMA: NT_DMA {
                _marker: PhantomData,
            },
            NT_PWM0: NT_PWM0 {
                _marker: PhantomData,
            },
            NT_PWM1: NT_PWM1 {
                _marker: PhantomData,
            },
            NT_PWM2: NT_PWM2 {
                _marker: PhantomData,
            },
            NT_PWM3: NT_PWM3 {
                _marker: PhantomData,
            },
            NT_PWM4: NT_PWM4 {
                _marker: PhantomData,
            },
            NT_PWM5: NT_PWM5 {
                _marker: PhantomData,
            },
            NT_PWM6: NT_PWM6 {
                _marker: PhantomData,
            },
            NT_PWM7: NT_PWM7 {
                _marker: PhantomData,
            },
            NT_PWM8: NT_PWM8 {
                _marker: PhantomData,
            },
            NT_CAP0: NT_CAP0 {
                _marker: PhantomData,
            },
            NT_CAP1: NT_CAP1 {
                _marker: PhantomData,
            },
            NT_CAP2: NT_CAP2 {
                _marker: PhantomData,
            },
            NT_CAP3: NT_CAP3 {
                _marker: PhantomData,
            },
            NT_CAP4: NT_CAP4 {
                _marker: PhantomData,
            },
            NT_CAP5: NT_CAP5 {
                _marker: PhantomData,
            },
            NT_QEP0: NT_QEP0 {
                _marker: PhantomData,
            },
            NT_QEP1: NT_QEP1 {
                _marker: PhantomData,
            },
            NT_BOOTFLASH: NT_BOOTFLASH {
                _marker: PhantomData,
            },
            NT_UART0: NT_UART0 {
                _marker: PhantomData,
            },
            NT_UART1: NT_UART1 {
                _marker: PhantomData,
            },
            NT_UART2: NT_UART2 {
                _marker: PhantomData,
            },
            NT_UART3: NT_UART3 {
                _marker: PhantomData,
            },
            NT_CMP: NT_CMP {
                _marker: PhantomData,
            },
            NT_SPI0: NT_SPI0 {
                _marker: PhantomData,
            },
            NT_SPI1: NT_SPI1 {
                _marker: PhantomData,
            },
            NT_SPI2: NT_SPI2 {
                _marker: PhantomData,
            },
            NT_SPI3: NT_SPI3 {
                _marker: PhantomData,
            },
            NT_USERFLASH: NT_USERFLASH {
                _marker: PhantomData,
            },
            NT_RTC: NT_RTC {
                _marker: PhantomData,
            },
        }
    }
}
