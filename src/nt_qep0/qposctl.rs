#[doc = "Reader of register QPOSCTL"]
pub type R = crate::R<u32, super::QPOSCTL>;
#[doc = "Writer for register QPOSCTL"]
pub type W = crate::W<u32, super::QPOSCTL>;
#[doc = "Register QPOSCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::QPOSCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PCSPW`"]
pub type PCSPW_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PCSPW`"]
pub struct PCSPW_W<'a> {
    w: &'a mut W,
}
impl<'a> PCSPW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
#[doc = "Reader of field `PCE`"]
pub type PCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCE`"]
pub struct PCE_W<'a> {
    w: &'a mut W,
}
impl<'a> PCE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `PCPOL`"]
pub type PCPOL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCPOL`"]
pub struct PCPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> PCPOL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `PCLOAD`"]
pub type PCLOAD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCLOAD`"]
pub struct PCLOAD_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLOAD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `PCSHDW`"]
pub type PCSHDW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCSHDW`"]
pub struct PCSHDW_W<'a> {
    w: &'a mut W,
}
impl<'a> PCSHDW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn pcspw(&self) -> PCSPW_R {
        PCSPW_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 12 - enable compare"]
    #[inline(always)]
    pub fn pce(&self) -> PCE_R {
        PCE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - polarity synchout"]
    #[inline(always)]
    pub fn pcpol(&self) -> PCPOL_R {
        PCPOL_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Lazy write rate"]
    #[inline(always)]
    pub fn pcload(&self) -> PCLOAD_R {
        PCLOAD_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Lazy load"]
    #[inline(always)]
    pub fn pcshdw(&self) -> PCSHDW_R {
        PCSHDW_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn pcspw(&mut self) -> PCSPW_W {
        PCSPW_W { w: self }
    }
    #[doc = "Bit 12 - enable compare"]
    #[inline(always)]
    pub fn pce(&mut self) -> PCE_W {
        PCE_W { w: self }
    }
    #[doc = "Bit 13 - polarity synchout"]
    #[inline(always)]
    pub fn pcpol(&mut self) -> PCPOL_W {
        PCPOL_W { w: self }
    }
    #[doc = "Bit 14 - Lazy write rate"]
    #[inline(always)]
    pub fn pcload(&mut self) -> PCLOAD_W {
        PCLOAD_W { w: self }
    }
    #[doc = "Bit 15 - Lazy load"]
    #[inline(always)]
    pub fn pcshdw(&mut self) -> PCSHDW_W {
        PCSHDW_W { w: self }
    }
}
