#[doc = "Reader of register SECOND"]
pub type R = crate::R<u32, super::SECOND>;
#[doc = "Writer for register SECOND"]
pub type W = crate::W<u32, super::SECOND>;
#[doc = "Register SECOND `reset()`'s with value 0"]
impl crate::ResetValue for super::SECOND {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SECOND`"]
pub type SECOND_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SECOND`"]
pub struct SECOND_W<'a> {
    w: &'a mut W,
}
impl<'a> SECOND_W<'a> {
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
    pub fn second(&self) -> SECOND_R {
        SECOND_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn second(&mut self) -> SECOND_W {
        SECOND_W { w: self }
    }
}
