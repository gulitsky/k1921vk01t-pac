#[doc = "Reader of register TZFLG"]
pub type R = crate::R<u32, super::TZFLG>;
#[doc = "Reader of field `INT`"]
pub type INT_R = crate::R<bool, bool>;
#[doc = "Reader of field `CBC`"]
pub type CBC_R = crate::R<bool, bool>;
#[doc = "Reader of field `OST`"]
pub type OST_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Status of external interrupts NVIC"]
    #[inline(always)]
    pub fn int(&self) -> INT_R {
        INT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Interrupt status in a cyclic mode"]
    #[inline(always)]
    pub fn cbc(&self) -> CBC_R {
        CBC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Interrupt status in one-shot mode"]
    #[inline(always)]
    pub fn ost(&self) -> OST_R {
        OST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
