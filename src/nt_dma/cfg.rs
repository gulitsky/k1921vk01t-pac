#[doc = "Writer for register CFG"]
pub type W = crate::W<u32, super::CFG>;
#[doc = "Register CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `MASTER_EN`"]
pub struct MASTER_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> MASTER_EN_W<'a> {
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
#[doc = "Write proxy for field `CHNL_PROT_CTRL`"]
pub struct CHNL_PROT_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> CHNL_PROT_CTRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | (((value as u32) & 0x07) << 5);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Enable DMA"]
    #[inline(always)]
    pub fn master_en(&mut self) -> MASTER_EN_W {
        MASTER_EN_W { w: self }
    }
    #[doc = "Bits 5:7 - Enable control HPROT.1 for access indication with buffering"]
    #[inline(always)]
    pub fn chnl_prot_ctrl(&mut self) -> CHNL_PROT_CTRL_W {
        CHNL_PROT_CTRL_W { w: self }
    }
}
