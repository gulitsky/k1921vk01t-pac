#[doc = "Reader of register MINUTE"]
pub type R = crate::R<u32, super::MINUTE>;
#[doc = "Writer for register MINUTE"]
pub type W = crate::W<u32, super::MINUTE>;
#[doc = "Register MINUTE `reset()`'s with value 0"]
impl crate::ResetValue for super::MINUTE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MINUTE`"]
pub type MINUTE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MINUTE`"]
pub struct MINUTE_W<'a> {
    w: &'a mut W,
}
impl<'a> MINUTE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn minute(&self) -> MINUTE_R {
        MINUTE_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn minute(&mut self) -> MINUTE_W {
        MINUTE_W { w: self }
    }
}
