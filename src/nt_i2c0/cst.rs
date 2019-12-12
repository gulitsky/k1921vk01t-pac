#[doc = "Reader of register CST"]
pub type R = crate::R<u32, super::CST>;
#[doc = "Writer for register CST"]
pub type W = crate::W<u32, super::CST>;
#[doc = "Register CST `reset()`'s with value 0"]
impl crate::ResetValue for super::CST {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BB`"]
pub type BB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BB`"]
pub struct BB_W<'a> {
    w: &'a mut W,
}
impl<'a> BB_W<'a> {
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
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOCDIV_A {
    #[doc = "0: disable clock"]
    DISABLE,
    #[doc = "1: clock divided by 4"]
    DIV4,
    #[doc = "2: clock divided by 8"]
    DIV8,
    #[doc = "3: clock divided by 16"]
    DIV16,
}
impl From<TOCDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: TOCDIV_A) -> Self {
        match variant {
            TOCDIV_A::DISABLE => 0,
            TOCDIV_A::DIV4 => 1,
            TOCDIV_A::DIV8 => 2,
            TOCDIV_A::DIV16 => 3,
        }
    }
}
#[doc = "Reader of field `TOCDIV`"]
pub type TOCDIV_R = crate::R<u8, TOCDIV_A>;
impl TOCDIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOCDIV_A {
        match self.bits {
            0 => TOCDIV_A::DISABLE,
            1 => TOCDIV_A::DIV4,
            2 => TOCDIV_A::DIV8,
            3 => TOCDIV_A::DIV16,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TOCDIV_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == TOCDIV_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == TOCDIV_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == TOCDIV_A::DIV16
    }
}
#[doc = "Write proxy for field `TOCDIV`"]
pub struct TOCDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> TOCDIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TOCDIV_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "disable clock"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TOCDIV_A::DISABLE)
    }
    #[doc = "clock divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(TOCDIV_A::DIV4)
    }
    #[doc = "clock divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(TOCDIV_A::DIV8)
    }
    #[doc = "clock divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(TOCDIV_A::DIV16)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u32) & 0x03) << 1);
        self.w
    }
}
#[doc = "Reader of field `TOERR`"]
pub type TOERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TOERR`"]
pub struct TOERR_W<'a> {
    w: &'a mut W,
}
impl<'a> TOERR_W<'a> {
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
#[doc = "Reader of field `TSDA`"]
pub type TSDA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSDA`"]
pub struct TSDA_W<'a> {
    w: &'a mut W,
}
impl<'a> TSDA_W<'a> {
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
#[doc = "Reader of field `TGSCL`"]
pub type TGSCL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TGSCL`"]
pub struct TGSCL_W<'a> {
    w: &'a mut W,
}
impl<'a> TGSCL_W<'a> {
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
#[doc = "Reader of field `PECNEXT`"]
pub type PECNEXT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PECNEXT`"]
pub struct PECNEXT_W<'a> {
    w: &'a mut W,
}
impl<'a> PECNEXT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `PECFAULT`"]
pub type PECFAULT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PECFAULT`"]
pub struct PECFAULT_W<'a> {
    w: &'a mut W,
}
impl<'a> PECFAULT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Flag employment bus"]
    #[inline(always)]
    pub fn bb(&self) -> BB_R {
        BB_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    pub fn tocdiv(&self) -> TOCDIV_R {
        TOCDIV_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 3 - Flag error simple bus"]
    #[inline(always)]
    pub fn toerr(&self) -> TOERR_R {
        TOERR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Bit test SDA"]
    #[inline(always)]
    pub fn tsda(&self) -> TSDA_R {
        TSDA_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Bit switch SCL"]
    #[inline(always)]
    pub fn tgscl(&self) -> TGSCL_R {
        TGSCL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Bit control transmit CRC"]
    #[inline(always)]
    pub fn pecnext(&self) -> PECNEXT_R {
        PECNEXT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Error flag"]
    #[inline(always)]
    pub fn pecfault(&self) -> PECFAULT_R {
        PECFAULT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Flag employment bus"]
    #[inline(always)]
    pub fn bb(&mut self) -> BB_W {
        BB_W { w: self }
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    pub fn tocdiv(&mut self) -> TOCDIV_W {
        TOCDIV_W { w: self }
    }
    #[doc = "Bit 3 - Flag error simple bus"]
    #[inline(always)]
    pub fn toerr(&mut self) -> TOERR_W {
        TOERR_W { w: self }
    }
    #[doc = "Bit 4 - Bit test SDA"]
    #[inline(always)]
    pub fn tsda(&mut self) -> TSDA_W {
        TSDA_W { w: self }
    }
    #[doc = "Bit 5 - Bit switch SCL"]
    #[inline(always)]
    pub fn tgscl(&mut self) -> TGSCL_W {
        TGSCL_W { w: self }
    }
    #[doc = "Bit 6 - Bit control transmit CRC"]
    #[inline(always)]
    pub fn pecnext(&mut self) -> PECNEXT_W {
        PECNEXT_W { w: self }
    }
    #[doc = "Bit 7 - Error flag"]
    #[inline(always)]
    pub fn pecfault(&mut self) -> PECFAULT_W {
        PECFAULT_W { w: self }
    }
}
