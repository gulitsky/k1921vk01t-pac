#[doc = "Reader of register MOSTAT"]
pub type R = crate::R<u32, super::MOSTAT>;
#[doc = "Reader of field `RXPND`"]
pub type RXPND_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXPND`"]
pub type TXPND_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXUPD`"]
pub type RXUPD_R = crate::R<bool, bool>;
#[doc = "Reader of field `NEWDAT`"]
pub type NEWDAT_R = crate::R<bool, bool>;
#[doc = "Reader of field `MSGLST`"]
pub type MSGLST_R = crate::R<bool, bool>;
#[doc = "Reader of field `MSGVAL`"]
pub type MSGVAL_R = crate::R<bool, bool>;
#[doc = "Reader of field `RTSEL`"]
pub type RTSEL_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXEN`"]
pub type RXEN_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXRQ`"]
pub type TXRQ_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXEN0`"]
pub type TXEN0_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXEN1`"]
pub type TXEN1_R = crate::R<bool, bool>;
#[doc = "Reader of field `DIR`"]
pub type DIR_R = crate::R<bool, bool>;
#[doc = "Reader of field `LIST`"]
pub type LIST_R = crate::R<u8, u8>;
#[doc = "Reader of field `PPREV`"]
pub type PPREV_R = crate::R<u8, u8>;
#[doc = "Reader of field `PNEXT`"]
pub type PNEXT_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - Indicator deadline"]
    #[inline(always)]
    pub fn rxpnd(&self) -> RXPND_R {
        RXPND_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Indicator end of transmission"]
    #[inline(always)]
    pub fn txpnd(&self) -> TXPND_R {
        TXPND_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Indicator changes"]
    #[inline(always)]
    pub fn rxupd(&self) -> RXUPD_R {
        RXUPD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - New data indicator"]
    #[inline(always)]
    pub fn newdat(&self) -> NEWDAT_R {
        NEWDAT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Bit message loss"]
    #[inline(always)]
    pub fn msglst(&self) -> MSGLST_R {
        MSGLST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Activity bit of the message object 0"]
    #[inline(always)]
    pub fn msgval(&self) -> MSGVAL_R {
        MSGVAL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - The indication of transmission / reception"]
    #[inline(always)]
    pub fn rtsel(&self) -> RTSEL_R {
        RTSEL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Bits allow reception"]
    #[inline(always)]
    pub fn rxen(&self) -> RXEN_R {
        RXEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Initiate transmission"]
    #[inline(always)]
    pub fn txrq(&self) -> TXRQ_R {
        TXRQ_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Enable bit transmission frame"]
    #[inline(always)]
    pub fn txen0(&self) -> TXEN0_R {
        TXEN0_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Enable bit transmission frame"]
    #[inline(always)]
    pub fn txen1(&self) -> TXEN1_R {
        TXEN1_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Bit allocation"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn list(&self) -> LIST_R {
        LIST_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn pprev(&self) -> PPREV_R {
        PPREV_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn pnext(&self) -> PNEXT_R {
        PNEXT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
