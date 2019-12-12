#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_gpioden0: [u8; 4usize],
    _reserved_1_gpioden1: [u8; 4usize],
    _reserved_2_gpioden2: [u8; 4usize],
    _reserved_3_gpioden3: [u8; 4usize],
    _reserved4: [u8; 8usize],
    #[doc = "0x18 - Port A alternative function selection register"]
    pub gpiopctla: GPIOPCTLA,
    #[doc = "0x1c - Port B alternative function selection register"]
    pub gpiopctlb: GPIOPCTLB,
    #[doc = "0x20 - Port C alternative function selection register"]
    pub gpiopctlc: GPIOPCTLC,
    #[doc = "0x24 - Port D alternative function selection register"]
    pub gpiopctld: GPIOPCTLD,
    #[doc = "0x28 - Port E alternative function selection register"]
    pub gpiopctle: GPIOPCTLE,
    #[doc = "0x2c - Port F alternative function selection register"]
    pub gpiopctlf: GPIOPCTLF,
    #[doc = "0x30 - Port G alternative function selection register"]
    pub gpiopctlg: GPIOPCTLG,
    #[doc = "0x34 - Port H alternative function selection register"]
    pub gpiopctlh: GPIOPCTLH,
    _reserved12: [u8; 16usize],
    _reserved_12_gpioodctl0: [u8; 4usize],
    _reserved_13_gpioodctl1: [u8; 4usize],
    _reserved_14_gpioodctl2: [u8; 4usize],
    _reserved_15_gpioodctl3: [u8; 4usize],
    _reserved16: [u8; 8usize],
    _reserved_16_gpiodsctl0: [u8; 4usize],
    _reserved_17_gpiodsctl1: [u8; 4usize],
    _reserved_18_gpiodsctl2: [u8; 4usize],
    _reserved_19_gpiodsctl3: [u8; 4usize],
    _reserved20: [u8; 8usize],
    _reserved_20_gpiopuctl0: [u8; 4usize],
    _reserved_21_gpiopuctl1: [u8; 4usize],
    _reserved_22_gpiopuctl2: [u8; 4usize],
    _reserved_23_gpiopuctl3: [u8; 4usize],
    _reserved24: [u8; 12usize],
    #[doc = "0x94 - PLL control register"]
    pub pll_ctrl: PLL_CTRL,
    #[doc = "0x98 - PLL output divider register"]
    pub pll_od: PLL_OD,
    #[doc = "0x9c - PLL reference divider register"]
    pub pll_nr: PLL_NR,
    #[doc = "0xa0 - PLL feedback divider register"]
    pub pll_nf: PLL_NF,
    #[doc = "0xa4 - External memory configuration register"]
    pub ext_mem_cfg: EXT_MEM_CFG,
    #[doc = "0xa8 - ADC 0-3 clock control register"]
    pub adc_ctrl0: ADC_CTRL0,
    #[doc = "0xac - ADC 4-7 clock control register"]
    pub adc_ctrl1: ADC_CTRL1,
    #[doc = "0xb0 - PWM prescalers sync register"]
    pub pwm_sync: PWM_SYNC,
    #[doc = "0xb4 - PWM sync control register"]
    pub pwm_ctrl: PWM_CTRL,
    #[doc = "0xb8 - System clock control register"]
    pub sys_clk: SYS_CLK,
    #[doc = "0xbc - Peripheral clock control register"]
    pub apb_clk: APB_CLK,
    #[doc = "0xc0 - UART clock control register"]
    pub uart_clk: UART_CLK,
    #[doc = "0xc4 - SPI clock control register"]
    pub spi_clk: SPI_CLK,
    #[doc = "0xc8 - Peripheral reset register 0"]
    pub per_rst0: PER_RST0,
    #[doc = "0xcc - Peripheral reset register 1"]
    pub per_rst1: PER_RST1,
    #[doc = "0xd0 - Control register resynchronization input ports A, B"]
    pub gpiose0: GPIOSE0,
    #[doc = "0xd4 - Control register resynchronization input ports C, D"]
    pub gpiose1: GPIOSE1,
    #[doc = "0xd8 - Control register resynchronization input ports E, F"]
    pub gpiose2: GPIOSE2,
    #[doc = "0xdc - Control register resynchronization input ports G, H"]
    pub gpiose3: GPIOSE3,
    _reserved43: [u8; 16usize],
    #[doc = "0xf0 - Register filter settings input ports A, B"]
    pub gpioqe0: GPIOQE0,
    #[doc = "0xf4 - Register filter settings input ports C, D"]
    pub gpioqe1: GPIOQE1,
    #[doc = "0xf8 - Register filter settings input ports E, F"]
    pub gpioqe2: GPIOQE2,
    #[doc = "0xfc - Register filter settings input ports G, H"]
    pub gpioqe3: GPIOQE3,
    #[doc = "0x100 - Register filter settings input ports A, B"]
    pub gpioqm0: GPIOQM0,
    #[doc = "0x104 - Register filter settings input ports C, D"]
    pub gpioqm1: GPIOQM1,
    #[doc = "0x108 - Register filter settings input ports E, F"]
    pub gpioqm2: GPIOQM2,
    #[doc = "0x10c - Register filter settings input ports G, H"]
    pub gpioqm3: GPIOQM3,
    #[doc = "0x110 - Register filter settings input ports A, B, C, D"]
    pub gpioqpad: GPIOQPAD,
    #[doc = "0x114 - Register filter settings input ports E, F, G, H"]
    pub gpioqpeh: GPIOQPEH,
    _reserved53: [u8; 24usize],
    #[doc = "0x130 - Customize the USB PHY"]
    pub usb_ctrl: USB_CTRL,
    _reserved54: [u8; 4usize],
    #[doc = "0x138 - Select source clk UART and SSP register"]
    pub uart_spi_clk_sel: UART_SPI_CLK_SEL,
    #[doc = "0x13c - ADC control register 2"]
    pub adc_ctrl2: ADC_CTRL2,
    _reserved56: [u8; 36usize],
    #[doc = "0x164 - Full erase flash (user and boot) register"]
    pub flash_full_erase: FLASH_FULL_ERASE,
}
impl RegisterBlock {
    #[doc = "0x00 - Port A digital enable register"]
    #[inline(always)]
    pub fn gpiodena(&self) -> &GPIODENA {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const GPIODENA) }
    }
    #[doc = "0x00 - Port A digital enable register"]
    #[inline(always)]
    pub fn gpiodena_mut(&self) -> &mut GPIODENA {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut GPIODENA) }
    }
    #[doc = "0x00 - Port A,B digital enable register"]
    #[inline(always)]
    pub fn gpioden0(&self) -> &GPIODEN0 {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const GPIODEN0) }
    }
    #[doc = "0x00 - Port A,B digital enable register"]
    #[inline(always)]
    pub fn gpioden0_mut(&self) -> &mut GPIODEN0 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut GPIODEN0) }
    }
    #[doc = "0x02 - Port B digital enable register"]
    #[inline(always)]
    pub fn gpiodenb(&self) -> &GPIODENB {
        unsafe { &*(((self as *const Self) as *const u8).add(2usize) as *const GPIODENB) }
    }
    #[doc = "0x02 - Port B digital enable register"]
    #[inline(always)]
    pub fn gpiodenb_mut(&self) -> &mut GPIODENB {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2usize) as *mut GPIODENB) }
    }
    #[doc = "0x04 - Port C digital enable register"]
    #[inline(always)]
    pub fn gpiodenc(&self) -> &GPIODENC {
        unsafe { &*(((self as *const Self) as *const u8).add(4usize) as *const GPIODENC) }
    }
    #[doc = "0x04 - Port C digital enable register"]
    #[inline(always)]
    pub fn gpiodenc_mut(&self) -> &mut GPIODENC {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4usize) as *mut GPIODENC) }
    }
    #[doc = "0x04 - Port C,D digital enable register"]
    #[inline(always)]
    pub fn gpioden1(&self) -> &GPIODEN1 {
        unsafe { &*(((self as *const Self) as *const u8).add(4usize) as *const GPIODEN1) }
    }
    #[doc = "0x04 - Port C,D digital enable register"]
    #[inline(always)]
    pub fn gpioden1_mut(&self) -> &mut GPIODEN1 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4usize) as *mut GPIODEN1) }
    }
    #[doc = "0x06 - Port D digital enable register"]
    #[inline(always)]
    pub fn gpiodend(&self) -> &GPIODEND {
        unsafe { &*(((self as *const Self) as *const u8).add(6usize) as *const GPIODEND) }
    }
    #[doc = "0x06 - Port D digital enable register"]
    #[inline(always)]
    pub fn gpiodend_mut(&self) -> &mut GPIODEND {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(6usize) as *mut GPIODEND) }
    }
    #[doc = "0x08 - Port E digital enable register"]
    #[inline(always)]
    pub fn gpiodene(&self) -> &GPIODENE {
        unsafe { &*(((self as *const Self) as *const u8).add(8usize) as *const GPIODENE) }
    }
    #[doc = "0x08 - Port E digital enable register"]
    #[inline(always)]
    pub fn gpiodene_mut(&self) -> &mut GPIODENE {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(8usize) as *mut GPIODENE) }
    }
    #[doc = "0x08 - Port E,F digital enable register"]
    #[inline(always)]
    pub fn gpioden2(&self) -> &GPIODEN2 {
        unsafe { &*(((self as *const Self) as *const u8).add(8usize) as *const GPIODEN2) }
    }
    #[doc = "0x08 - Port E,F digital enable register"]
    #[inline(always)]
    pub fn gpioden2_mut(&self) -> &mut GPIODEN2 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(8usize) as *mut GPIODEN2) }
    }
    #[doc = "0x0a - Port F digital enable register"]
    #[inline(always)]
    pub fn gpiodenf(&self) -> &GPIODENF {
        unsafe { &*(((self as *const Self) as *const u8).add(10usize) as *const GPIODENF) }
    }
    #[doc = "0x0a - Port F digital enable register"]
    #[inline(always)]
    pub fn gpiodenf_mut(&self) -> &mut GPIODENF {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(10usize) as *mut GPIODENF) }
    }
    #[doc = "0x0c - Port G digital enable register"]
    #[inline(always)]
    pub fn gpiodeng(&self) -> &GPIODENG {
        unsafe { &*(((self as *const Self) as *const u8).add(12usize) as *const GPIODENG) }
    }
    #[doc = "0x0c - Port G digital enable register"]
    #[inline(always)]
    pub fn gpiodeng_mut(&self) -> &mut GPIODENG {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(12usize) as *mut GPIODENG) }
    }
    #[doc = "0x0c - Port G,H digital enable register"]
    #[inline(always)]
    pub fn gpioden3(&self) -> &GPIODEN3 {
        unsafe { &*(((self as *const Self) as *const u8).add(12usize) as *const GPIODEN3) }
    }
    #[doc = "0x0c - Port G,H digital enable register"]
    #[inline(always)]
    pub fn gpioden3_mut(&self) -> &mut GPIODEN3 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(12usize) as *mut GPIODEN3) }
    }
    #[doc = "0x0e - Port H digital enable register"]
    #[inline(always)]
    pub fn gpiodenh(&self) -> &GPIODENH {
        unsafe { &*(((self as *const Self) as *const u8).add(14usize) as *const GPIODENH) }
    }
    #[doc = "0x0e - Port H digital enable register"]
    #[inline(always)]
    pub fn gpiodenh_mut(&self) -> &mut GPIODENH {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(14usize) as *mut GPIODENH) }
    }
    #[doc = "0x48 - Port A open-drain enable register"]
    #[inline(always)]
    pub fn gpioodctla(&self) -> &GPIOODCTLA {
        unsafe { &*(((self as *const Self) as *const u8).add(72usize) as *const GPIOODCTLA) }
    }
    #[doc = "0x48 - Port A open-drain enable register"]
    #[inline(always)]
    pub fn gpioodctla_mut(&self) -> &mut GPIOODCTLA {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(72usize) as *mut GPIOODCTLA) }
    }
    #[doc = "0x48 - Port A,B open-drain enable register"]
    #[inline(always)]
    pub fn gpioodctl0(&self) -> &GPIOODCTL0 {
        unsafe { &*(((self as *const Self) as *const u8).add(72usize) as *const GPIOODCTL0) }
    }
    #[doc = "0x48 - Port A,B open-drain enable register"]
    #[inline(always)]
    pub fn gpioodctl0_mut(&self) -> &mut GPIOODCTL0 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(72usize) as *mut GPIOODCTL0) }
    }
    #[doc = "0x4a - Port B open-drain enable register"]
    #[inline(always)]
    pub fn gpioodctlb(&self) -> &GPIOODCTLB {
        unsafe { &*(((self as *const Self) as *const u8).add(74usize) as *const GPIOODCTLB) }
    }
    #[doc = "0x4a - Port B open-drain enable register"]
    #[inline(always)]
    pub fn gpioodctlb_mut(&self) -> &mut GPIOODCTLB {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(74usize) as *mut GPIOODCTLB) }
    }
    #[doc = "0x4c - Port C open-drain enable register"]
    #[inline(always)]
    pub fn gpioodctlc(&self) -> &GPIOODCTLC {
        unsafe { &*(((self as *const Self) as *const u8).add(76usize) as *const GPIOODCTLC) }
    }
    #[doc = "0x4c - Port C open-drain enable register"]
    #[inline(always)]
    pub fn gpioodctlc_mut(&self) -> &mut GPIOODCTLC {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(76usize) as *mut GPIOODCTLC) }
    }
    #[doc = "0x4c - Port C,D open-drain enable register"]
    #[inline(always)]
    pub fn gpioodctl1(&self) -> &GPIOODCTL1 {
        unsafe { &*(((self as *const Self) as *const u8).add(76usize) as *const GPIOODCTL1) }
    }
    #[doc = "0x4c - Port C,D open-drain enable register"]
    #[inline(always)]
    pub fn gpioodctl1_mut(&self) -> &mut GPIOODCTL1 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(76usize) as *mut GPIOODCTL1) }
    }
    #[doc = "0x4e - Port D open-drain enable register"]
    #[inline(always)]
    pub fn gpioodctld(&self) -> &GPIOODCTLD {
        unsafe { &*(((self as *const Self) as *const u8).add(78usize) as *const GPIOODCTLD) }
    }
    #[doc = "0x4e - Port D open-drain enable register"]
    #[inline(always)]
    pub fn gpioodctld_mut(&self) -> &mut GPIOODCTLD {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(78usize) as *mut GPIOODCTLD) }
    }
    #[doc = "0x50 - Port E open-drain enable register"]
    #[inline(always)]
    pub fn gpioodctle(&self) -> &GPIOODCTLE {
        unsafe { &*(((self as *const Self) as *const u8).add(80usize) as *const GPIOODCTLE) }
    }
    #[doc = "0x50 - Port E open-drain enable register"]
    #[inline(always)]
    pub fn gpioodctle_mut(&self) -> &mut GPIOODCTLE {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(80usize) as *mut GPIOODCTLE) }
    }
    #[doc = "0x50 - Port E,F open-drain enable register"]
    #[inline(always)]
    pub fn gpioodctl2(&self) -> &GPIOODCTL2 {
        unsafe { &*(((self as *const Self) as *const u8).add(80usize) as *const GPIOODCTL2) }
    }
    #[doc = "0x50 - Port E,F open-drain enable register"]
    #[inline(always)]
    pub fn gpioodctl2_mut(&self) -> &mut GPIOODCTL2 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(80usize) as *mut GPIOODCTL2) }
    }
    #[doc = "0x52 - Port F open-drain enable register"]
    #[inline(always)]
    pub fn gpioodctlf(&self) -> &GPIOODCTLF {
        unsafe { &*(((self as *const Self) as *const u8).add(82usize) as *const GPIOODCTLF) }
    }
    #[doc = "0x52 - Port F open-drain enable register"]
    #[inline(always)]
    pub fn gpioodctlf_mut(&self) -> &mut GPIOODCTLF {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(82usize) as *mut GPIOODCTLF) }
    }
    #[doc = "0x54 - Port G open-drain enable register"]
    #[inline(always)]
    pub fn gpioodctlg(&self) -> &GPIOODCTLG {
        unsafe { &*(((self as *const Self) as *const u8).add(84usize) as *const GPIOODCTLG) }
    }
    #[doc = "0x54 - Port G open-drain enable register"]
    #[inline(always)]
    pub fn gpioodctlg_mut(&self) -> &mut GPIOODCTLG {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(84usize) as *mut GPIOODCTLG) }
    }
    #[doc = "0x54 - Port G,H open-drain enable register"]
    #[inline(always)]
    pub fn gpioodctl3(&self) -> &GPIOODCTL3 {
        unsafe { &*(((self as *const Self) as *const u8).add(84usize) as *const GPIOODCTL3) }
    }
    #[doc = "0x54 - Port G,H open-drain enable register"]
    #[inline(always)]
    pub fn gpioodctl3_mut(&self) -> &mut GPIOODCTL3 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(84usize) as *mut GPIOODCTL3) }
    }
    #[doc = "0x56 - Port H open-drain enable register"]
    #[inline(always)]
    pub fn gpioodctlh(&self) -> &GPIOODCTLH {
        unsafe { &*(((self as *const Self) as *const u8).add(86usize) as *const GPIOODCTLH) }
    }
    #[doc = "0x56 - Port H open-drain enable register"]
    #[inline(always)]
    pub fn gpioodctlh_mut(&self) -> &mut GPIOODCTLH {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(86usize) as *mut GPIOODCTLH) }
    }
    #[doc = "0x60 - Port A strength control register"]
    #[inline(always)]
    pub fn gpiodsctla(&self) -> &GPIODSCTLA {
        unsafe { &*(((self as *const Self) as *const u8).add(96usize) as *const GPIODSCTLA) }
    }
    #[doc = "0x60 - Port A strength control register"]
    #[inline(always)]
    pub fn gpiodsctla_mut(&self) -> &mut GPIODSCTLA {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(96usize) as *mut GPIODSCTLA) }
    }
    #[doc = "0x60 - Port A,B strength control register"]
    #[inline(always)]
    pub fn gpiodsctl0(&self) -> &GPIODSCTL0 {
        unsafe { &*(((self as *const Self) as *const u8).add(96usize) as *const GPIODSCTL0) }
    }
    #[doc = "0x60 - Port A,B strength control register"]
    #[inline(always)]
    pub fn gpiodsctl0_mut(&self) -> &mut GPIODSCTL0 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(96usize) as *mut GPIODSCTL0) }
    }
    #[doc = "0x62 - Port B strength control register"]
    #[inline(always)]
    pub fn gpiodsctlb(&self) -> &GPIODSCTLB {
        unsafe { &*(((self as *const Self) as *const u8).add(98usize) as *const GPIODSCTLB) }
    }
    #[doc = "0x62 - Port B strength control register"]
    #[inline(always)]
    pub fn gpiodsctlb_mut(&self) -> &mut GPIODSCTLB {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(98usize) as *mut GPIODSCTLB) }
    }
    #[doc = "0x64 - Port C strength control register"]
    #[inline(always)]
    pub fn gpiodsctlc(&self) -> &GPIODSCTLC {
        unsafe { &*(((self as *const Self) as *const u8).add(100usize) as *const GPIODSCTLC) }
    }
    #[doc = "0x64 - Port C strength control register"]
    #[inline(always)]
    pub fn gpiodsctlc_mut(&self) -> &mut GPIODSCTLC {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(100usize) as *mut GPIODSCTLC) }
    }
    #[doc = "0x64 - Port C,D strength control register"]
    #[inline(always)]
    pub fn gpiodsctl1(&self) -> &GPIODSCTL1 {
        unsafe { &*(((self as *const Self) as *const u8).add(100usize) as *const GPIODSCTL1) }
    }
    #[doc = "0x64 - Port C,D strength control register"]
    #[inline(always)]
    pub fn gpiodsctl1_mut(&self) -> &mut GPIODSCTL1 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(100usize) as *mut GPIODSCTL1) }
    }
    #[doc = "0x66 - Port D strength control register"]
    #[inline(always)]
    pub fn gpiodsctld(&self) -> &GPIODSCTLD {
        unsafe { &*(((self as *const Self) as *const u8).add(102usize) as *const GPIODSCTLD) }
    }
    #[doc = "0x66 - Port D strength control register"]
    #[inline(always)]
    pub fn gpiodsctld_mut(&self) -> &mut GPIODSCTLD {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(102usize) as *mut GPIODSCTLD) }
    }
    #[doc = "0x68 - Port E strength control register"]
    #[inline(always)]
    pub fn gpiodsctle(&self) -> &GPIODSCTLE {
        unsafe { &*(((self as *const Self) as *const u8).add(104usize) as *const GPIODSCTLE) }
    }
    #[doc = "0x68 - Port E strength control register"]
    #[inline(always)]
    pub fn gpiodsctle_mut(&self) -> &mut GPIODSCTLE {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(104usize) as *mut GPIODSCTLE) }
    }
    #[doc = "0x68 - Port E,F strength control register"]
    #[inline(always)]
    pub fn gpiodsctl2(&self) -> &GPIODSCTL2 {
        unsafe { &*(((self as *const Self) as *const u8).add(104usize) as *const GPIODSCTL2) }
    }
    #[doc = "0x68 - Port E,F strength control register"]
    #[inline(always)]
    pub fn gpiodsctl2_mut(&self) -> &mut GPIODSCTL2 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(104usize) as *mut GPIODSCTL2) }
    }
    #[doc = "0x6a - Port F strength control register"]
    #[inline(always)]
    pub fn gpiodsctlf(&self) -> &GPIODSCTLF {
        unsafe { &*(((self as *const Self) as *const u8).add(106usize) as *const GPIODSCTLF) }
    }
    #[doc = "0x6a - Port F strength control register"]
    #[inline(always)]
    pub fn gpiodsctlf_mut(&self) -> &mut GPIODSCTLF {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(106usize) as *mut GPIODSCTLF) }
    }
    #[doc = "0x6c - Port G strength control register"]
    #[inline(always)]
    pub fn gpiodsctlg(&self) -> &GPIODSCTLG {
        unsafe { &*(((self as *const Self) as *const u8).add(108usize) as *const GPIODSCTLG) }
    }
    #[doc = "0x6c - Port G strength control register"]
    #[inline(always)]
    pub fn gpiodsctlg_mut(&self) -> &mut GPIODSCTLG {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(108usize) as *mut GPIODSCTLG) }
    }
    #[doc = "0x6c - Port G,H strength control register"]
    #[inline(always)]
    pub fn gpiodsctl3(&self) -> &GPIODSCTL3 {
        unsafe { &*(((self as *const Self) as *const u8).add(108usize) as *const GPIODSCTL3) }
    }
    #[doc = "0x6c - Port G,H strength control register"]
    #[inline(always)]
    pub fn gpiodsctl3_mut(&self) -> &mut GPIODSCTL3 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(108usize) as *mut GPIODSCTL3) }
    }
    #[doc = "0x6e - Port H strength control register"]
    #[inline(always)]
    pub fn gpiodsctlh(&self) -> &GPIODSCTLH {
        unsafe { &*(((self as *const Self) as *const u8).add(110usize) as *const GPIODSCTLH) }
    }
    #[doc = "0x6e - Port H strength control register"]
    #[inline(always)]
    pub fn gpiodsctlh_mut(&self) -> &mut GPIODSCTLH {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(110usize) as *mut GPIODSCTLH) }
    }
    #[doc = "0x78 - Port A pull-up enable register"]
    #[inline(always)]
    pub fn gpiopuctla(&self) -> &GPIOPUCTLA {
        unsafe { &*(((self as *const Self) as *const u8).add(120usize) as *const GPIOPUCTLA) }
    }
    #[doc = "0x78 - Port A pull-up enable register"]
    #[inline(always)]
    pub fn gpiopuctla_mut(&self) -> &mut GPIOPUCTLA {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(120usize) as *mut GPIOPUCTLA) }
    }
    #[doc = "0x78 - Port A,B pull-up enable register"]
    #[inline(always)]
    pub fn gpiopuctl0(&self) -> &GPIOPUCTL0 {
        unsafe { &*(((self as *const Self) as *const u8).add(120usize) as *const GPIOPUCTL0) }
    }
    #[doc = "0x78 - Port A,B pull-up enable register"]
    #[inline(always)]
    pub fn gpiopuctl0_mut(&self) -> &mut GPIOPUCTL0 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(120usize) as *mut GPIOPUCTL0) }
    }
    #[doc = "0x7a - Port B pull-up enable register"]
    #[inline(always)]
    pub fn gpiopuctlb(&self) -> &GPIOPUCTLB {
        unsafe { &*(((self as *const Self) as *const u8).add(122usize) as *const GPIOPUCTLB) }
    }
    #[doc = "0x7a - Port B pull-up enable register"]
    #[inline(always)]
    pub fn gpiopuctlb_mut(&self) -> &mut GPIOPUCTLB {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(122usize) as *mut GPIOPUCTLB) }
    }
    #[doc = "0x7c - Port C pull-up enable register"]
    #[inline(always)]
    pub fn gpiopuctlc(&self) -> &GPIOPUCTLC {
        unsafe { &*(((self as *const Self) as *const u8).add(124usize) as *const GPIOPUCTLC) }
    }
    #[doc = "0x7c - Port C pull-up enable register"]
    #[inline(always)]
    pub fn gpiopuctlc_mut(&self) -> &mut GPIOPUCTLC {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(124usize) as *mut GPIOPUCTLC) }
    }
    #[doc = "0x7c - Port C,D pull-up enable register"]
    #[inline(always)]
    pub fn gpiopuctl1(&self) -> &GPIOPUCTL1 {
        unsafe { &*(((self as *const Self) as *const u8).add(124usize) as *const GPIOPUCTL1) }
    }
    #[doc = "0x7c - Port C,D pull-up enable register"]
    #[inline(always)]
    pub fn gpiopuctl1_mut(&self) -> &mut GPIOPUCTL1 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(124usize) as *mut GPIOPUCTL1) }
    }
    #[doc = "0x7e - Port D pull-up enable register"]
    #[inline(always)]
    pub fn gpiopuctld(&self) -> &GPIOPUCTLD {
        unsafe { &*(((self as *const Self) as *const u8).add(126usize) as *const GPIOPUCTLD) }
    }
    #[doc = "0x7e - Port D pull-up enable register"]
    #[inline(always)]
    pub fn gpiopuctld_mut(&self) -> &mut GPIOPUCTLD {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(126usize) as *mut GPIOPUCTLD) }
    }
    #[doc = "0x80 - Port E pull-up enable register"]
    #[inline(always)]
    pub fn gpiopuctle(&self) -> &GPIOPUCTLE {
        unsafe { &*(((self as *const Self) as *const u8).add(128usize) as *const GPIOPUCTLE) }
    }
    #[doc = "0x80 - Port E pull-up enable register"]
    #[inline(always)]
    pub fn gpiopuctle_mut(&self) -> &mut GPIOPUCTLE {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(128usize) as *mut GPIOPUCTLE) }
    }
    #[doc = "0x80 - Port E,F pull-up enable register"]
    #[inline(always)]
    pub fn gpiopuctl2(&self) -> &GPIOPUCTL2 {
        unsafe { &*(((self as *const Self) as *const u8).add(128usize) as *const GPIOPUCTL2) }
    }
    #[doc = "0x80 - Port E,F pull-up enable register"]
    #[inline(always)]
    pub fn gpiopuctl2_mut(&self) -> &mut GPIOPUCTL2 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(128usize) as *mut GPIOPUCTL2) }
    }
    #[doc = "0x82 - Port F pull-up enable register"]
    #[inline(always)]
    pub fn gpiopuctlf(&self) -> &GPIOPUCTLF {
        unsafe { &*(((self as *const Self) as *const u8).add(130usize) as *const GPIOPUCTLF) }
    }
    #[doc = "0x82 - Port F pull-up enable register"]
    #[inline(always)]
    pub fn gpiopuctlf_mut(&self) -> &mut GPIOPUCTLF {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(130usize) as *mut GPIOPUCTLF) }
    }
    #[doc = "0x84 - Port G pull-up enable register"]
    #[inline(always)]
    pub fn gpiopuctlg(&self) -> &GPIOPUCTLG {
        unsafe { &*(((self as *const Self) as *const u8).add(132usize) as *const GPIOPUCTLG) }
    }
    #[doc = "0x84 - Port G pull-up enable register"]
    #[inline(always)]
    pub fn gpiopuctlg_mut(&self) -> &mut GPIOPUCTLG {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(132usize) as *mut GPIOPUCTLG) }
    }
    #[doc = "0x84 - Port G,H pull-up enable register"]
    #[inline(always)]
    pub fn gpiopuctl3(&self) -> &GPIOPUCTL3 {
        unsafe { &*(((self as *const Self) as *const u8).add(132usize) as *const GPIOPUCTL3) }
    }
    #[doc = "0x84 - Port G,H pull-up enable register"]
    #[inline(always)]
    pub fn gpiopuctl3_mut(&self) -> &mut GPIOPUCTL3 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(132usize) as *mut GPIOPUCTL3) }
    }
    #[doc = "0x86 - Port H pull-up enable register"]
    #[inline(always)]
    pub fn gpiopuctlh(&self) -> &GPIOPUCTLH {
        unsafe { &*(((self as *const Self) as *const u8).add(134usize) as *const GPIOPUCTLH) }
    }
    #[doc = "0x86 - Port H pull-up enable register"]
    #[inline(always)]
    pub fn gpiopuctlh_mut(&self) -> &mut GPIOPUCTLH {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(134usize) as *mut GPIOPUCTLH) }
    }
}
#[doc = "Port A,B digital enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpioden0](gpioden0) module"]
pub type GPIODEN0 = crate::Reg<u32, _GPIODEN0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIODEN0;
#[doc = "`read()` method returns [gpioden0::R](gpioden0::R) reader structure"]
impl crate::Readable for GPIODEN0 {}
#[doc = "`write(|w| ..)` method takes [gpioden0::W](gpioden0::W) writer structure"]
impl crate::Writable for GPIODEN0 {}
#[doc = "Port A,B digital enable register"]
pub mod gpioden0;
#[doc = "Port A digital enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpiodena](gpiodena) module"]
pub type GPIODENA = crate::Reg<u16, _GPIODENA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIODENA;
#[doc = "`read()` method returns [gpiodena::R](gpiodena::R) reader structure"]
impl crate::Readable for GPIODENA {}
#[doc = "`write(|w| ..)` method takes [gpiodena::W](gpiodena::W) writer structure"]
impl crate::Writable for GPIODENA {}
#[doc = "Port A digital enable register"]
pub mod gpiodena;
#[doc = "Port B digital enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpiodenb](gpiodenb) module"]
pub type GPIODENB = crate::Reg<u16, _GPIODENB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIODENB;
#[doc = "`read()` method returns [gpiodenb::R](gpiodenb::R) reader structure"]
impl crate::Readable for GPIODENB {}
#[doc = "`write(|w| ..)` method takes [gpiodenb::W](gpiodenb::W) writer structure"]
impl crate::Writable for GPIODENB {}
#[doc = "Port B digital enable register"]
pub mod gpiodenb;
#[doc = "Port C,D digital enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpioden1](gpioden1) module"]
pub type GPIODEN1 = crate::Reg<u32, _GPIODEN1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIODEN1;
#[doc = "`read()` method returns [gpioden1::R](gpioden1::R) reader structure"]
impl crate::Readable for GPIODEN1 {}
#[doc = "`write(|w| ..)` method takes [gpioden1::W](gpioden1::W) writer structure"]
impl crate::Writable for GPIODEN1 {}
#[doc = "Port C,D digital enable register"]
pub mod gpioden1;
#[doc = "Port C digital enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpiodenc](gpiodenc) module"]
pub type GPIODENC = crate::Reg<u16, _GPIODENC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIODENC;
#[doc = "`read()` method returns [gpiodenc::R](gpiodenc::R) reader structure"]
impl crate::Readable for GPIODENC {}
#[doc = "`write(|w| ..)` method takes [gpiodenc::W](gpiodenc::W) writer structure"]
impl crate::Writable for GPIODENC {}
#[doc = "Port C digital enable register"]
pub mod gpiodenc;
#[doc = "Port D digital enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpiodend](gpiodend) module"]
pub type GPIODEND = crate::Reg<u16, _GPIODEND>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIODEND;
#[doc = "`read()` method returns [gpiodend::R](gpiodend::R) reader structure"]
impl crate::Readable for GPIODEND {}
#[doc = "`write(|w| ..)` method takes [gpiodend::W](gpiodend::W) writer structure"]
impl crate::Writable for GPIODEND {}
#[doc = "Port D digital enable register"]
pub mod gpiodend;
#[doc = "Port E,F digital enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpioden2](gpioden2) module"]
pub type GPIODEN2 = crate::Reg<u32, _GPIODEN2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIODEN2;
#[doc = "`read()` method returns [gpioden2::R](gpioden2::R) reader structure"]
impl crate::Readable for GPIODEN2 {}
#[doc = "`write(|w| ..)` method takes [gpioden2::W](gpioden2::W) writer structure"]
impl crate::Writable for GPIODEN2 {}
#[doc = "Port E,F digital enable register"]
pub mod gpioden2;
#[doc = "Port E digital enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpiodene](gpiodene) module"]
pub type GPIODENE = crate::Reg<u16, _GPIODENE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIODENE;
#[doc = "`read()` method returns [gpiodene::R](gpiodene::R) reader structure"]
impl crate::Readable for GPIODENE {}
#[doc = "`write(|w| ..)` method takes [gpiodene::W](gpiodene::W) writer structure"]
impl crate::Writable for GPIODENE {}
#[doc = "Port E digital enable register"]
pub mod gpiodene;
#[doc = "Port F digital enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpiodenf](gpiodenf) module"]
pub type GPIODENF = crate::Reg<u16, _GPIODENF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIODENF;
#[doc = "`read()` method returns [gpiodenf::R](gpiodenf::R) reader structure"]
impl crate::Readable for GPIODENF {}
#[doc = "`write(|w| ..)` method takes [gpiodenf::W](gpiodenf::W) writer structure"]
impl crate::Writable for GPIODENF {}
#[doc = "Port F digital enable register"]
pub mod gpiodenf;
#[doc = "Port G,H digital enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpioden3](gpioden3) module"]
pub type GPIODEN3 = crate::Reg<u32, _GPIODEN3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIODEN3;
#[doc = "`read()` method returns [gpioden3::R](gpioden3::R) reader structure"]
impl crate::Readable for GPIODEN3 {}
#[doc = "`write(|w| ..)` method takes [gpioden3::W](gpioden3::W) writer structure"]
impl crate::Writable for GPIODEN3 {}
#[doc = "Port G,H digital enable register"]
pub mod gpioden3;
#[doc = "Port G digital enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpiodeng](gpiodeng) module"]
pub type GPIODENG = crate::Reg<u16, _GPIODENG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIODENG;
#[doc = "`read()` method returns [gpiodeng::R](gpiodeng::R) reader structure"]
impl crate::Readable for GPIODENG {}
#[doc = "`write(|w| ..)` method takes [gpiodeng::W](gpiodeng::W) writer structure"]
impl crate::Writable for GPIODENG {}
#[doc = "Port G digital enable register"]
pub mod gpiodeng;
#[doc = "Port H digital enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpiodenh](gpiodenh) module"]
pub type GPIODENH = crate::Reg<u16, _GPIODENH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIODENH;
#[doc = "`read()` method returns [gpiodenh::R](gpiodenh::R) reader structure"]
impl crate::Readable for GPIODENH {}
#[doc = "`write(|w| ..)` method takes [gpiodenh::W](gpiodenh::W) writer structure"]
impl crate::Writable for GPIODENH {}
#[doc = "Port H digital enable register"]
pub mod gpiodenh;
#[doc = "Port A alternative function selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpiopctla](gpiopctla) module"]
pub type GPIOPCTLA = crate::Reg<u32, _GPIOPCTLA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOPCTLA;
#[doc = "`read()` method returns [gpiopctla::R](gpiopctla::R) reader structure"]
impl crate::Readable for GPIOPCTLA {}
#[doc = "`write(|w| ..)` method takes [gpiopctla::W](gpiopctla::W) writer structure"]
impl crate::Writable for GPIOPCTLA {}
#[doc = "Port A alternative function selection register"]
pub mod gpiopctla;
#[doc = "Port B alternative function selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpiopctlb](gpiopctlb) module"]
pub type GPIOPCTLB = crate::Reg<u32, _GPIOPCTLB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOPCTLB;
#[doc = "`read()` method returns [gpiopctlb::R](gpiopctlb::R) reader structure"]
impl crate::Readable for GPIOPCTLB {}
#[doc = "`write(|w| ..)` method takes [gpiopctlb::W](gpiopctlb::W) writer structure"]
impl crate::Writable for GPIOPCTLB {}
#[doc = "Port B alternative function selection register"]
pub mod gpiopctlb;
#[doc = "Port C alternative function selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpiopctlc](gpiopctlc) module"]
pub type GPIOPCTLC = crate::Reg<u32, _GPIOPCTLC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOPCTLC;
#[doc = "`read()` method returns [gpiopctlc::R](gpiopctlc::R) reader structure"]
impl crate::Readable for GPIOPCTLC {}
#[doc = "`write(|w| ..)` method takes [gpiopctlc::W](gpiopctlc::W) writer structure"]
impl crate::Writable for GPIOPCTLC {}
#[doc = "Port C alternative function selection register"]
pub mod gpiopctlc;
#[doc = "Port D alternative function selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpiopctld](gpiopctld) module"]
pub type GPIOPCTLD = crate::Reg<u32, _GPIOPCTLD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOPCTLD;
#[doc = "`read()` method returns [gpiopctld::R](gpiopctld::R) reader structure"]
impl crate::Readable for GPIOPCTLD {}
#[doc = "`write(|w| ..)` method takes [gpiopctld::W](gpiopctld::W) writer structure"]
impl crate::Writable for GPIOPCTLD {}
#[doc = "Port D alternative function selection register"]
pub mod gpiopctld;
#[doc = "Port E alternative function selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpiopctle](gpiopctle) module"]
pub type GPIOPCTLE = crate::Reg<u32, _GPIOPCTLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOPCTLE;
#[doc = "`read()` method returns [gpiopctle::R](gpiopctle::R) reader structure"]
impl crate::Readable for GPIOPCTLE {}
#[doc = "`write(|w| ..)` method takes [gpiopctle::W](gpiopctle::W) writer structure"]
impl crate::Writable for GPIOPCTLE {}
#[doc = "Port E alternative function selection register"]
pub mod gpiopctle;
#[doc = "Port F alternative function selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpiopctlf](gpiopctlf) module"]
pub type GPIOPCTLF = crate::Reg<u32, _GPIOPCTLF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOPCTLF;
#[doc = "`read()` method returns [gpiopctlf::R](gpiopctlf::R) reader structure"]
impl crate::Readable for GPIOPCTLF {}
#[doc = "`write(|w| ..)` method takes [gpiopctlf::W](gpiopctlf::W) writer structure"]
impl crate::Writable for GPIOPCTLF {}
#[doc = "Port F alternative function selection register"]
pub mod gpiopctlf;
#[doc = "Port G alternative function selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpiopctlg](gpiopctlg) module"]
pub type GPIOPCTLG = crate::Reg<u32, _GPIOPCTLG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOPCTLG;
#[doc = "`read()` method returns [gpiopctlg::R](gpiopctlg::R) reader structure"]
impl crate::Readable for GPIOPCTLG {}
#[doc = "`write(|w| ..)` method takes [gpiopctlg::W](gpiopctlg::W) writer structure"]
impl crate::Writable for GPIOPCTLG {}
#[doc = "Port G alternative function selection register"]
pub mod gpiopctlg;
#[doc = "Port H alternative function selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpiopctlh](gpiopctlh) module"]
pub type GPIOPCTLH = crate::Reg<u32, _GPIOPCTLH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOPCTLH;
#[doc = "`read()` method returns [gpiopctlh::R](gpiopctlh::R) reader structure"]
impl crate::Readable for GPIOPCTLH {}
#[doc = "`write(|w| ..)` method takes [gpiopctlh::W](gpiopctlh::W) writer structure"]
impl crate::Writable for GPIOPCTLH {}
#[doc = "Port H alternative function selection register"]
pub mod gpiopctlh;
#[doc = "Port A,B open-drain enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpioodctl0](gpioodctl0) module"]
pub type GPIOODCTL0 = crate::Reg<u32, _GPIOODCTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOODCTL0;
#[doc = "`read()` method returns [gpioodctl0::R](gpioodctl0::R) reader structure"]
impl crate::Readable for GPIOODCTL0 {}
#[doc = "`write(|w| ..)` method takes [gpioodctl0::W](gpioodctl0::W) writer structure"]
impl crate::Writable for GPIOODCTL0 {}
#[doc = "Port A,B open-drain enable register"]
pub mod gpioodctl0;
#[doc = "Port A open-drain enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpioodctla](gpioodctla) module"]
pub type GPIOODCTLA = crate::Reg<u16, _GPIOODCTLA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOODCTLA;
#[doc = "`read()` method returns [gpioodctla::R](gpioodctla::R) reader structure"]
impl crate::Readable for GPIOODCTLA {}
#[doc = "`write(|w| ..)` method takes [gpioodctla::W](gpioodctla::W) writer structure"]
impl crate::Writable for GPIOODCTLA {}
#[doc = "Port A open-drain enable register"]
pub mod gpioodctla;
#[doc = "Port B open-drain enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpioodctlb](gpioodctlb) module"]
pub type GPIOODCTLB = crate::Reg<u16, _GPIOODCTLB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOODCTLB;
#[doc = "`read()` method returns [gpioodctlb::R](gpioodctlb::R) reader structure"]
impl crate::Readable for GPIOODCTLB {}
#[doc = "`write(|w| ..)` method takes [gpioodctlb::W](gpioodctlb::W) writer structure"]
impl crate::Writable for GPIOODCTLB {}
#[doc = "Port B open-drain enable register"]
pub mod gpioodctlb;
#[doc = "Port C,D open-drain enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpioodctl1](gpioodctl1) module"]
pub type GPIOODCTL1 = crate::Reg<u32, _GPIOODCTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOODCTL1;
#[doc = "`read()` method returns [gpioodctl1::R](gpioodctl1::R) reader structure"]
impl crate::Readable for GPIOODCTL1 {}
#[doc = "`write(|w| ..)` method takes [gpioodctl1::W](gpioodctl1::W) writer structure"]
impl crate::Writable for GPIOODCTL1 {}
#[doc = "Port C,D open-drain enable register"]
pub mod gpioodctl1;
#[doc = "Port C open-drain enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpioodctlc](gpioodctlc) module"]
pub type GPIOODCTLC = crate::Reg<u16, _GPIOODCTLC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOODCTLC;
#[doc = "`read()` method returns [gpioodctlc::R](gpioodctlc::R) reader structure"]
impl crate::Readable for GPIOODCTLC {}
#[doc = "`write(|w| ..)` method takes [gpioodctlc::W](gpioodctlc::W) writer structure"]
impl crate::Writable for GPIOODCTLC {}
#[doc = "Port C open-drain enable register"]
pub mod gpioodctlc;
#[doc = "Port D open-drain enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpioodctld](gpioodctld) module"]
pub type GPIOODCTLD = crate::Reg<u16, _GPIOODCTLD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOODCTLD;
#[doc = "`read()` method returns [gpioodctld::R](gpioodctld::R) reader structure"]
impl crate::Readable for GPIOODCTLD {}
#[doc = "`write(|w| ..)` method takes [gpioodctld::W](gpioodctld::W) writer structure"]
impl crate::Writable for GPIOODCTLD {}
#[doc = "Port D open-drain enable register"]
pub mod gpioodctld;
#[doc = "Port E,F open-drain enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpioodctl2](gpioodctl2) module"]
pub type GPIOODCTL2 = crate::Reg<u32, _GPIOODCTL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOODCTL2;
#[doc = "`read()` method returns [gpioodctl2::R](gpioodctl2::R) reader structure"]
impl crate::Readable for GPIOODCTL2 {}
#[doc = "`write(|w| ..)` method takes [gpioodctl2::W](gpioodctl2::W) writer structure"]
impl crate::Writable for GPIOODCTL2 {}
#[doc = "Port E,F open-drain enable register"]
pub mod gpioodctl2;
#[doc = "Port E open-drain enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpioodctle](gpioodctle) module"]
pub type GPIOODCTLE = crate::Reg<u16, _GPIOODCTLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOODCTLE;
#[doc = "`read()` method returns [gpioodctle::R](gpioodctle::R) reader structure"]
impl crate::Readable for GPIOODCTLE {}
#[doc = "`write(|w| ..)` method takes [gpioodctle::W](gpioodctle::W) writer structure"]
impl crate::Writable for GPIOODCTLE {}
#[doc = "Port E open-drain enable register"]
pub mod gpioodctle;
#[doc = "Port F open-drain enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpioodctlf](gpioodctlf) module"]
pub type GPIOODCTLF = crate::Reg<u16, _GPIOODCTLF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOODCTLF;
#[doc = "`read()` method returns [gpioodctlf::R](gpioodctlf::R) reader structure"]
impl crate::Readable for GPIOODCTLF {}
#[doc = "`write(|w| ..)` method takes [gpioodctlf::W](gpioodctlf::W) writer structure"]
impl crate::Writable for GPIOODCTLF {}
#[doc = "Port F open-drain enable register"]
pub mod gpioodctlf;
#[doc = "Port G,H open-drain enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpioodctl3](gpioodctl3) module"]
pub type GPIOODCTL3 = crate::Reg<u32, _GPIOODCTL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOODCTL3;
#[doc = "`read()` method returns [gpioodctl3::R](gpioodctl3::R) reader structure"]
impl crate::Readable for GPIOODCTL3 {}
#[doc = "`write(|w| ..)` method takes [gpioodctl3::W](gpioodctl3::W) writer structure"]
impl crate::Writable for GPIOODCTL3 {}
#[doc = "Port G,H open-drain enable register"]
pub mod gpioodctl3;
#[doc = "Port G open-drain enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpioodctlg](gpioodctlg) module"]
pub type GPIOODCTLG = crate::Reg<u16, _GPIOODCTLG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOODCTLG;
#[doc = "`read()` method returns [gpioodctlg::R](gpioodctlg::R) reader structure"]
impl crate::Readable for GPIOODCTLG {}
#[doc = "`write(|w| ..)` method takes [gpioodctlg::W](gpioodctlg::W) writer structure"]
impl crate::Writable for GPIOODCTLG {}
#[doc = "Port G open-drain enable register"]
pub mod gpioodctlg;
#[doc = "Port H open-drain enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpioodctlh](gpioodctlh) module"]
pub type GPIOODCTLH = crate::Reg<u16, _GPIOODCTLH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOODCTLH;
#[doc = "`read()` method returns [gpioodctlh::R](gpioodctlh::R) reader structure"]
impl crate::Readable for GPIOODCTLH {}
#[doc = "`write(|w| ..)` method takes [gpioodctlh::W](gpioodctlh::W) writer structure"]
impl crate::Writable for GPIOODCTLH {}
#[doc = "Port H open-drain enable register"]
pub mod gpioodctlh;
#[doc = "Port A,B strength control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpiodsctl0](gpiodsctl0) module"]
pub type GPIODSCTL0 = crate::Reg<u32, _GPIODSCTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIODSCTL0;
#[doc = "`read()` method returns [gpiodsctl0::R](gpiodsctl0::R) reader structure"]
impl crate::Readable for GPIODSCTL0 {}
#[doc = "`write(|w| ..)` method takes [gpiodsctl0::W](gpiodsctl0::W) writer structure"]
impl crate::Writable for GPIODSCTL0 {}
#[doc = "Port A,B strength control register"]
pub mod gpiodsctl0;
#[doc = "Port A strength control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpiodsctla](gpiodsctla) module"]
pub type GPIODSCTLA = crate::Reg<u16, _GPIODSCTLA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIODSCTLA;
#[doc = "`read()` method returns [gpiodsctla::R](gpiodsctla::R) reader structure"]
impl crate::Readable for GPIODSCTLA {}
#[doc = "`write(|w| ..)` method takes [gpiodsctla::W](gpiodsctla::W) writer structure"]
impl crate::Writable for GPIODSCTLA {}
#[doc = "Port A strength control register"]
pub mod gpiodsctla;
#[doc = "Port B strength control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpiodsctlb](gpiodsctlb) module"]
pub type GPIODSCTLB = crate::Reg<u16, _GPIODSCTLB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIODSCTLB;
#[doc = "`read()` method returns [gpiodsctlb::R](gpiodsctlb::R) reader structure"]
impl crate::Readable for GPIODSCTLB {}
#[doc = "`write(|w| ..)` method takes [gpiodsctlb::W](gpiodsctlb::W) writer structure"]
impl crate::Writable for GPIODSCTLB {}
#[doc = "Port B strength control register"]
pub mod gpiodsctlb;
#[doc = "Port C,D strength control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpiodsctl1](gpiodsctl1) module"]
pub type GPIODSCTL1 = crate::Reg<u32, _GPIODSCTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIODSCTL1;
#[doc = "`read()` method returns [gpiodsctl1::R](gpiodsctl1::R) reader structure"]
impl crate::Readable for GPIODSCTL1 {}
#[doc = "`write(|w| ..)` method takes [gpiodsctl1::W](gpiodsctl1::W) writer structure"]
impl crate::Writable for GPIODSCTL1 {}
#[doc = "Port C,D strength control register"]
pub mod gpiodsctl1;
#[doc = "Port C strength control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpiodsctlc](gpiodsctlc) module"]
pub type GPIODSCTLC = crate::Reg<u16, _GPIODSCTLC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIODSCTLC;
#[doc = "`read()` method returns [gpiodsctlc::R](gpiodsctlc::R) reader structure"]
impl crate::Readable for GPIODSCTLC {}
#[doc = "`write(|w| ..)` method takes [gpiodsctlc::W](gpiodsctlc::W) writer structure"]
impl crate::Writable for GPIODSCTLC {}
#[doc = "Port C strength control register"]
pub mod gpiodsctlc;
#[doc = "Port D strength control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpiodsctld](gpiodsctld) module"]
pub type GPIODSCTLD = crate::Reg<u16, _GPIODSCTLD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIODSCTLD;
#[doc = "`read()` method returns [gpiodsctld::R](gpiodsctld::R) reader structure"]
impl crate::Readable for GPIODSCTLD {}
#[doc = "`write(|w| ..)` method takes [gpiodsctld::W](gpiodsctld::W) writer structure"]
impl crate::Writable for GPIODSCTLD {}
#[doc = "Port D strength control register"]
pub mod gpiodsctld;
#[doc = "Port E,F strength control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpiodsctl2](gpiodsctl2) module"]
pub type GPIODSCTL2 = crate::Reg<u32, _GPIODSCTL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIODSCTL2;
#[doc = "`read()` method returns [gpiodsctl2::R](gpiodsctl2::R) reader structure"]
impl crate::Readable for GPIODSCTL2 {}
#[doc = "`write(|w| ..)` method takes [gpiodsctl2::W](gpiodsctl2::W) writer structure"]
impl crate::Writable for GPIODSCTL2 {}
#[doc = "Port E,F strength control register"]
pub mod gpiodsctl2;
#[doc = "Port E strength control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpiodsctle](gpiodsctle) module"]
pub type GPIODSCTLE = crate::Reg<u16, _GPIODSCTLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIODSCTLE;
#[doc = "`read()` method returns [gpiodsctle::R](gpiodsctle::R) reader structure"]
impl crate::Readable for GPIODSCTLE {}
#[doc = "`write(|w| ..)` method takes [gpiodsctle::W](gpiodsctle::W) writer structure"]
impl crate::Writable for GPIODSCTLE {}
#[doc = "Port E strength control register"]
pub mod gpiodsctle;
#[doc = "Port F strength control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpiodsctlf](gpiodsctlf) module"]
pub type GPIODSCTLF = crate::Reg<u16, _GPIODSCTLF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIODSCTLF;
#[doc = "`read()` method returns [gpiodsctlf::R](gpiodsctlf::R) reader structure"]
impl crate::Readable for GPIODSCTLF {}
#[doc = "`write(|w| ..)` method takes [gpiodsctlf::W](gpiodsctlf::W) writer structure"]
impl crate::Writable for GPIODSCTLF {}
#[doc = "Port F strength control register"]
pub mod gpiodsctlf;
#[doc = "Port G,H strength control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpiodsctl3](gpiodsctl3) module"]
pub type GPIODSCTL3 = crate::Reg<u32, _GPIODSCTL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIODSCTL3;
#[doc = "`read()` method returns [gpiodsctl3::R](gpiodsctl3::R) reader structure"]
impl crate::Readable for GPIODSCTL3 {}
#[doc = "`write(|w| ..)` method takes [gpiodsctl3::W](gpiodsctl3::W) writer structure"]
impl crate::Writable for GPIODSCTL3 {}
#[doc = "Port G,H strength control register"]
pub mod gpiodsctl3;
#[doc = "Port G strength control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpiodsctlg](gpiodsctlg) module"]
pub type GPIODSCTLG = crate::Reg<u16, _GPIODSCTLG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIODSCTLG;
#[doc = "`read()` method returns [gpiodsctlg::R](gpiodsctlg::R) reader structure"]
impl crate::Readable for GPIODSCTLG {}
#[doc = "`write(|w| ..)` method takes [gpiodsctlg::W](gpiodsctlg::W) writer structure"]
impl crate::Writable for GPIODSCTLG {}
#[doc = "Port G strength control register"]
pub mod gpiodsctlg;
#[doc = "Port H strength control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpiodsctlh](gpiodsctlh) module"]
pub type GPIODSCTLH = crate::Reg<u16, _GPIODSCTLH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIODSCTLH;
#[doc = "`read()` method returns [gpiodsctlh::R](gpiodsctlh::R) reader structure"]
impl crate::Readable for GPIODSCTLH {}
#[doc = "`write(|w| ..)` method takes [gpiodsctlh::W](gpiodsctlh::W) writer structure"]
impl crate::Writable for GPIODSCTLH {}
#[doc = "Port H strength control register"]
pub mod gpiodsctlh;
#[doc = "Port A,B pull-up enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpiopuctl0](gpiopuctl0) module"]
pub type GPIOPUCTL0 = crate::Reg<u32, _GPIOPUCTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOPUCTL0;
#[doc = "`read()` method returns [gpiopuctl0::R](gpiopuctl0::R) reader structure"]
impl crate::Readable for GPIOPUCTL0 {}
#[doc = "`write(|w| ..)` method takes [gpiopuctl0::W](gpiopuctl0::W) writer structure"]
impl crate::Writable for GPIOPUCTL0 {}
#[doc = "Port A,B pull-up enable register"]
pub mod gpiopuctl0;
#[doc = "Port A pull-up enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpiopuctla](gpiopuctla) module"]
pub type GPIOPUCTLA = crate::Reg<u16, _GPIOPUCTLA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOPUCTLA;
#[doc = "`read()` method returns [gpiopuctla::R](gpiopuctla::R) reader structure"]
impl crate::Readable for GPIOPUCTLA {}
#[doc = "`write(|w| ..)` method takes [gpiopuctla::W](gpiopuctla::W) writer structure"]
impl crate::Writable for GPIOPUCTLA {}
#[doc = "Port A pull-up enable register"]
pub mod gpiopuctla;
#[doc = "Port B pull-up enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpiopuctlb](gpiopuctlb) module"]
pub type GPIOPUCTLB = crate::Reg<u16, _GPIOPUCTLB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOPUCTLB;
#[doc = "`read()` method returns [gpiopuctlb::R](gpiopuctlb::R) reader structure"]
impl crate::Readable for GPIOPUCTLB {}
#[doc = "`write(|w| ..)` method takes [gpiopuctlb::W](gpiopuctlb::W) writer structure"]
impl crate::Writable for GPIOPUCTLB {}
#[doc = "Port B pull-up enable register"]
pub mod gpiopuctlb;
#[doc = "Port C,D pull-up enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpiopuctl1](gpiopuctl1) module"]
pub type GPIOPUCTL1 = crate::Reg<u32, _GPIOPUCTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOPUCTL1;
#[doc = "`read()` method returns [gpiopuctl1::R](gpiopuctl1::R) reader structure"]
impl crate::Readable for GPIOPUCTL1 {}
#[doc = "`write(|w| ..)` method takes [gpiopuctl1::W](gpiopuctl1::W) writer structure"]
impl crate::Writable for GPIOPUCTL1 {}
#[doc = "Port C,D pull-up enable register"]
pub mod gpiopuctl1;
#[doc = "Port C pull-up enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpiopuctlc](gpiopuctlc) module"]
pub type GPIOPUCTLC = crate::Reg<u16, _GPIOPUCTLC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOPUCTLC;
#[doc = "`read()` method returns [gpiopuctlc::R](gpiopuctlc::R) reader structure"]
impl crate::Readable for GPIOPUCTLC {}
#[doc = "`write(|w| ..)` method takes [gpiopuctlc::W](gpiopuctlc::W) writer structure"]
impl crate::Writable for GPIOPUCTLC {}
#[doc = "Port C pull-up enable register"]
pub mod gpiopuctlc;
#[doc = "Port D pull-up enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpiopuctld](gpiopuctld) module"]
pub type GPIOPUCTLD = crate::Reg<u16, _GPIOPUCTLD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOPUCTLD;
#[doc = "`read()` method returns [gpiopuctld::R](gpiopuctld::R) reader structure"]
impl crate::Readable for GPIOPUCTLD {}
#[doc = "`write(|w| ..)` method takes [gpiopuctld::W](gpiopuctld::W) writer structure"]
impl crate::Writable for GPIOPUCTLD {}
#[doc = "Port D pull-up enable register"]
pub mod gpiopuctld;
#[doc = "Port E,F pull-up enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpiopuctl2](gpiopuctl2) module"]
pub type GPIOPUCTL2 = crate::Reg<u32, _GPIOPUCTL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOPUCTL2;
#[doc = "`read()` method returns [gpiopuctl2::R](gpiopuctl2::R) reader structure"]
impl crate::Readable for GPIOPUCTL2 {}
#[doc = "`write(|w| ..)` method takes [gpiopuctl2::W](gpiopuctl2::W) writer structure"]
impl crate::Writable for GPIOPUCTL2 {}
#[doc = "Port E,F pull-up enable register"]
pub mod gpiopuctl2;
#[doc = "Port E pull-up enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpiopuctle](gpiopuctle) module"]
pub type GPIOPUCTLE = crate::Reg<u16, _GPIOPUCTLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOPUCTLE;
#[doc = "`read()` method returns [gpiopuctle::R](gpiopuctle::R) reader structure"]
impl crate::Readable for GPIOPUCTLE {}
#[doc = "`write(|w| ..)` method takes [gpiopuctle::W](gpiopuctle::W) writer structure"]
impl crate::Writable for GPIOPUCTLE {}
#[doc = "Port E pull-up enable register"]
pub mod gpiopuctle;
#[doc = "Port F pull-up enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpiopuctlf](gpiopuctlf) module"]
pub type GPIOPUCTLF = crate::Reg<u16, _GPIOPUCTLF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOPUCTLF;
#[doc = "`read()` method returns [gpiopuctlf::R](gpiopuctlf::R) reader structure"]
impl crate::Readable for GPIOPUCTLF {}
#[doc = "`write(|w| ..)` method takes [gpiopuctlf::W](gpiopuctlf::W) writer structure"]
impl crate::Writable for GPIOPUCTLF {}
#[doc = "Port F pull-up enable register"]
pub mod gpiopuctlf;
#[doc = "Port G,H pull-up enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpiopuctl3](gpiopuctl3) module"]
pub type GPIOPUCTL3 = crate::Reg<u32, _GPIOPUCTL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOPUCTL3;
#[doc = "`read()` method returns [gpiopuctl3::R](gpiopuctl3::R) reader structure"]
impl crate::Readable for GPIOPUCTL3 {}
#[doc = "`write(|w| ..)` method takes [gpiopuctl3::W](gpiopuctl3::W) writer structure"]
impl crate::Writable for GPIOPUCTL3 {}
#[doc = "Port G,H pull-up enable register"]
pub mod gpiopuctl3;
#[doc = "Port G pull-up enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpiopuctlg](gpiopuctlg) module"]
pub type GPIOPUCTLG = crate::Reg<u16, _GPIOPUCTLG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOPUCTLG;
#[doc = "`read()` method returns [gpiopuctlg::R](gpiopuctlg::R) reader structure"]
impl crate::Readable for GPIOPUCTLG {}
#[doc = "`write(|w| ..)` method takes [gpiopuctlg::W](gpiopuctlg::W) writer structure"]
impl crate::Writable for GPIOPUCTLG {}
#[doc = "Port G pull-up enable register"]
pub mod gpiopuctlg;
#[doc = "Port H pull-up enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpiopuctlh](gpiopuctlh) module"]
pub type GPIOPUCTLH = crate::Reg<u16, _GPIOPUCTLH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOPUCTLH;
#[doc = "`read()` method returns [gpiopuctlh::R](gpiopuctlh::R) reader structure"]
impl crate::Readable for GPIOPUCTLH {}
#[doc = "`write(|w| ..)` method takes [gpiopuctlh::W](gpiopuctlh::W) writer structure"]
impl crate::Writable for GPIOPUCTLH {}
#[doc = "Port H pull-up enable register"]
pub mod gpiopuctlh;
#[doc = "PLL control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pll_ctrl](pll_ctrl) module"]
pub type PLL_CTRL = crate::Reg<u32, _PLL_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLL_CTRL;
#[doc = "`read()` method returns [pll_ctrl::R](pll_ctrl::R) reader structure"]
impl crate::Readable for PLL_CTRL {}
#[doc = "`write(|w| ..)` method takes [pll_ctrl::W](pll_ctrl::W) writer structure"]
impl crate::Writable for PLL_CTRL {}
#[doc = "PLL control register"]
pub mod pll_ctrl;
#[doc = "PLL output divider register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pll_od](pll_od) module"]
pub type PLL_OD = crate::Reg<u32, _PLL_OD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLL_OD;
#[doc = "`read()` method returns [pll_od::R](pll_od::R) reader structure"]
impl crate::Readable for PLL_OD {}
#[doc = "`write(|w| ..)` method takes [pll_od::W](pll_od::W) writer structure"]
impl crate::Writable for PLL_OD {}
#[doc = "PLL output divider register"]
pub mod pll_od;
#[doc = "PLL reference divider register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pll_nr](pll_nr) module"]
pub type PLL_NR = crate::Reg<u32, _PLL_NR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLL_NR;
#[doc = "`read()` method returns [pll_nr::R](pll_nr::R) reader structure"]
impl crate::Readable for PLL_NR {}
#[doc = "`write(|w| ..)` method takes [pll_nr::W](pll_nr::W) writer structure"]
impl crate::Writable for PLL_NR {}
#[doc = "PLL reference divider register"]
pub mod pll_nr;
#[doc = "PLL feedback divider register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pll_nf](pll_nf) module"]
pub type PLL_NF = crate::Reg<u32, _PLL_NF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLL_NF;
#[doc = "`read()` method returns [pll_nf::R](pll_nf::R) reader structure"]
impl crate::Readable for PLL_NF {}
#[doc = "`write(|w| ..)` method takes [pll_nf::W](pll_nf::W) writer structure"]
impl crate::Writable for PLL_NF {}
#[doc = "PLL feedback divider register"]
pub mod pll_nf;
#[doc = "External memory configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ext_mem_cfg](ext_mem_cfg) module"]
pub type EXT_MEM_CFG = crate::Reg<u32, _EXT_MEM_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXT_MEM_CFG;
#[doc = "`read()` method returns [ext_mem_cfg::R](ext_mem_cfg::R) reader structure"]
impl crate::Readable for EXT_MEM_CFG {}
#[doc = "`write(|w| ..)` method takes [ext_mem_cfg::W](ext_mem_cfg::W) writer structure"]
impl crate::Writable for EXT_MEM_CFG {}
#[doc = "External memory configuration register"]
pub mod ext_mem_cfg;
#[doc = "ADC 0-3 clock control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [adc_ctrl0](adc_ctrl0) module"]
pub type ADC_CTRL0 = crate::Reg<u32, _ADC_CTRL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_CTRL0;
#[doc = "`read()` method returns [adc_ctrl0::R](adc_ctrl0::R) reader structure"]
impl crate::Readable for ADC_CTRL0 {}
#[doc = "`write(|w| ..)` method takes [adc_ctrl0::W](adc_ctrl0::W) writer structure"]
impl crate::Writable for ADC_CTRL0 {}
#[doc = "ADC 0-3 clock control register"]
pub mod adc_ctrl0;
#[doc = "ADC 4-7 clock control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [adc_ctrl1](adc_ctrl1) module"]
pub type ADC_CTRL1 = crate::Reg<u32, _ADC_CTRL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_CTRL1;
#[doc = "`read()` method returns [adc_ctrl1::R](adc_ctrl1::R) reader structure"]
impl crate::Readable for ADC_CTRL1 {}
#[doc = "`write(|w| ..)` method takes [adc_ctrl1::W](adc_ctrl1::W) writer structure"]
impl crate::Writable for ADC_CTRL1 {}
#[doc = "ADC 4-7 clock control register"]
pub mod adc_ctrl1;
#[doc = "PWM prescalers sync register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pwm_sync](pwm_sync) module"]
pub type PWM_SYNC = crate::Reg<u32, _PWM_SYNC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWM_SYNC;
#[doc = "`read()` method returns [pwm_sync::R](pwm_sync::R) reader structure"]
impl crate::Readable for PWM_SYNC {}
#[doc = "`write(|w| ..)` method takes [pwm_sync::W](pwm_sync::W) writer structure"]
impl crate::Writable for PWM_SYNC {}
#[doc = "PWM prescalers sync register"]
pub mod pwm_sync;
#[doc = "PWM sync control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pwm_ctrl](pwm_ctrl) module"]
pub type PWM_CTRL = crate::Reg<u32, _PWM_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWM_CTRL;
#[doc = "`read()` method returns [pwm_ctrl::R](pwm_ctrl::R) reader structure"]
impl crate::Readable for PWM_CTRL {}
#[doc = "`write(|w| ..)` method takes [pwm_ctrl::W](pwm_ctrl::W) writer structure"]
impl crate::Writable for PWM_CTRL {}
#[doc = "PWM sync control register"]
pub mod pwm_ctrl;
#[doc = "System clock control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sys_clk](sys_clk) module"]
pub type SYS_CLK = crate::Reg<u32, _SYS_CLK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYS_CLK;
#[doc = "`read()` method returns [sys_clk::R](sys_clk::R) reader structure"]
impl crate::Readable for SYS_CLK {}
#[doc = "`write(|w| ..)` method takes [sys_clk::W](sys_clk::W) writer structure"]
impl crate::Writable for SYS_CLK {}
#[doc = "System clock control register"]
pub mod sys_clk;
#[doc = "Peripheral clock control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [apb_clk](apb_clk) module"]
pub type APB_CLK = crate::Reg<u32, _APB_CLK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB_CLK;
#[doc = "`read()` method returns [apb_clk::R](apb_clk::R) reader structure"]
impl crate::Readable for APB_CLK {}
#[doc = "`write(|w| ..)` method takes [apb_clk::W](apb_clk::W) writer structure"]
impl crate::Writable for APB_CLK {}
#[doc = "Peripheral clock control register"]
pub mod apb_clk;
#[doc = "UART clock control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uart_clk](uart_clk) module"]
pub type UART_CLK = crate::Reg<u32, _UART_CLK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_CLK;
#[doc = "`read()` method returns [uart_clk::R](uart_clk::R) reader structure"]
impl crate::Readable for UART_CLK {}
#[doc = "`write(|w| ..)` method takes [uart_clk::W](uart_clk::W) writer structure"]
impl crate::Writable for UART_CLK {}
#[doc = "UART clock control register"]
pub mod uart_clk;
#[doc = "SPI clock control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spi_clk](spi_clk) module"]
pub type SPI_CLK = crate::Reg<u32, _SPI_CLK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_CLK;
#[doc = "`read()` method returns [spi_clk::R](spi_clk::R) reader structure"]
impl crate::Readable for SPI_CLK {}
#[doc = "`write(|w| ..)` method takes [spi_clk::W](spi_clk::W) writer structure"]
impl crate::Writable for SPI_CLK {}
#[doc = "SPI clock control register"]
pub mod spi_clk;
#[doc = "Peripheral reset register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [per_rst0](per_rst0) module"]
pub type PER_RST0 = crate::Reg<u32, _PER_RST0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PER_RST0;
#[doc = "`read()` method returns [per_rst0::R](per_rst0::R) reader structure"]
impl crate::Readable for PER_RST0 {}
#[doc = "`write(|w| ..)` method takes [per_rst0::W](per_rst0::W) writer structure"]
impl crate::Writable for PER_RST0 {}
#[doc = "Peripheral reset register 0"]
pub mod per_rst0;
#[doc = "Peripheral reset register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [per_rst1](per_rst1) module"]
pub type PER_RST1 = crate::Reg<u32, _PER_RST1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PER_RST1;
#[doc = "`read()` method returns [per_rst1::R](per_rst1::R) reader structure"]
impl crate::Readable for PER_RST1 {}
#[doc = "`write(|w| ..)` method takes [per_rst1::W](per_rst1::W) writer structure"]
impl crate::Writable for PER_RST1 {}
#[doc = "Peripheral reset register 1"]
pub mod per_rst1;
#[doc = "Control register resynchronization input ports A, B\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpiose0](gpiose0) module"]
pub type GPIOSE0 = crate::Reg<u32, _GPIOSE0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOSE0;
#[doc = "`read()` method returns [gpiose0::R](gpiose0::R) reader structure"]
impl crate::Readable for GPIOSE0 {}
#[doc = "`write(|w| ..)` method takes [gpiose0::W](gpiose0::W) writer structure"]
impl crate::Writable for GPIOSE0 {}
#[doc = "Control register resynchronization input ports A, B"]
pub mod gpiose0;
#[doc = "Control register resynchronization input ports C, D\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpiose1](gpiose1) module"]
pub type GPIOSE1 = crate::Reg<u32, _GPIOSE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOSE1;
#[doc = "`read()` method returns [gpiose1::R](gpiose1::R) reader structure"]
impl crate::Readable for GPIOSE1 {}
#[doc = "`write(|w| ..)` method takes [gpiose1::W](gpiose1::W) writer structure"]
impl crate::Writable for GPIOSE1 {}
#[doc = "Control register resynchronization input ports C, D"]
pub mod gpiose1;
#[doc = "Control register resynchronization input ports E, F\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpiose2](gpiose2) module"]
pub type GPIOSE2 = crate::Reg<u32, _GPIOSE2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOSE2;
#[doc = "`read()` method returns [gpiose2::R](gpiose2::R) reader structure"]
impl crate::Readable for GPIOSE2 {}
#[doc = "`write(|w| ..)` method takes [gpiose2::W](gpiose2::W) writer structure"]
impl crate::Writable for GPIOSE2 {}
#[doc = "Control register resynchronization input ports E, F"]
pub mod gpiose2;
#[doc = "Control register resynchronization input ports G, H\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpiose3](gpiose3) module"]
pub type GPIOSE3 = crate::Reg<u32, _GPIOSE3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOSE3;
#[doc = "`read()` method returns [gpiose3::R](gpiose3::R) reader structure"]
impl crate::Readable for GPIOSE3 {}
#[doc = "`write(|w| ..)` method takes [gpiose3::W](gpiose3::W) writer structure"]
impl crate::Writable for GPIOSE3 {}
#[doc = "Control register resynchronization input ports G, H"]
pub mod gpiose3;
#[doc = "Register filter settings input ports A, B\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpioqe0](gpioqe0) module"]
pub type GPIOQE0 = crate::Reg<u32, _GPIOQE0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOQE0;
#[doc = "`read()` method returns [gpioqe0::R](gpioqe0::R) reader structure"]
impl crate::Readable for GPIOQE0 {}
#[doc = "`write(|w| ..)` method takes [gpioqe0::W](gpioqe0::W) writer structure"]
impl crate::Writable for GPIOQE0 {}
#[doc = "Register filter settings input ports A, B"]
pub mod gpioqe0;
#[doc = "Register filter settings input ports C, D\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpioqe1](gpioqe1) module"]
pub type GPIOQE1 = crate::Reg<u32, _GPIOQE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOQE1;
#[doc = "`read()` method returns [gpioqe1::R](gpioqe1::R) reader structure"]
impl crate::Readable for GPIOQE1 {}
#[doc = "`write(|w| ..)` method takes [gpioqe1::W](gpioqe1::W) writer structure"]
impl crate::Writable for GPIOQE1 {}
#[doc = "Register filter settings input ports C, D"]
pub mod gpioqe1;
#[doc = "Register filter settings input ports E, F\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpioqe2](gpioqe2) module"]
pub type GPIOQE2 = crate::Reg<u32, _GPIOQE2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOQE2;
#[doc = "`read()` method returns [gpioqe2::R](gpioqe2::R) reader structure"]
impl crate::Readable for GPIOQE2 {}
#[doc = "`write(|w| ..)` method takes [gpioqe2::W](gpioqe2::W) writer structure"]
impl crate::Writable for GPIOQE2 {}
#[doc = "Register filter settings input ports E, F"]
pub mod gpioqe2;
#[doc = "Register filter settings input ports G, H\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpioqe3](gpioqe3) module"]
pub type GPIOQE3 = crate::Reg<u32, _GPIOQE3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOQE3;
#[doc = "`read()` method returns [gpioqe3::R](gpioqe3::R) reader structure"]
impl crate::Readable for GPIOQE3 {}
#[doc = "`write(|w| ..)` method takes [gpioqe3::W](gpioqe3::W) writer structure"]
impl crate::Writable for GPIOQE3 {}
#[doc = "Register filter settings input ports G, H"]
pub mod gpioqe3;
#[doc = "Register filter settings input ports A, B\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpioqm0](gpioqm0) module"]
pub type GPIOQM0 = crate::Reg<u32, _GPIOQM0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOQM0;
#[doc = "`read()` method returns [gpioqm0::R](gpioqm0::R) reader structure"]
impl crate::Readable for GPIOQM0 {}
#[doc = "`write(|w| ..)` method takes [gpioqm0::W](gpioqm0::W) writer structure"]
impl crate::Writable for GPIOQM0 {}
#[doc = "Register filter settings input ports A, B"]
pub mod gpioqm0;
#[doc = "Register filter settings input ports C, D\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpioqm1](gpioqm1) module"]
pub type GPIOQM1 = crate::Reg<u32, _GPIOQM1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOQM1;
#[doc = "`read()` method returns [gpioqm1::R](gpioqm1::R) reader structure"]
impl crate::Readable for GPIOQM1 {}
#[doc = "`write(|w| ..)` method takes [gpioqm1::W](gpioqm1::W) writer structure"]
impl crate::Writable for GPIOQM1 {}
#[doc = "Register filter settings input ports C, D"]
pub mod gpioqm1;
#[doc = "Register filter settings input ports E, F\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpioqm2](gpioqm2) module"]
pub type GPIOQM2 = crate::Reg<u32, _GPIOQM2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOQM2;
#[doc = "`read()` method returns [gpioqm2::R](gpioqm2::R) reader structure"]
impl crate::Readable for GPIOQM2 {}
#[doc = "`write(|w| ..)` method takes [gpioqm2::W](gpioqm2::W) writer structure"]
impl crate::Writable for GPIOQM2 {}
#[doc = "Register filter settings input ports E, F"]
pub mod gpioqm2;
#[doc = "Register filter settings input ports G, H\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpioqm3](gpioqm3) module"]
pub type GPIOQM3 = crate::Reg<u32, _GPIOQM3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOQM3;
#[doc = "`read()` method returns [gpioqm3::R](gpioqm3::R) reader structure"]
impl crate::Readable for GPIOQM3 {}
#[doc = "`write(|w| ..)` method takes [gpioqm3::W](gpioqm3::W) writer structure"]
impl crate::Writable for GPIOQM3 {}
#[doc = "Register filter settings input ports G, H"]
pub mod gpioqm3;
#[doc = "Register filter settings input ports A, B, C, D\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpioqpad](gpioqpad) module"]
pub type GPIOQPAD = crate::Reg<u32, _GPIOQPAD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOQPAD;
#[doc = "`read()` method returns [gpioqpad::R](gpioqpad::R) reader structure"]
impl crate::Readable for GPIOQPAD {}
#[doc = "`write(|w| ..)` method takes [gpioqpad::W](gpioqpad::W) writer structure"]
impl crate::Writable for GPIOQPAD {}
#[doc = "Register filter settings input ports A, B, C, D"]
pub mod gpioqpad;
#[doc = "Register filter settings input ports E, F, G, H\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpioqpeh](gpioqpeh) module"]
pub type GPIOQPEH = crate::Reg<u32, _GPIOQPEH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOQPEH;
#[doc = "`read()` method returns [gpioqpeh::R](gpioqpeh::R) reader structure"]
impl crate::Readable for GPIOQPEH {}
#[doc = "`write(|w| ..)` method takes [gpioqpeh::W](gpioqpeh::W) writer structure"]
impl crate::Writable for GPIOQPEH {}
#[doc = "Register filter settings input ports E, F, G, H"]
pub mod gpioqpeh;
#[doc = "Customize the USB PHY\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [usb_ctrl](usb_ctrl) module"]
pub type USB_CTRL = crate::Reg<u32, _USB_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_CTRL;
#[doc = "`read()` method returns [usb_ctrl::R](usb_ctrl::R) reader structure"]
impl crate::Readable for USB_CTRL {}
#[doc = "`write(|w| ..)` method takes [usb_ctrl::W](usb_ctrl::W) writer structure"]
impl crate::Writable for USB_CTRL {}
#[doc = "Customize the USB PHY"]
pub mod usb_ctrl;
#[doc = "Select source clk UART and SSP register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uart_spi_clk_sel](uart_spi_clk_sel) module"]
pub type UART_SPI_CLK_SEL = crate::Reg<u32, _UART_SPI_CLK_SEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_SPI_CLK_SEL;
#[doc = "`read()` method returns [uart_spi_clk_sel::R](uart_spi_clk_sel::R) reader structure"]
impl crate::Readable for UART_SPI_CLK_SEL {}
#[doc = "`write(|w| ..)` method takes [uart_spi_clk_sel::W](uart_spi_clk_sel::W) writer structure"]
impl crate::Writable for UART_SPI_CLK_SEL {}
#[doc = "Select source clk UART and SSP register"]
pub mod uart_spi_clk_sel;
#[doc = "ADC control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [adc_ctrl2](adc_ctrl2) module"]
pub type ADC_CTRL2 = crate::Reg<u32, _ADC_CTRL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_CTRL2;
#[doc = "`read()` method returns [adc_ctrl2::R](adc_ctrl2::R) reader structure"]
impl crate::Readable for ADC_CTRL2 {}
#[doc = "`write(|w| ..)` method takes [adc_ctrl2::W](adc_ctrl2::W) writer structure"]
impl crate::Writable for ADC_CTRL2 {}
#[doc = "ADC control register 2"]
pub mod adc_ctrl2;
#[doc = "Full erase flash (user and boot) register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [flash_full_erase](flash_full_erase) module"]
pub type FLASH_FULL_ERASE = crate::Reg<u32, _FLASH_FULL_ERASE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH_FULL_ERASE;
#[doc = "`read()` method returns [flash_full_erase::R](flash_full_erase::R) reader structure"]
impl crate::Readable for FLASH_FULL_ERASE {}
#[doc = "`write(|w| ..)` method takes [flash_full_erase::W](flash_full_erase::W) writer structure"]
impl crate::Writable for FLASH_FULL_ERASE {}
#[doc = "Full erase flash (user and boot) register"]
pub mod flash_full_erase;
