#[doc = "Register control the operation of the message object 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mofcr](mofcr) module"]
pub type MOFCR = crate::Reg<u32, _MOFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MOFCR;
#[doc = "`read()` method returns [mofcr::R](mofcr::R) reader structure"]
impl crate::Readable for MOFCR {}
#[doc = "`write(|w| ..)` method takes [mofcr::W](mofcr::W) writer structure"]
impl crate::Writable for MOFCR {}
#[doc = "Register control the operation of the message object 0"]
pub mod mofcr;
#[doc = "Pointer register FIFO / gateway message object 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mofgpr](mofgpr) module"]
pub type MOFGPR = crate::Reg<u32, _MOFGPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MOFGPR;
#[doc = "`read()` method returns [mofgpr::R](mofgpr::R) reader structure"]
impl crate::Readable for MOFGPR {}
#[doc = "`write(|w| ..)` method takes [mofgpr::W](mofgpr::W) writer structure"]
impl crate::Writable for MOFGPR {}
#[doc = "Pointer register FIFO / gateway message object 0"]
pub mod mofgpr;
#[doc = "Pointer register interrupt message object 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [moipr](moipr) module"]
pub type MOIPR = crate::Reg<u32, _MOIPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MOIPR;
#[doc = "`read()` method returns [moipr::R](moipr::R) reader structure"]
impl crate::Readable for MOIPR {}
#[doc = "`write(|w| ..)` method takes [moipr::W](moipr::W) writer structure"]
impl crate::Writable for MOIPR {}
#[doc = "Pointer register interrupt message object 0"]
pub mod moipr;
#[doc = "Mask register message object 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [moamr](moamr) module"]
pub type MOAMR = crate::Reg<u32, _MOAMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MOAMR;
#[doc = "`read()` method returns [moamr::R](moamr::R) reader structure"]
impl crate::Readable for MOAMR {}
#[doc = "`write(|w| ..)` method takes [moamr::W](moamr::W) writer structure"]
impl crate::Writable for MOAMR {}
#[doc = "Mask register message object 0"]
pub mod moamr;
#[doc = "Low data registers of the message object 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [modatal](modatal) module"]
pub type MODATAL = crate::Reg<u32, _MODATAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MODATAL;
#[doc = "`read()` method returns [modatal::R](modatal::R) reader structure"]
impl crate::Readable for MODATAL {}
#[doc = "`write(|w| ..)` method takes [modatal::W](modatal::W) writer structure"]
impl crate::Writable for MODATAL {}
#[doc = "Low data registers of the message object 0"]
pub mod modatal;
#[doc = "High data registers of the message object 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [modatah](modatah) module"]
pub type MODATAH = crate::Reg<u32, _MODATAH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MODATAH;
#[doc = "`read()` method returns [modatah::R](modatah::R) reader structure"]
impl crate::Readable for MODATAH {}
#[doc = "`write(|w| ..)` method takes [modatah::W](modatah::W) writer structure"]
impl crate::Writable for MODATAH {}
#[doc = "High data registers of the message object 0"]
pub mod modatah;
#[doc = "Register arbitration message object 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [moar](moar) module"]
pub type MOAR = crate::Reg<u32, _MOAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MOAR;
#[doc = "`read()` method returns [moar::R](moar::R) reader structure"]
impl crate::Readable for MOAR {}
#[doc = "`write(|w| ..)` method takes [moar::W](moar::W) writer structure"]
impl crate::Writable for MOAR {}
#[doc = "Register arbitration message object 0"]
pub mod moar;
#[doc = "Control register Message object 0\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [moctr](moctr) module"]
pub type MOCTR = crate::Reg<u32, _MOCTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MOCTR;
#[doc = "`write(|w| ..)` method takes [moctr::W](moctr::W) writer structure"]
impl crate::Writable for MOCTR {}
#[doc = "Control register Message object 0"]
pub mod moctr;
#[doc = "Status register of the message object 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mostat](mostat) module"]
pub type MOSTAT = crate::Reg<u32, _MOSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MOSTAT;
#[doc = "`read()` method returns [mostat::R](mostat::R) reader structure"]
impl crate::Readable for MOSTAT {}
#[doc = "Status register of the message object 0"]
pub mod mostat;
