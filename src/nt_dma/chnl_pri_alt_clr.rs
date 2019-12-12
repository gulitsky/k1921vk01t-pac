#[doc = "Writer for register CHNL_PRI_ALT_CLR"]
pub type W = crate::W<u32, super::CHNL_PRI_ALT_CLR>;
#[doc = "Register CHNL_PRI_ALT_CLR `reset()`'s with value 0"]
impl crate::ResetValue for super::CHNL_PRI_ALT_CLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `DMA_ALTCLR0`"]
pub struct DMA_ALTCLR0_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ALTCLR0_W<'a> {
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
#[doc = "Write proxy for field `DMA_ALTCLR1`"]
pub struct DMA_ALTCLR1_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ALTCLR1_W<'a> {
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
#[doc = "Write proxy for field `DMA_ALTCLR2`"]
pub struct DMA_ALTCLR2_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ALTCLR2_W<'a> {
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
#[doc = "Write proxy for field `DMA_ALTCLR3`"]
pub struct DMA_ALTCLR3_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ALTCLR3_W<'a> {
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
#[doc = "Write proxy for field `DMA_ALTCLR4`"]
pub struct DMA_ALTCLR4_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ALTCLR4_W<'a> {
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
#[doc = "Write proxy for field `DMA_ALTCLR5`"]
pub struct DMA_ALTCLR5_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ALTCLR5_W<'a> {
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
#[doc = "Write proxy for field `DMA_ALTCLR6`"]
pub struct DMA_ALTCLR6_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ALTCLR6_W<'a> {
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
#[doc = "Write proxy for field `DMA_ALTCLR7`"]
pub struct DMA_ALTCLR7_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ALTCLR7_W<'a> {
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
#[doc = "Write proxy for field `DMA_ALTCLR8`"]
pub struct DMA_ALTCLR8_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ALTCLR8_W<'a> {
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
#[doc = "Write proxy for field `DMA_ALTCLR9`"]
pub struct DMA_ALTCLR9_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ALTCLR9_W<'a> {
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
#[doc = "Write proxy for field `DMA_ALTCLR10`"]
pub struct DMA_ALTCLR10_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ALTCLR10_W<'a> {
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
#[doc = "Write proxy for field `DMA_ALTCLR11`"]
pub struct DMA_ALTCLR11_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ALTCLR11_W<'a> {
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
#[doc = "Write proxy for field `DMA_ALTCLR12`"]
pub struct DMA_ALTCLR12_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ALTCLR12_W<'a> {
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
#[doc = "Write proxy for field `DMA_ALTCLR13`"]
pub struct DMA_ALTCLR13_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ALTCLR13_W<'a> {
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
#[doc = "Write proxy for field `DMA_ALTCLR14`"]
pub struct DMA_ALTCLR14_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ALTCLR14_W<'a> {
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
#[doc = "Write proxy for field `DMA_ALTCLR15`"]
pub struct DMA_ALTCLR15_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ALTCLR15_W<'a> {
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
#[doc = "Write proxy for field `DMA_ALTCLR16`"]
pub struct DMA_ALTCLR16_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ALTCLR16_W<'a> {
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
#[doc = "Write proxy for field `DMA_ALTCLR17`"]
pub struct DMA_ALTCLR17_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ALTCLR17_W<'a> {
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
#[doc = "Write proxy for field `DMA_ALTCLR18`"]
pub struct DMA_ALTCLR18_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ALTCLR18_W<'a> {
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
#[doc = "Write proxy for field `DMA_ALTCLR19`"]
pub struct DMA_ALTCLR19_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ALTCLR19_W<'a> {
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
#[doc = "Write proxy for field `DMA_ALTCLR20`"]
pub struct DMA_ALTCLR20_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ALTCLR20_W<'a> {
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
#[doc = "Write proxy for field `DMA_ALTCLR21`"]
pub struct DMA_ALTCLR21_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ALTCLR21_W<'a> {
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
#[doc = "Write proxy for field `DMA_ALTCLR22`"]
pub struct DMA_ALTCLR22_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ALTCLR22_W<'a> {
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
#[doc = "Write proxy for field `DMA_ALTCLR23`"]
pub struct DMA_ALTCLR23_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ALTCLR23_W<'a> {
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
    #[doc = "Bit 0 - Clear primary / alternate channel control data structure"]
    #[inline(always)]
    pub fn dma_altclr0(&mut self) -> DMA_ALTCLR0_W {
        DMA_ALTCLR0_W { w: self }
    }
    #[doc = "Bit 1 - Clear primary / alternate channel control data structure"]
    #[inline(always)]
    pub fn dma_altclr1(&mut self) -> DMA_ALTCLR1_W {
        DMA_ALTCLR1_W { w: self }
    }
    #[doc = "Bit 2 - Clear primary / alternate channel control data structure"]
    #[inline(always)]
    pub fn dma_altclr2(&mut self) -> DMA_ALTCLR2_W {
        DMA_ALTCLR2_W { w: self }
    }
    #[doc = "Bit 3 - Clear primary / alternate channel control data structure"]
    #[inline(always)]
    pub fn dma_altclr3(&mut self) -> DMA_ALTCLR3_W {
        DMA_ALTCLR3_W { w: self }
    }
    #[doc = "Bit 4 - Clear primary / alternate channel control data structure"]
    #[inline(always)]
    pub fn dma_altclr4(&mut self) -> DMA_ALTCLR4_W {
        DMA_ALTCLR4_W { w: self }
    }
    #[doc = "Bit 5 - Clear primary / alternate channel control data structure"]
    #[inline(always)]
    pub fn dma_altclr5(&mut self) -> DMA_ALTCLR5_W {
        DMA_ALTCLR5_W { w: self }
    }
    #[doc = "Bit 6 - Clear primary / alternate channel control data structure"]
    #[inline(always)]
    pub fn dma_altclr6(&mut self) -> DMA_ALTCLR6_W {
        DMA_ALTCLR6_W { w: self }
    }
    #[doc = "Bit 7 - Clear primary / alternate channel control data structure"]
    #[inline(always)]
    pub fn dma_altclr7(&mut self) -> DMA_ALTCLR7_W {
        DMA_ALTCLR7_W { w: self }
    }
    #[doc = "Bit 8 - Clear primary / alternate channel control data structure"]
    #[inline(always)]
    pub fn dma_altclr8(&mut self) -> DMA_ALTCLR8_W {
        DMA_ALTCLR8_W { w: self }
    }
    #[doc = "Bit 9 - Clear primary / alternate channel control data structure"]
    #[inline(always)]
    pub fn dma_altclr9(&mut self) -> DMA_ALTCLR9_W {
        DMA_ALTCLR9_W { w: self }
    }
    #[doc = "Bit 10 - Clear primary / alternate channel control data structure"]
    #[inline(always)]
    pub fn dma_altclr10(&mut self) -> DMA_ALTCLR10_W {
        DMA_ALTCLR10_W { w: self }
    }
    #[doc = "Bit 11 - Clear primary / alternate channel control data structure"]
    #[inline(always)]
    pub fn dma_altclr11(&mut self) -> DMA_ALTCLR11_W {
        DMA_ALTCLR11_W { w: self }
    }
    #[doc = "Bit 12 - Clear primary / alternate channel control data structure"]
    #[inline(always)]
    pub fn dma_altclr12(&mut self) -> DMA_ALTCLR12_W {
        DMA_ALTCLR12_W { w: self }
    }
    #[doc = "Bit 13 - Clear primary / alternate channel control data structure"]
    #[inline(always)]
    pub fn dma_altclr13(&mut self) -> DMA_ALTCLR13_W {
        DMA_ALTCLR13_W { w: self }
    }
    #[doc = "Bit 14 - Clear primary / alternate channel control data structure"]
    #[inline(always)]
    pub fn dma_altclr14(&mut self) -> DMA_ALTCLR14_W {
        DMA_ALTCLR14_W { w: self }
    }
    #[doc = "Bit 15 - Clear primary / alternate channel control data structure"]
    #[inline(always)]
    pub fn dma_altclr15(&mut self) -> DMA_ALTCLR15_W {
        DMA_ALTCLR15_W { w: self }
    }
    #[doc = "Bit 16 - Clear primary / alternate channel control data structure"]
    #[inline(always)]
    pub fn dma_altclr16(&mut self) -> DMA_ALTCLR16_W {
        DMA_ALTCLR16_W { w: self }
    }
    #[doc = "Bit 17 - Clear primary / alternate channel control data structure"]
    #[inline(always)]
    pub fn dma_altclr17(&mut self) -> DMA_ALTCLR17_W {
        DMA_ALTCLR17_W { w: self }
    }
    #[doc = "Bit 18 - Clear primary / alternate channel control data structure"]
    #[inline(always)]
    pub fn dma_altclr18(&mut self) -> DMA_ALTCLR18_W {
        DMA_ALTCLR18_W { w: self }
    }
    #[doc = "Bit 19 - Clear primary / alternate channel control data structure"]
    #[inline(always)]
    pub fn dma_altclr19(&mut self) -> DMA_ALTCLR19_W {
        DMA_ALTCLR19_W { w: self }
    }
    #[doc = "Bit 20 - Clear primary / alternate channel control data structure"]
    #[inline(always)]
    pub fn dma_altclr20(&mut self) -> DMA_ALTCLR20_W {
        DMA_ALTCLR20_W { w: self }
    }
    #[doc = "Bit 21 - Clear primary / alternate channel control data structure"]
    #[inline(always)]
    pub fn dma_altclr21(&mut self) -> DMA_ALTCLR21_W {
        DMA_ALTCLR21_W { w: self }
    }
    #[doc = "Bit 22 - Clear primary / alternate channel control data structure"]
    #[inline(always)]
    pub fn dma_altclr22(&mut self) -> DMA_ALTCLR22_W {
        DMA_ALTCLR22_W { w: self }
    }
    #[doc = "Bit 23 - Clear primary / alternate channel control data structure"]
    #[inline(always)]
    pub fn dma_altclr23(&mut self) -> DMA_ALTCLR23_W {
        DMA_ALTCLR23_W { w: self }
    }
}
