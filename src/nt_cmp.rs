#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Masked interrupt status register"]
    pub mis: MIS,
    #[doc = "0x04 - Interrupt Status Register"]
    pub ris: RIS,
    #[doc = "0x08 - Interrupt mask register"]
    pub inten: INTEN,
    #[doc = "0x0c - Reference voltage control DAC0 register"]
    pub refctl0: REFCTL0,
    #[doc = "0x10 - Reference voltage control DAC1 register"]
    pub refctl1: REFCTL1,
    #[doc = "0x14 - Reference voltage control DAC2 register"]
    pub refctl2: REFCTL2,
    _reserved6: [u8; 8usize],
    #[doc = "0x20 - Status register CMP0"]
    pub stat0: STAT0,
    #[doc = "0x24 - Control Register CMP0"]
    pub ctl0: CTL0,
    _reserved8: [u8; 24usize],
    #[doc = "0x40 - Status register CMP1"]
    pub stat1: STAT1,
    #[doc = "0x44 - Control Register CMP1"]
    pub ctl1: CTL1,
    _reserved10: [u8; 24usize],
    #[doc = "0x60 - Status register CMP2"]
    pub stat2: STAT2,
    #[doc = "0x64 - Control Register CMP2"]
    pub ctl2: CTL2,
    _reserved12: [u8; 20usize],
    #[doc = "0x7c - Power analog circuits register"]
    pub power: POWER,
}
#[doc = "Masked interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mis](mis) module"]
pub type MIS = crate::Reg<u32, _MIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MIS;
#[doc = "`read()` method returns [mis::R](mis::R) reader structure"]
impl crate::Readable for MIS {}
#[doc = "`write(|w| ..)` method takes [mis::W](mis::W) writer structure"]
impl crate::Writable for MIS {}
#[doc = "Masked interrupt status register"]
pub mod mis;
#[doc = "Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ris](ris) module"]
pub type RIS = crate::Reg<u32, _RIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RIS;
#[doc = "`read()` method returns [ris::R](ris::R) reader structure"]
impl crate::Readable for RIS {}
#[doc = "Interrupt Status Register"]
pub mod ris;
#[doc = "Interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [inten](inten) module"]
pub type INTEN = crate::Reg<u32, _INTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTEN;
#[doc = "`read()` method returns [inten::R](inten::R) reader structure"]
impl crate::Readable for INTEN {}
#[doc = "`write(|w| ..)` method takes [inten::W](inten::W) writer structure"]
impl crate::Writable for INTEN {}
#[doc = "Interrupt mask register"]
pub mod inten;
#[doc = "Reference voltage control DAC0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [refctl0](refctl0) module"]
pub type REFCTL0 = crate::Reg<u32, _REFCTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REFCTL0;
#[doc = "`read()` method returns [refctl0::R](refctl0::R) reader structure"]
impl crate::Readable for REFCTL0 {}
#[doc = "`write(|w| ..)` method takes [refctl0::W](refctl0::W) writer structure"]
impl crate::Writable for REFCTL0 {}
#[doc = "Reference voltage control DAC0 register"]
pub mod refctl0;
#[doc = "Reference voltage control DAC1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [refctl1](refctl1) module"]
pub type REFCTL1 = crate::Reg<u32, _REFCTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REFCTL1;
#[doc = "`read()` method returns [refctl1::R](refctl1::R) reader structure"]
impl crate::Readable for REFCTL1 {}
#[doc = "`write(|w| ..)` method takes [refctl1::W](refctl1::W) writer structure"]
impl crate::Writable for REFCTL1 {}
#[doc = "Reference voltage control DAC1 register"]
pub mod refctl1;
#[doc = "Reference voltage control DAC2 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [refctl2](refctl2) module"]
pub type REFCTL2 = crate::Reg<u32, _REFCTL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REFCTL2;
#[doc = "`read()` method returns [refctl2::R](refctl2::R) reader structure"]
impl crate::Readable for REFCTL2 {}
#[doc = "`write(|w| ..)` method takes [refctl2::W](refctl2::W) writer structure"]
impl crate::Writable for REFCTL2 {}
#[doc = "Reference voltage control DAC2 register"]
pub mod refctl2;
#[doc = "Status register CMP0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [stat0](stat0) module"]
pub type STAT0 = crate::Reg<u32, _STAT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STAT0;
#[doc = "`read()` method returns [stat0::R](stat0::R) reader structure"]
impl crate::Readable for STAT0 {}
#[doc = "Status register CMP0"]
pub mod stat0;
#[doc = "Control Register CMP0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctl0](ctl0) module"]
pub type CTL0 = crate::Reg<u32, _CTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTL0;
#[doc = "`read()` method returns [ctl0::R](ctl0::R) reader structure"]
impl crate::Readable for CTL0 {}
#[doc = "`write(|w| ..)` method takes [ctl0::W](ctl0::W) writer structure"]
impl crate::Writable for CTL0 {}
#[doc = "Control Register CMP0"]
pub mod ctl0;
#[doc = "Status register CMP1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [stat1](stat1) module"]
pub type STAT1 = crate::Reg<u32, _STAT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STAT1;
#[doc = "`read()` method returns [stat1::R](stat1::R) reader structure"]
impl crate::Readable for STAT1 {}
#[doc = "Status register CMP1"]
pub mod stat1;
#[doc = "Control Register CMP1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctl1](ctl1) module"]
pub type CTL1 = crate::Reg<u32, _CTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTL1;
#[doc = "`read()` method returns [ctl1::R](ctl1::R) reader structure"]
impl crate::Readable for CTL1 {}
#[doc = "`write(|w| ..)` method takes [ctl1::W](ctl1::W) writer structure"]
impl crate::Writable for CTL1 {}
#[doc = "Control Register CMP1"]
pub mod ctl1;
#[doc = "Status register CMP2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [stat2](stat2) module"]
pub type STAT2 = crate::Reg<u32, _STAT2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STAT2;
#[doc = "`read()` method returns [stat2::R](stat2::R) reader structure"]
impl crate::Readable for STAT2 {}
#[doc = "Status register CMP2"]
pub mod stat2;
#[doc = "Control Register CMP2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctl2](ctl2) module"]
pub type CTL2 = crate::Reg<u32, _CTL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTL2;
#[doc = "`read()` method returns [ctl2::R](ctl2::R) reader structure"]
impl crate::Readable for CTL2 {}
#[doc = "`write(|w| ..)` method takes [ctl2::W](ctl2::W) writer structure"]
impl crate::Writable for CTL2 {}
#[doc = "Control Register CMP2"]
pub mod ctl2;
#[doc = "Power analog circuits register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [power](power) module"]
pub type POWER = crate::Reg<u32, _POWER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _POWER;
#[doc = "`read()` method returns [power::R](power::R) reader structure"]
impl crate::Readable for POWER {}
#[doc = "`write(|w| ..)` method takes [power::W](power::W) writer structure"]
impl crate::Writable for POWER {}
#[doc = "Power analog circuits register"]
pub mod power;
