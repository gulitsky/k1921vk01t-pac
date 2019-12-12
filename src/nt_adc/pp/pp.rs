#[doc = "Reader of register PP"]
pub type R = crate::R<u32, super::PP>;
#[doc = "Writer for register PP"]
pub type W = crate::W<u32, super::PP>;
#[doc = "Register PP `reset()`'s with value 0"]
impl crate::ResetValue for super::PP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OM`"]
pub type OM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OM`"]
pub struct OM_W<'a> {
    w: &'a mut W,
}
impl<'a> OM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `ENA`"]
pub type ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENA`"]
pub struct ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:23 - ADC mode"]
    #[inline(always)]
    pub fn om(&self) -> OM_R {
        OM_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 31 - Enable ADC"]
    #[inline(always)]
    pub fn ena(&self) -> ENA_R {
        ENA_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 16:23 - ADC mode"]
    #[inline(always)]
    pub fn om(&mut self) -> OM_W {
        OM_W { w: self }
    }
    #[doc = "Bit 31 - Enable ADC"]
    #[inline(always)]
    pub fn ena(&mut self) -> ENA_W {
        ENA_W { w: self }
    }
}
