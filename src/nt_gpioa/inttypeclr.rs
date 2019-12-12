#[doc = "Reader of register INTTYPECLR"]
pub type R = crate::R<u32, super::INTTYPECLR>;
#[doc = "Writer for register INTTYPECLR"]
pub type W = crate::W<u32, super::INTTYPECLR>;
#[doc = "Register INTTYPECLR `reset()`'s with value 0"]
impl crate::ResetValue for super::INTTYPECLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INTTYPECLR`"]
pub type INTTYPECLR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `INTTYPECLR`"]
pub struct INTTYPECLR_W<'a> {
    w: &'a mut W,
}
impl<'a> INTTYPECLR_W<'a> {
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
    pub fn inttypeclr(&self) -> INTTYPECLR_R {
        INTTYPECLR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn inttypeclr(&mut self) -> INTTYPECLR_W {
        INTTYPECLR_W { w: self }
    }
}
