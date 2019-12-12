#[doc = "Reader of register INTPOLSET"]
pub type R = crate::R<u32, super::INTPOLSET>;
#[doc = "Writer for register INTPOLSET"]
pub type W = crate::W<u32, super::INTPOLSET>;
#[doc = "Register INTPOLSET `reset()`'s with value 0"]
impl crate::ResetValue for super::INTPOLSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INTPOLSET`"]
pub type INTPOLSET_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `INTPOLSET`"]
pub struct INTPOLSET_W<'a> {
    w: &'a mut W,
}
impl<'a> INTPOLSET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn intpolset(&self) -> INTPOLSET_R {
        INTPOLSET_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn intpolset(&mut self) -> INTPOLSET_W {
        INTPOLSET_W { w: self }
    }
}
