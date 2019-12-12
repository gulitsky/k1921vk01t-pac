#[doc = "Reader of register DMAINT"]
pub type R = crate::R<u32, super::DMAINT>;
#[doc = "Writer for register DMAINT"]
pub type W = crate::W<u32, super::DMAINT>;
#[doc = "Register DMAINT `reset()`'s with value 0"]
impl crate::ResetValue for super::DMAINT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TXPKTSENT`"]
pub type TXPKTSENT_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXUNDERRRUN`"]
pub type TXUNDERRRUN_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUSERR_TX`"]
pub type BUSERR_TX_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXPKTREC`"]
pub type RXPKTREC_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXOVER`"]
pub type RXOVER_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUSERR_RX`"]
pub type BUSERR_RX_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn txpktsent(&self) -> TXPKTSENT_R {
        TXPKTSENT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn txunderrrun(&self) -> TXUNDERRRUN_R {
        TXUNDERRRUN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn buserr_tx(&self) -> BUSERR_TX_R {
        BUSERR_TX_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rxpktrec(&self) -> RXPKTREC_R {
        RXPKTREC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rxover(&self) -> RXOVER_R {
        RXOVER_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn buserr_rx(&self) -> BUSERR_RX_R {
        BUSERR_RX_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {}
