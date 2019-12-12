#[doc = "Writer for register CHNL_ENABLE_CLR"]
pub type W = crate::W<u32, super::CHNL_ENABLE_CLR>;
#[doc = "Register CHNL_ENABLE_CLR `reset()`'s with value 0"]
impl crate::ResetValue for super::CHNL_ENABLE_CLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `DMA_ENCLR0`"]
pub struct DMA_ENCLR0_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ENCLR0_W<'a> {
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
#[doc = "Write proxy for field `DMA_ENCLR1`"]
pub struct DMA_ENCLR1_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ENCLR1_W<'a> {
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
#[doc = "Write proxy for field `DMA_ENCLR2`"]
pub struct DMA_ENCLR2_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ENCLR2_W<'a> {
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
#[doc = "Write proxy for field `DMA_ENCLR3`"]
pub struct DMA_ENCLR3_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ENCLR3_W<'a> {
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
#[doc = "Write proxy for field `DMA_ENCLR4`"]
pub struct DMA_ENCLR4_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ENCLR4_W<'a> {
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
#[doc = "Write proxy for field `DMA_ENCLR5`"]
pub struct DMA_ENCLR5_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ENCLR5_W<'a> {
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
#[doc = "Write proxy for field `DMA_ENCLR6`"]
pub struct DMA_ENCLR6_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ENCLR6_W<'a> {
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
#[doc = "Write proxy for field `DMA_ENCLR7`"]
pub struct DMA_ENCLR7_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ENCLR7_W<'a> {
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
#[doc = "Write proxy for field `DMA_ENCLR8`"]
pub struct DMA_ENCLR8_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ENCLR8_W<'a> {
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
#[doc = "Write proxy for field `DMA_ENCLR9`"]
pub struct DMA_ENCLR9_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ENCLR9_W<'a> {
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
#[doc = "Write proxy for field `DMA_ENCLR10`"]
pub struct DMA_ENCLR10_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ENCLR10_W<'a> {
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
#[doc = "Write proxy for field `DMA_ENCLR11`"]
pub struct DMA_ENCLR11_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ENCLR11_W<'a> {
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
#[doc = "Write proxy for field `DMA_ENCLR12`"]
pub struct DMA_ENCLR12_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ENCLR12_W<'a> {
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
#[doc = "Write proxy for field `DMA_ENCLR13`"]
pub struct DMA_ENCLR13_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ENCLR13_W<'a> {
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
#[doc = "Write proxy for field `DMA_ENCLR14`"]
pub struct DMA_ENCLR14_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ENCLR14_W<'a> {
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
#[doc = "Write proxy for field `DMA_ENCLR15`"]
pub struct DMA_ENCLR15_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ENCLR15_W<'a> {
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
#[doc = "Write proxy for field `DMA_ENCLR16`"]
pub struct DMA_ENCLR16_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ENCLR16_W<'a> {
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
#[doc = "Write proxy for field `DMA_ENCLR17`"]
pub struct DMA_ENCLR17_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ENCLR17_W<'a> {
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
#[doc = "Write proxy for field `DMA_ENCLR18`"]
pub struct DMA_ENCLR18_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ENCLR18_W<'a> {
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
#[doc = "Write proxy for field `DMA_ENCLR19`"]
pub struct DMA_ENCLR19_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ENCLR19_W<'a> {
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
#[doc = "Write proxy for field `DMA_ENCLR20`"]
pub struct DMA_ENCLR20_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ENCLR20_W<'a> {
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
#[doc = "Write proxy for field `DMA_ENCLR21`"]
pub struct DMA_ENCLR21_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ENCLR21_W<'a> {
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
#[doc = "Write proxy for field `DMA_ENCLR22`"]
pub struct DMA_ENCLR22_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ENCLR22_W<'a> {
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
#[doc = "Write proxy for field `DMA_ENCLR23`"]
pub struct DMA_ENCLR23_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ENCLR23_W<'a> {
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
    #[doc = "Bit 0 - Enables you to set the dma_active\\[ \\]"]
    #[inline(always)]
    pub fn dma_enclr0(&mut self) -> DMA_ENCLR0_W {
        DMA_ENCLR0_W { w: self }
    }
    #[doc = "Bit 1 - Enables you to set the dma_active\\[ \\]"]
    #[inline(always)]
    pub fn dma_enclr1(&mut self) -> DMA_ENCLR1_W {
        DMA_ENCLR1_W { w: self }
    }
    #[doc = "Bit 2 - Enables you to set the dma_active\\[ \\]"]
    #[inline(always)]
    pub fn dma_enclr2(&mut self) -> DMA_ENCLR2_W {
        DMA_ENCLR2_W { w: self }
    }
    #[doc = "Bit 3 - Enables you to set the dma_active\\[ \\]"]
    #[inline(always)]
    pub fn dma_enclr3(&mut self) -> DMA_ENCLR3_W {
        DMA_ENCLR3_W { w: self }
    }
    #[doc = "Bit 4 - Enables you to set the dma_active\\[ \\]"]
    #[inline(always)]
    pub fn dma_enclr4(&mut self) -> DMA_ENCLR4_W {
        DMA_ENCLR4_W { w: self }
    }
    #[doc = "Bit 5 - Enables you to set the dma_active\\[ \\]"]
    #[inline(always)]
    pub fn dma_enclr5(&mut self) -> DMA_ENCLR5_W {
        DMA_ENCLR5_W { w: self }
    }
    #[doc = "Bit 6 - Enables you to set the dma_active\\[ \\]"]
    #[inline(always)]
    pub fn dma_enclr6(&mut self) -> DMA_ENCLR6_W {
        DMA_ENCLR6_W { w: self }
    }
    #[doc = "Bit 7 - Enables you to set the dma_active\\[ \\]"]
    #[inline(always)]
    pub fn dma_enclr7(&mut self) -> DMA_ENCLR7_W {
        DMA_ENCLR7_W { w: self }
    }
    #[doc = "Bit 8 - Enables you to set the dma_active\\[ \\]"]
    #[inline(always)]
    pub fn dma_enclr8(&mut self) -> DMA_ENCLR8_W {
        DMA_ENCLR8_W { w: self }
    }
    #[doc = "Bit 9 - Enables you to set the dma_active\\[ \\]"]
    #[inline(always)]
    pub fn dma_enclr9(&mut self) -> DMA_ENCLR9_W {
        DMA_ENCLR9_W { w: self }
    }
    #[doc = "Bit 10 - Enables you to set the dma_active\\[ \\]"]
    #[inline(always)]
    pub fn dma_enclr10(&mut self) -> DMA_ENCLR10_W {
        DMA_ENCLR10_W { w: self }
    }
    #[doc = "Bit 11 - Enables you to set the dma_active\\[ \\]"]
    #[inline(always)]
    pub fn dma_enclr11(&mut self) -> DMA_ENCLR11_W {
        DMA_ENCLR11_W { w: self }
    }
    #[doc = "Bit 12 - Enables you to set the dma_active\\[ \\]"]
    #[inline(always)]
    pub fn dma_enclr12(&mut self) -> DMA_ENCLR12_W {
        DMA_ENCLR12_W { w: self }
    }
    #[doc = "Bit 13 - Enables you to set the dma_active\\[ \\]"]
    #[inline(always)]
    pub fn dma_enclr13(&mut self) -> DMA_ENCLR13_W {
        DMA_ENCLR13_W { w: self }
    }
    #[doc = "Bit 14 - Enables you to set the dma_active\\[ \\]"]
    #[inline(always)]
    pub fn dma_enclr14(&mut self) -> DMA_ENCLR14_W {
        DMA_ENCLR14_W { w: self }
    }
    #[doc = "Bit 15 - Enables you to set the dma_active\\[ \\]"]
    #[inline(always)]
    pub fn dma_enclr15(&mut self) -> DMA_ENCLR15_W {
        DMA_ENCLR15_W { w: self }
    }
    #[doc = "Bit 16 - Enables you to set the dma_active\\[ \\]"]
    #[inline(always)]
    pub fn dma_enclr16(&mut self) -> DMA_ENCLR16_W {
        DMA_ENCLR16_W { w: self }
    }
    #[doc = "Bit 17 - Enables you to set the dma_active\\[ \\]"]
    #[inline(always)]
    pub fn dma_enclr17(&mut self) -> DMA_ENCLR17_W {
        DMA_ENCLR17_W { w: self }
    }
    #[doc = "Bit 18 - Enables you to set the dma_active\\[ \\]"]
    #[inline(always)]
    pub fn dma_enclr18(&mut self) -> DMA_ENCLR18_W {
        DMA_ENCLR18_W { w: self }
    }
    #[doc = "Bit 19 - Enables you to set the dma_active\\[ \\]"]
    #[inline(always)]
    pub fn dma_enclr19(&mut self) -> DMA_ENCLR19_W {
        DMA_ENCLR19_W { w: self }
    }
    #[doc = "Bit 20 - Enables you to set the dma_active\\[ \\]"]
    #[inline(always)]
    pub fn dma_enclr20(&mut self) -> DMA_ENCLR20_W {
        DMA_ENCLR20_W { w: self }
    }
    #[doc = "Bit 21 - Enables you to set the dma_active\\[ \\]"]
    #[inline(always)]
    pub fn dma_enclr21(&mut self) -> DMA_ENCLR21_W {
        DMA_ENCLR21_W { w: self }
    }
    #[doc = "Bit 22 - Enables you to set the dma_active\\[ \\]"]
    #[inline(always)]
    pub fn dma_enclr22(&mut self) -> DMA_ENCLR22_W {
        DMA_ENCLR22_W { w: self }
    }
    #[doc = "Bit 23 - Enables you to set the dma_active\\[ \\]"]
    #[inline(always)]
    pub fn dma_enclr23(&mut self) -> DMA_ENCLR23_W {
        DMA_ENCLR23_W { w: self }
    }
}
