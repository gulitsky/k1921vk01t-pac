#[doc = "Reader of register QPOSINIT"]
pub type R = crate::R<u32, super::QPOSINIT>;
#[doc = "Writer for register QPOSINIT"]
pub type W = crate::W<u32, super::QPOSINIT>;
#[doc = "Register QPOSINIT `reset()`'s with value 0"]
impl crate::ResetValue for super::QPOSINIT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `QPOSINIT`"]
pub type QPOSINIT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `QPOSINIT`"]
pub struct QPOSINIT_W<'a> {
    w: &'a mut W,
}
impl<'a> QPOSINIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn qposinit(&self) -> QPOSINIT_R {
        QPOSINIT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn qposinit(&mut self) -> QPOSINIT_W {
        QPOSINIT_W { w: self }
    }
}
