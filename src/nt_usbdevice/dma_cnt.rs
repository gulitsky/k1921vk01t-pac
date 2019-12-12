#[doc = "Reader of register DMA_CNT"]
pub type R = crate::R<u32, super::DMA_CNT>;
#[doc = "Writer for register DMA_CNT"]
pub type W = crate::W<u32, super::DMA_CNT>;
#[doc = "Register DMA_CNT `reset()`'s with value 0"]
impl crate::ResetValue for super::DMA_CNT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DMA_CNT`"]
pub type DMA_CNT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DMA_CNT`"]
pub struct DMA_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn dma_cnt(&self) -> DMA_CNT_R {
        DMA_CNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn dma_cnt(&mut self) -> DMA_CNT_W {
        DMA_CNT_W { w: self }
    }
}
