#[doc = "Higher byte masked access register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [maskhighbyte](maskhighbyte) module"]
pub type MASKHIGHBYTE = crate::Reg<u32, _MASKHIGHBYTE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASKHIGHBYTE;
#[doc = "`read()` method returns [maskhighbyte::R](maskhighbyte::R) reader structure"]
impl crate::Readable for MASKHIGHBYTE {}
#[doc = "`write(|w| ..)` method takes [maskhighbyte::W](maskhighbyte::W) writer structure"]
impl crate::Writable for MASKHIGHBYTE {}
#[doc = "Higher byte masked access register"]
pub mod maskhighbyte;
