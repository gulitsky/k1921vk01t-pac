#[doc = "Lower byte masked access register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [masklowbyte](masklowbyte) module"]
pub type MASKLOWBYTE = crate::Reg<u32, _MASKLOWBYTE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASKLOWBYTE;
#[doc = "`read()` method returns [masklowbyte::R](masklowbyte::R) reader structure"]
impl crate::Readable for MASKLOWBYTE {}
#[doc = "`write(|w| ..)` method takes [masklowbyte::W](masklowbyte::W) writer structure"]
impl crate::Writable for MASKLOWBYTE {}
#[doc = "Lower byte masked access register"]
pub mod masklowbyte;
