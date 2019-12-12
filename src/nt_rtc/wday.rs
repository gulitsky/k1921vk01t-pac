#[doc = "Reader of register WDAY"]
pub type R = crate::R<u32, super::WDAY>;
#[doc = "Writer for register WDAY"]
pub type W = crate::W<u32, super::WDAY>;
#[doc = "Register WDAY `reset()`'s with value 0"]
impl crate::ResetValue for super::WDAY {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DAYWEEK`"]
pub type DAYWEEK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DAYWEEK`"]
pub struct DAYWEEK_W<'a> {
    w: &'a mut W,
}
impl<'a> DAYWEEK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn dayweek(&self) -> DAYWEEK_R {
        DAYWEEK_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn dayweek(&mut self) -> DAYWEEK_W {
        DAYWEEK_W { w: self }
    }
}
