#[doc = "Reader of register MOAMR"]
pub type R = crate::R<u32, super::MOAMR>;
#[doc = "Writer for register MOAMR"]
pub type W = crate::W<u32, super::MOAMR>;
#[doc = "Register MOAMR `reset()`'s with value 0"]
impl crate::ResetValue for super::MOAMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AM`"]
pub type AM_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `AM`"]
pub struct AM_W<'a> {
    w: &'a mut W,
}
impl<'a> AM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1fff_ffff) | ((value as u32) & 0x1fff_ffff);
        self.w
    }
}
#[doc = "Reader of field `MIDE`"]
pub type MIDE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MIDE`"]
pub struct MIDE_W<'a> {
    w: &'a mut W,
}
impl<'a> MIDE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:28"]
    #[inline(always)]
    pub fn am(&self) -> AM_R {
        AM_R::new((self.bits & 0x1fff_ffff) as u32)
    }
    #[doc = "Bit 29 - Mask bit IDE message"]
    #[inline(always)]
    pub fn mide(&self) -> MIDE_R {
        MIDE_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:28"]
    #[inline(always)]
    pub fn am(&mut self) -> AM_W {
        AM_W { w: self }
    }
    #[doc = "Bit 29 - Mask bit IDE message"]
    #[inline(always)]
    pub fn mide(&mut self) -> MIDE_W {
        MIDE_W { w: self }
    }
}
