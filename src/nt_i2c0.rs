#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Data register"]
    pub sda: SDA,
    #[doc = "0x04 - Status register"]
    pub st: ST,
    #[doc = "0x08 - Status and control register"]
    pub cst: CST,
    #[doc = "0x0c - Control register 0"]
    pub ctl0: CTL0,
    #[doc = "0x10 - Register own address"]
    pub addr: ADDR,
    #[doc = "0x14 - Control register 1"]
    pub ctl1: CTL1,
    #[doc = "0x18 - Prescaler load register"]
    pub topr: TOPR,
    #[doc = "0x1c - Control register 2"]
    pub ctl2: CTL2,
}
#[doc = "Data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sda](sda) module"]
pub type SDA = crate::Reg<u32, _SDA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDA;
#[doc = "`read()` method returns [sda::R](sda::R) reader structure"]
impl crate::Readable for SDA {}
#[doc = "`write(|w| ..)` method takes [sda::W](sda::W) writer structure"]
impl crate::Writable for SDA {}
#[doc = "Data register"]
pub mod sda;
#[doc = "Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [st](st) module"]
pub type ST = crate::Reg<u32, _ST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST;
#[doc = "`read()` method returns [st::R](st::R) reader structure"]
impl crate::Readable for ST {}
#[doc = "`write(|w| ..)` method takes [st::W](st::W) writer structure"]
impl crate::Writable for ST {}
#[doc = "Status register"]
pub mod st;
#[doc = "Status and control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cst](cst) module"]
pub type CST = crate::Reg<u32, _CST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CST;
#[doc = "`read()` method returns [cst::R](cst::R) reader structure"]
impl crate::Readable for CST {}
#[doc = "`write(|w| ..)` method takes [cst::W](cst::W) writer structure"]
impl crate::Writable for CST {}
#[doc = "Status and control register"]
pub mod cst;
#[doc = "Control register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctl0](ctl0) module"]
pub type CTL0 = crate::Reg<u32, _CTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTL0;
#[doc = "`read()` method returns [ctl0::R](ctl0::R) reader structure"]
impl crate::Readable for CTL0 {}
#[doc = "`write(|w| ..)` method takes [ctl0::W](ctl0::W) writer structure"]
impl crate::Writable for CTL0 {}
#[doc = "Control register 0"]
pub mod ctl0;
#[doc = "Register own address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [addr](addr) module"]
pub type ADDR = crate::Reg<u32, _ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDR;
#[doc = "`read()` method returns [addr::R](addr::R) reader structure"]
impl crate::Readable for ADDR {}
#[doc = "`write(|w| ..)` method takes [addr::W](addr::W) writer structure"]
impl crate::Writable for ADDR {}
#[doc = "Register own address"]
pub mod addr;
#[doc = "Control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctl1](ctl1) module"]
pub type CTL1 = crate::Reg<u32, _CTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTL1;
#[doc = "`read()` method returns [ctl1::R](ctl1::R) reader structure"]
impl crate::Readable for CTL1 {}
#[doc = "`write(|w| ..)` method takes [ctl1::W](ctl1::W) writer structure"]
impl crate::Writable for CTL1 {}
#[doc = "Control register 1"]
pub mod ctl1;
#[doc = "Prescaler load register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [topr](topr) module"]
pub type TOPR = crate::Reg<u32, _TOPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TOPR;
#[doc = "`read()` method returns [topr::R](topr::R) reader structure"]
impl crate::Readable for TOPR {}
#[doc = "`write(|w| ..)` method takes [topr::W](topr::W) writer structure"]
impl crate::Writable for TOPR {}
#[doc = "Prescaler load register"]
pub mod topr;
#[doc = "Control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctl2](ctl2) module"]
pub type CTL2 = crate::Reg<u32, _CTL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTL2;
#[doc = "`read()` method returns [ctl2::R](ctl2::R) reader structure"]
impl crate::Readable for CTL2 {}
#[doc = "`write(|w| ..)` method takes [ctl2::W](ctl2::W) writer structure"]
impl crate::Writable for CTL2 {}
#[doc = "Control register 2"]
pub mod ctl2;
