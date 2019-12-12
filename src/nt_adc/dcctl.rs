#[doc = "Digital comparator control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dcctl](dcctl) module"]
pub type DCCTL = crate::Reg<u32, _DCCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCCTL;
#[doc = "`read()` method returns [dcctl::R](dcctl::R) reader structure"]
impl crate::Readable for DCCTL {}
#[doc = "`write(|w| ..)` method takes [dcctl::W](dcctl::W) writer structure"]
impl crate::Writable for DCCTL {}
#[doc = "Digital comparator control register"]
pub mod dcctl;
