#[doc = "Writer for register CHNL_PRIORITY_CLR"]
pub type W = crate::W<u32, super::CHNL_PRIORITY_CLR>;
#[doc = "Register CHNL_PRIORITY_CLR `reset()`'s with value 0"]
impl crate::ResetValue for super::CHNL_PRIORITY_CLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `DMA_PRICLR0`"]
pub struct DMA_PRICLR0_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_PRICLR0_W<'a> {
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
#[doc = "Write proxy for field `DMA_PRICLR1`"]
pub struct DMA_PRICLR1_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_PRICLR1_W<'a> {
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
#[doc = "Write proxy for field `DMA_PRICLR2`"]
pub struct DMA_PRICLR2_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_PRICLR2_W<'a> {
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
#[doc = "Write proxy for field `DMA_PRICLR3`"]
pub struct DMA_PRICLR3_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_PRICLR3_W<'a> {
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
#[doc = "Write proxy for field `DMA_PRICLR4`"]
pub struct DMA_PRICLR4_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_PRICLR4_W<'a> {
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
#[doc = "Write proxy for field `DMA_PRICLR5`"]
pub struct DMA_PRICLR5_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_PRICLR5_W<'a> {
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
#[doc = "Write proxy for field `DMA_PRICLR6`"]
pub struct DMA_PRICLR6_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_PRICLR6_W<'a> {
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
#[doc = "Write proxy for field `DMA_PRICLR7`"]
pub struct DMA_PRICLR7_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_PRICLR7_W<'a> {
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
#[doc = "Write proxy for field `DMA_PRICLR8`"]
pub struct DMA_PRICLR8_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_PRICLR8_W<'a> {
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
#[doc = "Write proxy for field `DMA_PRICLR9`"]
pub struct DMA_PRICLR9_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_PRICLR9_W<'a> {
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
#[doc = "Write proxy for field `DMA_PRICLR10`"]
pub struct DMA_PRICLR10_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_PRICLR10_W<'a> {
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
#[doc = "Write proxy for field `DMA_PRICLR11`"]
pub struct DMA_PRICLR11_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_PRICLR11_W<'a> {
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
#[doc = "Write proxy for field `DMA_PRICLR12`"]
pub struct DMA_PRICLR12_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_PRICLR12_W<'a> {
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
#[doc = "Write proxy for field `DMA_PRICLR13`"]
pub struct DMA_PRICLR13_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_PRICLR13_W<'a> {
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
#[doc = "Write proxy for field `DMA_PRICLR14`"]
pub struct DMA_PRICLR14_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_PRICLR14_W<'a> {
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
#[doc = "Write proxy for field `DMA_PRICLR15`"]
pub struct DMA_PRICLR15_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_PRICLR15_W<'a> {
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
#[doc = "Write proxy for field `DMA_PRICLR16`"]
pub struct DMA_PRICLR16_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_PRICLR16_W<'a> {
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
#[doc = "Write proxy for field `DMA_PRICLR17`"]
pub struct DMA_PRICLR17_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_PRICLR17_W<'a> {
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
#[doc = "Write proxy for field `DMA_PRICLR18`"]
pub struct DMA_PRICLR18_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_PRICLR18_W<'a> {
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
#[doc = "Write proxy for field `DMA_PRICLR19`"]
pub struct DMA_PRICLR19_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_PRICLR19_W<'a> {
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
#[doc = "Write proxy for field `DMA_PRICLR20`"]
pub struct DMA_PRICLR20_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_PRICLR20_W<'a> {
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
#[doc = "Write proxy for field `DMA_PRICLR21`"]
pub struct DMA_PRICLR21_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_PRICLR21_W<'a> {
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
#[doc = "Write proxy for field `DMA_PRICLR22`"]
pub struct DMA_PRICLR22_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_PRICLR22_W<'a> {
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
#[doc = "Write proxy for field `DMA_PRICLR23`"]
pub struct DMA_PRICLR23_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_PRICLR23_W<'a> {
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
    #[doc = "Bit 0 - Clear the priority channel0"]
    #[inline(always)]
    pub fn dma_priclr0(&mut self) -> DMA_PRICLR0_W {
        DMA_PRICLR0_W { w: self }
    }
    #[doc = "Bit 1 - Clear the priority channel1"]
    #[inline(always)]
    pub fn dma_priclr1(&mut self) -> DMA_PRICLR1_W {
        DMA_PRICLR1_W { w: self }
    }
    #[doc = "Bit 2 - Clear the priority channel2"]
    #[inline(always)]
    pub fn dma_priclr2(&mut self) -> DMA_PRICLR2_W {
        DMA_PRICLR2_W { w: self }
    }
    #[doc = "Bit 3 - Clear the priority channel3"]
    #[inline(always)]
    pub fn dma_priclr3(&mut self) -> DMA_PRICLR3_W {
        DMA_PRICLR3_W { w: self }
    }
    #[doc = "Bit 4 - Clear the priority channel4"]
    #[inline(always)]
    pub fn dma_priclr4(&mut self) -> DMA_PRICLR4_W {
        DMA_PRICLR4_W { w: self }
    }
    #[doc = "Bit 5 - Clear the priority channel5"]
    #[inline(always)]
    pub fn dma_priclr5(&mut self) -> DMA_PRICLR5_W {
        DMA_PRICLR5_W { w: self }
    }
    #[doc = "Bit 6 - Clear the priority channel6"]
    #[inline(always)]
    pub fn dma_priclr6(&mut self) -> DMA_PRICLR6_W {
        DMA_PRICLR6_W { w: self }
    }
    #[doc = "Bit 7 - Clear the priority channel7"]
    #[inline(always)]
    pub fn dma_priclr7(&mut self) -> DMA_PRICLR7_W {
        DMA_PRICLR7_W { w: self }
    }
    #[doc = "Bit 8 - Clear the priority channel8"]
    #[inline(always)]
    pub fn dma_priclr8(&mut self) -> DMA_PRICLR8_W {
        DMA_PRICLR8_W { w: self }
    }
    #[doc = "Bit 9 - Clear the priority channel9"]
    #[inline(always)]
    pub fn dma_priclr9(&mut self) -> DMA_PRICLR9_W {
        DMA_PRICLR9_W { w: self }
    }
    #[doc = "Bit 10 - Clear the priority channel10"]
    #[inline(always)]
    pub fn dma_priclr10(&mut self) -> DMA_PRICLR10_W {
        DMA_PRICLR10_W { w: self }
    }
    #[doc = "Bit 11 - Clear the priority channel11"]
    #[inline(always)]
    pub fn dma_priclr11(&mut self) -> DMA_PRICLR11_W {
        DMA_PRICLR11_W { w: self }
    }
    #[doc = "Bit 12 - Clear the priority channel12"]
    #[inline(always)]
    pub fn dma_priclr12(&mut self) -> DMA_PRICLR12_W {
        DMA_PRICLR12_W { w: self }
    }
    #[doc = "Bit 13 - Clear the priority channel13"]
    #[inline(always)]
    pub fn dma_priclr13(&mut self) -> DMA_PRICLR13_W {
        DMA_PRICLR13_W { w: self }
    }
    #[doc = "Bit 14 - Clear the priority channel14"]
    #[inline(always)]
    pub fn dma_priclr14(&mut self) -> DMA_PRICLR14_W {
        DMA_PRICLR14_W { w: self }
    }
    #[doc = "Bit 15 - Clear the priority channel15"]
    #[inline(always)]
    pub fn dma_priclr15(&mut self) -> DMA_PRICLR15_W {
        DMA_PRICLR15_W { w: self }
    }
    #[doc = "Bit 16 - Clear the priority channel16"]
    #[inline(always)]
    pub fn dma_priclr16(&mut self) -> DMA_PRICLR16_W {
        DMA_PRICLR16_W { w: self }
    }
    #[doc = "Bit 17 - Clear the priority channel17"]
    #[inline(always)]
    pub fn dma_priclr17(&mut self) -> DMA_PRICLR17_W {
        DMA_PRICLR17_W { w: self }
    }
    #[doc = "Bit 18 - Clear the priority channel18"]
    #[inline(always)]
    pub fn dma_priclr18(&mut self) -> DMA_PRICLR18_W {
        DMA_PRICLR18_W { w: self }
    }
    #[doc = "Bit 19 - Clear the priority channel19"]
    #[inline(always)]
    pub fn dma_priclr19(&mut self) -> DMA_PRICLR19_W {
        DMA_PRICLR19_W { w: self }
    }
    #[doc = "Bit 20 - Clear the priority channel20"]
    #[inline(always)]
    pub fn dma_priclr20(&mut self) -> DMA_PRICLR20_W {
        DMA_PRICLR20_W { w: self }
    }
    #[doc = "Bit 21 - Clear the priority channel21"]
    #[inline(always)]
    pub fn dma_priclr21(&mut self) -> DMA_PRICLR21_W {
        DMA_PRICLR21_W { w: self }
    }
    #[doc = "Bit 22 - Clear the priority channel22"]
    #[inline(always)]
    pub fn dma_priclr22(&mut self) -> DMA_PRICLR22_W {
        DMA_PRICLR22_W { w: self }
    }
    #[doc = "Bit 23 - Clear the priority channel23"]
    #[inline(always)]
    pub fn dma_priclr23(&mut self) -> DMA_PRICLR23_W {
        DMA_PRICLR23_W { w: self }
    }
}
