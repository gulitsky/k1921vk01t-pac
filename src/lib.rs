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
extern "C" {
    fn WWDG();
    fn I2C0();
    fn I2C1();
    fn TIM0();
    fn TIM1();
    fn TIM2();
    fn DMA_STREAM0();
    fn DMA_STREAM1();
    fn DMA_STREAM2();
    fn DMA_STREAM3();
    fn DMA_STREAM4();
    fn DMA_STREAM5();
    fn DMA_STREAM6();
    fn DMA_STREAM7();
    fn DMA_STREAM8();
    fn DMA_STREAM9();
    fn DMA_STREAM10();
    fn DMA_STREAM11();
    fn DMA_STREAM12();
    fn DMA_STREAM13();
    fn DMA_STREAM14();
    fn DMA_STREAM15();
    fn DMA_STREAM16();
    fn DMA_STREAM17();
    fn DMA_STREAM18();
    fn DMA_STREAM19();
    fn DMA_STREAM20();
    fn DMA_STREAM21();
    fn DMA_STREAM22();
    fn DMA_STREAM23();
    fn UART0_MS();
    fn UART0_RX();
    fn UART0_TX();
    fn UART0_RT();
    fn UART0_E();
    fn UART0();
    fn UART1_MS();
    fn UART1_RX();
    fn UART1_TX();
    fn UART1_RT();
    fn UART1_E();
    fn UART1();
    fn UART2_MS();
    fn UART2_RX();
    fn UART2_TX();
    fn UART2_RT();
    fn UART2_E();
    fn UART2();
    fn UART3_MS();
    fn UART3_RX();
    fn UART3_TX();
    fn UART3_RT();
    fn UART3_E();
    fn UART3();
    fn PWM0();
    fn PWM0_HD();
    fn PWM0_TZ();
    fn PWM1();
    fn PWM1_HD();
    fn PWM1_TZ();
    fn PWM2();
    fn PWM2_HD();
    fn PWM2_TZ();
    fn PWM3();
    fn PWM3_HD();
    fn PWM3_TZ();
    fn PWM4();
    fn PWM4_HD();
    fn PWM4_TZ();
    fn PWM5();
    fn PWM5_HD();
    fn PWM5_TZ();
    fn PWM6();
    fn PWM6_HD();
    fn PWM6_TZ();
    fn PWM7();
    fn PWM7_HD();
    fn PWM7_TZ();
    fn PWM8();
    fn PWM8_HD();
    fn PWM8_TZ();
    fn ADC_SEQ0();
    fn ADC_SEQ1();
    fn ADC_SEQ2();
    fn ADC_SEQ3();
    fn ADC_SEQ4();
    fn ADC_SEQ5();
    fn ADC_SEQ6();
    fn ADC_SEQ7();
    fn ADC_COMPINT();
    fn CAP0();
    fn CAP1();
    fn CAP2();
    fn CAP3();
    fn CAP4();
    fn CAP5();
    fn QEP0();
    fn QEP1();
    fn BOOTFLASH();
    fn CMP0();
    fn CMP1();
    fn CMP2();
    fn SPI0();
    fn SPI1();
    fn SPI2();
    fn SPI3();
    fn USERFLASH();
    fn GPIOA();
    fn GPIOB();
    fn GPIOC();
    fn GPIOD();
    fn GPIOE();
    fn GPIOF();
    fn GPIOG();
    fn GPIOH();
    fn ETHERNET();
    fn CAN0();
    fn CAN1();
    fn CAN2();
    fn CAN3();
    fn CAN4();
    fn CAN5();
    fn CAN6();
    fn CAN7();
    fn CAN8();
    fn CAN9();
    fn CAN10();
    fn CAN11();
    fn CAN12();
    fn CAN13();
    fn CAN14();
    fn CAN15();
    fn RTC();
    fn USB();
}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 134] = [
    Vector { _handler: WWDG },
    Vector { _handler: I2C0 },
    Vector { _handler: I2C1 },
    Vector { _handler: TIM0 },
    Vector { _handler: TIM1 },
    Vector { _handler: TIM2 },
    Vector {
        _handler: DMA_STREAM0,
    },
    Vector {
        _handler: DMA_STREAM1,
    },
    Vector {
        _handler: DMA_STREAM2,
    },
    Vector {
        _handler: DMA_STREAM3,
    },
    Vector {
        _handler: DMA_STREAM4,
    },
    Vector {
        _handler: DMA_STREAM5,
    },
    Vector {
        _handler: DMA_STREAM6,
    },
    Vector {
        _handler: DMA_STREAM7,
    },
    Vector {
        _handler: DMA_STREAM8,
    },
    Vector {
        _handler: DMA_STREAM9,
    },
    Vector {
        _handler: DMA_STREAM10,
    },
    Vector {
        _handler: DMA_STREAM11,
    },
    Vector {
        _handler: DMA_STREAM12,
    },
    Vector {
        _handler: DMA_STREAM13,
    },
    Vector {
        _handler: DMA_STREAM14,
    },
    Vector {
        _handler: DMA_STREAM15,
    },
    Vector {
        _handler: DMA_STREAM16,
    },
    Vector {
        _handler: DMA_STREAM17,
    },
    Vector {
        _handler: DMA_STREAM18,
    },
    Vector {
        _handler: DMA_STREAM19,
    },
    Vector {
        _handler: DMA_STREAM20,
    },
    Vector {
        _handler: DMA_STREAM21,
    },
    Vector {
        _handler: DMA_STREAM22,
    },
    Vector {
        _handler: DMA_STREAM23,
    },
    Vector { _handler: UART0_MS },
    Vector { _handler: UART0_RX },
    Vector { _handler: UART0_TX },
    Vector { _handler: UART0_RT },
    Vector { _handler: UART0_E },
    Vector { _handler: UART0 },
    Vector { _handler: UART1_MS },
    Vector { _handler: UART1_RX },
    Vector { _handler: UART1_TX },
    Vector { _handler: UART1_RT },
    Vector { _handler: UART1_E },
    Vector { _handler: UART1 },
    Vector { _handler: UART2_MS },
    Vector { _handler: UART2_RX },
    Vector { _handler: UART2_TX },
    Vector { _handler: UART2_RT },
    Vector { _handler: UART2_E },
    Vector { _handler: UART2 },
    Vector { _handler: UART3_MS },
    Vector { _handler: UART3_RX },
    Vector { _handler: UART3_TX },
    Vector { _handler: UART3_RT },
    Vector { _handler: UART3_E },
    Vector { _handler: UART3 },
    Vector { _handler: PWM0 },
    Vector { _handler: PWM0_HD },
    Vector { _handler: PWM0_TZ },
    Vector { _handler: PWM1 },
    Vector { _handler: PWM1_HD },
    Vector { _handler: PWM1_TZ },
    Vector { _handler: PWM2 },
    Vector { _handler: PWM2_HD },
    Vector { _handler: PWM2_TZ },
    Vector { _handler: PWM3 },
    Vector { _handler: PWM3_HD },
    Vector { _handler: PWM3_TZ },
    Vector { _handler: PWM4 },
    Vector { _handler: PWM4_HD },
    Vector { _handler: PWM4_TZ },
    Vector { _handler: PWM5 },
    Vector { _handler: PWM5_HD },
    Vector { _handler: PWM5_TZ },
    Vector { _handler: PWM6 },
    Vector { _handler: PWM6_HD },
    Vector { _handler: PWM6_TZ },
    Vector { _handler: PWM7 },
    Vector { _handler: PWM7_HD },
    Vector { _handler: PWM7_TZ },
    Vector { _handler: PWM8 },
    Vector { _handler: PWM8_HD },
    Vector { _handler: PWM8_TZ },
    Vector { _handler: ADC_SEQ0 },
    Vector { _handler: ADC_SEQ1 },
    Vector { _handler: ADC_SEQ2 },
    Vector { _handler: ADC_SEQ3 },
    Vector { _handler: ADC_SEQ4 },
    Vector { _handler: ADC_SEQ5 },
    Vector { _handler: ADC_SEQ6 },
    Vector { _handler: ADC_SEQ7 },
    Vector {
        _handler: ADC_COMPINT,
    },
    Vector { _handler: CAP0 },
    Vector { _handler: CAP1 },
    Vector { _handler: CAP2 },
    Vector { _handler: CAP3 },
    Vector { _handler: CAP4 },
    Vector { _handler: CAP5 },
    Vector { _handler: QEP0 },
    Vector { _handler: QEP1 },
    Vector {
        _handler: BOOTFLASH,
    },
    Vector { _handler: CMP0 },
    Vector { _handler: CMP1 },
    Vector { _handler: CMP2 },
    Vector { _handler: SPI0 },
    Vector { _handler: SPI1 },
    Vector { _handler: SPI2 },
    Vector { _handler: SPI3 },
    Vector {
        _handler: USERFLASH,
    },
    Vector { _handler: GPIOA },
    Vector { _handler: GPIOB },
    Vector { _handler: GPIOC },
    Vector { _handler: GPIOD },
    Vector { _handler: GPIOE },
    Vector { _handler: GPIOF },
    Vector { _handler: GPIOG },
    Vector { _handler: GPIOH },
    Vector { _handler: ETHERNET },
    Vector { _handler: CAN0 },
    Vector { _handler: CAN1 },
    Vector { _handler: CAN2 },
    Vector { _handler: CAN3 },
    Vector { _handler: CAN4 },
    Vector { _handler: CAN5 },
    Vector { _handler: CAN6 },
    Vector { _handler: CAN7 },
    Vector { _handler: CAN8 },
    Vector { _handler: CAN9 },
    Vector { _handler: CAN10 },
    Vector { _handler: CAN11 },
    Vector { _handler: CAN12 },
    Vector { _handler: CAN13 },
    Vector { _handler: CAN14 },
    Vector { _handler: CAN15 },
    Vector { _handler: RTC },
    Vector { _handler: USB },
];
#[doc = r"Enumeration of all the interrupts"]
#[derive(Copy, Clone, Debug)]
pub enum Interrupt {
    #[doc = "0 - WWDG"]
    WWDG,
    #[doc = "1 - I2C0"]
    I2C0,
    #[doc = "2 - I2C1"]
    I2C1,
    #[doc = "3 - TIM0"]
    TIM0,
    #[doc = "4 - TIM1"]
    TIM1,
    #[doc = "5 - TIM2"]
    TIM2,
    #[doc = "6 - DMA_Stream0"]
    DMA_STREAM0,
    #[doc = "7 - DMA_Stream1"]
    DMA_STREAM1,
    #[doc = "8 - DMA_Stream2"]
    DMA_STREAM2,
    #[doc = "9 - DMA_Stream3"]
    DMA_STREAM3,
    #[doc = "10 - DMA_Stream4"]
    DMA_STREAM4,
    #[doc = "11 - DMA_Stream5"]
    DMA_STREAM5,
    #[doc = "12 - DMA_Stream6"]
    DMA_STREAM6,
    #[doc = "13 - DMA_Stream7"]
    DMA_STREAM7,
    #[doc = "14 - DMA_Stream8"]
    DMA_STREAM8,
    #[doc = "15 - DMA_Stream9"]
    DMA_STREAM9,
    #[doc = "16 - DMA_Stream10"]
    DMA_STREAM10,
    #[doc = "17 - DMA_Stream11"]
    DMA_STREAM11,
    #[doc = "18 - DMA_Stream12"]
    DMA_STREAM12,
    #[doc = "19 - DMA_Stream13"]
    DMA_STREAM13,
    #[doc = "20 - DMA_Stream14"]
    DMA_STREAM14,
    #[doc = "21 - DMA_Stream15"]
    DMA_STREAM15,
    #[doc = "22 - DMA_Stream16"]
    DMA_STREAM16,
    #[doc = "23 - DMA_Stream17"]
    DMA_STREAM17,
    #[doc = "24 - DMA_Stream18"]
    DMA_STREAM18,
    #[doc = "25 - DMA_Stream19"]
    DMA_STREAM19,
    #[doc = "26 - DMA_Stream20"]
    DMA_STREAM20,
    #[doc = "27 - DMA_Stream21"]
    DMA_STREAM21,
    #[doc = "28 - DMA_Stream22"]
    DMA_STREAM22,
    #[doc = "29 - DMA_Stream23"]
    DMA_STREAM23,
    #[doc = "30 - UART0_MS"]
    UART0_MS,
    #[doc = "31 - UART0_RX"]
    UART0_RX,
    #[doc = "32 - UART0_TX"]
    UART0_TX,
    #[doc = "33 - UART0_RT"]
    UART0_RT,
    #[doc = "34 - UART0_E"]
    UART0_E,
    #[doc = "35 - UART0"]
    UART0,
    #[doc = "36 - UART1_MS"]
    UART1_MS,
    #[doc = "37 - UART1_RX"]
    UART1_RX,
    #[doc = "38 - UART1_TX"]
    UART1_TX,
    #[doc = "39 - UART1_RT"]
    UART1_RT,
    #[doc = "40 - UART1_E"]
    UART1_E,
    #[doc = "41 - UART1"]
    UART1,
    #[doc = "42 - UART2_MS"]
    UART2_MS,
    #[doc = "43 - UART2_RX"]
    UART2_RX,
    #[doc = "44 - UART2_TX"]
    UART2_TX,
    #[doc = "45 - UART2_RT"]
    UART2_RT,
    #[doc = "46 - UART2_E"]
    UART2_E,
    #[doc = "47 - UART2"]
    UART2,
    #[doc = "48 - UART3_MS"]
    UART3_MS,
    #[doc = "49 - UART3_RX"]
    UART3_RX,
    #[doc = "50 - UART3_TX"]
    UART3_TX,
    #[doc = "51 - UART3_RT"]
    UART3_RT,
    #[doc = "52 - UART3_E"]
    UART3_E,
    #[doc = "53 - UART3"]
    UART3,
    #[doc = "54 - PWM0"]
    PWM0,
    #[doc = "55 - PWM0_HD"]
    PWM0_HD,
    #[doc = "56 - PWM0_TZ"]
    PWM0_TZ,
    #[doc = "57 - PWM1"]
    PWM1,
    #[doc = "58 - PWM1_HD"]
    PWM1_HD,
    #[doc = "59 - PWM1_TZ"]
    PWM1_TZ,
    #[doc = "60 - PWM2"]
    PWM2,
    #[doc = "61 - PWM2_HD"]
    PWM2_HD,
    #[doc = "62 - PWM2_TZ"]
    PWM2_TZ,
    #[doc = "63 - PWM3"]
    PWM3,
    #[doc = "64 - PWM3_HD"]
    PWM3_HD,
    #[doc = "65 - PWM3_TZ"]
    PWM3_TZ,
    #[doc = "66 - PWM4"]
    PWM4,
    #[doc = "67 - PWM4_HD"]
    PWM4_HD,
    #[doc = "68 - PWM4_TZ"]
    PWM4_TZ,
    #[doc = "69 - PWM5"]
    PWM5,
    #[doc = "70 - PWM5_HD"]
    PWM5_HD,
    #[doc = "71 - PWM5_TZ"]
    PWM5_TZ,
    #[doc = "72 - PWM6"]
    PWM6,
    #[doc = "73 - PWM6_HD"]
    PWM6_HD,
    #[doc = "74 - PWM6_TZ"]
    PWM6_TZ,
    #[doc = "75 - PWM7"]
    PWM7,
    #[doc = "76 - PWM7_HD"]
    PWM7_HD,
    #[doc = "77 - PWM7_TZ"]
    PWM7_TZ,
    #[doc = "78 - PWM8"]
    PWM8,
    #[doc = "79 - PWM8_HD"]
    PWM8_HD,
    #[doc = "80 - PWM8_TZ"]
    PWM8_TZ,
    #[doc = "81 - ADC_SEQ0"]
    ADC_SEQ0,
    #[doc = "82 - ADC_SEQ1"]
    ADC_SEQ1,
    #[doc = "83 - ADC_SEQ2"]
    ADC_SEQ2,
    #[doc = "84 - ADC_SEQ3"]
    ADC_SEQ3,
    #[doc = "85 - ADC_SEQ4"]
    ADC_SEQ4,
    #[doc = "86 - ADC_SEQ5"]
    ADC_SEQ5,
    #[doc = "87 - ADC_SEQ6"]
    ADC_SEQ6,
    #[doc = "88 - ADC_SEQ7"]
    ADC_SEQ7,
    #[doc = "89 - ADC_CompInt"]
    ADC_COMPINT,
    #[doc = "90 - CAP0"]
    CAP0,
    #[doc = "91 - CAP1"]
    CAP1,
    #[doc = "92 - CAP2"]
    CAP2,
    #[doc = "93 - CAP3"]
    CAP3,
    #[doc = "94 - CAP4"]
    CAP4,
    #[doc = "95 - CAP5"]
    CAP5,
    #[doc = "96 - QEP0"]
    QEP0,
    #[doc = "97 - QEP1"]
    QEP1,
    #[doc = "98 - BootFlash"]
    BOOTFLASH,
    #[doc = "99 - CMP0"]
    CMP0,
    #[doc = "100 - CMP1"]
    CMP1,
    #[doc = "101 - CMP2"]
    CMP2,
    #[doc = "102 - SPI0"]
    SPI0,
    #[doc = "103 - SPI1"]
    SPI1,
    #[doc = "104 - SPI2"]
    SPI2,
    #[doc = "105 - SPI3"]
    SPI3,
    #[doc = "106 - UserFlash"]
    USERFLASH,
    #[doc = "107 - GPIOA"]
    GPIOA,
    #[doc = "108 - GPIOB"]
    GPIOB,
    #[doc = "109 - GPIOC"]
    GPIOC,
    #[doc = "110 - GPIOD"]
    GPIOD,
    #[doc = "111 - GPIOE"]
    GPIOE,
    #[doc = "112 - GPIOF"]
    GPIOF,
    #[doc = "113 - GPIOG"]
    GPIOG,
    #[doc = "114 - GPIOH"]
    GPIOH,
    #[doc = "115 - Ethernet"]
    ETHERNET,
    #[doc = "116 - CAN0"]
    CAN0,
    #[doc = "117 - CAN1"]
    CAN1,
    #[doc = "118 - CAN2"]
    CAN2,
    #[doc = "119 - CAN3"]
    CAN3,
    #[doc = "120 - CAN4"]
    CAN4,
    #[doc = "121 - CAN5"]
    CAN5,
    #[doc = "122 - CAN6"]
    CAN6,
    #[doc = "123 - CAN7"]
    CAN7,
    #[doc = "124 - CAN8"]
    CAN8,
    #[doc = "125 - CAN9"]
    CAN9,
    #[doc = "126 - CAN10"]
    CAN10,
    #[doc = "127 - CAN11"]
    CAN11,
    #[doc = "128 - CAN12"]
    CAN12,
    #[doc = "129 - CAN13"]
    CAN13,
    #[doc = "130 - CAN14"]
    CAN14,
    #[doc = "131 - CAN15"]
    CAN15,
    #[doc = "132 - RTC"]
    RTC,
    #[doc = "133 - USB"]
    USB,
}
unsafe impl bare_metal::Nr for Interrupt {
    #[inline]
    fn nr(&self) -> u8 {
        match *self {
            Interrupt::WWDG => 0,
            Interrupt::I2C0 => 1,
            Interrupt::I2C1 => 2,
            Interrupt::TIM0 => 3,
            Interrupt::TIM1 => 4,
            Interrupt::TIM2 => 5,
            Interrupt::DMA_STREAM0 => 6,
            Interrupt::DMA_STREAM1 => 7,
            Interrupt::DMA_STREAM2 => 8,
            Interrupt::DMA_STREAM3 => 9,
            Interrupt::DMA_STREAM4 => 10,
            Interrupt::DMA_STREAM5 => 11,
            Interrupt::DMA_STREAM6 => 12,
            Interrupt::DMA_STREAM7 => 13,
            Interrupt::DMA_STREAM8 => 14,
            Interrupt::DMA_STREAM9 => 15,
            Interrupt::DMA_STREAM10 => 16,
            Interrupt::DMA_STREAM11 => 17,
            Interrupt::DMA_STREAM12 => 18,
            Interrupt::DMA_STREAM13 => 19,
            Interrupt::DMA_STREAM14 => 20,
            Interrupt::DMA_STREAM15 => 21,
            Interrupt::DMA_STREAM16 => 22,
            Interrupt::DMA_STREAM17 => 23,
            Interrupt::DMA_STREAM18 => 24,
            Interrupt::DMA_STREAM19 => 25,
            Interrupt::DMA_STREAM20 => 26,
            Interrupt::DMA_STREAM21 => 27,
            Interrupt::DMA_STREAM22 => 28,
            Interrupt::DMA_STREAM23 => 29,
            Interrupt::UART0_MS => 30,
            Interrupt::UART0_RX => 31,
            Interrupt::UART0_TX => 32,
            Interrupt::UART0_RT => 33,
            Interrupt::UART0_E => 34,
            Interrupt::UART0 => 35,
            Interrupt::UART1_MS => 36,
            Interrupt::UART1_RX => 37,
            Interrupt::UART1_TX => 38,
            Interrupt::UART1_RT => 39,
            Interrupt::UART1_E => 40,
            Interrupt::UART1 => 41,
            Interrupt::UART2_MS => 42,
            Interrupt::UART2_RX => 43,
            Interrupt::UART2_TX => 44,
            Interrupt::UART2_RT => 45,
            Interrupt::UART2_E => 46,
            Interrupt::UART2 => 47,
            Interrupt::UART3_MS => 48,
            Interrupt::UART3_RX => 49,
            Interrupt::UART3_TX => 50,
            Interrupt::UART3_RT => 51,
            Interrupt::UART3_E => 52,
            Interrupt::UART3 => 53,
            Interrupt::PWM0 => 54,
            Interrupt::PWM0_HD => 55,
            Interrupt::PWM0_TZ => 56,
            Interrupt::PWM1 => 57,
            Interrupt::PWM1_HD => 58,
            Interrupt::PWM1_TZ => 59,
            Interrupt::PWM2 => 60,
            Interrupt::PWM2_HD => 61,
            Interrupt::PWM2_TZ => 62,
            Interrupt::PWM3 => 63,
            Interrupt::PWM3_HD => 64,
            Interrupt::PWM3_TZ => 65,
            Interrupt::PWM4 => 66,
            Interrupt::PWM4_HD => 67,
            Interrupt::PWM4_TZ => 68,
            Interrupt::PWM5 => 69,
            Interrupt::PWM5_HD => 70,
            Interrupt::PWM5_TZ => 71,
            Interrupt::PWM6 => 72,
            Interrupt::PWM6_HD => 73,
            Interrupt::PWM6_TZ => 74,
            Interrupt::PWM7 => 75,
            Interrupt::PWM7_HD => 76,
            Interrupt::PWM7_TZ => 77,
            Interrupt::PWM8 => 78,
            Interrupt::PWM8_HD => 79,
            Interrupt::PWM8_TZ => 80,
            Interrupt::ADC_SEQ0 => 81,
            Interrupt::ADC_SEQ1 => 82,
            Interrupt::ADC_SEQ2 => 83,
            Interrupt::ADC_SEQ3 => 84,
            Interrupt::ADC_SEQ4 => 85,
            Interrupt::ADC_SEQ5 => 86,
            Interrupt::ADC_SEQ6 => 87,
            Interrupt::ADC_SEQ7 => 88,
            Interrupt::ADC_COMPINT => 89,
            Interrupt::CAP0 => 90,
            Interrupt::CAP1 => 91,
            Interrupt::CAP2 => 92,
            Interrupt::CAP3 => 93,
            Interrupt::CAP4 => 94,
            Interrupt::CAP5 => 95,
            Interrupt::QEP0 => 96,
            Interrupt::QEP1 => 97,
            Interrupt::BOOTFLASH => 98,
            Interrupt::CMP0 => 99,
            Interrupt::CMP1 => 100,
            Interrupt::CMP2 => 101,
            Interrupt::SPI0 => 102,
            Interrupt::SPI1 => 103,
            Interrupt::SPI2 => 104,
            Interrupt::SPI3 => 105,
            Interrupt::USERFLASH => 106,
            Interrupt::GPIOA => 107,
            Interrupt::GPIOB => 108,
            Interrupt::GPIOC => 109,
            Interrupt::GPIOD => 110,
            Interrupt::GPIOE => 111,
            Interrupt::GPIOF => 112,
            Interrupt::GPIOG => 113,
            Interrupt::GPIOH => 114,
            Interrupt::ETHERNET => 115,
            Interrupt::CAN0 => 116,
            Interrupt::CAN1 => 117,
            Interrupt::CAN2 => 118,
            Interrupt::CAN3 => 119,
            Interrupt::CAN4 => 120,
            Interrupt::CAN5 => 121,
            Interrupt::CAN6 => 122,
            Interrupt::CAN7 => 123,
            Interrupt::CAN8 => 124,
            Interrupt::CAN9 => 125,
            Interrupt::CAN10 => 126,
            Interrupt::CAN11 => 127,
            Interrupt::CAN12 => 128,
            Interrupt::CAN13 => 129,
            Interrupt::CAN14 => 130,
            Interrupt::CAN15 => 131,
            Interrupt::RTC => 132,
            Interrupt::USB => 133,
        }
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
