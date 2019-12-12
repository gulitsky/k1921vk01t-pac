#[doc = "Reader of register ECEINT"]
pub type R = crate::R<u32, super::ECEINT>;
#[doc = "Writer for register ECEINT"]
pub type W = crate::W<u32, super::ECEINT>;
#[doc = "Register ECEINT `reset()`'s with value 0"]
impl crate::ResetValue for super::ECEINT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CEVT0`"]
pub type CEVT0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CEVT0`"]
pub struct CEVT0_W<'a> {
    w: &'a mut W,
}
impl<'a> CEVT0_W<'a> {
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
#[doc = "Reader of field `CEVT1`"]
pub type CEVT1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CEVT1`"]
pub struct CEVT1_W<'a> {
    w: &'a mut W,
}
impl<'a> CEVT1_W<'a> {
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
#[doc = "Reader of field `CEVT2`"]
pub type CEVT2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CEVT2`"]
pub struct CEVT2_W<'a> {
    w: &'a mut W,
}
impl<'a> CEVT2_W<'a> {
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
#[doc = "Reader of field `CEVT3`"]
pub type CEVT3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CEVT3`"]
pub struct CEVT3_W<'a> {
    w: &'a mut W,
}
impl<'a> CEVT3_W<'a> {
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
#[doc = "Reader of field `CTR_OVF`"]
pub type CTR_OVF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTR_OVF`"]
pub struct CTR_OVF_W<'a> {
    w: &'a mut W,
}
impl<'a> CTR_OVF_W<'a> {
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
#[doc = "Reader of field `CTR_PRD`"]
pub type CTR_PRD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTR_PRD`"]
pub struct CTR_PRD_W<'a> {
    w: &'a mut W,
}
impl<'a> CTR_PRD_W<'a> {
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
#[doc = "Reader of field `CTR_CMP`"]
pub type CTR_CMP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTR_CMP`"]
pub struct CTR_CMP_W<'a> {
    w: &'a mut W,
}
impl<'a> CTR_CMP_W<'a> {
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
impl R {
    #[doc = "Bit 1 - enable int CEVT0"]
    #[inline(always)]
    pub fn cevt0(&self) -> CEVT0_R {
        CEVT0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - enable int CEVT1"]
    #[inline(always)]
    pub fn cevt1(&self) -> CEVT1_R {
        CEVT1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - enable int CEVT2"]
    #[inline(always)]
    pub fn cevt2(&self) -> CEVT2_R {
        CEVT2_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - enable int CEVT3"]
    #[inline(always)]
    pub fn cevt3(&self) -> CEVT3_R {
        CEVT3_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - enable int CTR_OVF"]
    #[inline(always)]
    pub fn ctr_ovf(&self) -> CTR_OVF_R {
        CTR_OVF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - enable int CTR=PRD"]
    #[inline(always)]
    pub fn ctr_prd(&self) -> CTR_PRD_R {
        CTR_PRD_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - enable int CTR=CMP"]
    #[inline(always)]
    pub fn ctr_cmp(&self) -> CTR_CMP_R {
        CTR_CMP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - enable int CEVT0"]
    #[inline(always)]
    pub fn cevt0(&mut self) -> CEVT0_W {
        CEVT0_W { w: self }
    }
    #[doc = "Bit 2 - enable int CEVT1"]
    #[inline(always)]
    pub fn cevt1(&mut self) -> CEVT1_W {
        CEVT1_W { w: self }
    }
    #[doc = "Bit 3 - enable int CEVT2"]
    #[inline(always)]
    pub fn cevt2(&mut self) -> CEVT2_W {
        CEVT2_W { w: self }
    }
    #[doc = "Bit 4 - enable int CEVT3"]
    #[inline(always)]
    pub fn cevt3(&mut self) -> CEVT3_W {
        CEVT3_W { w: self }
    }
    #[doc = "Bit 5 - enable int CTR_OVF"]
    #[inline(always)]
    pub fn ctr_ovf(&mut self) -> CTR_OVF_W {
        CTR_OVF_W { w: self }
    }
    #[doc = "Bit 6 - enable int CTR=PRD"]
    #[inline(always)]
    pub fn ctr_prd(&mut self) -> CTR_PRD_W {
        CTR_PRD_W { w: self }
    }
    #[doc = "Bit 7 - enable int CTR=CMP"]
    #[inline(always)]
    pub fn ctr_cmp(&mut self) -> CTR_CMP_W {
        CTR_CMP_W { w: self }
    }
}
