#[doc = "Reader of register ETCLR"]
pub type R = crate::R<u32, super::ETCLR>;
#[doc = "Writer for register ETCLR"]
pub type W = crate::W<u32, super::ETCLR>;
#[doc = "Register ETCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::ETCLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INT`"]
pub type INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INT`"]
pub struct INT_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `SOCA`"]
pub type SOCA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOCA`"]
pub struct SOCA_W<'a> {
    w: &'a mut W,
}
impl<'a> SOCA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `SOCB`"]
pub type SOCB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOCB`"]
pub struct SOCB_W<'a> {
    w: &'a mut W,
}
impl<'a> SOCB_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - reset the status of the external interrupt EPWMxINT"]
    #[inline(always)]
    pub fn int(&self) -> INT_R {
        INT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - reset the status of the external ADC EPWMxSOCA"]
    #[inline(always)]
    pub fn soca(&self) -> SOCA_R {
        SOCA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - reset the status of the external ADC EPWMxSOCB"]
    #[inline(always)]
    pub fn socb(&self) -> SOCB_R {
        SOCB_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - reset the status of the external interrupt EPWMxINT"]
    #[inline(always)]
    pub fn int(&mut self) -> INT_W {
        INT_W { w: self }
    }
    #[doc = "Bit 2 - reset the status of the external ADC EPWMxSOCA"]
    #[inline(always)]
    pub fn soca(&mut self) -> SOCA_W {
        SOCA_W { w: self }
    }
    #[doc = "Bit 3 - reset the status of the external ADC EPWMxSOCB"]
    #[inline(always)]
    pub fn socb(&mut self) -> SOCB_W {
        SOCB_W { w: self }
    }
}
