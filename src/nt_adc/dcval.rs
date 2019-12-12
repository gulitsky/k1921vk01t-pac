#[doc = "Digital comparator measure value register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dcval](dcval) module"]
pub type DCVAL = crate::Reg<u32, _DCVAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCVAL;
#[doc = "`read()` method returns [dcval::R](dcval::R) reader structure"]
impl crate::Readable for DCVAL {}
#[doc = "Digital comparator measure value register"]
pub mod dcval;
