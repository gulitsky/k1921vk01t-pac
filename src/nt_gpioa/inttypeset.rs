#[doc = "Reader of register INTTYPESET"]
pub type R = crate::R<u32, super::INTTYPESET>;
#[doc = "Writer for register INTTYPESET"]
pub type W = crate::W<u32, super::INTTYPESET>;
#[doc = "Register INTTYPESET `reset()`'s with value 0"]
impl crate::ResetValue for super::INTTYPESET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INTTYPESET`"]
pub type INTTYPESET_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `INTTYPESET`"]
pub struct INTTYPESET_W<'a> {
    w: &'a mut W,
}
impl<'a> INTTYPESET_W<'a> {
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
    pub fn inttypeset(&self) -> INTTYPESET_R {
        INTTYPESET_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn inttypeset(&mut self) -> INTTYPESET_W {
        INTTYPESET_W { w: self }
    }
}
