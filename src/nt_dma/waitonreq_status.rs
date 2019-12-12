#[doc = "Reader of register WAITONREQ_STATUS"]
pub type R = crate::R<u32, super::WAITONREQ_STATUS>;
#[doc = "Reader of field `DMA_WAITREQ0`"]
pub type DMA_WAITREQ0_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMA_WAITREQ1`"]
pub type DMA_WAITREQ1_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMA_WAITREQ2`"]
pub type DMA_WAITREQ2_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMA_WAITREQ3`"]
pub type DMA_WAITREQ3_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMA_WAITREQ4`"]
pub type DMA_WAITREQ4_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMA_WAITREQ5`"]
pub type DMA_WAITREQ5_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMA_WAITREQ6`"]
pub type DMA_WAITREQ6_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMA_WAITREQ7`"]
pub type DMA_WAITREQ7_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMA_WAITREQ8`"]
pub type DMA_WAITREQ8_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMA_WAITREQ9`"]
pub type DMA_WAITREQ9_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMA_WAITREQ10`"]
pub type DMA_WAITREQ10_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMA_WAITREQ11`"]
pub type DMA_WAITREQ11_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMA_WAITREQ12`"]
pub type DMA_WAITREQ12_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMA_WAITREQ13`"]
pub type DMA_WAITREQ13_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMA_WAITREQ14`"]
pub type DMA_WAITREQ14_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMA_WAITREQ15`"]
pub type DMA_WAITREQ15_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMA_WAITREQ16`"]
pub type DMA_WAITREQ16_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMA_WAITREQ17`"]
pub type DMA_WAITREQ17_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMA_WAITREQ18`"]
pub type DMA_WAITREQ18_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMA_WAITREQ19`"]
pub type DMA_WAITREQ19_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMA_WAITREQ20`"]
pub type DMA_WAITREQ20_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMA_WAITREQ21`"]
pub type DMA_WAITREQ21_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMA_WAITREQ22`"]
pub type DMA_WAITREQ22_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMA_WAITREQ23`"]
pub type DMA_WAITREQ23_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Returns the status of the DMA request signals"]
    #[inline(always)]
    pub fn dma_waitreq0(&self) -> DMA_WAITREQ0_R {
        DMA_WAITREQ0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Returns the status of the DMA request signals"]
    #[inline(always)]
    pub fn dma_waitreq1(&self) -> DMA_WAITREQ1_R {
        DMA_WAITREQ1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Returns the status of the DMA request signals"]
    #[inline(always)]
    pub fn dma_waitreq2(&self) -> DMA_WAITREQ2_R {
        DMA_WAITREQ2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Returns the status of the DMA request signals"]
    #[inline(always)]
    pub fn dma_waitreq3(&self) -> DMA_WAITREQ3_R {
        DMA_WAITREQ3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Returns the status of the DMA request signals"]
    #[inline(always)]
    pub fn dma_waitreq4(&self) -> DMA_WAITREQ4_R {
        DMA_WAITREQ4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Returns the status of the DMA request signals"]
    #[inline(always)]
    pub fn dma_waitreq5(&self) -> DMA_WAITREQ5_R {
        DMA_WAITREQ5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Returns the status of the DMA request signals"]
    #[inline(always)]
    pub fn dma_waitreq6(&self) -> DMA_WAITREQ6_R {
        DMA_WAITREQ6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Returns the status of the DMA request signals"]
    #[inline(always)]
    pub fn dma_waitreq7(&self) -> DMA_WAITREQ7_R {
        DMA_WAITREQ7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Returns the status of the DMA request signals"]
    #[inline(always)]
    pub fn dma_waitreq8(&self) -> DMA_WAITREQ8_R {
        DMA_WAITREQ8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Returns the status of the DMA request signals"]
    #[inline(always)]
    pub fn dma_waitreq9(&self) -> DMA_WAITREQ9_R {
        DMA_WAITREQ9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Returns the status of the DMA request signals"]
    #[inline(always)]
    pub fn dma_waitreq10(&self) -> DMA_WAITREQ10_R {
        DMA_WAITREQ10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Returns the status of the DMA request signals"]
    #[inline(always)]
    pub fn dma_waitreq11(&self) -> DMA_WAITREQ11_R {
        DMA_WAITREQ11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Returns the status of the DMA request signals"]
    #[inline(always)]
    pub fn dma_waitreq12(&self) -> DMA_WAITREQ12_R {
        DMA_WAITREQ12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Returns the status of the DMA request signals"]
    #[inline(always)]
    pub fn dma_waitreq13(&self) -> DMA_WAITREQ13_R {
        DMA_WAITREQ13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Returns the status of the DMA request signals"]
    #[inline(always)]
    pub fn dma_waitreq14(&self) -> DMA_WAITREQ14_R {
        DMA_WAITREQ14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Returns the status of the DMA request signals"]
    #[inline(always)]
    pub fn dma_waitreq15(&self) -> DMA_WAITREQ15_R {
        DMA_WAITREQ15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Returns the status of the DMA request signals"]
    #[inline(always)]
    pub fn dma_waitreq16(&self) -> DMA_WAITREQ16_R {
        DMA_WAITREQ16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Returns the status of the DMA request signals"]
    #[inline(always)]
    pub fn dma_waitreq17(&self) -> DMA_WAITREQ17_R {
        DMA_WAITREQ17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Returns the status of the DMA request signals"]
    #[inline(always)]
    pub fn dma_waitreq18(&self) -> DMA_WAITREQ18_R {
        DMA_WAITREQ18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Returns the status of the DMA request signals"]
    #[inline(always)]
    pub fn dma_waitreq19(&self) -> DMA_WAITREQ19_R {
        DMA_WAITREQ19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Returns the status of the DMA request signals"]
    #[inline(always)]
    pub fn dma_waitreq20(&self) -> DMA_WAITREQ20_R {
        DMA_WAITREQ20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Returns the status of the DMA request signals"]
    #[inline(always)]
    pub fn dma_waitreq21(&self) -> DMA_WAITREQ21_R {
        DMA_WAITREQ21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Returns the status of the DMA request signals"]
    #[inline(always)]
    pub fn dma_waitreq22(&self) -> DMA_WAITREQ22_R {
        DMA_WAITREQ22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Returns the status of the DMA request signals"]
    #[inline(always)]
    pub fn dma_waitreq23(&self) -> DMA_WAITREQ23_R {
        DMA_WAITREQ23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
