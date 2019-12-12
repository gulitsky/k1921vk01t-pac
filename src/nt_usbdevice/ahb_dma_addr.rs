#[doc = "Reader of register AHB_DMA_ADDR"]
pub type R = crate::R<u32, super::AHB_DMA_ADDR>;
#[doc = "Writer for register AHB_DMA_ADDR"]
pub type W = crate::W<u32, super::AHB_DMA_ADDR>;
#[doc = "Register AHB_DMA_ADDR `reset()`'s with value 0"]
impl crate::ResetValue for super::AHB_DMA_ADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AHBADDR`"]
pub type AHBADDR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `AHBADDR`"]
pub struct AHBADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> AHBADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - It specifies the address from which the DMA has to read / write.The address must WORD (32- bits) aligned"]
    #[inline(always)]
    pub fn ahbaddr(&self) -> AHBADDR_R {
        AHBADDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - It specifies the address from which the DMA has to read / write.The address must WORD (32- bits) aligned"]
    #[inline(always)]
    pub fn ahbaddr(&mut self) -> AHBADDR_W {
        AHBADDR_W { w: self }
    }
}
