#[doc = "Reader of register USB_STATUS"]
pub type R = crate::R<u32, super::USB_STATUS>;
#[doc = "Reader of field `UNEXPPID`"]
pub type UNEXPPID_R = crate::R<bool, bool>;
#[doc = "Reader of field `SHORTPKT`"]
pub type SHORTPKT_R = crate::R<bool, bool>;
#[doc = "Reader of field `ERRPID`"]
pub type ERRPID_R = crate::R<bool, bool>;
#[doc = "Reader of field `OUTSPACEVAILABLE`"]
pub type OUTSPACEVAILABLE_R = crate::R<bool, bool>;
#[doc = "Reader of field `INDATAREADY`"]
pub type INDATAREADY_R = crate::R<bool, bool>;
#[doc = "Reader of field `PIDERR`"]
pub type PIDERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `CRCERR`"]
pub type CRCERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `TOGGLEERR`"]
pub type TOGGLEERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `TIMEOUT`"]
pub type TIMEOUT_R = crate::R<bool, bool>;
#[doc = "Reader of field `NYET`"]
pub type NYET_R = crate::R<bool, bool>;
#[doc = "Reader of field `NAK`"]
pub type NAK_R = crate::R<bool, bool>;
#[doc = "Reader of field `PAGECROSS`"]
pub type PAGECROSS_R = crate::R<bool, bool>;
#[doc = "Reader of field `PING`"]
pub type PING_R = crate::R<bool, bool>;
#[doc = "Reader of field `XACTNERR`"]
pub type XACTNERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `BABBLE`"]
pub type BABBLE_R = crate::R<bool, bool>;
#[doc = "Reader of field `HALTED`"]
pub type HALTED_R = crate::R<bool, bool>;
#[doc = "Reader of field `USBINT`"]
pub type USBINT_R = crate::R<bool, bool>;
#[doc = "Reader of field `USBERRINT`"]
pub type USBERRINT_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 8 - Receiving flag PID, different from the expected"]
    #[inline(always)]
    pub fn unexppid(&self) -> UNEXPPID_R {
        UNEXPPID_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Flag receiving fewer bits than indicated in the register TOTAL_BYTES_TRANS TBT"]
    #[inline(always)]
    pub fn shortpkt(&self) -> SHORTPKT_R {
        SHORTPKT_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Flag to receive a response to the label ERROR PID Complete Split"]
    #[inline(always)]
    pub fn errpid(&self) -> ERRPID_R {
        ERRPID_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Request flag outbound operation with an empty data buffer"]
    #[inline(always)]
    pub fn outspacevailable(&self) -> OUTSPACEVAILABLE_R {
        OUTSPACEVAILABLE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Flag successful completion of an inbound operation"]
    #[inline(always)]
    pub fn indataready(&self) -> INDATAREADY_R {
        INDATAREADY_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Receiving flag incorrect PID"]
    #[inline(always)]
    pub fn piderr(&self) -> PIDERR_R {
        PIDERR_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - CRC error in receiving data"]
    #[inline(always)]
    pub fn crcerr(&self) -> CRCERR_R {
        CRCERR_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Receiving flag bit wrong DATATOGGLE"]
    #[inline(always)]
    pub fn toggleerr(&self) -> TOGGLEERR_R {
        TOGGLEERR_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Flag timeout response"]
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Receiving flag tags NYET"]
    #[inline(always)]
    pub fn nyet(&self) -> NYET_R {
        NYET_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Receiving flag tags NAK"]
    #[inline(always)]
    pub fn nak(&self) -> NAK_R {
        NAK_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Flag requirements transmission pagecross"]
    #[inline(always)]
    pub fn pagecross(&self) -> PAGECROSS_R {
        PAGECROSS_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Flag response"]
    #[inline(always)]
    pub fn ping(&self) -> PING_R {
        PING_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn xactnerr(&self) -> XACTNERR_R {
        XACTNERR_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Flag of finding the system in state Packet Babble"]
    #[inline(always)]
    pub fn babble(&self) -> BABBLE_R {
        BABBLE_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Flag of finding the system in state Stall or Packet Babble"]
    #[inline(always)]
    pub fn halted(&self) -> HALTED_R {
        HALTED_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Flag of the last operation"]
    #[inline(always)]
    pub fn usbint(&self) -> USBINT_R {
        USBINT_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Error flag that indicates that the last operation was completed due to an erroneous bus state"]
    #[inline(always)]
    pub fn usberrint(&self) -> USBERRINT_R {
        USBERRINT_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
