#[doc = "Reader of register TBCTR"]
pub type R = crate::R<u32, super::TBCTR>;
#[doc = "Writer for register TBCTR"]
pub type W = crate::W<u32, super::TBCTR>;
#[doc = "Register TBCTR `reset()`'s with value 0"]
impl crate::ResetValue for super::TBCTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TBCTR`"]
pub type TBCTR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TBCTR`"]
pub struct TBCTR_W<'a> {
    w: &'a mut W,
}
impl<'a> TBCTR_W<'a> {
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
    pub fn tbctr(&self) -> TBCTR_R {
        TBCTR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn tbctr(&mut self) -> TBCTR_W {
        TBCTR_W { w: self }
    }
}
