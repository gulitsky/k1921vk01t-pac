#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TIMER register"]
    pub tsctr: TSCTR,
    #[doc = "0x04 - Load Timer register"]
    pub ctrphs: CTRPHS,
    #[doc = "0x08 - Capture register 0"]
    pub cap0: CAP0,
    #[doc = "0x0c - Capture register 1"]
    pub cap1: CAP1,
    #[doc = "0x10 - Capture register 2"]
    pub cap2: CAP2,
    #[doc = "0x14 - Capture register 3"]
    pub cap3: CAP3,
    _reserved6: [u8; 16usize],
    #[doc = "0x28 - Register capture control 0"]
    pub ecctl0: ECCTL0,
    #[doc = "0x2c - Register capture control 1"]
    pub ecctl1: ECCTL1,
    #[doc = "0x30 - Register interrupt mask"]
    pub eceint: ECEINT,
    #[doc = "0x34 - Register interrupt status"]
    pub ecflg: ECFLG,
    #[doc = "0x38 - Register reset interrupt"]
    pub ecclr: ECCLR,
    #[doc = "0x3c - Test register interrupt generation"]
    pub ecfrc: ECFRC,
    #[doc = "0x40 - Active interrupt status register"]
    pub peint: PEINT,
}
#[doc = "TIMER register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tsctr](tsctr) module"]
pub type TSCTR = crate::Reg<u32, _TSCTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TSCTR;
#[doc = "`read()` method returns [tsctr::R](tsctr::R) reader structure"]
impl crate::Readable for TSCTR {}
#[doc = "`write(|w| ..)` method takes [tsctr::W](tsctr::W) writer structure"]
impl crate::Writable for TSCTR {}
#[doc = "TIMER register"]
pub mod tsctr;
#[doc = "Load Timer register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctrphs](ctrphs) module"]
pub type CTRPHS = crate::Reg<u32, _CTRPHS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRPHS;
#[doc = "`read()` method returns [ctrphs::R](ctrphs::R) reader structure"]
impl crate::Readable for CTRPHS {}
#[doc = "`write(|w| ..)` method takes [ctrphs::W](ctrphs::W) writer structure"]
impl crate::Writable for CTRPHS {}
#[doc = "Load Timer register"]
pub mod ctrphs;
#[doc = "Capture register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cap0](cap0) module"]
pub type CAP0 = crate::Reg<u32, _CAP0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAP0;
#[doc = "`read()` method returns [cap0::R](cap0::R) reader structure"]
impl crate::Readable for CAP0 {}
#[doc = "`write(|w| ..)` method takes [cap0::W](cap0::W) writer structure"]
impl crate::Writable for CAP0 {}
#[doc = "Capture register 0"]
pub mod cap0;
#[doc = "Capture register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cap1](cap1) module"]
pub type CAP1 = crate::Reg<u32, _CAP1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAP1;
#[doc = "`read()` method returns [cap1::R](cap1::R) reader structure"]
impl crate::Readable for CAP1 {}
#[doc = "`write(|w| ..)` method takes [cap1::W](cap1::W) writer structure"]
impl crate::Writable for CAP1 {}
#[doc = "Capture register 1"]
pub mod cap1;
#[doc = "Capture register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cap2](cap2) module"]
pub type CAP2 = crate::Reg<u32, _CAP2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAP2;
#[doc = "`read()` method returns [cap2::R](cap2::R) reader structure"]
impl crate::Readable for CAP2 {}
#[doc = "`write(|w| ..)` method takes [cap2::W](cap2::W) writer structure"]
impl crate::Writable for CAP2 {}
#[doc = "Capture register 2"]
pub mod cap2;
#[doc = "Capture register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cap3](cap3) module"]
pub type CAP3 = crate::Reg<u32, _CAP3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAP3;
#[doc = "`read()` method returns [cap3::R](cap3::R) reader structure"]
impl crate::Readable for CAP3 {}
#[doc = "`write(|w| ..)` method takes [cap3::W](cap3::W) writer structure"]
impl crate::Writable for CAP3 {}
#[doc = "Capture register 3"]
pub mod cap3;
#[doc = "Register capture control 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ecctl0](ecctl0) module"]
pub type ECCTL0 = crate::Reg<u32, _ECCTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ECCTL0;
#[doc = "`read()` method returns [ecctl0::R](ecctl0::R) reader structure"]
impl crate::Readable for ECCTL0 {}
#[doc = "`write(|w| ..)` method takes [ecctl0::W](ecctl0::W) writer structure"]
impl crate::Writable for ECCTL0 {}
#[doc = "Register capture control 0"]
pub mod ecctl0;
#[doc = "Register capture control 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ecctl1](ecctl1) module"]
pub type ECCTL1 = crate::Reg<u32, _ECCTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ECCTL1;
#[doc = "`read()` method returns [ecctl1::R](ecctl1::R) reader structure"]
impl crate::Readable for ECCTL1 {}
#[doc = "`write(|w| ..)` method takes [ecctl1::W](ecctl1::W) writer structure"]
impl crate::Writable for ECCTL1 {}
#[doc = "Register capture control 1"]
pub mod ecctl1;
#[doc = "Register interrupt mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [eceint](eceint) module"]
pub type ECEINT = crate::Reg<u32, _ECEINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ECEINT;
#[doc = "`read()` method returns [eceint::R](eceint::R) reader structure"]
impl crate::Readable for ECEINT {}
#[doc = "`write(|w| ..)` method takes [eceint::W](eceint::W) writer structure"]
impl crate::Writable for ECEINT {}
#[doc = "Register interrupt mask"]
pub mod eceint;
#[doc = "Register interrupt status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ecflg](ecflg) module"]
pub type ECFLG = crate::Reg<u32, _ECFLG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ECFLG;
#[doc = "`read()` method returns [ecflg::R](ecflg::R) reader structure"]
impl crate::Readable for ECFLG {}
#[doc = "Register interrupt status"]
pub mod ecflg;
#[doc = "Register reset interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ecclr](ecclr) module"]
pub type ECCLR = crate::Reg<u32, _ECCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ECCLR;
#[doc = "`read()` method returns [ecclr::R](ecclr::R) reader structure"]
impl crate::Readable for ECCLR {}
#[doc = "`write(|w| ..)` method takes [ecclr::W](ecclr::W) writer structure"]
impl crate::Writable for ECCLR {}
#[doc = "Register reset interrupt"]
pub mod ecclr;
#[doc = "Test register interrupt generation\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ecfrc](ecfrc) module"]
pub type ECFRC = crate::Reg<u32, _ECFRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ECFRC;
#[doc = "`read()` method returns [ecfrc::R](ecfrc::R) reader structure"]
impl crate::Readable for ECFRC {}
#[doc = "`write(|w| ..)` method takes [ecfrc::W](ecfrc::W) writer structure"]
impl crate::Writable for ECFRC {}
#[doc = "Test register interrupt generation"]
pub mod ecfrc;
#[doc = "Active interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [peint](peint) module"]
pub type PEINT = crate::Reg<u32, _PEINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PEINT;
#[doc = "`read()` method returns [peint::R](peint::R) reader structure"]
impl crate::Readable for PEINT {}
#[doc = "`write(|w| ..)` method takes [peint::W](peint::W) writer structure"]
impl crate::Writable for PEINT {}
#[doc = "Active interrupt status register"]
pub mod peint;
