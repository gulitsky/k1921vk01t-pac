#[doc = "Reader of register OUTENSET"]
pub type R = crate::R<u32, super::OUTENSET>;
#[doc = "Writer for register OUTENSET"]
pub type W = crate::W<u32, super::OUTENSET>;
#[doc = "Register OUTENSET `reset()`'s with value 0"]
impl crate::ResetValue for super::OUTENSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OUTENSET`"]
pub type OUTENSET_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `OUTENSET`"]
pub struct OUTENSET_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTENSET_W<'a> {
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
    pub fn outenset(&self) -> OUTENSET_R {
        OUTENSET_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn outenset(&mut self) -> OUTENSET_W {
        OUTENSET_W { w: self }
    }
}
