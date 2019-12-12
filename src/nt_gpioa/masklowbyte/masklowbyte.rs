#[doc = "Reader of register MASKLOWBYTE"]
pub type R = crate::R<u32, super::MASKLOWBYTE>;
#[doc = "Writer for register MASKLOWBYTE"]
pub type W = crate::W<u32, super::MASKLOWBYTE>;
#[doc = "Register MASKLOWBYTE `reset()`'s with value 0"]
impl crate::ResetValue for super::MASKLOWBYTE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MASKLB`"]
pub type MASKLB_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MASKLB`"]
pub struct MASKLB_W<'a> {
    w: &'a mut W,
}
impl<'a> MASKLB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Data for lower byte masked access"]
    #[inline(always)]
    pub fn masklb(&self) -> MASKLB_R {
        MASKLB_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data for lower byte masked access"]
    #[inline(always)]
    pub fn masklb(&mut self) -> MASKLB_W {
        MASKLB_W { w: self }
    }
}
