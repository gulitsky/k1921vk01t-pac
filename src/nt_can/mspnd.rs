#[doc = "Register waiting interrupts0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mspnd](mspnd) module"]
pub type MSPND = crate::Reg<u32, _MSPND>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MSPND;
#[doc = "`read()` method returns [mspnd::R](mspnd::R) reader structure"]
impl crate::Readable for MSPND {}
#[doc = "`write(|w| ..)` method takes [mspnd::W](mspnd::W) writer structure"]
impl crate::Writable for MSPND {}
#[doc = "Register waiting interrupts0"]
pub mod mspnd;
