#[doc = "Reader of register CHNL_PRIORITY_SET"]
pub type R = crate::R<u32, super::CHNL_PRIORITY_SET>;
#[doc = "Writer for register CHNL_PRIORITY_SET"]
pub type W = crate::W<u32, super::CHNL_PRIORITY_SET>;
#[doc = "Register CHNL_PRIORITY_SET `reset()`'s with value 0"]
impl crate::ResetValue for super::CHNL_PRIORITY_SET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DMA_PRISET0`"]
pub type DMA_PRISET0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_PRISET0`"]
pub struct DMA_PRISET0_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_PRISET0_W<'a> {
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
#[doc = "Reader of field `DMA_PRISET1`"]
pub type DMA_PRISET1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_PRISET1`"]
pub struct DMA_PRISET1_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_PRISET1_W<'a> {
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
#[doc = "Reader of field `DMA_PRISET2`"]
pub type DMA_PRISET2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_PRISET2`"]
pub struct DMA_PRISET2_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_PRISET2_W<'a> {
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
#[doc = "Reader of field `DMA_PRISET3`"]
pub type DMA_PRISET3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_PRISET3`"]
pub struct DMA_PRISET3_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_PRISET3_W<'a> {
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
#[doc = "Reader of field `DMA_PRISET4`"]
pub type DMA_PRISET4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_PRISET4`"]
pub struct DMA_PRISET4_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_PRISET4_W<'a> {
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
#[doc = "Reader of field `DMA_PRISET5`"]
pub type DMA_PRISET5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_PRISET5`"]
pub struct DMA_PRISET5_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_PRISET5_W<'a> {
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
#[doc = "Reader of field `DMA_PRISET6`"]
pub type DMA_PRISET6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_PRISET6`"]
pub struct DMA_PRISET6_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_PRISET6_W<'a> {
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
#[doc = "Reader of field `DMA_PRISET7`"]
pub type DMA_PRISET7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_PRISET7`"]
pub struct DMA_PRISET7_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_PRISET7_W<'a> {
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
#[doc = "Reader of field `DMA_PRISET8`"]
pub type DMA_PRISET8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_PRISET8`"]
pub struct DMA_PRISET8_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_PRISET8_W<'a> {
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
#[doc = "Reader of field `DMA_PRISET9`"]
pub type DMA_PRISET9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_PRISET9`"]
pub struct DMA_PRISET9_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_PRISET9_W<'a> {
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
#[doc = "Reader of field `DMA_PRISET10`"]
pub type DMA_PRISET10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_PRISET10`"]
pub struct DMA_PRISET10_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_PRISET10_W<'a> {
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
#[doc = "Reader of field `DMA_PRISET11`"]
pub type DMA_PRISET11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_PRISET11`"]
pub struct DMA_PRISET11_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_PRISET11_W<'a> {
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
#[doc = "Reader of field `DMA_PRISET12`"]
pub type DMA_PRISET12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_PRISET12`"]
pub struct DMA_PRISET12_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_PRISET12_W<'a> {
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
#[doc = "Reader of field `DMA_PRISET13`"]
pub type DMA_PRISET13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_PRISET13`"]
pub struct DMA_PRISET13_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_PRISET13_W<'a> {
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
#[doc = "Reader of field `DMA_PRISET14`"]
pub type DMA_PRISET14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_PRISET14`"]
pub struct DMA_PRISET14_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_PRISET14_W<'a> {
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
#[doc = "Reader of field `DMA_PRISET15`"]
pub type DMA_PRISET15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_PRISET15`"]
pub struct DMA_PRISET15_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_PRISET15_W<'a> {
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
#[doc = "Reader of field `DMA_PRISET16`"]
pub type DMA_PRISET16_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_PRISET16`"]
pub struct DMA_PRISET16_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_PRISET16_W<'a> {
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
#[doc = "Reader of field `DMA_PRISET17`"]
pub type DMA_PRISET17_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_PRISET17`"]
pub struct DMA_PRISET17_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_PRISET17_W<'a> {
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
#[doc = "Reader of field `DMA_PRISET18`"]
pub type DMA_PRISET18_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_PRISET18`"]
pub struct DMA_PRISET18_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_PRISET18_W<'a> {
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
#[doc = "Reader of field `DMA_PRISET19`"]
pub type DMA_PRISET19_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_PRISET19`"]
pub struct DMA_PRISET19_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_PRISET19_W<'a> {
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
#[doc = "Reader of field `DMA_PRISET20`"]
pub type DMA_PRISET20_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_PRISET20`"]
pub struct DMA_PRISET20_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_PRISET20_W<'a> {
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
#[doc = "Reader of field `DMA_PRISET21`"]
pub type DMA_PRISET21_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_PRISET21`"]
pub struct DMA_PRISET21_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_PRISET21_W<'a> {
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
#[doc = "Reader of field `DMA_PRISET22`"]
pub type DMA_PRISET22_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_PRISET22`"]
pub struct DMA_PRISET22_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_PRISET22_W<'a> {
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
#[doc = "Reader of field `DMA_PRISET23`"]
pub type DMA_PRISET23_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_PRISET23`"]
pub struct DMA_PRISET23_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_PRISET23_W<'a> {
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
    #[doc = "Bit 0 - Set the priority channel0"]
    #[inline(always)]
    pub fn dma_priset0(&self) -> DMA_PRISET0_R {
        DMA_PRISET0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Set the priority channel1"]
    #[inline(always)]
    pub fn dma_priset1(&self) -> DMA_PRISET1_R {
        DMA_PRISET1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Set the priority channel2"]
    #[inline(always)]
    pub fn dma_priset2(&self) -> DMA_PRISET2_R {
        DMA_PRISET2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Set the priority channel3"]
    #[inline(always)]
    pub fn dma_priset3(&self) -> DMA_PRISET3_R {
        DMA_PRISET3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Set the priority channel4"]
    #[inline(always)]
    pub fn dma_priset4(&self) -> DMA_PRISET4_R {
        DMA_PRISET4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Set the priority channel5"]
    #[inline(always)]
    pub fn dma_priset5(&self) -> DMA_PRISET5_R {
        DMA_PRISET5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Set the priority channel6"]
    #[inline(always)]
    pub fn dma_priset6(&self) -> DMA_PRISET6_R {
        DMA_PRISET6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Set the priority channel7"]
    #[inline(always)]
    pub fn dma_priset7(&self) -> DMA_PRISET7_R {
        DMA_PRISET7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Set the priority channel8"]
    #[inline(always)]
    pub fn dma_priset8(&self) -> DMA_PRISET8_R {
        DMA_PRISET8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Set the priority channel9"]
    #[inline(always)]
    pub fn dma_priset9(&self) -> DMA_PRISET9_R {
        DMA_PRISET9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Set the priority channel10"]
    #[inline(always)]
    pub fn dma_priset10(&self) -> DMA_PRISET10_R {
        DMA_PRISET10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Set the priority channel11"]
    #[inline(always)]
    pub fn dma_priset11(&self) -> DMA_PRISET11_R {
        DMA_PRISET11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Set the priority channel12"]
    #[inline(always)]
    pub fn dma_priset12(&self) -> DMA_PRISET12_R {
        DMA_PRISET12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Set the priority channel13"]
    #[inline(always)]
    pub fn dma_priset13(&self) -> DMA_PRISET13_R {
        DMA_PRISET13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Set the priority channel14"]
    #[inline(always)]
    pub fn dma_priset14(&self) -> DMA_PRISET14_R {
        DMA_PRISET14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Set the priority channel15"]
    #[inline(always)]
    pub fn dma_priset15(&self) -> DMA_PRISET15_R {
        DMA_PRISET15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Set the priority channel16"]
    #[inline(always)]
    pub fn dma_priset16(&self) -> DMA_PRISET16_R {
        DMA_PRISET16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Set the priority channel17"]
    #[inline(always)]
    pub fn dma_priset17(&self) -> DMA_PRISET17_R {
        DMA_PRISET17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Set the priority channel18"]
    #[inline(always)]
    pub fn dma_priset18(&self) -> DMA_PRISET18_R {
        DMA_PRISET18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Set the priority channel19"]
    #[inline(always)]
    pub fn dma_priset19(&self) -> DMA_PRISET19_R {
        DMA_PRISET19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Set the priority channel20"]
    #[inline(always)]
    pub fn dma_priset20(&self) -> DMA_PRISET20_R {
        DMA_PRISET20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Set the priority channel21"]
    #[inline(always)]
    pub fn dma_priset21(&self) -> DMA_PRISET21_R {
        DMA_PRISET21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Set the priority channel22"]
    #[inline(always)]
    pub fn dma_priset22(&self) -> DMA_PRISET22_R {
        DMA_PRISET22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Set the priority channel23"]
    #[inline(always)]
    pub fn dma_priset23(&self) -> DMA_PRISET23_R {
        DMA_PRISET23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set the priority channel0"]
    #[inline(always)]
    pub fn dma_priset0(&mut self) -> DMA_PRISET0_W {
        DMA_PRISET0_W { w: self }
    }
    #[doc = "Bit 1 - Set the priority channel1"]
    #[inline(always)]
    pub fn dma_priset1(&mut self) -> DMA_PRISET1_W {
        DMA_PRISET1_W { w: self }
    }
    #[doc = "Bit 2 - Set the priority channel2"]
    #[inline(always)]
    pub fn dma_priset2(&mut self) -> DMA_PRISET2_W {
        DMA_PRISET2_W { w: self }
    }
    #[doc = "Bit 3 - Set the priority channel3"]
    #[inline(always)]
    pub fn dma_priset3(&mut self) -> DMA_PRISET3_W {
        DMA_PRISET3_W { w: self }
    }
    #[doc = "Bit 4 - Set the priority channel4"]
    #[inline(always)]
    pub fn dma_priset4(&mut self) -> DMA_PRISET4_W {
        DMA_PRISET4_W { w: self }
    }
    #[doc = "Bit 5 - Set the priority channel5"]
    #[inline(always)]
    pub fn dma_priset5(&mut self) -> DMA_PRISET5_W {
        DMA_PRISET5_W { w: self }
    }
    #[doc = "Bit 6 - Set the priority channel6"]
    #[inline(always)]
    pub fn dma_priset6(&mut self) -> DMA_PRISET6_W {
        DMA_PRISET6_W { w: self }
    }
    #[doc = "Bit 7 - Set the priority channel7"]
    #[inline(always)]
    pub fn dma_priset7(&mut self) -> DMA_PRISET7_W {
        DMA_PRISET7_W { w: self }
    }
    #[doc = "Bit 8 - Set the priority channel8"]
    #[inline(always)]
    pub fn dma_priset8(&mut self) -> DMA_PRISET8_W {
        DMA_PRISET8_W { w: self }
    }
    #[doc = "Bit 9 - Set the priority channel9"]
    #[inline(always)]
    pub fn dma_priset9(&mut self) -> DMA_PRISET9_W {
        DMA_PRISET9_W { w: self }
    }
    #[doc = "Bit 10 - Set the priority channel10"]
    #[inline(always)]
    pub fn dma_priset10(&mut self) -> DMA_PRISET10_W {
        DMA_PRISET10_W { w: self }
    }
    #[doc = "Bit 11 - Set the priority channel11"]
    #[inline(always)]
    pub fn dma_priset11(&mut self) -> DMA_PRISET11_W {
        DMA_PRISET11_W { w: self }
    }
    #[doc = "Bit 12 - Set the priority channel12"]
    #[inline(always)]
    pub fn dma_priset12(&mut self) -> DMA_PRISET12_W {
        DMA_PRISET12_W { w: self }
    }
    #[doc = "Bit 13 - Set the priority channel13"]
    #[inline(always)]
    pub fn dma_priset13(&mut self) -> DMA_PRISET13_W {
        DMA_PRISET13_W { w: self }
    }
    #[doc = "Bit 14 - Set the priority channel14"]
    #[inline(always)]
    pub fn dma_priset14(&mut self) -> DMA_PRISET14_W {
        DMA_PRISET14_W { w: self }
    }
    #[doc = "Bit 15 - Set the priority channel15"]
    #[inline(always)]
    pub fn dma_priset15(&mut self) -> DMA_PRISET15_W {
        DMA_PRISET15_W { w: self }
    }
    #[doc = "Bit 16 - Set the priority channel16"]
    #[inline(always)]
    pub fn dma_priset16(&mut self) -> DMA_PRISET16_W {
        DMA_PRISET16_W { w: self }
    }
    #[doc = "Bit 17 - Set the priority channel17"]
    #[inline(always)]
    pub fn dma_priset17(&mut self) -> DMA_PRISET17_W {
        DMA_PRISET17_W { w: self }
    }
    #[doc = "Bit 18 - Set the priority channel18"]
    #[inline(always)]
    pub fn dma_priset18(&mut self) -> DMA_PRISET18_W {
        DMA_PRISET18_W { w: self }
    }
    #[doc = "Bit 19 - Set the priority channel19"]
    #[inline(always)]
    pub fn dma_priset19(&mut self) -> DMA_PRISET19_W {
        DMA_PRISET19_W { w: self }
    }
    #[doc = "Bit 20 - Set the priority channel20"]
    #[inline(always)]
    pub fn dma_priset20(&mut self) -> DMA_PRISET20_W {
        DMA_PRISET20_W { w: self }
    }
    #[doc = "Bit 21 - Set the priority channel21"]
    #[inline(always)]
    pub fn dma_priset21(&mut self) -> DMA_PRISET21_W {
        DMA_PRISET21_W { w: self }
    }
    #[doc = "Bit 22 - Set the priority channel22"]
    #[inline(always)]
    pub fn dma_priset22(&mut self) -> DMA_PRISET22_W {
        DMA_PRISET22_W { w: self }
    }
    #[doc = "Bit 23 - Set the priority channel23"]
    #[inline(always)]
    pub fn dma_priset23(&mut self) -> DMA_PRISET23_W {
        DMA_PRISET23_W { w: self }
    }
}
