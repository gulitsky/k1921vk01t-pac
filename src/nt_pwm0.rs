#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timer Control Register"]
    pub tbctl: TBCTL,
    #[doc = "0x04 - Timer status register"]
    pub tbsts: TBSTS,
    #[doc = "0x08 - register Phases"]
    pub tbphs: TBPHS,
    #[doc = "0x0c - Register current value of timer"]
    pub tbctr: TBCTR,
    #[doc = "0x10 - Register maximum Timer"]
    pub tbprd: TBPRD,
    #[doc = "0x14 - No description"]
    pub cmpctl: CMPCTL,
    #[doc = "0x18 - Register threshold A"]
    pub cmpa: CMPA,
    #[doc = "0x1c - Register threshold B"]
    pub cmpb: CMPB,
    #[doc = "0x20 - Register handlers for output A"]
    pub aqctla: AQCTLA,
    #[doc = "0x24 - Register handlers for output B"]
    pub aqctlb: AQCTLB,
    #[doc = "0x28 - Register handlers for a single program management"]
    pub aqsfrc: AQSFRC,
    #[doc = "0x2c - Register handlers for the cyclic program Management"]
    pub aqcsfrc: AQCSFRC,
    #[doc = "0x30 - Generator Control Register PWM Dead Time"]
    pub dbctl: DBCTL,
    #[doc = "0x34 - Dead time control register"]
    pub dbred: DBRED,
    #[doc = "0x38 - Dead time control register"]
    pub dbfed: DBFED,
    #[doc = "0x3c - Register the source of the accident"]
    pub tzsel: TZSEL,
    #[doc = "0x40 - Control Register detector alarm signal"]
    pub tzctl: TZCTL,
    #[doc = "0x44 - Interrupt mask register Signal Detection Alarms"]
    pub tzeint: TZEINT,
    #[doc = "0x48 - Flags register interrupt signal detector Accidents"]
    pub tzflg: TZFLG,
    #[doc = "0x4c - Register reset interrupt flag detector alarm signal"]
    pub tzclr: TZCLR,
    #[doc = "0x50 - Register software emulation of faults"]
    pub tzfrc: TZFRC,
    #[doc = "0x54 - A Source event trigger"]
    pub etsel: ETSEL,
    #[doc = "0x58 - Prescaler register the event trigger"]
    pub etps: ETPS,
    #[doc = "0x5c - Register Flags event trigger"]
    pub etflg: ETFLG,
    #[doc = "0x60 - Register reset flags trigger events"]
    pub etclr: ETCLR,
    #[doc = "0x64 - Register software emulation events"]
    pub etfrc: ETFRC,
    #[doc = "0x68 - Control Register modulator"]
    pub pcctl: PCCTL,
    #[doc = "0x6c - Register Configuration Block PWM High Definition"]
    pub hrcnfg: HRCNFG,
    #[doc = "0x70 - Register width filtering"]
    pub fwdth: FWDTH,
    _reserved29: [u8; 20usize],
    #[doc = "0x88 - Register source event retention"]
    pub hdsel: HDSEL,
    #[doc = "0x8c - Control Register detector hold events"]
    pub hdctl: HDCTL,
    #[doc = "0x90 - Register software activation threshold trigger"]
    pub hdeint: HDEINT,
    #[doc = "0x94 - Registrer HD flag interrupt"]
    pub hdflg: HDFLG,
    #[doc = "0x98 - Register clear HD flag"]
    pub hdclr: HDCLR,
    #[doc = "0x9c - Register software activation threshold trigger"]
    pub hdfrc: HDFRC,
    #[doc = "0xa0 - Register clear HD interrupt"]
    pub hdintclr: HDINTCLR,
    #[doc = "0xa4 - Register clear TZ interrupt"]
    pub tzintclr: TZINTCLR,
    #[doc = "0xa8 - Register clear interrupt"]
    pub intclr: INTCLR,
}
#[doc = "Timer Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tbctl](tbctl) module"]
pub type TBCTL = crate::Reg<u32, _TBCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TBCTL;
#[doc = "`read()` method returns [tbctl::R](tbctl::R) reader structure"]
impl crate::Readable for TBCTL {}
#[doc = "`write(|w| ..)` method takes [tbctl::W](tbctl::W) writer structure"]
impl crate::Writable for TBCTL {}
#[doc = "Timer Control Register"]
pub mod tbctl;
#[doc = "Timer status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tbsts](tbsts) module"]
pub type TBSTS = crate::Reg<u32, _TBSTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TBSTS;
#[doc = "`read()` method returns [tbsts::R](tbsts::R) reader structure"]
impl crate::Readable for TBSTS {}
#[doc = "`write(|w| ..)` method takes [tbsts::W](tbsts::W) writer structure"]
impl crate::Writable for TBSTS {}
#[doc = "Timer status register"]
pub mod tbsts;
#[doc = "register Phases\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tbphs](tbphs) module"]
pub type TBPHS = crate::Reg<u32, _TBPHS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TBPHS;
#[doc = "`read()` method returns [tbphs::R](tbphs::R) reader structure"]
impl crate::Readable for TBPHS {}
#[doc = "`write(|w| ..)` method takes [tbphs::W](tbphs::W) writer structure"]
impl crate::Writable for TBPHS {}
#[doc = "register Phases"]
pub mod tbphs;
#[doc = "Register current value of timer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tbctr](tbctr) module"]
pub type TBCTR = crate::Reg<u32, _TBCTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TBCTR;
#[doc = "`read()` method returns [tbctr::R](tbctr::R) reader structure"]
impl crate::Readable for TBCTR {}
#[doc = "`write(|w| ..)` method takes [tbctr::W](tbctr::W) writer structure"]
impl crate::Writable for TBCTR {}
#[doc = "Register current value of timer"]
pub mod tbctr;
#[doc = "Register maximum Timer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tbprd](tbprd) module"]
pub type TBPRD = crate::Reg<u32, _TBPRD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TBPRD;
#[doc = "`read()` method returns [tbprd::R](tbprd::R) reader structure"]
impl crate::Readable for TBPRD {}
#[doc = "`write(|w| ..)` method takes [tbprd::W](tbprd::W) writer structure"]
impl crate::Writable for TBPRD {}
#[doc = "Register maximum Timer"]
pub mod tbprd;
#[doc = "No description\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cmpctl](cmpctl) module"]
pub type CMPCTL = crate::Reg<u32, _CMPCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPCTL;
#[doc = "`read()` method returns [cmpctl::R](cmpctl::R) reader structure"]
impl crate::Readable for CMPCTL {}
#[doc = "`write(|w| ..)` method takes [cmpctl::W](cmpctl::W) writer structure"]
impl crate::Writable for CMPCTL {}
#[doc = "No description"]
pub mod cmpctl;
#[doc = "Register threshold A\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cmpa](cmpa) module"]
pub type CMPA = crate::Reg<u32, _CMPA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPA;
#[doc = "`read()` method returns [cmpa::R](cmpa::R) reader structure"]
impl crate::Readable for CMPA {}
#[doc = "`write(|w| ..)` method takes [cmpa::W](cmpa::W) writer structure"]
impl crate::Writable for CMPA {}
#[doc = "Register threshold A"]
pub mod cmpa;
#[doc = "Register threshold B\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cmpb](cmpb) module"]
pub type CMPB = crate::Reg<u32, _CMPB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPB;
#[doc = "`read()` method returns [cmpb::R](cmpb::R) reader structure"]
impl crate::Readable for CMPB {}
#[doc = "`write(|w| ..)` method takes [cmpb::W](cmpb::W) writer structure"]
impl crate::Writable for CMPB {}
#[doc = "Register threshold B"]
pub mod cmpb;
#[doc = "Register handlers for output A\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [aqctla](aqctla) module"]
pub type AQCTLA = crate::Reg<u32, _AQCTLA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AQCTLA;
#[doc = "`read()` method returns [aqctla::R](aqctla::R) reader structure"]
impl crate::Readable for AQCTLA {}
#[doc = "`write(|w| ..)` method takes [aqctla::W](aqctla::W) writer structure"]
impl crate::Writable for AQCTLA {}
#[doc = "Register handlers for output A"]
pub mod aqctla;
#[doc = "Register handlers for output B\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [aqctlb](aqctlb) module"]
pub type AQCTLB = crate::Reg<u32, _AQCTLB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AQCTLB;
#[doc = "`read()` method returns [aqctlb::R](aqctlb::R) reader structure"]
impl crate::Readable for AQCTLB {}
#[doc = "`write(|w| ..)` method takes [aqctlb::W](aqctlb::W) writer structure"]
impl crate::Writable for AQCTLB {}
#[doc = "Register handlers for output B"]
pub mod aqctlb;
#[doc = "Register handlers for a single program management\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [aqsfrc](aqsfrc) module"]
pub type AQSFRC = crate::Reg<u32, _AQSFRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AQSFRC;
#[doc = "`read()` method returns [aqsfrc::R](aqsfrc::R) reader structure"]
impl crate::Readable for AQSFRC {}
#[doc = "`write(|w| ..)` method takes [aqsfrc::W](aqsfrc::W) writer structure"]
impl crate::Writable for AQSFRC {}
#[doc = "Register handlers for a single program management"]
pub mod aqsfrc;
#[doc = "Register handlers for the cyclic program Management\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [aqcsfrc](aqcsfrc) module"]
pub type AQCSFRC = crate::Reg<u32, _AQCSFRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AQCSFRC;
#[doc = "`read()` method returns [aqcsfrc::R](aqcsfrc::R) reader structure"]
impl crate::Readable for AQCSFRC {}
#[doc = "`write(|w| ..)` method takes [aqcsfrc::W](aqcsfrc::W) writer structure"]
impl crate::Writable for AQCSFRC {}
#[doc = "Register handlers for the cyclic program Management"]
pub mod aqcsfrc;
#[doc = "Generator Control Register PWM Dead Time\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dbctl](dbctl) module"]
pub type DBCTL = crate::Reg<u32, _DBCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DBCTL;
#[doc = "`read()` method returns [dbctl::R](dbctl::R) reader structure"]
impl crate::Readable for DBCTL {}
#[doc = "`write(|w| ..)` method takes [dbctl::W](dbctl::W) writer structure"]
impl crate::Writable for DBCTL {}
#[doc = "Generator Control Register PWM Dead Time"]
pub mod dbctl;
#[doc = "Dead time control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dbred](dbred) module"]
pub type DBRED = crate::Reg<u32, _DBRED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DBRED;
#[doc = "`read()` method returns [dbred::R](dbred::R) reader structure"]
impl crate::Readable for DBRED {}
#[doc = "`write(|w| ..)` method takes [dbred::W](dbred::W) writer structure"]
impl crate::Writable for DBRED {}
#[doc = "Dead time control register"]
pub mod dbred;
#[doc = "Dead time control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dbfed](dbfed) module"]
pub type DBFED = crate::Reg<u32, _DBFED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DBFED;
#[doc = "`read()` method returns [dbfed::R](dbfed::R) reader structure"]
impl crate::Readable for DBFED {}
#[doc = "`write(|w| ..)` method takes [dbfed::W](dbfed::W) writer structure"]
impl crate::Writable for DBFED {}
#[doc = "Dead time control register"]
pub mod dbfed;
#[doc = "Register the source of the accident\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tzsel](tzsel) module"]
pub type TZSEL = crate::Reg<u32, _TZSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZSEL;
#[doc = "`read()` method returns [tzsel::R](tzsel::R) reader structure"]
impl crate::Readable for TZSEL {}
#[doc = "`write(|w| ..)` method takes [tzsel::W](tzsel::W) writer structure"]
impl crate::Writable for TZSEL {}
#[doc = "Register the source of the accident"]
pub mod tzsel;
#[doc = "Control Register detector alarm signal\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tzctl](tzctl) module"]
pub type TZCTL = crate::Reg<u32, _TZCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZCTL;
#[doc = "`read()` method returns [tzctl::R](tzctl::R) reader structure"]
impl crate::Readable for TZCTL {}
#[doc = "`write(|w| ..)` method takes [tzctl::W](tzctl::W) writer structure"]
impl crate::Writable for TZCTL {}
#[doc = "Control Register detector alarm signal"]
pub mod tzctl;
#[doc = "Interrupt mask register Signal Detection Alarms\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tzeint](tzeint) module"]
pub type TZEINT = crate::Reg<u32, _TZEINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZEINT;
#[doc = "`read()` method returns [tzeint::R](tzeint::R) reader structure"]
impl crate::Readable for TZEINT {}
#[doc = "`write(|w| ..)` method takes [tzeint::W](tzeint::W) writer structure"]
impl crate::Writable for TZEINT {}
#[doc = "Interrupt mask register Signal Detection Alarms"]
pub mod tzeint;
#[doc = "Flags register interrupt signal detector Accidents\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tzflg](tzflg) module"]
pub type TZFLG = crate::Reg<u32, _TZFLG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZFLG;
#[doc = "`read()` method returns [tzflg::R](tzflg::R) reader structure"]
impl crate::Readable for TZFLG {}
#[doc = "Flags register interrupt signal detector Accidents"]
pub mod tzflg;
#[doc = "Register reset interrupt flag detector alarm signal\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tzclr](tzclr) module"]
pub type TZCLR = crate::Reg<u32, _TZCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZCLR;
#[doc = "`read()` method returns [tzclr::R](tzclr::R) reader structure"]
impl crate::Readable for TZCLR {}
#[doc = "`write(|w| ..)` method takes [tzclr::W](tzclr::W) writer structure"]
impl crate::Writable for TZCLR {}
#[doc = "Register reset interrupt flag detector alarm signal"]
pub mod tzclr;
#[doc = "Register software emulation of faults\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tzfrc](tzfrc) module"]
pub type TZFRC = crate::Reg<u32, _TZFRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZFRC;
#[doc = "`read()` method returns [tzfrc::R](tzfrc::R) reader structure"]
impl crate::Readable for TZFRC {}
#[doc = "`write(|w| ..)` method takes [tzfrc::W](tzfrc::W) writer structure"]
impl crate::Writable for TZFRC {}
#[doc = "Register software emulation of faults"]
pub mod tzfrc;
#[doc = "A Source event trigger\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [etsel](etsel) module"]
pub type ETSEL = crate::Reg<u32, _ETSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETSEL;
#[doc = "`read()` method returns [etsel::R](etsel::R) reader structure"]
impl crate::Readable for ETSEL {}
#[doc = "`write(|w| ..)` method takes [etsel::W](etsel::W) writer structure"]
impl crate::Writable for ETSEL {}
#[doc = "A Source event trigger"]
pub mod etsel;
#[doc = "Prescaler register the event trigger\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [etps](etps) module"]
pub type ETPS = crate::Reg<u32, _ETPS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETPS;
#[doc = "`read()` method returns [etps::R](etps::R) reader structure"]
impl crate::Readable for ETPS {}
#[doc = "`write(|w| ..)` method takes [etps::W](etps::W) writer structure"]
impl crate::Writable for ETPS {}
#[doc = "Prescaler register the event trigger"]
pub mod etps;
#[doc = "Register Flags event trigger\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [etflg](etflg) module"]
pub type ETFLG = crate::Reg<u32, _ETFLG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETFLG;
#[doc = "`read()` method returns [etflg::R](etflg::R) reader structure"]
impl crate::Readable for ETFLG {}
#[doc = "Register Flags event trigger"]
pub mod etflg;
#[doc = "Register reset flags trigger events\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [etclr](etclr) module"]
pub type ETCLR = crate::Reg<u32, _ETCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETCLR;
#[doc = "`read()` method returns [etclr::R](etclr::R) reader structure"]
impl crate::Readable for ETCLR {}
#[doc = "`write(|w| ..)` method takes [etclr::W](etclr::W) writer structure"]
impl crate::Writable for ETCLR {}
#[doc = "Register reset flags trigger events"]
pub mod etclr;
#[doc = "Register software emulation events\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [etfrc](etfrc) module"]
pub type ETFRC = crate::Reg<u32, _ETFRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETFRC;
#[doc = "`read()` method returns [etfrc::R](etfrc::R) reader structure"]
impl crate::Readable for ETFRC {}
#[doc = "`write(|w| ..)` method takes [etfrc::W](etfrc::W) writer structure"]
impl crate::Writable for ETFRC {}
#[doc = "Register software emulation events"]
pub mod etfrc;
#[doc = "Control Register modulator\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcctl](pcctl) module"]
pub type PCCTL = crate::Reg<u32, _PCCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCCTL;
#[doc = "`read()` method returns [pcctl::R](pcctl::R) reader structure"]
impl crate::Readable for PCCTL {}
#[doc = "`write(|w| ..)` method takes [pcctl::W](pcctl::W) writer structure"]
impl crate::Writable for PCCTL {}
#[doc = "Control Register modulator"]
pub mod pcctl;
#[doc = "Register Configuration Block PWM High Definition\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hrcnfg](hrcnfg) module"]
pub type HRCNFG = crate::Reg<u32, _HRCNFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HRCNFG;
#[doc = "`read()` method returns [hrcnfg::R](hrcnfg::R) reader structure"]
impl crate::Readable for HRCNFG {}
#[doc = "`write(|w| ..)` method takes [hrcnfg::W](hrcnfg::W) writer structure"]
impl crate::Writable for HRCNFG {}
#[doc = "Register Configuration Block PWM High Definition"]
pub mod hrcnfg;
#[doc = "Register width filtering\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fwdth](fwdth) module"]
pub type FWDTH = crate::Reg<u32, _FWDTH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FWDTH;
#[doc = "`read()` method returns [fwdth::R](fwdth::R) reader structure"]
impl crate::Readable for FWDTH {}
#[doc = "`write(|w| ..)` method takes [fwdth::W](fwdth::W) writer structure"]
impl crate::Writable for FWDTH {}
#[doc = "Register width filtering"]
pub mod fwdth;
#[doc = "Register source event retention\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hdsel](hdsel) module"]
pub type HDSEL = crate::Reg<u32, _HDSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HDSEL;
#[doc = "`read()` method returns [hdsel::R](hdsel::R) reader structure"]
impl crate::Readable for HDSEL {}
#[doc = "`write(|w| ..)` method takes [hdsel::W](hdsel::W) writer structure"]
impl crate::Writable for HDSEL {}
#[doc = "Register source event retention"]
pub mod hdsel;
#[doc = "Control Register detector hold events\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hdctl](hdctl) module"]
pub type HDCTL = crate::Reg<u32, _HDCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HDCTL;
#[doc = "`read()` method returns [hdctl::R](hdctl::R) reader structure"]
impl crate::Readable for HDCTL {}
#[doc = "`write(|w| ..)` method takes [hdctl::W](hdctl::W) writer structure"]
impl crate::Writable for HDCTL {}
#[doc = "Control Register detector hold events"]
pub mod hdctl;
#[doc = "Register software activation threshold trigger\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hdeint](hdeint) module"]
pub type HDEINT = crate::Reg<u32, _HDEINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HDEINT;
#[doc = "`read()` method returns [hdeint::R](hdeint::R) reader structure"]
impl crate::Readable for HDEINT {}
#[doc = "`write(|w| ..)` method takes [hdeint::W](hdeint::W) writer structure"]
impl crate::Writable for HDEINT {}
#[doc = "Register software activation threshold trigger"]
pub mod hdeint;
#[doc = "Registrer HD flag interrupt\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hdflg](hdflg) module"]
pub type HDFLG = crate::Reg<u32, _HDFLG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HDFLG;
#[doc = "`read()` method returns [hdflg::R](hdflg::R) reader structure"]
impl crate::Readable for HDFLG {}
#[doc = "Registrer HD flag interrupt"]
pub mod hdflg;
#[doc = "Register clear HD flag\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hdclr](hdclr) module"]
pub type HDCLR = crate::Reg<u32, _HDCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HDCLR;
#[doc = "`read()` method returns [hdclr::R](hdclr::R) reader structure"]
impl crate::Readable for HDCLR {}
#[doc = "`write(|w| ..)` method takes [hdclr::W](hdclr::W) writer structure"]
impl crate::Writable for HDCLR {}
#[doc = "Register clear HD flag"]
pub mod hdclr;
#[doc = "Register software activation threshold trigger\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hdfrc](hdfrc) module"]
pub type HDFRC = crate::Reg<u32, _HDFRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HDFRC;
#[doc = "`read()` method returns [hdfrc::R](hdfrc::R) reader structure"]
impl crate::Readable for HDFRC {}
#[doc = "`write(|w| ..)` method takes [hdfrc::W](hdfrc::W) writer structure"]
impl crate::Writable for HDFRC {}
#[doc = "Register software activation threshold trigger"]
pub mod hdfrc;
#[doc = "Register clear HD interrupt\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hdintclr](hdintclr) module"]
pub type HDINTCLR = crate::Reg<u32, _HDINTCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HDINTCLR;
#[doc = "`write(|w| ..)` method takes [hdintclr::W](hdintclr::W) writer structure"]
impl crate::Writable for HDINTCLR {}
#[doc = "Register clear HD interrupt"]
pub mod hdintclr;
#[doc = "Register clear TZ interrupt\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tzintclr](tzintclr) module"]
pub type TZINTCLR = crate::Reg<u32, _TZINTCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZINTCLR;
#[doc = "`write(|w| ..)` method takes [tzintclr::W](tzintclr::W) writer structure"]
impl crate::Writable for TZINTCLR {}
#[doc = "Register clear TZ interrupt"]
pub mod tzintclr;
#[doc = "Register clear interrupt\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intclr](intclr) module"]
pub type INTCLR = crate::Reg<u32, _INTCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTCLR;
#[doc = "`write(|w| ..)` method takes [intclr::W](intclr::W) writer structure"]
impl crate::Writable for INTCLR {}
#[doc = "Register clear interrupt"]
pub mod intclr;
