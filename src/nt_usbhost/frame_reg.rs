#[doc = "Reader of register FRAME_REG"]
pub type R = crate::R<u32, super::FRAME_REG>;
#[doc = "Writer for register FRAME_REG"]
pub type W = crate::W<u32, super::FRAME_REG>;
#[doc = "Register FRAME_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::FRAME_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FRAMEINDEX`"]
pub type FRAMEINDEX_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `FRAMEINDEX`"]
pub struct FRAMEINDEX_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAMEINDEX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | ((value as u32) & 0x3fff);
        self.w
    }
}
#[doc = "Reader of field `FRAMELENGTH`"]
pub type FRAMELENGTH_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:13 - Field of the countdown"]
    #[inline(always)]
    pub fn frameindex(&self) -> FRAMEINDEX_R {
        FRAMEINDEX_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:21 - Micro frame length"]
    #[inline(always)]
    pub fn framelength(&self) -> FRAMELENGTH_R {
        FRAMELENGTH_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:13 - Field of the countdown"]
    #[inline(always)]
    pub fn frameindex(&mut self) -> FRAMEINDEX_W {
        FRAMEINDEX_W { w: self }
    }
}
