#[doc = "Reader of register MCFG"]
pub type R = crate::R<u32, super::MCFG>;
#[doc = "Writer for register MCFG"]
pub type W = crate::W<u32, super::MCFG>;
#[doc = "Register MCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::MCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SCANINC`"]
pub type SCANINC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCANINC`"]
pub struct SCANINC_W<'a> {
    w: &'a mut W,
}
impl<'a> SCANINC_W<'a> {
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
#[doc = "Reader of field `NOPRE`"]
pub type NOPRE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NOPRE`"]
pub struct NOPRE_W<'a> {
    w: &'a mut W,
}
impl<'a> NOPRE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLOCK_SELECT_A {
    #[doc = "0: 1/4 divider"]
    DIV4,
    #[doc = "2: 1/6 divider"]
    DIV6,
    #[doc = "3: 1/8 divider"]
    DIV8,
    #[doc = "4: 1/10 divider"]
    DIV10,
    #[doc = "5: 1/14 divider"]
    DIV14,
    #[doc = "6: 1/20 divider"]
    DIV20,
    #[doc = "7: 1/28 divider"]
    DIV28,
}
impl From<CLOCK_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: CLOCK_SELECT_A) -> Self {
        match variant {
            CLOCK_SELECT_A::DIV4 => 0,
            CLOCK_SELECT_A::DIV6 => 2,
            CLOCK_SELECT_A::DIV8 => 3,
            CLOCK_SELECT_A::DIV10 => 4,
            CLOCK_SELECT_A::DIV14 => 5,
            CLOCK_SELECT_A::DIV20 => 6,
            CLOCK_SELECT_A::DIV28 => 7,
        }
    }
}
#[doc = "Reader of field `CLOCK_SELECT`"]
pub type CLOCK_SELECT_R = crate::R<u8, CLOCK_SELECT_A>;
impl CLOCK_SELECT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CLOCK_SELECT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CLOCK_SELECT_A::DIV4),
            2 => Val(CLOCK_SELECT_A::DIV6),
            3 => Val(CLOCK_SELECT_A::DIV8),
            4 => Val(CLOCK_SELECT_A::DIV10),
            5 => Val(CLOCK_SELECT_A::DIV14),
            6 => Val(CLOCK_SELECT_A::DIV20),
            7 => Val(CLOCK_SELECT_A::DIV28),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == CLOCK_SELECT_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV6`"]
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == CLOCK_SELECT_A::DIV6
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == CLOCK_SELECT_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV10`"]
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        *self == CLOCK_SELECT_A::DIV10
    }
    #[doc = "Checks if the value of the field is `DIV14`"]
    #[inline(always)]
    pub fn is_div14(&self) -> bool {
        *self == CLOCK_SELECT_A::DIV14
    }
    #[doc = "Checks if the value of the field is `DIV20`"]
    #[inline(always)]
    pub fn is_div20(&self) -> bool {
        *self == CLOCK_SELECT_A::DIV20
    }
    #[doc = "Checks if the value of the field is `DIV28`"]
    #[inline(always)]
    pub fn is_div28(&self) -> bool {
        *self == CLOCK_SELECT_A::DIV28
    }
}
#[doc = "Write proxy for field `CLOCK_SELECT`"]
pub struct CLOCK_SELECT_W<'a> {
    w: &'a mut W,
}
impl<'a> CLOCK_SELECT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLOCK_SELECT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "1/4 divider"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(CLOCK_SELECT_A::DIV4)
    }
    #[doc = "1/6 divider"]
    #[inline(always)]
    pub fn div6(self) -> &'a mut W {
        self.variant(CLOCK_SELECT_A::DIV6)
    }
    #[doc = "1/8 divider"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(CLOCK_SELECT_A::DIV8)
    }
    #[doc = "1/10 divider"]
    #[inline(always)]
    pub fn div10(self) -> &'a mut W {
        self.variant(CLOCK_SELECT_A::DIV10)
    }
    #[doc = "1/14 divider"]
    #[inline(always)]
    pub fn div14(self) -> &'a mut W {
        self.variant(CLOCK_SELECT_A::DIV14)
    }
    #[doc = "1/20 divider"]
    #[inline(always)]
    pub fn div20(self) -> &'a mut W {
        self.variant(CLOCK_SELECT_A::DIV20)
    }
    #[doc = "1/28 divider"]
    #[inline(always)]
    pub fn div28(self) -> &'a mut W {
        self.variant(CLOCK_SELECT_A::DIV28)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 2)) | (((value as u32) & 0x07) << 2);
        self.w
    }
}
#[doc = "Reader of field `RESETMGMT`"]
pub type RESETMGMT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESETMGMT`"]
pub struct RESETMGMT_W<'a> {
    w: &'a mut W,
}
impl<'a> RESETMGMT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Reading module addresses all MII PHY"]
    #[inline(always)]
    pub fn scaninc(&self) -> SCANINC_R {
        SCANINC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Control bit preamble"]
    #[inline(always)]
    pub fn nopre(&self) -> NOPRE_R {
        NOPRE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn clock_select(&self) -> CLOCK_SELECT_R {
        CLOCK_SELECT_R::new(((self.bits >> 2) & 0x07) as u8)
    }
    #[doc = "Bit 15 - Reset bit MII management module"]
    #[inline(always)]
    pub fn resetmgmt(&self) -> RESETMGMT_R {
        RESETMGMT_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reading module addresses all MII PHY"]
    #[inline(always)]
    pub fn scaninc(&mut self) -> SCANINC_W {
        SCANINC_W { w: self }
    }
    #[doc = "Bit 1 - Control bit preamble"]
    #[inline(always)]
    pub fn nopre(&mut self) -> NOPRE_W {
        NOPRE_W { w: self }
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn clock_select(&mut self) -> CLOCK_SELECT_W {
        CLOCK_SELECT_W { w: self }
    }
    #[doc = "Bit 15 - Reset bit MII management module"]
    #[inline(always)]
    pub fn resetmgmt(&mut self) -> RESETMGMT_W {
        RESETMGMT_W { w: self }
    }
}
