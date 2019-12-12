#[doc = "Sequencer ADC channels selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mux](mux) module"]
pub type MUX = crate::Reg<u32, _MUX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MUX;
#[doc = "`read()` method returns [mux::R](mux::R) reader structure"]
impl crate::Readable for MUX {}
#[doc = "`write(|w| ..)` method takes [mux::W](mux::W) writer structure"]
impl crate::Writable for MUX {}
#[doc = "Sequencer ADC channels selection register"]
pub mod mux;
#[doc = "Sequencer control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctl](ctl) module"]
pub type CTL = crate::Reg<u32, _CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTL;
#[doc = "`read()` method returns [ctl::R](ctl::R) reader structure"]
impl crate::Readable for CTL {}
#[doc = "`write(|w| ..)` method takes [ctl::W](ctl::W) writer structure"]
impl crate::Writable for CTL {}
#[doc = "Sequencer control register"]
pub mod ctl;
#[doc = "Sequencer FIFO load status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fstat](fstat) module"]
pub type FSTAT = crate::Reg<u32, _FSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FSTAT;
#[doc = "`read()` method returns [fstat::R](fstat::R) reader structure"]
impl crate::Readable for FSTAT {}
#[doc = "Sequencer FIFO load status register"]
pub mod fstat;
#[doc = "Sequencer ADC restart counter register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [op](op) module"]
pub type OP = crate::Reg<u32, _OP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OP;
#[doc = "`read()` method returns [op::R](op::R) reader structure"]
impl crate::Readable for OP {}
#[doc = "`write(|w| ..)` method takes [op::W](op::W) writer structure"]
impl crate::Writable for OP {}
#[doc = "Sequencer ADC restart counter register"]
pub mod op;
#[doc = "Sequencer digital comparator selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dcp](dcp) module"]
pub type DCP = crate::Reg<u32, _DCP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCP;
#[doc = "`read()` method returns [dcp::R](dcp::R) reader structure"]
impl crate::Readable for DCP {}
#[doc = "`write(|w| ..)` method takes [dcp::W](dcp::W) writer structure"]
impl crate::Writable for DCP {}
#[doc = "Sequencer digital comparator selection register"]
pub mod dcp;
#[doc = "Sequencer ADC restart timer register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tmr](tmr) module"]
pub type TMR = crate::Reg<u32, _TMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TMR;
#[doc = "`read()` method returns [tmr::R](tmr::R) reader structure"]
impl crate::Readable for TMR {}
#[doc = "`write(|w| ..)` method takes [tmr::W](tmr::W) writer structure"]
impl crate::Writable for TMR {}
#[doc = "Sequencer ADC restart timer register"]
pub mod tmr;
#[doc = "Sequencer FIFO register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fifo](fifo) module"]
pub type FIFO = crate::Reg<u32, _FIFO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFO;
#[doc = "`read()` method returns [fifo::R](fifo::R) reader structure"]
impl crate::Readable for FIFO {}
#[doc = "Sequencer FIFO register"]
pub mod fifo;
