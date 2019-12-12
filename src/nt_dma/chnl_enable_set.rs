#[doc = "Reader of register CHNL_ENABLE_SET"]
pub type R = crate::R<u32, super::CHNL_ENABLE_SET>;
#[doc = "Writer for register CHNL_ENABLE_SET"]
pub type W = crate::W<u32, super::CHNL_ENABLE_SET>;
#[doc = "Register CHNL_ENABLE_SET `reset()`'s with value 0"]
impl crate::ResetValue for super::CHNL_ENABLE_SET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DMA_ENSET0`"]
pub type DMA_ENSET0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_ENSET0`"]
pub struct DMA_ENSET0_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ENSET0_W<'a> {
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
#[doc = "Reader of field `DMA_ENSET1`"]
pub type DMA_ENSET1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_ENSET1`"]
pub struct DMA_ENSET1_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ENSET1_W<'a> {
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
#[doc = "Reader of field `DMA_ENSET2`"]
pub type DMA_ENSET2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_ENSET2`"]
pub struct DMA_ENSET2_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ENSET2_W<'a> {
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
#[doc = "Reader of field `DMA_ENSET3`"]
pub type DMA_ENSET3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_ENSET3`"]
pub struct DMA_ENSET3_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ENSET3_W<'a> {
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
#[doc = "Reader of field `DMA_ENSET4`"]
pub type DMA_ENSET4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_ENSET4`"]
pub struct DMA_ENSET4_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ENSET4_W<'a> {
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
#[doc = "Reader of field `DMA_ENSET5`"]
pub type DMA_ENSET5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_ENSET5`"]
pub struct DMA_ENSET5_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ENSET5_W<'a> {
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
#[doc = "Reader of field `DMA_ENSET6`"]
pub type DMA_ENSET6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_ENSET6`"]
pub struct DMA_ENSET6_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ENSET6_W<'a> {
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
#[doc = "Reader of field `DMA_ENSET7`"]
pub type DMA_ENSET7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_ENSET7`"]
pub struct DMA_ENSET7_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ENSET7_W<'a> {
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
#[doc = "Reader of field `DMA_ENSET8`"]
pub type DMA_ENSET8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_ENSET8`"]
pub struct DMA_ENSET8_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ENSET8_W<'a> {
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
#[doc = "Reader of field `DMA_ENSET9`"]
pub type DMA_ENSET9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_ENSET9`"]
pub struct DMA_ENSET9_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ENSET9_W<'a> {
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
#[doc = "Reader of field `DMA_ENSET10`"]
pub type DMA_ENSET10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_ENSET10`"]
pub struct DMA_ENSET10_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ENSET10_W<'a> {
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
#[doc = "Reader of field `DMA_ENSET11`"]
pub type DMA_ENSET11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_ENSET11`"]
pub struct DMA_ENSET11_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ENSET11_W<'a> {
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
#[doc = "Reader of field `DMA_ENSET12`"]
pub type DMA_ENSET12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_ENSET12`"]
pub struct DMA_ENSET12_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ENSET12_W<'a> {
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
#[doc = "Reader of field `DMA_ENSET13`"]
pub type DMA_ENSET13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_ENSET13`"]
pub struct DMA_ENSET13_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ENSET13_W<'a> {
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
#[doc = "Reader of field `DMA_ENSET14`"]
pub type DMA_ENSET14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_ENSET14`"]
pub struct DMA_ENSET14_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ENSET14_W<'a> {
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
#[doc = "Reader of field `DMA_ENSET15`"]
pub type DMA_ENSET15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_ENSET15`"]
pub struct DMA_ENSET15_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ENSET15_W<'a> {
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
#[doc = "Reader of field `DMA_ENSET16`"]
pub type DMA_ENSET16_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_ENSET16`"]
pub struct DMA_ENSET16_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ENSET16_W<'a> {
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
#[doc = "Reader of field `DMA_ENSET17`"]
pub type DMA_ENSET17_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_ENSET17`"]
pub struct DMA_ENSET17_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ENSET17_W<'a> {
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
#[doc = "Reader of field `DMA_ENSET18`"]
pub type DMA_ENSET18_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_ENSET18`"]
pub struct DMA_ENSET18_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ENSET18_W<'a> {
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
#[doc = "Reader of field `DMA_ENSET19`"]
pub type DMA_ENSET19_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_ENSET19`"]
pub struct DMA_ENSET19_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ENSET19_W<'a> {
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
#[doc = "Reader of field `DMA_ENSET20`"]
pub type DMA_ENSET20_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_ENSET20`"]
pub struct DMA_ENSET20_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ENSET20_W<'a> {
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
#[doc = "Reader of field `DMA_ENSET21`"]
pub type DMA_ENSET21_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_ENSET21`"]
pub struct DMA_ENSET21_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ENSET21_W<'a> {
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
#[doc = "Reader of field `DMA_ENSET22`"]
pub type DMA_ENSET22_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_ENSET22`"]
pub struct DMA_ENSET22_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ENSET22_W<'a> {
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
#[doc = "Reader of field `DMA_ENSET23`"]
pub type DMA_ENSET23_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_ENSET23`"]
pub struct DMA_ENSET23_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ENSET23_W<'a> {
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
    #[doc = "Bit 0 - Returns the status of dma_active\\[ \\]"]
    #[inline(always)]
    pub fn dma_enset0(&self) -> DMA_ENSET0_R {
        DMA_ENSET0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Returns the status of dma_active\\[ \\]"]
    #[inline(always)]
    pub fn dma_enset1(&self) -> DMA_ENSET1_R {
        DMA_ENSET1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Returns the status of dma_active\\[ \\]"]
    #[inline(always)]
    pub fn dma_enset2(&self) -> DMA_ENSET2_R {
        DMA_ENSET2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Returns the status of dma_active\\[ \\]"]
    #[inline(always)]
    pub fn dma_enset3(&self) -> DMA_ENSET3_R {
        DMA_ENSET3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Returns the status of dma_active\\[ \\]"]
    #[inline(always)]
    pub fn dma_enset4(&self) -> DMA_ENSET4_R {
        DMA_ENSET4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Returns the status of dma_active\\[ \\]"]
    #[inline(always)]
    pub fn dma_enset5(&self) -> DMA_ENSET5_R {
        DMA_ENSET5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Returns the status of dma_active\\[ \\]"]
    #[inline(always)]
    pub fn dma_enset6(&self) -> DMA_ENSET6_R {
        DMA_ENSET6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Returns the status of dma_active\\[ \\]"]
    #[inline(always)]
    pub fn dma_enset7(&self) -> DMA_ENSET7_R {
        DMA_ENSET7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Returns the status of dma_active\\[ \\]"]
    #[inline(always)]
    pub fn dma_enset8(&self) -> DMA_ENSET8_R {
        DMA_ENSET8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Returns the status of dma_active\\[ \\]"]
    #[inline(always)]
    pub fn dma_enset9(&self) -> DMA_ENSET9_R {
        DMA_ENSET9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Returns the status of dma_active\\[ \\]"]
    #[inline(always)]
    pub fn dma_enset10(&self) -> DMA_ENSET10_R {
        DMA_ENSET10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Returns the status of dma_active\\[ \\]"]
    #[inline(always)]
    pub fn dma_enset11(&self) -> DMA_ENSET11_R {
        DMA_ENSET11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Returns the status of dma_active\\[ \\]"]
    #[inline(always)]
    pub fn dma_enset12(&self) -> DMA_ENSET12_R {
        DMA_ENSET12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Returns the status of dma_active\\[ \\]"]
    #[inline(always)]
    pub fn dma_enset13(&self) -> DMA_ENSET13_R {
        DMA_ENSET13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Returns the status of dma_active\\[ \\]"]
    #[inline(always)]
    pub fn dma_enset14(&self) -> DMA_ENSET14_R {
        DMA_ENSET14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Returns the status of dma_active\\[ \\]"]
    #[inline(always)]
    pub fn dma_enset15(&self) -> DMA_ENSET15_R {
        DMA_ENSET15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Returns the status of dma_active\\[ \\]"]
    #[inline(always)]
    pub fn dma_enset16(&self) -> DMA_ENSET16_R {
        DMA_ENSET16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Returns the status of dma_active\\[ \\]"]
    #[inline(always)]
    pub fn dma_enset17(&self) -> DMA_ENSET17_R {
        DMA_ENSET17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Returns the status of dma_active\\[ \\]"]
    #[inline(always)]
    pub fn dma_enset18(&self) -> DMA_ENSET18_R {
        DMA_ENSET18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Returns the status of dma_active\\[ \\]"]
    #[inline(always)]
    pub fn dma_enset19(&self) -> DMA_ENSET19_R {
        DMA_ENSET19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Returns the status of dma_active\\[ \\]"]
    #[inline(always)]
    pub fn dma_enset20(&self) -> DMA_ENSET20_R {
        DMA_ENSET20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Returns the status of dma_active\\[ \\]"]
    #[inline(always)]
    pub fn dma_enset21(&self) -> DMA_ENSET21_R {
        DMA_ENSET21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Returns the status of dma_active\\[ \\]"]
    #[inline(always)]
    pub fn dma_enset22(&self) -> DMA_ENSET22_R {
        DMA_ENSET22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Returns the status of dma_active\\[ \\]"]
    #[inline(always)]
    pub fn dma_enset23(&self) -> DMA_ENSET23_R {
        DMA_ENSET23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Returns the status of dma_active\\[ \\]"]
    #[inline(always)]
    pub fn dma_enset0(&mut self) -> DMA_ENSET0_W {
        DMA_ENSET0_W { w: self }
    }
    #[doc = "Bit 1 - Returns the status of dma_active\\[ \\]"]
    #[inline(always)]
    pub fn dma_enset1(&mut self) -> DMA_ENSET1_W {
        DMA_ENSET1_W { w: self }
    }
    #[doc = "Bit 2 - Returns the status of dma_active\\[ \\]"]
    #[inline(always)]
    pub fn dma_enset2(&mut self) -> DMA_ENSET2_W {
        DMA_ENSET2_W { w: self }
    }
    #[doc = "Bit 3 - Returns the status of dma_active\\[ \\]"]
    #[inline(always)]
    pub fn dma_enset3(&mut self) -> DMA_ENSET3_W {
        DMA_ENSET3_W { w: self }
    }
    #[doc = "Bit 4 - Returns the status of dma_active\\[ \\]"]
    #[inline(always)]
    pub fn dma_enset4(&mut self) -> DMA_ENSET4_W {
        DMA_ENSET4_W { w: self }
    }
    #[doc = "Bit 5 - Returns the status of dma_active\\[ \\]"]
    #[inline(always)]
    pub fn dma_enset5(&mut self) -> DMA_ENSET5_W {
        DMA_ENSET5_W { w: self }
    }
    #[doc = "Bit 6 - Returns the status of dma_active\\[ \\]"]
    #[inline(always)]
    pub fn dma_enset6(&mut self) -> DMA_ENSET6_W {
        DMA_ENSET6_W { w: self }
    }
    #[doc = "Bit 7 - Returns the status of dma_active\\[ \\]"]
    #[inline(always)]
    pub fn dma_enset7(&mut self) -> DMA_ENSET7_W {
        DMA_ENSET7_W { w: self }
    }
    #[doc = "Bit 8 - Returns the status of dma_active\\[ \\]"]
    #[inline(always)]
    pub fn dma_enset8(&mut self) -> DMA_ENSET8_W {
        DMA_ENSET8_W { w: self }
    }
    #[doc = "Bit 9 - Returns the status of dma_active\\[ \\]"]
    #[inline(always)]
    pub fn dma_enset9(&mut self) -> DMA_ENSET9_W {
        DMA_ENSET9_W { w: self }
    }
    #[doc = "Bit 10 - Returns the status of dma_active\\[ \\]"]
    #[inline(always)]
    pub fn dma_enset10(&mut self) -> DMA_ENSET10_W {
        DMA_ENSET10_W { w: self }
    }
    #[doc = "Bit 11 - Returns the status of dma_active\\[ \\]"]
    #[inline(always)]
    pub fn dma_enset11(&mut self) -> DMA_ENSET11_W {
        DMA_ENSET11_W { w: self }
    }
    #[doc = "Bit 12 - Returns the status of dma_active\\[ \\]"]
    #[inline(always)]
    pub fn dma_enset12(&mut self) -> DMA_ENSET12_W {
        DMA_ENSET12_W { w: self }
    }
    #[doc = "Bit 13 - Returns the status of dma_active\\[ \\]"]
    #[inline(always)]
    pub fn dma_enset13(&mut self) -> DMA_ENSET13_W {
        DMA_ENSET13_W { w: self }
    }
    #[doc = "Bit 14 - Returns the status of dma_active\\[ \\]"]
    #[inline(always)]
    pub fn dma_enset14(&mut self) -> DMA_ENSET14_W {
        DMA_ENSET14_W { w: self }
    }
    #[doc = "Bit 15 - Returns the status of dma_active\\[ \\]"]
    #[inline(always)]
    pub fn dma_enset15(&mut self) -> DMA_ENSET15_W {
        DMA_ENSET15_W { w: self }
    }
    #[doc = "Bit 16 - Returns the status of dma_active\\[ \\]"]
    #[inline(always)]
    pub fn dma_enset16(&mut self) -> DMA_ENSET16_W {
        DMA_ENSET16_W { w: self }
    }
    #[doc = "Bit 17 - Returns the status of dma_active\\[ \\]"]
    #[inline(always)]
    pub fn dma_enset17(&mut self) -> DMA_ENSET17_W {
        DMA_ENSET17_W { w: self }
    }
    #[doc = "Bit 18 - Returns the status of dma_active\\[ \\]"]
    #[inline(always)]
    pub fn dma_enset18(&mut self) -> DMA_ENSET18_W {
        DMA_ENSET18_W { w: self }
    }
    #[doc = "Bit 19 - Returns the status of dma_active\\[ \\]"]
    #[inline(always)]
    pub fn dma_enset19(&mut self) -> DMA_ENSET19_W {
        DMA_ENSET19_W { w: self }
    }
    #[doc = "Bit 20 - Returns the status of dma_active\\[ \\]"]
    #[inline(always)]
    pub fn dma_enset20(&mut self) -> DMA_ENSET20_W {
        DMA_ENSET20_W { w: self }
    }
    #[doc = "Bit 21 - Returns the status of dma_active\\[ \\]"]
    #[inline(always)]
    pub fn dma_enset21(&mut self) -> DMA_ENSET21_W {
        DMA_ENSET21_W { w: self }
    }
    #[doc = "Bit 22 - Returns the status of dma_active\\[ \\]"]
    #[inline(always)]
    pub fn dma_enset22(&mut self) -> DMA_ENSET22_W {
        DMA_ENSET22_W { w: self }
    }
    #[doc = "Bit 23 - Returns the status of dma_active\\[ \\]"]
    #[inline(always)]
    pub fn dma_enset23(&mut self) -> DMA_ENSET23_W {
        DMA_ENSET23_W { w: self }
    }
}
