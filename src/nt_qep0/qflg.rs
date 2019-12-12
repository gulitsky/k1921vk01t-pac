#[doc = "Reader of register QFLG"]
pub type R = crate::R<u32, super::QFLG>;
#[doc = "Reader of field `INT`"]
pub type INT_R = crate::R<bool, bool>;
#[doc = "Reader of field `PCE`"]
pub type PCE_R = crate::R<bool, bool>;
#[doc = "Reader of field `QPE`"]
pub type QPE_R = crate::R<bool, bool>;
#[doc = "Reader of field `QDC`"]
pub type QDC_R = crate::R<bool, bool>;
#[doc = "Reader of field `WTO`"]
pub type WTO_R = crate::R<bool, bool>;
#[doc = "Reader of field `PCU`"]
pub type PCU_R = crate::R<bool, bool>;
#[doc = "Reader of field `PCO`"]
pub type PCO_R = crate::R<bool, bool>;
#[doc = "Reader of field `PCR`"]
pub type PCR_R = crate::R<bool, bool>;
#[doc = "Reader of field `PCM`"]
pub type PCM_R = crate::R<bool, bool>;
#[doc = "Reader of field `SEL`"]
pub type SEL_R = crate::R<bool, bool>;
#[doc = "Reader of field `IEL`"]
pub type IEL_R = crate::R<bool, bool>;
#[doc = "Reader of field `UTO`"]
pub type UTO_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - outint flag"]
    #[inline(always)]
    pub fn int(&self) -> INT_R {
        INT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - enable countpos interrupt flag"]
    #[inline(always)]
    pub fn pce(&self) -> PCE_R {
        PCE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - errorphase interrupt flag"]
    #[inline(always)]
    pub fn qpe(&self) -> QPE_R {
        QPE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Change direction interrupt flag"]
    #[inline(always)]
    pub fn qdc(&self) -> QDC_R {
        QDC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Watchdog interrupt flag"]
    #[inline(always)]
    pub fn wto(&self) -> WTO_R {
        WTO_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - interrupt countmin flag"]
    #[inline(always)]
    pub fn pcu(&self) -> PCU_R {
        PCU_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - interrupt countmax flag"]
    #[inline(always)]
    pub fn pco(&self) -> PCO_R {
        PCO_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - ready compare int flag"]
    #[inline(always)]
    pub fn pcr(&self) -> PCR_R {
        PCR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - enable compare int flag"]
    #[inline(always)]
    pub fn pcm(&self) -> PCM_R {
        PCM_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Strobe int flag"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Index int flag"]
    #[inline(always)]
    pub fn iel(&self) -> IEL_R {
        IEL_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - outflow time int flag"]
    #[inline(always)]
    pub fn uto(&self) -> UTO_R {
        UTO_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
