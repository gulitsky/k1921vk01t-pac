#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Data input register"]
    pub data: DATA,
    #[doc = "0x04 - Data output register"]
    pub dataout: DATAOUT,
    _reserved2: [u8; 8usize],
    #[doc = "0x10 - Output enable set register"]
    pub outenset: OUTENSET,
    #[doc = "0x14 - Output enable clear register"]
    pub outenclr: OUTENCLR,
    #[doc = "0x18 - Alternative function set register"]
    pub altfuncset: ALTFUNCSET,
    #[doc = "0x1c - Alternative function clear register"]
    pub altfuncclr: ALTFUNCCLR,
    #[doc = "0x20 - Interrupt enable set register"]
    pub intenset: INTENSET,
    #[doc = "0x24 - Interrupt enable clear register"]
    pub intenclr: INTENCLR,
    #[doc = "0x28 - Interrupt type set register"]
    pub inttypeset: INTTYPESET,
    #[doc = "0x2c - Interrupt type clear register"]
    pub inttypeclr: INTTYPECLR,
    #[doc = "0x30 - Interrupt polarity set register"]
    pub intpolset: INTPOLSET,
    #[doc = "0x34 - Interrupt polarity clear register"]
    pub intpolclr: INTPOLCLR,
    #[doc = "0x38 - Interrupt status register"]
    pub intstatus: INTSTATUS,
    _reserved13: [u8; 964usize],
    #[doc = "0x400 - MASKLOWBYTE"]
    pub masklowbyte: [MASKLOWBYTE; 256],
    #[doc = "0x800 - MASKHIGHBYTE"]
    pub maskhighbyte: [MASKHIGHBYTE; 256],
}
#[doc = r"Register block"]
#[repr(C)]
pub struct MASKLOWBYTE {
    #[doc = "0x00 - Lower byte masked access register"]
    pub masklowbyte: self::masklowbyte::MASKLOWBYTE,
}
#[doc = r"Register block"]
#[doc = "MASKLOWBYTE"]
pub mod masklowbyte;
#[doc = r"Register block"]
#[repr(C)]
pub struct MASKHIGHBYTE {
    #[doc = "0x00 - Higher byte masked access register"]
    pub maskhighbyte: self::maskhighbyte::MASKHIGHBYTE,
}
#[doc = r"Register block"]
#[doc = "MASKHIGHBYTE"]
pub mod maskhighbyte;
#[doc = "Data input register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [data](data) module"]
pub type DATA = crate::Reg<u32, _DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA;
#[doc = "`read()` method returns [data::R](data::R) reader structure"]
impl crate::Readable for DATA {}
#[doc = "`write(|w| ..)` method takes [data::W](data::W) writer structure"]
impl crate::Writable for DATA {}
#[doc = "Data input register"]
pub mod data;
#[doc = "Data output register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dataout](dataout) module"]
pub type DATAOUT = crate::Reg<u32, _DATAOUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATAOUT;
#[doc = "`read()` method returns [dataout::R](dataout::R) reader structure"]
impl crate::Readable for DATAOUT {}
#[doc = "`write(|w| ..)` method takes [dataout::W](dataout::W) writer structure"]
impl crate::Writable for DATAOUT {}
#[doc = "Data output register"]
pub mod dataout;
#[doc = "Output enable set register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [outenset](outenset) module"]
pub type OUTENSET = crate::Reg<u32, _OUTENSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUTENSET;
#[doc = "`read()` method returns [outenset::R](outenset::R) reader structure"]
impl crate::Readable for OUTENSET {}
#[doc = "`write(|w| ..)` method takes [outenset::W](outenset::W) writer structure"]
impl crate::Writable for OUTENSET {}
#[doc = "Output enable set register"]
pub mod outenset;
#[doc = "Output enable clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [outenclr](outenclr) module"]
pub type OUTENCLR = crate::Reg<u32, _OUTENCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUTENCLR;
#[doc = "`read()` method returns [outenclr::R](outenclr::R) reader structure"]
impl crate::Readable for OUTENCLR {}
#[doc = "`write(|w| ..)` method takes [outenclr::W](outenclr::W) writer structure"]
impl crate::Writable for OUTENCLR {}
#[doc = "Output enable clear register"]
pub mod outenclr;
#[doc = "Alternative function set register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [altfuncset](altfuncset) module"]
pub type ALTFUNCSET = crate::Reg<u32, _ALTFUNCSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALTFUNCSET;
#[doc = "`read()` method returns [altfuncset::R](altfuncset::R) reader structure"]
impl crate::Readable for ALTFUNCSET {}
#[doc = "`write(|w| ..)` method takes [altfuncset::W](altfuncset::W) writer structure"]
impl crate::Writable for ALTFUNCSET {}
#[doc = "Alternative function set register"]
pub mod altfuncset;
#[doc = "Alternative function clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [altfuncclr](altfuncclr) module"]
pub type ALTFUNCCLR = crate::Reg<u32, _ALTFUNCCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALTFUNCCLR;
#[doc = "`read()` method returns [altfuncclr::R](altfuncclr::R) reader structure"]
impl crate::Readable for ALTFUNCCLR {}
#[doc = "`write(|w| ..)` method takes [altfuncclr::W](altfuncclr::W) writer structure"]
impl crate::Writable for ALTFUNCCLR {}
#[doc = "Alternative function clear register"]
pub mod altfuncclr;
#[doc = "Interrupt enable set register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intenset](intenset) module"]
pub type INTENSET = crate::Reg<u32, _INTENSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTENSET;
#[doc = "`read()` method returns [intenset::R](intenset::R) reader structure"]
impl crate::Readable for INTENSET {}
#[doc = "`write(|w| ..)` method takes [intenset::W](intenset::W) writer structure"]
impl crate::Writable for INTENSET {}
#[doc = "Interrupt enable set register"]
pub mod intenset;
#[doc = "Interrupt enable clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intenclr](intenclr) module"]
pub type INTENCLR = crate::Reg<u32, _INTENCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTENCLR;
#[doc = "`read()` method returns [intenclr::R](intenclr::R) reader structure"]
impl crate::Readable for INTENCLR {}
#[doc = "`write(|w| ..)` method takes [intenclr::W](intenclr::W) writer structure"]
impl crate::Writable for INTENCLR {}
#[doc = "Interrupt enable clear register"]
pub mod intenclr;
#[doc = "Interrupt type set register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [inttypeset](inttypeset) module"]
pub type INTTYPESET = crate::Reg<u32, _INTTYPESET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTTYPESET;
#[doc = "`read()` method returns [inttypeset::R](inttypeset::R) reader structure"]
impl crate::Readable for INTTYPESET {}
#[doc = "`write(|w| ..)` method takes [inttypeset::W](inttypeset::W) writer structure"]
impl crate::Writable for INTTYPESET {}
#[doc = "Interrupt type set register"]
pub mod inttypeset;
#[doc = "Interrupt type clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [inttypeclr](inttypeclr) module"]
pub type INTTYPECLR = crate::Reg<u32, _INTTYPECLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTTYPECLR;
#[doc = "`read()` method returns [inttypeclr::R](inttypeclr::R) reader structure"]
impl crate::Readable for INTTYPECLR {}
#[doc = "`write(|w| ..)` method takes [inttypeclr::W](inttypeclr::W) writer structure"]
impl crate::Writable for INTTYPECLR {}
#[doc = "Interrupt type clear register"]
pub mod inttypeclr;
#[doc = "Interrupt polarity set register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intpolset](intpolset) module"]
pub type INTPOLSET = crate::Reg<u32, _INTPOLSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTPOLSET;
#[doc = "`read()` method returns [intpolset::R](intpolset::R) reader structure"]
impl crate::Readable for INTPOLSET {}
#[doc = "`write(|w| ..)` method takes [intpolset::W](intpolset::W) writer structure"]
impl crate::Writable for INTPOLSET {}
#[doc = "Interrupt polarity set register"]
pub mod intpolset;
#[doc = "Interrupt polarity clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intpolclr](intpolclr) module"]
pub type INTPOLCLR = crate::Reg<u32, _INTPOLCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTPOLCLR;
#[doc = "`read()` method returns [intpolclr::R](intpolclr::R) reader structure"]
impl crate::Readable for INTPOLCLR {}
#[doc = "`write(|w| ..)` method takes [intpolclr::W](intpolclr::W) writer structure"]
impl crate::Writable for INTPOLCLR {}
#[doc = "Interrupt polarity clear register"]
pub mod intpolclr;
#[doc = "Interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intstatus](intstatus) module"]
pub type INTSTATUS = crate::Reg<u32, _INTSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTSTATUS;
#[doc = "`read()` method returns [intstatus::R](intstatus::R) reader structure"]
impl crate::Readable for INTSTATUS {}
#[doc = "`write(|w| ..)` method takes [intstatus::W](intstatus::W) writer structure"]
impl crate::Writable for INTSTATUS {}
#[doc = "Interrupt status register"]
pub mod intstatus;
