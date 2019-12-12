#[doc = "Reader of register DMARxStatus"]
pub type R = crate::R<u32, super::DMARXSTATUS>;
#[doc = "Writer for register DMARxStatus"]
pub type W = crate::W<u32, super::DMARXSTATUS>;
#[doc = "Register DMARxStatus `reset()`'s with value 0"]
impl crate::ResetValue for super::DMARXSTATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RXPKTREC`"]
pub type RXPKTREC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXPKTREC`"]
pub struct RXPKTREC_W<'a> {
    w: &'a mut W,
}
impl<'a> RXPKTREC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `TXOVERFLOW`"]
pub type TXOVERFLOW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXOVERFLOW`"]
pub struct TXOVERFLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> TXOVERFLOW_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `BUSERROR`"]
pub type BUSERROR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BUSERROR`"]
pub struct BUSERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> BUSERROR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `RX_PKT_COUNT`"]
pub type RX_PKT_COUNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RX_PKT_COUNT`"]
pub struct RX_PKT_COUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_PKT_COUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Flag successfully receive one or more packets"]
    #[inline(always)]
    pub fn rxpktrec(&self) -> RXPKTREC_R {
        RXPKTREC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - Data Indicator"]
    #[inline(always)]
    pub fn txoverflow(&self) -> TXOVERFLOW_R {
        TXOVERFLOW_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Flag a bus error"]
    #[inline(always)]
    pub fn buserror(&self) -> BUSERROR_R {
        BUSERROR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn rx_pkt_count(&self) -> RX_PKT_COUNT_R {
        RX_PKT_COUNT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Flag successfully receive one or more packets"]
    #[inline(always)]
    pub fn rxpktrec(&mut self) -> RXPKTREC_W {
        RXPKTREC_W { w: self }
    }
    #[doc = "Bit 2 - Data Indicator"]
    #[inline(always)]
    pub fn txoverflow(&mut self) -> TXOVERFLOW_W {
        TXOVERFLOW_W { w: self }
    }
    #[doc = "Bit 3 - Flag a bus error"]
    #[inline(always)]
    pub fn buserror(&mut self) -> BUSERROR_W {
        BUSERROR_W { w: self }
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn rx_pkt_count(&mut self) -> RX_PKT_COUNT_W {
        RX_PKT_COUNT_W { w: self }
    }
}
