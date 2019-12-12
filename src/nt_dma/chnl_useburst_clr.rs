#[doc = "Writer for register CHNL_USEBURST_CLR"]
pub type W = crate::W<u32, super::CHNL_USEBURST_CLR>;
#[doc = "Register CHNL_USEBURST_CLR `reset()`'s with value 0"]
impl crate::ResetValue for super::CHNL_USEBURST_CLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `DMA_BURSTCLR0`"]
pub struct DMA_BURSTCLR0_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_BURSTCLR0_W<'a> {
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
#[doc = "Write proxy for field `DMA_BURSTCLR1`"]
pub struct DMA_BURSTCLR1_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_BURSTCLR1_W<'a> {
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
#[doc = "Write proxy for field `DMA_BURSTCLR2`"]
pub struct DMA_BURSTCLR2_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_BURSTCLR2_W<'a> {
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
#[doc = "Write proxy for field `DMA_BURSTCLR3`"]
pub struct DMA_BURSTCLR3_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_BURSTCLR3_W<'a> {
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
#[doc = "Write proxy for field `DMA_BURSTCLR4`"]
pub struct DMA_BURSTCLR4_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_BURSTCLR4_W<'a> {
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
#[doc = "Write proxy for field `DMA_BURSTCLR5`"]
pub struct DMA_BURSTCLR5_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_BURSTCLR5_W<'a> {
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
#[doc = "Write proxy for field `DMA_BURSTCLR6`"]
pub struct DMA_BURSTCLR6_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_BURSTCLR6_W<'a> {
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
#[doc = "Write proxy for field `DMA_BURSTCLR7`"]
pub struct DMA_BURSTCLR7_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_BURSTCLR7_W<'a> {
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
#[doc = "Write proxy for field `DMA_BURSTCLR8`"]
pub struct DMA_BURSTCLR8_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_BURSTCLR8_W<'a> {
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
#[doc = "Write proxy for field `DMA_BURSTCLR9`"]
pub struct DMA_BURSTCLR9_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_BURSTCLR9_W<'a> {
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
#[doc = "Write proxy for field `DMA_BURSTCLR10`"]
pub struct DMA_BURSTCLR10_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_BURSTCLR10_W<'a> {
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
#[doc = "Write proxy for field `DMA_BURSTCLR11`"]
pub struct DMA_BURSTCLR11_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_BURSTCLR11_W<'a> {
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
#[doc = "Write proxy for field `DMA_BURSTCLR12`"]
pub struct DMA_BURSTCLR12_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_BURSTCLR12_W<'a> {
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
#[doc = "Write proxy for field `DMA_BURSTCLR13`"]
pub struct DMA_BURSTCLR13_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_BURSTCLR13_W<'a> {
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
#[doc = "Write proxy for field `DMA_BURSTCLR14`"]
pub struct DMA_BURSTCLR14_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_BURSTCLR14_W<'a> {
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
#[doc = "Write proxy for field `DMA_BURSTCLR15`"]
pub struct DMA_BURSTCLR15_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_BURSTCLR15_W<'a> {
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
#[doc = "Write proxy for field `DMA_BURSTCLR16`"]
pub struct DMA_BURSTCLR16_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_BURSTCLR16_W<'a> {
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
#[doc = "Write proxy for field `DMA_BURSTCLR17`"]
pub struct DMA_BURSTCLR17_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_BURSTCLR17_W<'a> {
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
#[doc = "Write proxy for field `DMA_BURSTCLR18`"]
pub struct DMA_BURSTCLR18_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_BURSTCLR18_W<'a> {
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
#[doc = "Write proxy for field `DMA_BURSTCLR19`"]
pub struct DMA_BURSTCLR19_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_BURSTCLR19_W<'a> {
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
#[doc = "Write proxy for field `DMA_BURSTCLR20`"]
pub struct DMA_BURSTCLR20_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_BURSTCLR20_W<'a> {
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
#[doc = "Write proxy for field `DMA_BURSTCLR21`"]
pub struct DMA_BURSTCLR21_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_BURSTCLR21_W<'a> {
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
#[doc = "Write proxy for field `DMA_BURSTCLR22`"]
pub struct DMA_BURSTCLR22_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_BURSTCLR22_W<'a> {
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
#[doc = "Write proxy for field `DMA_BURSTCLR23`"]
pub struct DMA_BURSTCLR23_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_BURSTCLR23_W<'a> {
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
impl W {
    #[doc = "Bit 0 - reserved"]
    #[inline(always)]
    pub fn dma_burstclr0(&mut self) -> DMA_BURSTCLR0_W {
        DMA_BURSTCLR0_W { w: self }
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn dma_burstclr1(&mut self) -> DMA_BURSTCLR1_W {
        DMA_BURSTCLR1_W { w: self }
    }
    #[doc = "Bit 2 - reserved"]
    #[inline(always)]
    pub fn dma_burstclr2(&mut self) -> DMA_BURSTCLR2_W {
        DMA_BURSTCLR2_W { w: self }
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn dma_burstclr3(&mut self) -> DMA_BURSTCLR3_W {
        DMA_BURSTCLR3_W { w: self }
    }
    #[doc = "Bit 4 - reserved"]
    #[inline(always)]
    pub fn dma_burstclr4(&mut self) -> DMA_BURSTCLR4_W {
        DMA_BURSTCLR4_W { w: self }
    }
    #[doc = "Bit 5 - reserved"]
    #[inline(always)]
    pub fn dma_burstclr5(&mut self) -> DMA_BURSTCLR5_W {
        DMA_BURSTCLR5_W { w: self }
    }
    #[doc = "Bit 6 - reserved"]
    #[inline(always)]
    pub fn dma_burstclr6(&mut self) -> DMA_BURSTCLR6_W {
        DMA_BURSTCLR6_W { w: self }
    }
    #[doc = "Bit 7 - reserved"]
    #[inline(always)]
    pub fn dma_burstclr7(&mut self) -> DMA_BURSTCLR7_W {
        DMA_BURSTCLR7_W { w: self }
    }
    #[doc = "Bit 8 - reserved"]
    #[inline(always)]
    pub fn dma_burstclr8(&mut self) -> DMA_BURSTCLR8_W {
        DMA_BURSTCLR8_W { w: self }
    }
    #[doc = "Bit 9 - reserved"]
    #[inline(always)]
    pub fn dma_burstclr9(&mut self) -> DMA_BURSTCLR9_W {
        DMA_BURSTCLR9_W { w: self }
    }
    #[doc = "Bit 10 - reserved"]
    #[inline(always)]
    pub fn dma_burstclr10(&mut self) -> DMA_BURSTCLR10_W {
        DMA_BURSTCLR10_W { w: self }
    }
    #[doc = "Bit 11 - reserved"]
    #[inline(always)]
    pub fn dma_burstclr11(&mut self) -> DMA_BURSTCLR11_W {
        DMA_BURSTCLR11_W { w: self }
    }
    #[doc = "Bit 12 - reserved"]
    #[inline(always)]
    pub fn dma_burstclr12(&mut self) -> DMA_BURSTCLR12_W {
        DMA_BURSTCLR12_W { w: self }
    }
    #[doc = "Bit 13 - reserved"]
    #[inline(always)]
    pub fn dma_burstclr13(&mut self) -> DMA_BURSTCLR13_W {
        DMA_BURSTCLR13_W { w: self }
    }
    #[doc = "Bit 14 - reserved"]
    #[inline(always)]
    pub fn dma_burstclr14(&mut self) -> DMA_BURSTCLR14_W {
        DMA_BURSTCLR14_W { w: self }
    }
    #[doc = "Bit 15 - reserved"]
    #[inline(always)]
    pub fn dma_burstclr15(&mut self) -> DMA_BURSTCLR15_W {
        DMA_BURSTCLR15_W { w: self }
    }
    #[doc = "Bit 16 - reserved"]
    #[inline(always)]
    pub fn dma_burstclr16(&mut self) -> DMA_BURSTCLR16_W {
        DMA_BURSTCLR16_W { w: self }
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn dma_burstclr17(&mut self) -> DMA_BURSTCLR17_W {
        DMA_BURSTCLR17_W { w: self }
    }
    #[doc = "Bit 18 - reserved"]
    #[inline(always)]
    pub fn dma_burstclr18(&mut self) -> DMA_BURSTCLR18_W {
        DMA_BURSTCLR18_W { w: self }
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn dma_burstclr19(&mut self) -> DMA_BURSTCLR19_W {
        DMA_BURSTCLR19_W { w: self }
    }
    #[doc = "Bit 20 - reserved"]
    #[inline(always)]
    pub fn dma_burstclr20(&mut self) -> DMA_BURSTCLR20_W {
        DMA_BURSTCLR20_W { w: self }
    }
    #[doc = "Bit 21 - reserved"]
    #[inline(always)]
    pub fn dma_burstclr21(&mut self) -> DMA_BURSTCLR21_W {
        DMA_BURSTCLR21_W { w: self }
    }
    #[doc = "Bit 22 - reserved"]
    #[inline(always)]
    pub fn dma_burstclr22(&mut self) -> DMA_BURSTCLR22_W {
        DMA_BURSTCLR22_W { w: self }
    }
    #[doc = "Bit 23 - reserved"]
    #[inline(always)]
    pub fn dma_burstclr23(&mut self) -> DMA_BURSTCLR23_W {
        DMA_BURSTCLR23_W { w: self }
    }
}
