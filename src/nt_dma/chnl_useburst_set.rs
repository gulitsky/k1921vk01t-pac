#[doc = "Reader of register CHNL_USEBURST_SET"]
pub type R = crate::R<u32, super::CHNL_USEBURST_SET>;
#[doc = "Writer for register CHNL_USEBURST_SET"]
pub type W = crate::W<u32, super::CHNL_USEBURST_SET>;
#[doc = "Register CHNL_USEBURST_SET `reset()`'s with value 0"]
impl crate::ResetValue for super::CHNL_USEBURST_SET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DMA_BURSTSET0`"]
pub type DMA_BURSTSET0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_BURSTSET0`"]
pub struct DMA_BURSTSET0_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_BURSTSET0_W<'a> {
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
#[doc = "Reader of field `DMA_BURSTSET1`"]
pub type DMA_BURSTSET1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_BURSTSET1`"]
pub struct DMA_BURSTSET1_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_BURSTSET1_W<'a> {
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
#[doc = "Reader of field `DMA_BURSTSET2`"]
pub type DMA_BURSTSET2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_BURSTSET2`"]
pub struct DMA_BURSTSET2_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_BURSTSET2_W<'a> {
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
#[doc = "Reader of field `DMA_BURSTSET3`"]
pub type DMA_BURSTSET3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_BURSTSET3`"]
pub struct DMA_BURSTSET3_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_BURSTSET3_W<'a> {
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
#[doc = "Reader of field `DMA_BURSTSET4`"]
pub type DMA_BURSTSET4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_BURSTSET4`"]
pub struct DMA_BURSTSET4_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_BURSTSET4_W<'a> {
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
#[doc = "Reader of field `DMA_BURSTSET5`"]
pub type DMA_BURSTSET5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_BURSTSET5`"]
pub struct DMA_BURSTSET5_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_BURSTSET5_W<'a> {
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
#[doc = "Reader of field `DMA_BURSTSET6`"]
pub type DMA_BURSTSET6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_BURSTSET6`"]
pub struct DMA_BURSTSET6_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_BURSTSET6_W<'a> {
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
#[doc = "Reader of field `DMA_BURSTSET7`"]
pub type DMA_BURSTSET7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_BURSTSET7`"]
pub struct DMA_BURSTSET7_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_BURSTSET7_W<'a> {
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
#[doc = "Reader of field `DMA_BURSTSET8`"]
pub type DMA_BURSTSET8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_BURSTSET8`"]
pub struct DMA_BURSTSET8_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_BURSTSET8_W<'a> {
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
#[doc = "Reader of field `DMA_BURSTSET9`"]
pub type DMA_BURSTSET9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_BURSTSET9`"]
pub struct DMA_BURSTSET9_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_BURSTSET9_W<'a> {
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
#[doc = "Reader of field `DMA_BURSTSET10`"]
pub type DMA_BURSTSET10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_BURSTSET10`"]
pub struct DMA_BURSTSET10_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_BURSTSET10_W<'a> {
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
#[doc = "Reader of field `DMA_BURSTSET11`"]
pub type DMA_BURSTSET11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_BURSTSET11`"]
pub struct DMA_BURSTSET11_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_BURSTSET11_W<'a> {
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
#[doc = "Reader of field `DMA_BURSTSET12`"]
pub type DMA_BURSTSET12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_BURSTSET12`"]
pub struct DMA_BURSTSET12_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_BURSTSET12_W<'a> {
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
#[doc = "Reader of field `DMA_BURSTSET13`"]
pub type DMA_BURSTSET13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_BURSTSET13`"]
pub struct DMA_BURSTSET13_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_BURSTSET13_W<'a> {
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
#[doc = "Reader of field `DMA_BURSTSET14`"]
pub type DMA_BURSTSET14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_BURSTSET14`"]
pub struct DMA_BURSTSET14_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_BURSTSET14_W<'a> {
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
#[doc = "Reader of field `DMA_BURSTSET15`"]
pub type DMA_BURSTSET15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_BURSTSET15`"]
pub struct DMA_BURSTSET15_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_BURSTSET15_W<'a> {
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
#[doc = "Reader of field `DMA_BURSTSET16`"]
pub type DMA_BURSTSET16_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_BURSTSET16`"]
pub struct DMA_BURSTSET16_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_BURSTSET16_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `DMA_BURSTSET17`"]
pub type DMA_BURSTSET17_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_BURSTSET17`"]
pub struct DMA_BURSTSET17_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_BURSTSET17_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `DMA_BURSTSET18`"]
pub type DMA_BURSTSET18_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_BURSTSET18`"]
pub struct DMA_BURSTSET18_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_BURSTSET18_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `DMA_BURSTSET19`"]
pub type DMA_BURSTSET19_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_BURSTSET19`"]
pub struct DMA_BURSTSET19_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_BURSTSET19_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `DMA_BURSTSET20`"]
pub type DMA_BURSTSET20_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_BURSTSET20`"]
pub struct DMA_BURSTSET20_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_BURSTSET20_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `DMA_BURSTSET21`"]
pub type DMA_BURSTSET21_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_BURSTSET21`"]
pub struct DMA_BURSTSET21_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_BURSTSET21_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `DMA_BURSTSET22`"]
pub type DMA_BURSTSET22_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_BURSTSET22`"]
pub struct DMA_BURSTSET22_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_BURSTSET22_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `DMA_BURSTSET23`"]
pub type DMA_BURSTSET23_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_BURSTSET23`"]
pub struct DMA_BURSTSET23_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_BURSTSET23_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Returns the status of the DMA single request signals"]
    #[inline(always)]
    pub fn dma_burstset0(&self) -> DMA_BURSTSET0_R {
        DMA_BURSTSET0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Returns the status of the DMA single request signals"]
    #[inline(always)]
    pub fn dma_burstset1(&self) -> DMA_BURSTSET1_R {
        DMA_BURSTSET1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Returns the status of the DMA single request signals"]
    #[inline(always)]
    pub fn dma_burstset2(&self) -> DMA_BURSTSET2_R {
        DMA_BURSTSET2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Returns the status of the DMA single request signals"]
    #[inline(always)]
    pub fn dma_burstset3(&self) -> DMA_BURSTSET3_R {
        DMA_BURSTSET3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Returns the status of the DMA single request signals"]
    #[inline(always)]
    pub fn dma_burstset4(&self) -> DMA_BURSTSET4_R {
        DMA_BURSTSET4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Returns the status of the DMA single request signals"]
    #[inline(always)]
    pub fn dma_burstset5(&self) -> DMA_BURSTSET5_R {
        DMA_BURSTSET5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Returns the status of the DMA single request signals"]
    #[inline(always)]
    pub fn dma_burstset6(&self) -> DMA_BURSTSET6_R {
        DMA_BURSTSET6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Returns the status of the DMA single request signals"]
    #[inline(always)]
    pub fn dma_burstset7(&self) -> DMA_BURSTSET7_R {
        DMA_BURSTSET7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Returns the status of the DMA single request signals"]
    #[inline(always)]
    pub fn dma_burstset8(&self) -> DMA_BURSTSET8_R {
        DMA_BURSTSET8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Returns the status of the DMA single request signals"]
    #[inline(always)]
    pub fn dma_burstset9(&self) -> DMA_BURSTSET9_R {
        DMA_BURSTSET9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Returns the status of the DMA single request signals"]
    #[inline(always)]
    pub fn dma_burstset10(&self) -> DMA_BURSTSET10_R {
        DMA_BURSTSET10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Returns the status of the DMA single request signals"]
    #[inline(always)]
    pub fn dma_burstset11(&self) -> DMA_BURSTSET11_R {
        DMA_BURSTSET11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Returns the status of the DMA single request signals"]
    #[inline(always)]
    pub fn dma_burstset12(&self) -> DMA_BURSTSET12_R {
        DMA_BURSTSET12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Returns the status of the DMA single request signals"]
    #[inline(always)]
    pub fn dma_burstset13(&self) -> DMA_BURSTSET13_R {
        DMA_BURSTSET13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Returns the status of the DMA single request signals"]
    #[inline(always)]
    pub fn dma_burstset14(&self) -> DMA_BURSTSET14_R {
        DMA_BURSTSET14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Returns the status of the DMA single request signals"]
    #[inline(always)]
    pub fn dma_burstset15(&self) -> DMA_BURSTSET15_R {
        DMA_BURSTSET15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Returns the status of the DMA single request signals"]
    #[inline(always)]
    pub fn dma_burstset16(&self) -> DMA_BURSTSET16_R {
        DMA_BURSTSET16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Returns the status of the DMA single request signals"]
    #[inline(always)]
    pub fn dma_burstset17(&self) -> DMA_BURSTSET17_R {
        DMA_BURSTSET17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Returns the status of the DMA single request signals"]
    #[inline(always)]
    pub fn dma_burstset18(&self) -> DMA_BURSTSET18_R {
        DMA_BURSTSET18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Returns the status of the DMA single request signals"]
    #[inline(always)]
    pub fn dma_burstset19(&self) -> DMA_BURSTSET19_R {
        DMA_BURSTSET19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Returns the status of the DMA single request signals"]
    #[inline(always)]
    pub fn dma_burstset20(&self) -> DMA_BURSTSET20_R {
        DMA_BURSTSET20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Returns the status of the DMA single request signals"]
    #[inline(always)]
    pub fn dma_burstset21(&self) -> DMA_BURSTSET21_R {
        DMA_BURSTSET21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Returns the status of the DMA single request signals"]
    #[inline(always)]
    pub fn dma_burstset22(&self) -> DMA_BURSTSET22_R {
        DMA_BURSTSET22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Returns the status of the DMA single request signals"]
    #[inline(always)]
    pub fn dma_burstset23(&self) -> DMA_BURSTSET23_R {
        DMA_BURSTSET23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Returns the status of the DMA single request signals"]
    #[inline(always)]
    pub fn dma_burstset0(&mut self) -> DMA_BURSTSET0_W {
        DMA_BURSTSET0_W { w: self }
    }
    #[doc = "Bit 1 - Returns the status of the DMA single request signals"]
    #[inline(always)]
    pub fn dma_burstset1(&mut self) -> DMA_BURSTSET1_W {
        DMA_BURSTSET1_W { w: self }
    }
    #[doc = "Bit 2 - Returns the status of the DMA single request signals"]
    #[inline(always)]
    pub fn dma_burstset2(&mut self) -> DMA_BURSTSET2_W {
        DMA_BURSTSET2_W { w: self }
    }
    #[doc = "Bit 3 - Returns the status of the DMA single request signals"]
    #[inline(always)]
    pub fn dma_burstset3(&mut self) -> DMA_BURSTSET3_W {
        DMA_BURSTSET3_W { w: self }
    }
    #[doc = "Bit 4 - Returns the status of the DMA single request signals"]
    #[inline(always)]
    pub fn dma_burstset4(&mut self) -> DMA_BURSTSET4_W {
        DMA_BURSTSET4_W { w: self }
    }
    #[doc = "Bit 5 - Returns the status of the DMA single request signals"]
    #[inline(always)]
    pub fn dma_burstset5(&mut self) -> DMA_BURSTSET5_W {
        DMA_BURSTSET5_W { w: self }
    }
    #[doc = "Bit 6 - Returns the status of the DMA single request signals"]
    #[inline(always)]
    pub fn dma_burstset6(&mut self) -> DMA_BURSTSET6_W {
        DMA_BURSTSET6_W { w: self }
    }
    #[doc = "Bit 7 - Returns the status of the DMA single request signals"]
    #[inline(always)]
    pub fn dma_burstset7(&mut self) -> DMA_BURSTSET7_W {
        DMA_BURSTSET7_W { w: self }
    }
    #[doc = "Bit 8 - Returns the status of the DMA single request signals"]
    #[inline(always)]
    pub fn dma_burstset8(&mut self) -> DMA_BURSTSET8_W {
        DMA_BURSTSET8_W { w: self }
    }
    #[doc = "Bit 9 - Returns the status of the DMA single request signals"]
    #[inline(always)]
    pub fn dma_burstset9(&mut self) -> DMA_BURSTSET9_W {
        DMA_BURSTSET9_W { w: self }
    }
    #[doc = "Bit 10 - Returns the status of the DMA single request signals"]
    #[inline(always)]
    pub fn dma_burstset10(&mut self) -> DMA_BURSTSET10_W {
        DMA_BURSTSET10_W { w: self }
    }
    #[doc = "Bit 11 - Returns the status of the DMA single request signals"]
    #[inline(always)]
    pub fn dma_burstset11(&mut self) -> DMA_BURSTSET11_W {
        DMA_BURSTSET11_W { w: self }
    }
    #[doc = "Bit 12 - Returns the status of the DMA single request signals"]
    #[inline(always)]
    pub fn dma_burstset12(&mut self) -> DMA_BURSTSET12_W {
        DMA_BURSTSET12_W { w: self }
    }
    #[doc = "Bit 13 - Returns the status of the DMA single request signals"]
    #[inline(always)]
    pub fn dma_burstset13(&mut self) -> DMA_BURSTSET13_W {
        DMA_BURSTSET13_W { w: self }
    }
    #[doc = "Bit 14 - Returns the status of the DMA single request signals"]
    #[inline(always)]
    pub fn dma_burstset14(&mut self) -> DMA_BURSTSET14_W {
        DMA_BURSTSET14_W { w: self }
    }
    #[doc = "Bit 15 - Returns the status of the DMA single request signals"]
    #[inline(always)]
    pub fn dma_burstset15(&mut self) -> DMA_BURSTSET15_W {
        DMA_BURSTSET15_W { w: self }
    }
    #[doc = "Bit 16 - Returns the status of the DMA single request signals"]
    #[inline(always)]
    pub fn dma_burstset16(&mut self) -> DMA_BURSTSET16_W {
        DMA_BURSTSET16_W { w: self }
    }
    #[doc = "Bit 17 - Returns the status of the DMA single request signals"]
    #[inline(always)]
    pub fn dma_burstset17(&mut self) -> DMA_BURSTSET17_W {
        DMA_BURSTSET17_W { w: self }
    }
    #[doc = "Bit 18 - Returns the status of the DMA single request signals"]
    #[inline(always)]
    pub fn dma_burstset18(&mut self) -> DMA_BURSTSET18_W {
        DMA_BURSTSET18_W { w: self }
    }
    #[doc = "Bit 19 - Returns the status of the DMA single request signals"]
    #[inline(always)]
    pub fn dma_burstset19(&mut self) -> DMA_BURSTSET19_W {
        DMA_BURSTSET19_W { w: self }
    }
    #[doc = "Bit 20 - Returns the status of the DMA single request signals"]
    #[inline(always)]
    pub fn dma_burstset20(&mut self) -> DMA_BURSTSET20_W {
        DMA_BURSTSET20_W { w: self }
    }
    #[doc = "Bit 21 - Returns the status of the DMA single request signals"]
    #[inline(always)]
    pub fn dma_burstset21(&mut self) -> DMA_BURSTSET21_W {
        DMA_BURSTSET21_W { w: self }
    }
    #[doc = "Bit 22 - Returns the status of the DMA single request signals"]
    #[inline(always)]
    pub fn dma_burstset22(&mut self) -> DMA_BURSTSET22_W {
        DMA_BURSTSET22_W { w: self }
    }
    #[doc = "Bit 23 - Returns the status of the DMA single request signals"]
    #[inline(always)]
    pub fn dma_burstset23(&mut self) -> DMA_BURSTSET23_W {
        DMA_BURSTSET23_W { w: self }
    }
}
