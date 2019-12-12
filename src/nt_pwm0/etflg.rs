#[doc = "Reader of register ETFLG"]
pub type R = crate::R<u32, super::ETFLG>;
#[doc = "Reader of field `INT`"]
pub type INT_R = crate::R<bool, bool>;
#[doc = "Reader of field `SOCA`"]
pub type SOCA_R = crate::R<bool, bool>;
#[doc = "Reader of field `SOCB`"]
pub type SOCB_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Status of the external interrupt EPWMxINT"]
    #[inline(always)]
    pub fn int(&self) -> INT_R {
        INT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - Status of the external ADC EPWMxSOCA"]
    #[inline(always)]
    pub fn soca(&self) -> SOCA_R {
        SOCA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Status of the external ADC EPWMxSOCB"]
    #[inline(always)]
    pub fn socb(&self) -> SOCB_R {
        SOCB_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
