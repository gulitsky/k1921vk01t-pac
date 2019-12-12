#[doc = "Reader of register QFRC"]
pub type R = crate::R<u32, super::QFRC>;
#[doc = "Writer for register QFRC"]
pub type W = crate::W<u32, super::QFRC>;
#[doc = "Register QFRC `reset()`'s with value 0"]
impl crate::ResetValue for super::QFRC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `QPE`"]
pub type QPE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `QPE`"]
pub struct QPE_W<'a> {
    w: &'a mut W,
}
impl<'a> QPE_W<'a> {
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
#[doc = "Reader of field `QDC`"]
pub type QDC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `QDC`"]
pub struct QDC_W<'a> {
    w: &'a mut W,
}
impl<'a> QDC_W<'a> {
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
#[doc = "Reader of field `WTO`"]
pub type WTO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WTO`"]
pub struct WTO_W<'a> {
    w: &'a mut W,
}
impl<'a> WTO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `PCU`"]
pub type PCU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCU`"]
pub struct PCU_W<'a> {
    w: &'a mut W,
}
impl<'a> PCU_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `PCO`"]
pub type PCO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCO`"]
pub struct PCO_W<'a> {
    w: &'a mut W,
}
impl<'a> PCO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `PCR`"]
pub type PCR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCR`"]
pub struct PCR_W<'a> {
    w: &'a mut W,
}
impl<'a> PCR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `PCM`"]
pub type PCM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCM`"]
pub struct PCM_W<'a> {
    w: &'a mut W,
}
impl<'a> PCM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `SEL`"]
pub type SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SEL`"]
pub struct SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `IEL`"]
pub type IEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IEL`"]
pub struct IEL_W<'a> {
    w: &'a mut W,
}
impl<'a> IEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `UTO`"]
pub type UTO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UTO`"]
pub struct UTO_W<'a> {
    w: &'a mut W,
}
impl<'a> UTO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - enable flag"]
    #[inline(always)]
    pub fn pce(&self) -> PCE_R {
        PCE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - enable flag"]
    #[inline(always)]
    pub fn qpe(&self) -> QPE_R {
        QPE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - enable flag"]
    #[inline(always)]
    pub fn qdc(&self) -> QDC_R {
        QDC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - enable flag"]
    #[inline(always)]
    pub fn wto(&self) -> WTO_R {
        WTO_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - enable flag"]
    #[inline(always)]
    pub fn pcu(&self) -> PCU_R {
        PCU_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - enable flag"]
    #[inline(always)]
    pub fn pco(&self) -> PCO_R {
        PCO_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - enable flag"]
    #[inline(always)]
    pub fn pcr(&self) -> PCR_R {
        PCR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - enable flag"]
    #[inline(always)]
    pub fn pcm(&self) -> PCM_R {
        PCM_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - enable flag"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - enable flag"]
    #[inline(always)]
    pub fn iel(&self) -> IEL_R {
        IEL_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - enable flag"]
    #[inline(always)]
    pub fn uto(&self) -> UTO_R {
        UTO_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - enable flag"]
    #[inline(always)]
    pub fn pce(&mut self) -> PCE_W {
        PCE_W { w: self }
    }
    #[doc = "Bit 2 - enable flag"]
    #[inline(always)]
    pub fn qpe(&mut self) -> QPE_W {
        QPE_W { w: self }
    }
    #[doc = "Bit 3 - enable flag"]
    #[inline(always)]
    pub fn qdc(&mut self) -> QDC_W {
        QDC_W { w: self }
    }
    #[doc = "Bit 4 - enable flag"]
    #[inline(always)]
    pub fn wto(&mut self) -> WTO_W {
        WTO_W { w: self }
    }
    #[doc = "Bit 5 - enable flag"]
    #[inline(always)]
    pub fn pcu(&mut self) -> PCU_W {
        PCU_W { w: self }
    }
    #[doc = "Bit 6 - enable flag"]
    #[inline(always)]
    pub fn pco(&mut self) -> PCO_W {
        PCO_W { w: self }
    }
    #[doc = "Bit 7 - enable flag"]
    #[inline(always)]
    pub fn pcr(&mut self) -> PCR_W {
        PCR_W { w: self }
    }
    #[doc = "Bit 8 - enable flag"]
    #[inline(always)]
    pub fn pcm(&mut self) -> PCM_W {
        PCM_W { w: self }
    }
    #[doc = "Bit 9 - enable flag"]
    #[inline(always)]
    pub fn sel(&mut self) -> SEL_W {
        SEL_W { w: self }
    }
    #[doc = "Bit 10 - enable flag"]
    #[inline(always)]
    pub fn iel(&mut self) -> IEL_W {
        IEL_W { w: self }
    }
    #[doc = "Bit 11 - enable flag"]
    #[inline(always)]
    pub fn uto(&mut self) -> UTO_W {
        UTO_W { w: self }
    }
}
