#[doc = "Reader of register NSR"]
pub type R = crate::R<u32, super::NSR>;
#[doc = "Writer for register NSR"]
pub type W = crate::W<u32, super::NSR>;
#[doc = "Register NSR `reset()`'s with value 0"]
impl crate::ResetValue for super::NSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LEC_A {
    #[doc = "0: no error"]
    NOERR,
    #[doc = "1: stuff error"]
    STUFFERR,
    #[doc = "2: form error"]
    FORMERR,
    #[doc = "3: acknowlegment error"]
    ACKERR,
    #[doc = "4: bit 1 error"]
    BIT1ERR,
    #[doc = "5: bit 0 error"]
    BIT0ERR,
    #[doc = "6: CRC error"]
    CRCERR,
    #[doc = "7: enable hardware write"]
    WRITEEN,
}
impl From<LEC_A> for u8 {
    #[inline(always)]
    fn from(variant: LEC_A) -> Self {
        match variant {
            LEC_A::NOERR => 0,
            LEC_A::STUFFERR => 1,
            LEC_A::FORMERR => 2,
            LEC_A::ACKERR => 3,
            LEC_A::BIT1ERR => 4,
            LEC_A::BIT0ERR => 5,
            LEC_A::CRCERR => 6,
            LEC_A::WRITEEN => 7,
        }
    }
}
#[doc = "Reader of field `LEC`"]
pub type LEC_R = crate::R<u8, LEC_A>;
impl LEC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LEC_A {
        match self.bits {
            0 => LEC_A::NOERR,
            1 => LEC_A::STUFFERR,
            2 => LEC_A::FORMERR,
            3 => LEC_A::ACKERR,
            4 => LEC_A::BIT1ERR,
            5 => LEC_A::BIT0ERR,
            6 => LEC_A::CRCERR,
            7 => LEC_A::WRITEEN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NOERR`"]
    #[inline(always)]
    pub fn is_no_err(&self) -> bool {
        *self == LEC_A::NOERR
    }
    #[doc = "Checks if the value of the field is `STUFFERR`"]
    #[inline(always)]
    pub fn is_stuff_err(&self) -> bool {
        *self == LEC_A::STUFFERR
    }
    #[doc = "Checks if the value of the field is `FORMERR`"]
    #[inline(always)]
    pub fn is_form_err(&self) -> bool {
        *self == LEC_A::FORMERR
    }
    #[doc = "Checks if the value of the field is `ACKERR`"]
    #[inline(always)]
    pub fn is_ack_err(&self) -> bool {
        *self == LEC_A::ACKERR
    }
    #[doc = "Checks if the value of the field is `BIT1ERR`"]
    #[inline(always)]
    pub fn is_bit1err(&self) -> bool {
        *self == LEC_A::BIT1ERR
    }
    #[doc = "Checks if the value of the field is `BIT0ERR`"]
    #[inline(always)]
    pub fn is_bit0err(&self) -> bool {
        *self == LEC_A::BIT0ERR
    }
    #[doc = "Checks if the value of the field is `CRCERR`"]
    #[inline(always)]
    pub fn is_crcerr(&self) -> bool {
        *self == LEC_A::CRCERR
    }
    #[doc = "Checks if the value of the field is `WRITEEN`"]
    #[inline(always)]
    pub fn is_write_en(&self) -> bool {
        *self == LEC_A::WRITEEN
    }
}
#[doc = "Write proxy for field `LEC`"]
pub struct LEC_W<'a> {
    w: &'a mut W,
}
impl<'a> LEC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LEC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "no error"]
    #[inline(always)]
    pub fn no_err(self) -> &'a mut W {
        self.variant(LEC_A::NOERR)
    }
    #[doc = "stuff error"]
    #[inline(always)]
    pub fn stuff_err(self) -> &'a mut W {
        self.variant(LEC_A::STUFFERR)
    }
    #[doc = "form error"]
    #[inline(always)]
    pub fn form_err(self) -> &'a mut W {
        self.variant(LEC_A::FORMERR)
    }
    #[doc = "acknowlegment error"]
    #[inline(always)]
    pub fn ack_err(self) -> &'a mut W {
        self.variant(LEC_A::ACKERR)
    }
    #[doc = "bit 1 error"]
    #[inline(always)]
    pub fn bit1err(self) -> &'a mut W {
        self.variant(LEC_A::BIT1ERR)
    }
    #[doc = "bit 0 error"]
    #[inline(always)]
    pub fn bit0err(self) -> &'a mut W {
        self.variant(LEC_A::BIT0ERR)
    }
    #[doc = "CRC error"]
    #[inline(always)]
    pub fn crcerr(self) -> &'a mut W {
        self.variant(LEC_A::CRCERR)
    }
    #[doc = "enable hardware write"]
    #[inline(always)]
    pub fn write_en(self) -> &'a mut W {
        self.variant(LEC_A::WRITEEN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `TXOK`"]
pub type TXOK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXOK`"]
pub struct TXOK_W<'a> {
    w: &'a mut W,
}
impl<'a> TXOK_W<'a> {
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
#[doc = "Reader of field `RXOK`"]
pub type RXOK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXOK`"]
pub struct RXOK_W<'a> {
    w: &'a mut W,
}
impl<'a> RXOK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `ALERT`"]
pub type ALERT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ALERT`"]
pub struct ALERT_W<'a> {
    w: &'a mut W,
}
impl<'a> ALERT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `EWRN`"]
pub type EWRN_R = crate::R<bool, bool>;
#[doc = "Reader of field `BOFF`"]
pub type BOFF_R = crate::R<bool, bool>;
#[doc = "Reader of field `LLE`"]
pub type LLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LLE`"]
pub struct LLE_W<'a> {
    w: &'a mut W,
}
impl<'a> LLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `LOE`"]
pub type LOE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOE`"]
pub struct LOE_W<'a> {
    w: &'a mut W,
}
impl<'a> LOE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `SUSACK`"]
pub type SUSACK_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn lec(&self) -> LEC_R {
        LEC_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - Flag successful message transmission"]
    #[inline(always)]
    pub fn txok(&self) -> TXOK_R {
        TXOK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Flag successful reception of messages"]
    #[inline(always)]
    pub fn rxok(&self) -> RXOK_R {
        RXOK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Warning flag ALERT"]
    #[inline(always)]
    pub fn alert(&self) -> ALERT_R {
        ALERT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Flag critical errors"]
    #[inline(always)]
    pub fn ewrn(&self) -> EWRN_R {
        EWRN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Status flag 'is disconnected from the bus'"]
    #[inline(always)]
    pub fn boff(&self) -> BOFF_R {
        BOFF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Error flag list"]
    #[inline(always)]
    pub fn lle(&self) -> LLE_R {
        LLE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Error Flag Room list"]
    #[inline(always)]
    pub fn loe(&self) -> LOE_R {
        LOE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Indicator cluster node suspend mode"]
    #[inline(always)]
    pub fn susack(&self) -> SUSACK_R {
        SUSACK_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn lec(&mut self) -> LEC_W {
        LEC_W { w: self }
    }
    #[doc = "Bit 3 - Flag successful message transmission"]
    #[inline(always)]
    pub fn txok(&mut self) -> TXOK_W {
        TXOK_W { w: self }
    }
    #[doc = "Bit 4 - Flag successful reception of messages"]
    #[inline(always)]
    pub fn rxok(&mut self) -> RXOK_W {
        RXOK_W { w: self }
    }
    #[doc = "Bit 5 - Warning flag ALERT"]
    #[inline(always)]
    pub fn alert(&mut self) -> ALERT_W {
        ALERT_W { w: self }
    }
    #[doc = "Bit 8 - Error flag list"]
    #[inline(always)]
    pub fn lle(&mut self) -> LLE_W {
        LLE_W { w: self }
    }
    #[doc = "Bit 9 - Error Flag Room list"]
    #[inline(always)]
    pub fn loe(&mut self) -> LOE_W {
        LOE_W { w: self }
    }
}
