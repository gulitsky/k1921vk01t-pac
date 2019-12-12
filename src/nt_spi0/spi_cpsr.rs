#[doc = "Reader of register SPI_CPSR"]
pub type R = crate::R<u32, super::SPI_CPSR>;
#[doc = "Writer for register SPI_CPSR"]
pub type W = crate::W<u32, super::SPI_CPSR>;
#[doc = "Register SPI_CPSR `reset()`'s with value 0"]
impl crate::ResetValue for super::SPI_CPSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CPSDVSR`"]
pub type CPSDVSR_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn cpsdvsr(&self) -> CPSDVSR_R {
        CPSDVSR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {}
