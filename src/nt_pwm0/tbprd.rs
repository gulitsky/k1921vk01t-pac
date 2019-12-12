#[doc = "Reader of register TBPRD"]
pub type R = crate::R<u32, super::TBPRD>;
#[doc = "Writer for register TBPRD"]
pub type W = crate::W<u32, super::TBPRD>;
#[doc = "Register TBPRD `reset()`'s with value 0"]
impl crate::ResetValue for super::TBPRD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TBPRD`"]
pub type TBPRD_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TBPRD`"]
pub struct TBPRD_W<'a> {
    w: &'a mut W,
}
impl<'a> TBPRD_W<'a> {
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
    pub fn tbprd(&self) -> TBPRD_R {
        TBPRD_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn tbprd(&mut self) -> TBPRD_W {
        TBPRD_W { w: self }
    }
}
