#[doc = "Reader of register ALTFUNCCLR"]
pub type R = crate::R<u32, super::ALTFUNCCLR>;
#[doc = "Writer for register ALTFUNCCLR"]
pub type W = crate::W<u32, super::ALTFUNCCLR>;
#[doc = "Register ALTFUNCCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::ALTFUNCCLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ALTFUNCCLR`"]
pub type ALTFUNCCLR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ALTFUNCCLR`"]
pub struct ALTFUNCCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> ALTFUNCCLR_W<'a> {
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
    pub fn altfuncclr(&self) -> ALTFUNCCLR_R {
        ALTFUNCCLR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn altfuncclr(&mut self) -> ALTFUNCCLR_W {
        ALTFUNCCLR_W { w: self }
    }
}
