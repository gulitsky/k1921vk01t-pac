#[doc = "Digital comparator range register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dccmp](dccmp) module"]
pub type DCCMP = crate::Reg<u32, _DCCMP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCCMP;
#[doc = "`read()` method returns [dccmp::R](dccmp::R) reader structure"]
impl crate::Readable for DCCMP {}
#[doc = "`write(|w| ..)` method takes [dccmp::W](dccmp::W) writer structure"]
impl crate::Writable for DCCMP {}
#[doc = "Digital comparator range register"]
pub mod dccmp;
