#[doc = "Writer for register CHNL_REQ_MASK_CLR"]
pub type W = crate::W<u32, super::CHNL_REQ_MASK_CLR>;
#[doc = "Register CHNL_REQ_MASK_CLR `reset()`'s with value 0"]
impl crate::ResetValue for super::CHNL_REQ_MASK_CLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `DMA_REQMASKCLR0`"]
pub struct DMA_REQMASKCLR0_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_REQMASKCLR0_W<'a> {
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
#[doc = "Write proxy for field `DMA_REQMASKCLR1`"]
pub struct DMA_REQMASKCLR1_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_REQMASKCLR1_W<'a> {
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
#[doc = "Write proxy for field `DMA_REQMASKCLR2`"]
pub struct DMA_REQMASKCLR2_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_REQMASKCLR2_W<'a> {
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
#[doc = "Write proxy for field `DMA_REQMASKCLR3`"]
pub struct DMA_REQMASKCLR3_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_REQMASKCLR3_W<'a> {
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
#[doc = "Write proxy for field `DMA_REQMASKCLR4`"]
pub struct DMA_REQMASKCLR4_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_REQMASKCLR4_W<'a> {
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
#[doc = "Write proxy for field `DMA_REQMASKCLR5`"]
pub struct DMA_REQMASKCLR5_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_REQMASKCLR5_W<'a> {
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
#[doc = "Write proxy for field `DMA_REQMASKCLR6`"]
pub struct DMA_REQMASKCLR6_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_REQMASKCLR6_W<'a> {
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
#[doc = "Write proxy for field `DMA_REQMASKCLR7`"]
pub struct DMA_REQMASKCLR7_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_REQMASKCLR7_W<'a> {
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
#[doc = "Write proxy for field `DMA_REQMASKCLR8`"]
pub struct DMA_REQMASKCLR8_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_REQMASKCLR8_W<'a> {
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
#[doc = "Write proxy for field `DMA_REQMASKCLR9`"]
pub struct DMA_REQMASKCLR9_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_REQMASKCLR9_W<'a> {
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
#[doc = "Write proxy for field `DMA_REQMASKCLR10`"]
pub struct DMA_REQMASKCLR10_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_REQMASKCLR10_W<'a> {
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
#[doc = "Write proxy for field `DMA_REQMASKCLR11`"]
pub struct DMA_REQMASKCLR11_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_REQMASKCLR11_W<'a> {
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
#[doc = "Write proxy for field `DMA_REQMASKCLR12`"]
pub struct DMA_REQMASKCLR12_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_REQMASKCLR12_W<'a> {
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
#[doc = "Write proxy for field `DMA_REQMASKCLR13`"]
pub struct DMA_REQMASKCLR13_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_REQMASKCLR13_W<'a> {
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
#[doc = "Write proxy for field `DMA_REQMASKCLR14`"]
pub struct DMA_REQMASKCLR14_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_REQMASKCLR14_W<'a> {
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
#[doc = "Write proxy for field `DMA_REQMASKCLR15`"]
pub struct DMA_REQMASKCLR15_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_REQMASKCLR15_W<'a> {
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
#[doc = "Write proxy for field `DMA_REQMASKCLR16`"]
pub struct DMA_REQMASKCLR16_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_REQMASKCLR16_W<'a> {
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
#[doc = "Write proxy for field `DMA_REQMASKCLR17`"]
pub struct DMA_REQMASKCLR17_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_REQMASKCLR17_W<'a> {
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
#[doc = "Write proxy for field `DMA_REQMASKCLR18`"]
pub struct DMA_REQMASKCLR18_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_REQMASKCLR18_W<'a> {
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
#[doc = "Write proxy for field `DMA_REQMASKCLR19`"]
pub struct DMA_REQMASKCLR19_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_REQMASKCLR19_W<'a> {
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
#[doc = "Write proxy for field `DMA_REQMASKCLR20`"]
pub struct DMA_REQMASKCLR20_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_REQMASKCLR20_W<'a> {
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
#[doc = "Write proxy for field `DMA_REQMASKCLR21`"]
pub struct DMA_REQMASKCLR21_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_REQMASKCLR21_W<'a> {
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
#[doc = "Write proxy for field `DMA_REQMASKCLR22`"]
pub struct DMA_REQMASKCLR22_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_REQMASKCLR22_W<'a> {
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
#[doc = "Write proxy for field `DMA_REQMASKCLR23`"]
pub struct DMA_REQMASKCLR23_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_REQMASKCLR23_W<'a> {
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
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dma_reqmaskclr0(&mut self) -> DMA_REQMASKCLR0_W {
        DMA_REQMASKCLR0_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dma_reqmaskclr1(&mut self) -> DMA_REQMASKCLR1_W {
        DMA_REQMASKCLR1_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dma_reqmaskclr2(&mut self) -> DMA_REQMASKCLR2_W {
        DMA_REQMASKCLR2_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dma_reqmaskclr3(&mut self) -> DMA_REQMASKCLR3_W {
        DMA_REQMASKCLR3_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn dma_reqmaskclr4(&mut self) -> DMA_REQMASKCLR4_W {
        DMA_REQMASKCLR4_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn dma_reqmaskclr5(&mut self) -> DMA_REQMASKCLR5_W {
        DMA_REQMASKCLR5_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn dma_reqmaskclr6(&mut self) -> DMA_REQMASKCLR6_W {
        DMA_REQMASKCLR6_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn dma_reqmaskclr7(&mut self) -> DMA_REQMASKCLR7_W {
        DMA_REQMASKCLR7_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn dma_reqmaskclr8(&mut self) -> DMA_REQMASKCLR8_W {
        DMA_REQMASKCLR8_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn dma_reqmaskclr9(&mut self) -> DMA_REQMASKCLR9_W {
        DMA_REQMASKCLR9_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn dma_reqmaskclr10(&mut self) -> DMA_REQMASKCLR10_W {
        DMA_REQMASKCLR10_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn dma_reqmaskclr11(&mut self) -> DMA_REQMASKCLR11_W {
        DMA_REQMASKCLR11_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn dma_reqmaskclr12(&mut self) -> DMA_REQMASKCLR12_W {
        DMA_REQMASKCLR12_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn dma_reqmaskclr13(&mut self) -> DMA_REQMASKCLR13_W {
        DMA_REQMASKCLR13_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn dma_reqmaskclr14(&mut self) -> DMA_REQMASKCLR14_W {
        DMA_REQMASKCLR14_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn dma_reqmaskclr15(&mut self) -> DMA_REQMASKCLR15_W {
        DMA_REQMASKCLR15_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn dma_reqmaskclr16(&mut self) -> DMA_REQMASKCLR16_W {
        DMA_REQMASKCLR16_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn dma_reqmaskclr17(&mut self) -> DMA_REQMASKCLR17_W {
        DMA_REQMASKCLR17_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn dma_reqmaskclr18(&mut self) -> DMA_REQMASKCLR18_W {
        DMA_REQMASKCLR18_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn dma_reqmaskclr19(&mut self) -> DMA_REQMASKCLR19_W {
        DMA_REQMASKCLR19_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn dma_reqmaskclr20(&mut self) -> DMA_REQMASKCLR20_W {
        DMA_REQMASKCLR20_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn dma_reqmaskclr21(&mut self) -> DMA_REQMASKCLR21_W {
        DMA_REQMASKCLR21_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn dma_reqmaskclr22(&mut self) -> DMA_REQMASKCLR22_W {
        DMA_REQMASKCLR22_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn dma_reqmaskclr23(&mut self) -> DMA_REQMASKCLR23_W {
        DMA_REQMASKCLR23_W { w: self }
    }
}
