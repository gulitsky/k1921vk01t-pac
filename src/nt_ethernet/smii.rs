#[doc = "Reader of register SMII"]
pub type R = crate::R<u32, super::SMII>;
#[doc = "Reader of field `SPEED`"]
pub type SPEED_R = crate::R<bool, bool>;
#[doc = "Reader of field `DUPLEX`"]
pub type DUPLEX_R = crate::R<bool, bool>;
#[doc = "Reader of field `LINK`"]
pub type LINK_R = crate::R<bool, bool>;
#[doc = "Reader of field `JABBER`"]
pub type JABBER_R = crate::R<bool, bool>;
#[doc = "Reader of field `CLASH`"]
pub type CLASH_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Speed Indicator"]
    #[inline(always)]
    pub fn speed(&self) -> SPEED_R {
        SPEED_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Mode indicator"]
    #[inline(always)]
    pub fn duplex(&self) -> DUPLEX_R {
        DUPLEX_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Status Indicator LINK"]
    #[inline(always)]
    pub fn link(&self) -> LINK_R {
        LINK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Flag of conditions Jabber"]
    #[inline(always)]
    pub fn jabber(&self) -> JABBER_R {
        JABBER_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Flag of the selected mode"]
    #[inline(always)]
    pub fn clash(&self) -> CLASH_R {
        CLASH_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
