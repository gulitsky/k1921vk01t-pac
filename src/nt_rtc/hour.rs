#[doc = "Reader of register HOUR"]
pub type R = crate::R<u32, super::HOUR>;
#[doc = "Writer for register HOUR"]
pub type W = crate::W<u32, super::HOUR>;
#[doc = "Register HOUR `reset()`'s with value 0"]
impl crate::ResetValue for super::HOUR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HOUR`"]
pub type HOUR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HOUR`"]
pub struct HOUR_W<'a> {
    w: &'a mut W,
}
impl<'a> HOUR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Hours in BCD format"]
    #[inline(always)]
    pub fn hour(&self) -> HOUR_R {
        HOUR_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Hours in BCD format"]
    #[inline(always)]
    pub fn hour(&mut self) -> HOUR_W {
        HOUR_W { w: self }
    }
}
