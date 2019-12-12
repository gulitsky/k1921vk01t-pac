#[doc = "Reader of register MASKHIGHBYTE"]
pub type R = crate::R<u32, super::MASKHIGHBYTE>;
#[doc = "Writer for register MASKHIGHBYTE"]
pub type W = crate::W<u32, super::MASKHIGHBYTE>;
#[doc = "Register MASKHIGHBYTE `reset()`'s with value 0"]
impl crate::ResetValue for super::MASKHIGHBYTE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MASKHB`"]
pub type MASKHB_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MASKHB`"]
pub struct MASKHB_W<'a> {
    w: &'a mut W,
}
impl<'a> MASKHB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:15 - Data for higher byte masked access"]
    #[inline(always)]
    pub fn maskhb(&self) -> MASKHB_R {
        MASKHB_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - Data for higher byte masked access"]
    #[inline(always)]
    pub fn maskhb(&mut self) -> MASKHB_W {
        MASKHB_W { w: self }
    }
}
