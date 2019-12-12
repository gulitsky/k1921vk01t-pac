#[doc = "Reader of register CHNL_REQ_MASK_SET"]
pub type R = crate::R<u32, super::CHNL_REQ_MASK_SET>;
#[doc = "Writer for register CHNL_REQ_MASK_SET"]
pub type W = crate::W<u32, super::CHNL_REQ_MASK_SET>;
#[doc = "Register CHNL_REQ_MASK_SET `reset()`'s with value 0"]
impl crate::ResetValue for super::CHNL_REQ_MASK_SET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DMA_REQMASKSET0`"]
pub type DMA_REQMASKSET0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_REQMASKSET0`"]
pub struct DMA_REQMASKSET0_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_REQMASKSET0_W<'a> {
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
#[doc = "Reader of field `DMA_REQMASKSET1`"]
pub type DMA_REQMASKSET1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_REQMASKSET1`"]
pub struct DMA_REQMASKSET1_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_REQMASKSET1_W<'a> {
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
#[doc = "Reader of field `DMA_REQMASKSET2`"]
pub type DMA_REQMASKSET2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_REQMASKSET2`"]
pub struct DMA_REQMASKSET2_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_REQMASKSET2_W<'a> {
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
#[doc = "Reader of field `DMA_REQMASKSET3`"]
pub type DMA_REQMASKSET3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_REQMASKSET3`"]
pub struct DMA_REQMASKSET3_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_REQMASKSET3_W<'a> {
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
#[doc = "Reader of field `DMA_REQMASKSET4`"]
pub type DMA_REQMASKSET4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_REQMASKSET4`"]
pub struct DMA_REQMASKSET4_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_REQMASKSET4_W<'a> {
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
#[doc = "Reader of field `DMA_REQMASKSET5`"]
pub type DMA_REQMASKSET5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_REQMASKSET5`"]
pub struct DMA_REQMASKSET5_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_REQMASKSET5_W<'a> {
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
#[doc = "Reader of field `DMA_REQMASKSET6`"]
pub type DMA_REQMASKSET6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_REQMASKSET6`"]
pub struct DMA_REQMASKSET6_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_REQMASKSET6_W<'a> {
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
#[doc = "Reader of field `DMA_REQMASKSET7`"]
pub type DMA_REQMASKSET7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_REQMASKSET7`"]
pub struct DMA_REQMASKSET7_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_REQMASKSET7_W<'a> {
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
#[doc = "Reader of field `DMA_REQMASKSET8`"]
pub type DMA_REQMASKSET8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_REQMASKSET8`"]
pub struct DMA_REQMASKSET8_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_REQMASKSET8_W<'a> {
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
#[doc = "Reader of field `DMA_REQMASKSET9`"]
pub type DMA_REQMASKSET9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_REQMASKSET9`"]
pub struct DMA_REQMASKSET9_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_REQMASKSET9_W<'a> {
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
#[doc = "Reader of field `DMA_REQMASKSET10`"]
pub type DMA_REQMASKSET10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_REQMASKSET10`"]
pub struct DMA_REQMASKSET10_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_REQMASKSET10_W<'a> {
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
#[doc = "Reader of field `DMA_REQMASKSET11`"]
pub type DMA_REQMASKSET11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_REQMASKSET11`"]
pub struct DMA_REQMASKSET11_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_REQMASKSET11_W<'a> {
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
#[doc = "Reader of field `DMA_REQMASKSET12`"]
pub type DMA_REQMASKSET12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_REQMASKSET12`"]
pub struct DMA_REQMASKSET12_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_REQMASKSET12_W<'a> {
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
#[doc = "Reader of field `DMA_REQMASKSET13`"]
pub type DMA_REQMASKSET13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_REQMASKSET13`"]
pub struct DMA_REQMASKSET13_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_REQMASKSET13_W<'a> {
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
#[doc = "Reader of field `DMA_REQMASKSET14`"]
pub type DMA_REQMASKSET14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_REQMASKSET14`"]
pub struct DMA_REQMASKSET14_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_REQMASKSET14_W<'a> {
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
#[doc = "Reader of field `DMA_REQMASKSET15`"]
pub type DMA_REQMASKSET15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_REQMASKSET15`"]
pub struct DMA_REQMASKSET15_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_REQMASKSET15_W<'a> {
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
#[doc = "Reader of field `DMA_REQMASKSET16`"]
pub type DMA_REQMASKSET16_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_REQMASKSET16`"]
pub struct DMA_REQMASKSET16_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_REQMASKSET16_W<'a> {
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
#[doc = "Reader of field `DMA_REQMASKSET17`"]
pub type DMA_REQMASKSET17_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_REQMASKSET17`"]
pub struct DMA_REQMASKSET17_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_REQMASKSET17_W<'a> {
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
#[doc = "Reader of field `DMA_REQMASKSET18`"]
pub type DMA_REQMASKSET18_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_REQMASKSET18`"]
pub struct DMA_REQMASKSET18_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_REQMASKSET18_W<'a> {
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
#[doc = "Reader of field `DMA_REQMASKSET19`"]
pub type DMA_REQMASKSET19_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_REQMASKSET19`"]
pub struct DMA_REQMASKSET19_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_REQMASKSET19_W<'a> {
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
#[doc = "Reader of field `DMA_REQMASKSET20`"]
pub type DMA_REQMASKSET20_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_REQMASKSET20`"]
pub struct DMA_REQMASKSET20_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_REQMASKSET20_W<'a> {
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
#[doc = "Reader of field `DMA_REQMASKSET21`"]
pub type DMA_REQMASKSET21_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_REQMASKSET21`"]
pub struct DMA_REQMASKSET21_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_REQMASKSET21_W<'a> {
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
#[doc = "Reader of field `DMA_REQMASKSET22`"]
pub type DMA_REQMASKSET22_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_REQMASKSET22`"]
pub struct DMA_REQMASKSET22_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_REQMASKSET22_W<'a> {
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
#[doc = "Reader of field `DMA_REQMASKSET23`"]
pub type DMA_REQMASKSET23_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_REQMASKSET23`"]
pub struct DMA_REQMASKSET23_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_REQMASKSET23_W<'a> {
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
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dma_reqmaskset0(&self) -> DMA_REQMASKSET0_R {
        DMA_REQMASKSET0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dma_reqmaskset1(&self) -> DMA_REQMASKSET1_R {
        DMA_REQMASKSET1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dma_reqmaskset2(&self) -> DMA_REQMASKSET2_R {
        DMA_REQMASKSET2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dma_reqmaskset3(&self) -> DMA_REQMASKSET3_R {
        DMA_REQMASKSET3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn dma_reqmaskset4(&self) -> DMA_REQMASKSET4_R {
        DMA_REQMASKSET4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn dma_reqmaskset5(&self) -> DMA_REQMASKSET5_R {
        DMA_REQMASKSET5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn dma_reqmaskset6(&self) -> DMA_REQMASKSET6_R {
        DMA_REQMASKSET6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn dma_reqmaskset7(&self) -> DMA_REQMASKSET7_R {
        DMA_REQMASKSET7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn dma_reqmaskset8(&self) -> DMA_REQMASKSET8_R {
        DMA_REQMASKSET8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn dma_reqmaskset9(&self) -> DMA_REQMASKSET9_R {
        DMA_REQMASKSET9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn dma_reqmaskset10(&self) -> DMA_REQMASKSET10_R {
        DMA_REQMASKSET10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn dma_reqmaskset11(&self) -> DMA_REQMASKSET11_R {
        DMA_REQMASKSET11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn dma_reqmaskset12(&self) -> DMA_REQMASKSET12_R {
        DMA_REQMASKSET12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn dma_reqmaskset13(&self) -> DMA_REQMASKSET13_R {
        DMA_REQMASKSET13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn dma_reqmaskset14(&self) -> DMA_REQMASKSET14_R {
        DMA_REQMASKSET14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn dma_reqmaskset15(&self) -> DMA_REQMASKSET15_R {
        DMA_REQMASKSET15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn dma_reqmaskset16(&self) -> DMA_REQMASKSET16_R {
        DMA_REQMASKSET16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn dma_reqmaskset17(&self) -> DMA_REQMASKSET17_R {
        DMA_REQMASKSET17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn dma_reqmaskset18(&self) -> DMA_REQMASKSET18_R {
        DMA_REQMASKSET18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn dma_reqmaskset19(&self) -> DMA_REQMASKSET19_R {
        DMA_REQMASKSET19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn dma_reqmaskset20(&self) -> DMA_REQMASKSET20_R {
        DMA_REQMASKSET20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn dma_reqmaskset21(&self) -> DMA_REQMASKSET21_R {
        DMA_REQMASKSET21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn dma_reqmaskset22(&self) -> DMA_REQMASKSET22_R {
        DMA_REQMASKSET22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn dma_reqmaskset23(&self) -> DMA_REQMASKSET23_R {
        DMA_REQMASKSET23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dma_reqmaskset0(&mut self) -> DMA_REQMASKSET0_W {
        DMA_REQMASKSET0_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dma_reqmaskset1(&mut self) -> DMA_REQMASKSET1_W {
        DMA_REQMASKSET1_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dma_reqmaskset2(&mut self) -> DMA_REQMASKSET2_W {
        DMA_REQMASKSET2_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dma_reqmaskset3(&mut self) -> DMA_REQMASKSET3_W {
        DMA_REQMASKSET3_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn dma_reqmaskset4(&mut self) -> DMA_REQMASKSET4_W {
        DMA_REQMASKSET4_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn dma_reqmaskset5(&mut self) -> DMA_REQMASKSET5_W {
        DMA_REQMASKSET5_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn dma_reqmaskset6(&mut self) -> DMA_REQMASKSET6_W {
        DMA_REQMASKSET6_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn dma_reqmaskset7(&mut self) -> DMA_REQMASKSET7_W {
        DMA_REQMASKSET7_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn dma_reqmaskset8(&mut self) -> DMA_REQMASKSET8_W {
        DMA_REQMASKSET8_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn dma_reqmaskset9(&mut self) -> DMA_REQMASKSET9_W {
        DMA_REQMASKSET9_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn dma_reqmaskset10(&mut self) -> DMA_REQMASKSET10_W {
        DMA_REQMASKSET10_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn dma_reqmaskset11(&mut self) -> DMA_REQMASKSET11_W {
        DMA_REQMASKSET11_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn dma_reqmaskset12(&mut self) -> DMA_REQMASKSET12_W {
        DMA_REQMASKSET12_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn dma_reqmaskset13(&mut self) -> DMA_REQMASKSET13_W {
        DMA_REQMASKSET13_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn dma_reqmaskset14(&mut self) -> DMA_REQMASKSET14_W {
        DMA_REQMASKSET14_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn dma_reqmaskset15(&mut self) -> DMA_REQMASKSET15_W {
        DMA_REQMASKSET15_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn dma_reqmaskset16(&mut self) -> DMA_REQMASKSET16_W {
        DMA_REQMASKSET16_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn dma_reqmaskset17(&mut self) -> DMA_REQMASKSET17_W {
        DMA_REQMASKSET17_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn dma_reqmaskset18(&mut self) -> DMA_REQMASKSET18_W {
        DMA_REQMASKSET18_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn dma_reqmaskset19(&mut self) -> DMA_REQMASKSET19_W {
        DMA_REQMASKSET19_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn dma_reqmaskset20(&mut self) -> DMA_REQMASKSET20_W {
        DMA_REQMASKSET20_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn dma_reqmaskset21(&mut self) -> DMA_REQMASKSET21_W {
        DMA_REQMASKSET21_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn dma_reqmaskset22(&mut self) -> DMA_REQMASKSET22_W {
        DMA_REQMASKSET22_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn dma_reqmaskset23(&mut self) -> DMA_REQMASKSET23_W {
        DMA_REQMASKSET23_W { w: self }
    }
}
