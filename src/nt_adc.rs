#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Enable sequencer register"]
    pub actss: ACTSS,
    #[doc = "0x04 - Raw interrupt status register"]
    pub ris: RIS,
    #[doc = "0x08 - Interrupt mask register"]
    pub im: IM,
    #[doc = "0x0c - Interrupt status and clear register"]
    pub isc: ISC,
    #[doc = "0x10 - FIFO overflow status register"]
    pub ostat: OSTAT,
    #[doc = "0x14 - Sequencer start event selection register"]
    pub emux: EMUX,
    #[doc = "0x18 - FIFO underflow status register"]
    pub ustat: USTAT,
    _reserved7: [u8; 4usize],
    #[doc = "0x20 - ADC0, ADC1 start delay register"]
    pub spc0: SPC0,
    #[doc = "0x24 - ADC2, ADC3 start delay register"]
    pub spc1: SPC1,
    #[doc = "0x28 - ADC4, ADC5 start delay register"]
    pub spc2: SPC2,
    #[doc = "0x2c - ADC6, ADC7 start delay register"]
    pub spc3: SPC3,
    #[doc = "0x30 - ADC8, ADC9 start delay register"]
    pub spc4: SPC4,
    #[doc = "0x34 - ADC10, ADC11 start delay register"]
    pub spc5: SPC5,
    #[doc = "0x38 - Average control (ADC0-7) register"]
    pub sac: SAC,
    #[doc = "0x3c - Interrupt counter reset control / Average control (ADC8-11) register"]
    pub ricnt: RICNT,
    #[doc = "0x40 - SEQ"]
    pub seq: [SEQ; 8],
    #[doc = "0x140 - DCCTL"]
    pub dcctl: [DCCTL; 24],
    #[doc = "0x1a0 - DCCMP"]
    pub dccmp: [DCCMP; 24],
    #[doc = "0x200 - DCVAL"]
    pub dcval: [DCVAL; 24],
    _reserved19: [u8; 140usize],
    #[doc = "0x2ec - Digital comparator output trigger status register"]
    pub dcrtc: DCRTC,
    _reserved20: [u8; 16usize],
    #[doc = "0x300 - PP"]
    pub pp: [PP; 12],
    _reserved21: [u8; 200usize],
    #[doc = "0x3f8 - Sequencer sync register"]
    pub pssi: PSSI,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct SEQ {
    #[doc = "0x00 - Sequencer ADC channels selection register"]
    pub mux: self::seq::MUX,
    #[doc = "0x04 - Sequencer control register"]
    pub ctl: self::seq::CTL,
    _reserved2: [u8; 4usize],
    #[doc = "0x0c - Sequencer FIFO load status register"]
    pub fstat: self::seq::FSTAT,
    #[doc = "0x10 - Sequencer ADC restart counter register"]
    pub op: self::seq::OP,
    #[doc = "0x14 - Sequencer digital comparator selection register"]
    pub dcp: self::seq::DCP,
    #[doc = "0x18 - Sequencer ADC restart timer register"]
    pub tmr: self::seq::TMR,
    #[doc = "0x1c - Sequencer FIFO register"]
    pub fifo: self::seq::FIFO,
}
#[doc = r"Register block"]
#[doc = "SEQ"]
pub mod seq;
#[doc = r"Register block"]
#[repr(C)]
pub struct DCCTL {
    #[doc = "0x00 - Digital comparator control register"]
    pub dcctl: self::dcctl::DCCTL,
}
#[doc = r"Register block"]
#[doc = "DCCTL"]
pub mod dcctl;
#[doc = r"Register block"]
#[repr(C)]
pub struct DCCMP {
    #[doc = "0x00 - Digital comparator range register"]
    pub dccmp: self::dccmp::DCCMP,
}
#[doc = r"Register block"]
#[doc = "DCCMP"]
pub mod dccmp;
#[doc = r"Register block"]
#[repr(C)]
pub struct DCVAL {
    #[doc = "0x00 - Digital comparator measure value register"]
    pub dcval: self::dcval::DCVAL,
}
#[doc = r"Register block"]
#[doc = "DCVAL"]
pub mod dcval;
#[doc = r"Register block"]
#[repr(C)]
pub struct PP {
    #[doc = "0x00 - ADC control register"]
    pub pp: self::pp::PP,
}
#[doc = r"Register block"]
#[doc = "PP"]
pub mod pp;
#[doc = "Enable sequencer register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [actss](actss) module"]
pub type ACTSS = crate::Reg<u32, _ACTSS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACTSS;
#[doc = "`read()` method returns [actss::R](actss::R) reader structure"]
impl crate::Readable for ACTSS {}
#[doc = "`write(|w| ..)` method takes [actss::W](actss::W) writer structure"]
impl crate::Writable for ACTSS {}
#[doc = "Enable sequencer register"]
pub mod actss;
#[doc = "Raw interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ris](ris) module"]
pub type RIS = crate::Reg<u32, _RIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RIS;
#[doc = "`read()` method returns [ris::R](ris::R) reader structure"]
impl crate::Readable for RIS {}
#[doc = "Raw interrupt status register"]
pub mod ris;
#[doc = "Interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [im](im) module"]
pub type IM = crate::Reg<u32, _IM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IM;
#[doc = "`read()` method returns [im::R](im::R) reader structure"]
impl crate::Readable for IM {}
#[doc = "`write(|w| ..)` method takes [im::W](im::W) writer structure"]
impl crate::Writable for IM {}
#[doc = "Interrupt mask register"]
pub mod im;
#[doc = "Interrupt status and clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [isc](isc) module"]
pub type ISC = crate::Reg<u32, _ISC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISC;
#[doc = "`read()` method returns [isc::R](isc::R) reader structure"]
impl crate::Readable for ISC {}
#[doc = "`write(|w| ..)` method takes [isc::W](isc::W) writer structure"]
impl crate::Writable for ISC {}
#[doc = "Interrupt status and clear register"]
pub mod isc;
#[doc = "FIFO overflow status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ostat](ostat) module"]
pub type OSTAT = crate::Reg<u32, _OSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OSTAT;
#[doc = "`read()` method returns [ostat::R](ostat::R) reader structure"]
impl crate::Readable for OSTAT {}
#[doc = "`write(|w| ..)` method takes [ostat::W](ostat::W) writer structure"]
impl crate::Writable for OSTAT {}
#[doc = "FIFO overflow status register"]
pub mod ostat;
#[doc = "Sequencer start event selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [emux](emux) module"]
pub type EMUX = crate::Reg<u32, _EMUX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EMUX;
#[doc = "`read()` method returns [emux::R](emux::R) reader structure"]
impl crate::Readable for EMUX {}
#[doc = "`write(|w| ..)` method takes [emux::W](emux::W) writer structure"]
impl crate::Writable for EMUX {}
#[doc = "Sequencer start event selection register"]
pub mod emux;
#[doc = "FIFO underflow status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ustat](ustat) module"]
pub type USTAT = crate::Reg<u32, _USTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USTAT;
#[doc = "`read()` method returns [ustat::R](ustat::R) reader structure"]
impl crate::Readable for USTAT {}
#[doc = "`write(|w| ..)` method takes [ustat::W](ustat::W) writer structure"]
impl crate::Writable for USTAT {}
#[doc = "FIFO underflow status register"]
pub mod ustat;
#[doc = "ADC0, ADC1 start delay register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spc0](spc0) module"]
pub type SPC0 = crate::Reg<u32, _SPC0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPC0;
#[doc = "`read()` method returns [spc0::R](spc0::R) reader structure"]
impl crate::Readable for SPC0 {}
#[doc = "`write(|w| ..)` method takes [spc0::W](spc0::W) writer structure"]
impl crate::Writable for SPC0 {}
#[doc = "ADC0, ADC1 start delay register"]
pub mod spc0;
#[doc = "ADC2, ADC3 start delay register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spc1](spc1) module"]
pub type SPC1 = crate::Reg<u32, _SPC1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPC1;
#[doc = "`read()` method returns [spc1::R](spc1::R) reader structure"]
impl crate::Readable for SPC1 {}
#[doc = "`write(|w| ..)` method takes [spc1::W](spc1::W) writer structure"]
impl crate::Writable for SPC1 {}
#[doc = "ADC2, ADC3 start delay register"]
pub mod spc1;
#[doc = "ADC4, ADC5 start delay register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spc2](spc2) module"]
pub type SPC2 = crate::Reg<u32, _SPC2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPC2;
#[doc = "`read()` method returns [spc2::R](spc2::R) reader structure"]
impl crate::Readable for SPC2 {}
#[doc = "`write(|w| ..)` method takes [spc2::W](spc2::W) writer structure"]
impl crate::Writable for SPC2 {}
#[doc = "ADC4, ADC5 start delay register"]
pub mod spc2;
#[doc = "ADC6, ADC7 start delay register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spc3](spc3) module"]
pub type SPC3 = crate::Reg<u32, _SPC3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPC3;
#[doc = "`read()` method returns [spc3::R](spc3::R) reader structure"]
impl crate::Readable for SPC3 {}
#[doc = "`write(|w| ..)` method takes [spc3::W](spc3::W) writer structure"]
impl crate::Writable for SPC3 {}
#[doc = "ADC6, ADC7 start delay register"]
pub mod spc3;
#[doc = "ADC8, ADC9 start delay register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spc4](spc4) module"]
pub type SPC4 = crate::Reg<u32, _SPC4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPC4;
#[doc = "`read()` method returns [spc4::R](spc4::R) reader structure"]
impl crate::Readable for SPC4 {}
#[doc = "`write(|w| ..)` method takes [spc4::W](spc4::W) writer structure"]
impl crate::Writable for SPC4 {}
#[doc = "ADC8, ADC9 start delay register"]
pub mod spc4;
#[doc = "ADC10, ADC11 start delay register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spc5](spc5) module"]
pub type SPC5 = crate::Reg<u32, _SPC5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPC5;
#[doc = "`read()` method returns [spc5::R](spc5::R) reader structure"]
impl crate::Readable for SPC5 {}
#[doc = "`write(|w| ..)` method takes [spc5::W](spc5::W) writer structure"]
impl crate::Writable for SPC5 {}
#[doc = "ADC10, ADC11 start delay register"]
pub mod spc5;
#[doc = "Average control (ADC0-7) register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sac](sac) module"]
pub type SAC = crate::Reg<u32, _SAC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAC;
#[doc = "`read()` method returns [sac::R](sac::R) reader structure"]
impl crate::Readable for SAC {}
#[doc = "`write(|w| ..)` method takes [sac::W](sac::W) writer structure"]
impl crate::Writable for SAC {}
#[doc = "Average control (ADC0-7) register"]
pub mod sac;
#[doc = "Interrupt counter reset control / Average control (ADC8-11) register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ricnt](ricnt) module"]
pub type RICNT = crate::Reg<u32, _RICNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RICNT;
#[doc = "`read()` method returns [ricnt::R](ricnt::R) reader structure"]
impl crate::Readable for RICNT {}
#[doc = "`write(|w| ..)` method takes [ricnt::W](ricnt::W) writer structure"]
impl crate::Writable for RICNT {}
#[doc = "Interrupt counter reset control / Average control (ADC8-11) register"]
pub mod ricnt;
#[doc = "Digital comparator output trigger status register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dcrtc](dcrtc) module"]
pub type DCRTC = crate::Reg<u32, _DCRTC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCRTC;
#[doc = "`write(|w| ..)` method takes [dcrtc::W](dcrtc::W) writer structure"]
impl crate::Writable for DCRTC {}
#[doc = "Digital comparator output trigger status register"]
pub mod dcrtc;
#[doc = "Sequencer sync register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pssi](pssi) module"]
pub type PSSI = crate::Reg<u32, _PSSI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PSSI;
#[doc = "`read()` method returns [pssi::R](pssi::R) reader structure"]
impl crate::Readable for PSSI {}
#[doc = "`write(|w| ..)` method takes [pssi::W](pssi::W) writer structure"]
impl crate::Writable for PSSI {}
#[doc = "Sequencer sync register"]
pub mod pssi;
