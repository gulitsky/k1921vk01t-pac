#[doc = "Reader of register DMATxStatus"]
pub type R = crate::R<u32, super::DMATXSTATUS>;
#[doc = "Writer for register DMATxStatus"]
pub type W = crate::W<u32, super::DMATXSTATUS>;
#[doc = "Register DMATxStatus `reset()`'s with value 0"]
impl crate::ResetValue for super::DMATXSTATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TXPKTREC`"]
pub type TXPKTREC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXPKTREC`"]
pub struct TXPKTREC_W<'a> {
    w: &'a mut W,
}
impl<'a> TXPKTREC_W<'a> {
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
#[doc = "Reader of field `TX_PKT_COUNT`"]
pub type TX_PKT_COUNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TX_PKT_COUNT`"]
pub struct TX_PKT_COUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_PKT_COUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Flag successfully transmit one or more packets"]
    #[inline(always)]
    pub fn txpktrec(&self) -> TXPKTREC_R {
        TXPKTREC_R::new((self.bits & 0x01) != 0)
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
    pub fn tx_pkt_count(&self) -> TX_PKT_COUNT_R {
        TX_PKT_COUNT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Flag successfully transmit one or more packets"]
    #[inline(always)]
    pub fn txpktrec(&mut self) -> TXPKTREC_W {
        TXPKTREC_W { w: self }
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
    pub fn tx_pkt_count(&mut self) -> TX_PKT_COUNT_W {
        TX_PKT_COUNT_W { w: self }
    }
}
