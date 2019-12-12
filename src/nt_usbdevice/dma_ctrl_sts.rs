#[doc = "Reader of register DMA_CTRL_STS"]
pub type R = crate::R<u32, super::DMA_CTRL_STS>;
#[doc = "Writer for register DMA_CTRL_STS"]
pub type W = crate::W<u32, super::DMA_CTRL_STS>;
#[doc = "Register DMA_CTRL_STS `reset()`'s with value 0"]
impl crate::ResetValue for super::DMA_CTRL_STS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DMA_EP_ADDR`"]
pub type DMA_EP_ADDR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DMA_EP_ADDR`"]
pub struct DMA_EP_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_EP_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `DMARW`"]
pub type DMARW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMARW`"]
pub struct DMARW_W<'a> {
    w: &'a mut W,
}
impl<'a> DMARW_W<'a> {
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
#[doc = "Reader of field `DMAEN`"]
pub type DMAEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMAEN`"]
pub struct DMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAEN_W<'a> {
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
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn dma_ep_addr(&self) -> DMA_EP_ADDR_R {
        DMA_EP_ADDR_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Type DMA operations 1-Read, 0-Write"]
    #[inline(always)]
    pub fn dmarw(&self) -> DMARW_R {
        DMARW_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enable DMA"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn dma_ep_addr(&mut self) -> DMA_EP_ADDR_W {
        DMA_EP_ADDR_W { w: self }
    }
    #[doc = "Bit 4 - Type DMA operations 1-Read, 0-Write"]
    #[inline(always)]
    pub fn dmarw(&mut self) -> DMARW_W {
        DMARW_W { w: self }
    }
    #[doc = "Bit 5 - Enable DMA"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W {
        DMAEN_W { w: self }
    }
}
