#[doc = "Writer for register CHNL_SW_REQUEST"]
pub type W = crate::W<u32, super::CHNL_SW_REQUEST>;
#[doc = "Register CHNL_SW_REQUEST `reset()`'s with value 0"]
impl crate::ResetValue for super::CHNL_SW_REQUEST {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `DMA_SWREQ0`"]
pub struct DMA_SWREQ0_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_SWREQ0_W<'a> {
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
#[doc = "Write proxy for field `DMA_SWREQ1`"]
pub struct DMA_SWREQ1_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_SWREQ1_W<'a> {
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
#[doc = "Write proxy for field `DMA_SWREQ2`"]
pub struct DMA_SWREQ2_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_SWREQ2_W<'a> {
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
#[doc = "Write proxy for field `DMA_SWREQ3`"]
pub struct DMA_SWREQ3_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_SWREQ3_W<'a> {
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
#[doc = "Write proxy for field `DMA_SWREQ4`"]
pub struct DMA_SWREQ4_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_SWREQ4_W<'a> {
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
#[doc = "Write proxy for field `DMA_SWREQ5`"]
pub struct DMA_SWREQ5_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_SWREQ5_W<'a> {
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
#[doc = "Write proxy for field `DMA_SWREQ6`"]
pub struct DMA_SWREQ6_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_SWREQ6_W<'a> {
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
#[doc = "Write proxy for field `DMA_SWREQ7`"]
pub struct DMA_SWREQ7_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_SWREQ7_W<'a> {
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
#[doc = "Write proxy for field `DMA_SWREQ8`"]
pub struct DMA_SWREQ8_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_SWREQ8_W<'a> {
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
#[doc = "Write proxy for field `DMA_SWREQ9`"]
pub struct DMA_SWREQ9_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_SWREQ9_W<'a> {
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
#[doc = "Write proxy for field `DMA_SWREQ10`"]
pub struct DMA_SWREQ10_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_SWREQ10_W<'a> {
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
#[doc = "Write proxy for field `DMA_SWREQ11`"]
pub struct DMA_SWREQ11_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_SWREQ11_W<'a> {
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
#[doc = "Write proxy for field `DMA_SWREQ12`"]
pub struct DMA_SWREQ12_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_SWREQ12_W<'a> {
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
#[doc = "Write proxy for field `DMA_SWREQ13`"]
pub struct DMA_SWREQ13_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_SWREQ13_W<'a> {
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
#[doc = "Write proxy for field `DMA_SWREQ14`"]
pub struct DMA_SWREQ14_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_SWREQ14_W<'a> {
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
#[doc = "Write proxy for field `DMA_SWREQ15`"]
pub struct DMA_SWREQ15_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_SWREQ15_W<'a> {
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
#[doc = "Write proxy for field `DMA_SWREQ16`"]
pub struct DMA_SWREQ16_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_SWREQ16_W<'a> {
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
#[doc = "Write proxy for field `DMA_SWREQ17`"]
pub struct DMA_SWREQ17_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_SWREQ17_W<'a> {
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
#[doc = "Write proxy for field `DMA_SWREQ18`"]
pub struct DMA_SWREQ18_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_SWREQ18_W<'a> {
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
#[doc = "Write proxy for field `DMA_SWREQ19`"]
pub struct DMA_SWREQ19_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_SWREQ19_W<'a> {
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
#[doc = "Write proxy for field `DMA_SWREQ20`"]
pub struct DMA_SWREQ20_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_SWREQ20_W<'a> {
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
#[doc = "Write proxy for field `DMA_SWREQ21`"]
pub struct DMA_SWREQ21_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_SWREQ21_W<'a> {
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
#[doc = "Write proxy for field `DMA_SWREQ22`"]
pub struct DMA_SWREQ22_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_SWREQ22_W<'a> {
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
#[doc = "Write proxy for field `DMA_SWREQ23`"]
pub struct DMA_SWREQ23_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_SWREQ23_W<'a> {
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
    #[doc = "Bit 0 - Set software request on channel 0"]
    #[inline(always)]
    pub fn dma_swreq0(&mut self) -> DMA_SWREQ0_W {
        DMA_SWREQ0_W { w: self }
    }
    #[doc = "Bit 1 - Set software request on channel 1"]
    #[inline(always)]
    pub fn dma_swreq1(&mut self) -> DMA_SWREQ1_W {
        DMA_SWREQ1_W { w: self }
    }
    #[doc = "Bit 2 - Set software request on channel 2"]
    #[inline(always)]
    pub fn dma_swreq2(&mut self) -> DMA_SWREQ2_W {
        DMA_SWREQ2_W { w: self }
    }
    #[doc = "Bit 3 - Set software request on channel 3"]
    #[inline(always)]
    pub fn dma_swreq3(&mut self) -> DMA_SWREQ3_W {
        DMA_SWREQ3_W { w: self }
    }
    #[doc = "Bit 4 - Set software request on channel 4"]
    #[inline(always)]
    pub fn dma_swreq4(&mut self) -> DMA_SWREQ4_W {
        DMA_SWREQ4_W { w: self }
    }
    #[doc = "Bit 5 - Set software request on channel 5"]
    #[inline(always)]
    pub fn dma_swreq5(&mut self) -> DMA_SWREQ5_W {
        DMA_SWREQ5_W { w: self }
    }
    #[doc = "Bit 6 - Set software request on channel 6"]
    #[inline(always)]
    pub fn dma_swreq6(&mut self) -> DMA_SWREQ6_W {
        DMA_SWREQ6_W { w: self }
    }
    #[doc = "Bit 7 - Set software request on channel 7"]
    #[inline(always)]
    pub fn dma_swreq7(&mut self) -> DMA_SWREQ7_W {
        DMA_SWREQ7_W { w: self }
    }
    #[doc = "Bit 8 - Set software request on channel 8"]
    #[inline(always)]
    pub fn dma_swreq8(&mut self) -> DMA_SWREQ8_W {
        DMA_SWREQ8_W { w: self }
    }
    #[doc = "Bit 9 - Set software request on channel 9"]
    #[inline(always)]
    pub fn dma_swreq9(&mut self) -> DMA_SWREQ9_W {
        DMA_SWREQ9_W { w: self }
    }
    #[doc = "Bit 10 - Set software request on channel 10"]
    #[inline(always)]
    pub fn dma_swreq10(&mut self) -> DMA_SWREQ10_W {
        DMA_SWREQ10_W { w: self }
    }
    #[doc = "Bit 11 - Set software request on channel 11"]
    #[inline(always)]
    pub fn dma_swreq11(&mut self) -> DMA_SWREQ11_W {
        DMA_SWREQ11_W { w: self }
    }
    #[doc = "Bit 12 - Set software request on channel 12"]
    #[inline(always)]
    pub fn dma_swreq12(&mut self) -> DMA_SWREQ12_W {
        DMA_SWREQ12_W { w: self }
    }
    #[doc = "Bit 13 - Set software request on channel 13"]
    #[inline(always)]
    pub fn dma_swreq13(&mut self) -> DMA_SWREQ13_W {
        DMA_SWREQ13_W { w: self }
    }
    #[doc = "Bit 14 - Set software request on channel 14"]
    #[inline(always)]
    pub fn dma_swreq14(&mut self) -> DMA_SWREQ14_W {
        DMA_SWREQ14_W { w: self }
    }
    #[doc = "Bit 15 - Set software request on channel 15"]
    #[inline(always)]
    pub fn dma_swreq15(&mut self) -> DMA_SWREQ15_W {
        DMA_SWREQ15_W { w: self }
    }
    #[doc = "Bit 16 - Set software request on channel 16"]
    #[inline(always)]
    pub fn dma_swreq16(&mut self) -> DMA_SWREQ16_W {
        DMA_SWREQ16_W { w: self }
    }
    #[doc = "Bit 17 - Set software request on channel 17"]
    #[inline(always)]
    pub fn dma_swreq17(&mut self) -> DMA_SWREQ17_W {
        DMA_SWREQ17_W { w: self }
    }
    #[doc = "Bit 18 - Set software request on channel 18"]
    #[inline(always)]
    pub fn dma_swreq18(&mut self) -> DMA_SWREQ18_W {
        DMA_SWREQ18_W { w: self }
    }
    #[doc = "Bit 19 - Set software request on channel 19"]
    #[inline(always)]
    pub fn dma_swreq19(&mut self) -> DMA_SWREQ19_W {
        DMA_SWREQ19_W { w: self }
    }
    #[doc = "Bit 20 - Set software request on channel 20"]
    #[inline(always)]
    pub fn dma_swreq20(&mut self) -> DMA_SWREQ20_W {
        DMA_SWREQ20_W { w: self }
    }
    #[doc = "Bit 21 - Set software request on channel 21"]
    #[inline(always)]
    pub fn dma_swreq21(&mut self) -> DMA_SWREQ21_W {
        DMA_SWREQ21_W { w: self }
    }
    #[doc = "Bit 22 - Set software request on channel 22"]
    #[inline(always)]
    pub fn dma_swreq22(&mut self) -> DMA_SWREQ22_W {
        DMA_SWREQ22_W { w: self }
    }
    #[doc = "Bit 23 - Set software request on channel 23"]
    #[inline(always)]
    pub fn dma_swreq23(&mut self) -> DMA_SWREQ23_W {
        DMA_SWREQ23_W { w: self }
    }
}
