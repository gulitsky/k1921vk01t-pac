#[doc = "Reader of register TBSTS"]
pub type R = crate::R<u32, super::TBSTS>;
#[doc = "Writer for register TBSTS"]
pub type W = crate::W<u32, super::TBSTS>;
#[doc = "Register TBSTS `reset()`'s with value 0"]
impl crate::ResetValue for super::TBSTS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CTRDIR`"]
pub type CTRDIR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTRDIR`"]
pub struct CTRDIR_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRDIR_W<'a> {
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
#[doc = "Reader of field `SYNCI`"]
pub type SYNCI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYNCI`"]
pub struct SYNCI_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCI_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `CTRMAX`"]
pub type CTRMAX_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Current timer counting direction"]
    #[inline(always)]
    pub fn ctrdir(&self) -> CTRDIR_R {
        CTRDIR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Synchronization status"]
    #[inline(always)]
    pub fn synci(&self) -> SYNCI_R {
        SYNCI_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Position of the peak value"]
    #[inline(always)]
    pub fn ctrmax(&self) -> CTRMAX_R {
        CTRMAX_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Current timer counting direction"]
    #[inline(always)]
    pub fn ctrdir(&mut self) -> CTRDIR_W {
        CTRDIR_W { w: self }
    }
    #[doc = "Bit 1 - Synchronization status"]
    #[inline(always)]
    pub fn synci(&mut self) -> SYNCI_W {
        SYNCI_W { w: self }
    }
}
