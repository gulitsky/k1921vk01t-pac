#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Position count register"]
    pub qposcnt: QPOSCNT,
    #[doc = "0x04 - Shadow count register"]
    pub qposinit: QPOSINIT,
    #[doc = "0x08 - Period register"]
    pub qposmax: QPOSMAX,
    #[doc = "0x0c - Count compare position reg"]
    pub qposcmp: QPOSCMP,
    #[doc = "0x10 - Register meaning count position"]
    pub qposilat: QPOSILAT,
    #[doc = "0x14 - Strobe register"]
    pub qposslat: QPOSSLAT,
    #[doc = "0x18 - Time event register"]
    pub qposlat: QPOSLAT,
    #[doc = "0x1c - Time reading timer register"]
    pub qutmr: QUTMR,
    #[doc = "0x20 - Period time reading timer register"]
    pub quprd: QUPRD,
    #[doc = "0x24 - Watchdog timer register"]
    pub qwdtmr: QWDTMR,
    #[doc = "0x28 - Period watchdog timer register"]
    pub qwdprd: QWDPRD,
    #[doc = "0x2c - Input control register"]
    pub qdecctl: QDECCTL,
    #[doc = "0x30 - Quadrature decoder control register"]
    pub qepctl: QEPCTL,
    #[doc = "0x34 - Capture register"]
    pub qcapctl: QCAPCTL,
    #[doc = "0x38 - Register countposition control"]
    pub qposctl: QPOSCTL,
    #[doc = "0x3c - Interrupt mask"]
    pub qeint: QEINT,
    #[doc = "0x40 - Flag interrupt"]
    pub qflg: QFLG,
    #[doc = "0x44 - Reset interrupt"]
    pub qclr: QCLR,
    #[doc = "0x48 - Emulation interrupt"]
    pub qfrc: QFRC,
    #[doc = "0x4c - Status interrupt"]
    pub qepsts: QEPSTS,
    #[doc = "0x50 - Timer register"]
    pub qctmr: QCTMR,
    #[doc = "0x54 - Period register"]
    pub qcprd: QCPRD,
    #[doc = "0x58 - Register Storage Timer"]
    pub qctmrlat: QCTMRLAT,
    #[doc = "0x5c - Register Storage Period"]
    pub qcprdlat: QCPRDLAT,
    _reserved24: [u8; 16usize],
    #[doc = "0x70 - Clear interrupt register"]
    pub intclr: INTCLR,
}
#[doc = "Position count register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [qposcnt](qposcnt) module"]
pub type QPOSCNT = crate::Reg<u32, _QPOSCNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QPOSCNT;
#[doc = "`read()` method returns [qposcnt::R](qposcnt::R) reader structure"]
impl crate::Readable for QPOSCNT {}
#[doc = "`write(|w| ..)` method takes [qposcnt::W](qposcnt::W) writer structure"]
impl crate::Writable for QPOSCNT {}
#[doc = "Position count register"]
pub mod qposcnt;
#[doc = "Shadow count register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [qposinit](qposinit) module"]
pub type QPOSINIT = crate::Reg<u32, _QPOSINIT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QPOSINIT;
#[doc = "`read()` method returns [qposinit::R](qposinit::R) reader structure"]
impl crate::Readable for QPOSINIT {}
#[doc = "`write(|w| ..)` method takes [qposinit::W](qposinit::W) writer structure"]
impl crate::Writable for QPOSINIT {}
#[doc = "Shadow count register"]
pub mod qposinit;
#[doc = "Period register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [qposmax](qposmax) module"]
pub type QPOSMAX = crate::Reg<u32, _QPOSMAX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QPOSMAX;
#[doc = "`read()` method returns [qposmax::R](qposmax::R) reader structure"]
impl crate::Readable for QPOSMAX {}
#[doc = "`write(|w| ..)` method takes [qposmax::W](qposmax::W) writer structure"]
impl crate::Writable for QPOSMAX {}
#[doc = "Period register"]
pub mod qposmax;
#[doc = "Count compare position reg\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [qposcmp](qposcmp) module"]
pub type QPOSCMP = crate::Reg<u32, _QPOSCMP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QPOSCMP;
#[doc = "`read()` method returns [qposcmp::R](qposcmp::R) reader structure"]
impl crate::Readable for QPOSCMP {}
#[doc = "`write(|w| ..)` method takes [qposcmp::W](qposcmp::W) writer structure"]
impl crate::Writable for QPOSCMP {}
#[doc = "Count compare position reg"]
pub mod qposcmp;
#[doc = "Register meaning count position\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [qposilat](qposilat) module"]
pub type QPOSILAT = crate::Reg<u32, _QPOSILAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QPOSILAT;
#[doc = "`read()` method returns [qposilat::R](qposilat::R) reader structure"]
impl crate::Readable for QPOSILAT {}
#[doc = "Register meaning count position"]
pub mod qposilat;
#[doc = "Strobe register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [qposslat](qposslat) module"]
pub type QPOSSLAT = crate::Reg<u32, _QPOSSLAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QPOSSLAT;
#[doc = "`read()` method returns [qposslat::R](qposslat::R) reader structure"]
impl crate::Readable for QPOSSLAT {}
#[doc = "Strobe register"]
pub mod qposslat;
#[doc = "Time event register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [qposlat](qposlat) module"]
pub type QPOSLAT = crate::Reg<u32, _QPOSLAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QPOSLAT;
#[doc = "`read()` method returns [qposlat::R](qposlat::R) reader structure"]
impl crate::Readable for QPOSLAT {}
#[doc = "Time event register"]
pub mod qposlat;
#[doc = "Time reading timer register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [qutmr](qutmr) module"]
pub type QUTMR = crate::Reg<u32, _QUTMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QUTMR;
#[doc = "`read()` method returns [qutmr::R](qutmr::R) reader structure"]
impl crate::Readable for QUTMR {}
#[doc = "`write(|w| ..)` method takes [qutmr::W](qutmr::W) writer structure"]
impl crate::Writable for QUTMR {}
#[doc = "Time reading timer register"]
pub mod qutmr;
#[doc = "Period time reading timer register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [quprd](quprd) module"]
pub type QUPRD = crate::Reg<u32, _QUPRD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QUPRD;
#[doc = "`read()` method returns [quprd::R](quprd::R) reader structure"]
impl crate::Readable for QUPRD {}
#[doc = "`write(|w| ..)` method takes [quprd::W](quprd::W) writer structure"]
impl crate::Writable for QUPRD {}
#[doc = "Period time reading timer register"]
pub mod quprd;
#[doc = "Watchdog timer register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [qwdtmr](qwdtmr) module"]
pub type QWDTMR = crate::Reg<u32, _QWDTMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QWDTMR;
#[doc = "`read()` method returns [qwdtmr::R](qwdtmr::R) reader structure"]
impl crate::Readable for QWDTMR {}
#[doc = "`write(|w| ..)` method takes [qwdtmr::W](qwdtmr::W) writer structure"]
impl crate::Writable for QWDTMR {}
#[doc = "Watchdog timer register"]
pub mod qwdtmr;
#[doc = "Period watchdog timer register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [qwdprd](qwdprd) module"]
pub type QWDPRD = crate::Reg<u32, _QWDPRD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QWDPRD;
#[doc = "`read()` method returns [qwdprd::R](qwdprd::R) reader structure"]
impl crate::Readable for QWDPRD {}
#[doc = "`write(|w| ..)` method takes [qwdprd::W](qwdprd::W) writer structure"]
impl crate::Writable for QWDPRD {}
#[doc = "Period watchdog timer register"]
pub mod qwdprd;
#[doc = "Input control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [qdecctl](qdecctl) module"]
pub type QDECCTL = crate::Reg<u32, _QDECCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QDECCTL;
#[doc = "`read()` method returns [qdecctl::R](qdecctl::R) reader structure"]
impl crate::Readable for QDECCTL {}
#[doc = "`write(|w| ..)` method takes [qdecctl::W](qdecctl::W) writer structure"]
impl crate::Writable for QDECCTL {}
#[doc = "Input control register"]
pub mod qdecctl;
#[doc = "Quadrature decoder control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [qepctl](qepctl) module"]
pub type QEPCTL = crate::Reg<u32, _QEPCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QEPCTL;
#[doc = "`read()` method returns [qepctl::R](qepctl::R) reader structure"]
impl crate::Readable for QEPCTL {}
#[doc = "`write(|w| ..)` method takes [qepctl::W](qepctl::W) writer structure"]
impl crate::Writable for QEPCTL {}
#[doc = "Quadrature decoder control register"]
pub mod qepctl;
#[doc = "Capture register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [qcapctl](qcapctl) module"]
pub type QCAPCTL = crate::Reg<u32, _QCAPCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QCAPCTL;
#[doc = "`read()` method returns [qcapctl::R](qcapctl::R) reader structure"]
impl crate::Readable for QCAPCTL {}
#[doc = "`write(|w| ..)` method takes [qcapctl::W](qcapctl::W) writer structure"]
impl crate::Writable for QCAPCTL {}
#[doc = "Capture register"]
pub mod qcapctl;
#[doc = "Register countposition control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [qposctl](qposctl) module"]
pub type QPOSCTL = crate::Reg<u32, _QPOSCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QPOSCTL;
#[doc = "`read()` method returns [qposctl::R](qposctl::R) reader structure"]
impl crate::Readable for QPOSCTL {}
#[doc = "`write(|w| ..)` method takes [qposctl::W](qposctl::W) writer structure"]
impl crate::Writable for QPOSCTL {}
#[doc = "Register countposition control"]
pub mod qposctl;
#[doc = "Interrupt mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [qeint](qeint) module"]
pub type QEINT = crate::Reg<u32, _QEINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QEINT;
#[doc = "`read()` method returns [qeint::R](qeint::R) reader structure"]
impl crate::Readable for QEINT {}
#[doc = "`write(|w| ..)` method takes [qeint::W](qeint::W) writer structure"]
impl crate::Writable for QEINT {}
#[doc = "Interrupt mask"]
pub mod qeint;
#[doc = "Flag interrupt\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [qflg](qflg) module"]
pub type QFLG = crate::Reg<u32, _QFLG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QFLG;
#[doc = "`read()` method returns [qflg::R](qflg::R) reader structure"]
impl crate::Readable for QFLG {}
#[doc = "Flag interrupt"]
pub mod qflg;
#[doc = "Reset interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [qclr](qclr) module"]
pub type QCLR = crate::Reg<u32, _QCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QCLR;
#[doc = "`read()` method returns [qclr::R](qclr::R) reader structure"]
impl crate::Readable for QCLR {}
#[doc = "`write(|w| ..)` method takes [qclr::W](qclr::W) writer structure"]
impl crate::Writable for QCLR {}
#[doc = "Reset interrupt"]
pub mod qclr;
#[doc = "Emulation interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [qfrc](qfrc) module"]
pub type QFRC = crate::Reg<u32, _QFRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QFRC;
#[doc = "`read()` method returns [qfrc::R](qfrc::R) reader structure"]
impl crate::Readable for QFRC {}
#[doc = "`write(|w| ..)` method takes [qfrc::W](qfrc::W) writer structure"]
impl crate::Writable for QFRC {}
#[doc = "Emulation interrupt"]
pub mod qfrc;
#[doc = "Status interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [qepsts](qepsts) module"]
pub type QEPSTS = crate::Reg<u32, _QEPSTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QEPSTS;
#[doc = "`read()` method returns [qepsts::R](qepsts::R) reader structure"]
impl crate::Readable for QEPSTS {}
#[doc = "`write(|w| ..)` method takes [qepsts::W](qepsts::W) writer structure"]
impl crate::Writable for QEPSTS {}
#[doc = "Status interrupt"]
pub mod qepsts;
#[doc = "Timer register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [qctmr](qctmr) module"]
pub type QCTMR = crate::Reg<u32, _QCTMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QCTMR;
#[doc = "`read()` method returns [qctmr::R](qctmr::R) reader structure"]
impl crate::Readable for QCTMR {}
#[doc = "`write(|w| ..)` method takes [qctmr::W](qctmr::W) writer structure"]
impl crate::Writable for QCTMR {}
#[doc = "Timer register"]
pub mod qctmr;
#[doc = "Period register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [qcprd](qcprd) module"]
pub type QCPRD = crate::Reg<u32, _QCPRD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QCPRD;
#[doc = "`read()` method returns [qcprd::R](qcprd::R) reader structure"]
impl crate::Readable for QCPRD {}
#[doc = "`write(|w| ..)` method takes [qcprd::W](qcprd::W) writer structure"]
impl crate::Writable for QCPRD {}
#[doc = "Period register"]
pub mod qcprd;
#[doc = "Register Storage Timer\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [qctmrlat](qctmrlat) module"]
pub type QCTMRLAT = crate::Reg<u32, _QCTMRLAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QCTMRLAT;
#[doc = "`read()` method returns [qctmrlat::R](qctmrlat::R) reader structure"]
impl crate::Readable for QCTMRLAT {}
#[doc = "Register Storage Timer"]
pub mod qctmrlat;
#[doc = "Register Storage Period\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [qcprdlat](qcprdlat) module"]
pub type QCPRDLAT = crate::Reg<u32, _QCPRDLAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QCPRDLAT;
#[doc = "`read()` method returns [qcprdlat::R](qcprdlat::R) reader structure"]
impl crate::Readable for QCPRDLAT {}
#[doc = "`write(|w| ..)` method takes [qcprdlat::W](qcprdlat::W) writer structure"]
impl crate::Writable for QCPRDLAT {}
#[doc = "Register Storage Period"]
pub mod qcprdlat;
#[doc = "Clear interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intclr](intclr) module"]
pub type INTCLR = crate::Reg<u32, _INTCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTCLR;
#[doc = "`read()` method returns [intclr::R](intclr::R) reader structure"]
impl crate::Readable for INTCLR {}
#[doc = "`write(|w| ..)` method takes [intclr::W](intclr::W) writer structure"]
impl crate::Writable for INTCLR {}
#[doc = "Clear interrupt register"]
pub mod intclr;
