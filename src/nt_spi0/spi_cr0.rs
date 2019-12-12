#[doc = "Reader of register SPI_CR0"]
pub type R = crate::R<u32, super::SPI_CR0>;
#[doc = "Writer for register SPI_CR0"]
pub type W = crate::W<u32, super::SPI_CR0>;
#[doc = "Register SPI_CR0 `reset()`'s with value 0"]
impl crate::ResetValue for super::SPI_CR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSS_A {
    #[doc = "3: data size 4 bit"]
    _4BIT,
    #[doc = "4: data size 5 bit"]
    _5BIT,
    #[doc = "5: data size 6 bit"]
    _6BIT,
    #[doc = "6: data size 7 bit"]
    _7BIT,
    #[doc = "7: data size 8 bit"]
    _8BIT,
    #[doc = "8: data size 9 bit"]
    _9BIT,
    #[doc = "9: data size 10 bit"]
    _10BIT,
    #[doc = "10: data size 11 bit"]
    _11BIT,
    #[doc = "11: data size 12 bit"]
    _12BIT,
    #[doc = "12: data size 13 bit"]
    _13BIT,
    #[doc = "13: data size 14 bit"]
    _14BIT,
    #[doc = "14: data size 15 bit"]
    _15BIT,
    #[doc = "15: data size 16 bit"]
    _16BIT,
}
impl From<DSS_A> for u8 {
    #[inline(always)]
    fn from(variant: DSS_A) -> Self {
        match variant {
            DSS_A::_4BIT => 3,
            DSS_A::_5BIT => 4,
            DSS_A::_6BIT => 5,
            DSS_A::_7BIT => 6,
            DSS_A::_8BIT => 7,
            DSS_A::_9BIT => 8,
            DSS_A::_10BIT => 9,
            DSS_A::_11BIT => 10,
            DSS_A::_12BIT => 11,
            DSS_A::_13BIT => 12,
            DSS_A::_14BIT => 13,
            DSS_A::_15BIT => 14,
            DSS_A::_16BIT => 15,
        }
    }
}
#[doc = "Reader of field `DSS`"]
pub type DSS_R = crate::R<u8, DSS_A>;
impl DSS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DSS_A> {
        use crate::Variant::*;
        match self.bits {
            3 => Val(DSS_A::_4BIT),
            4 => Val(DSS_A::_5BIT),
            5 => Val(DSS_A::_6BIT),
            6 => Val(DSS_A::_7BIT),
            7 => Val(DSS_A::_8BIT),
            8 => Val(DSS_A::_9BIT),
            9 => Val(DSS_A::_10BIT),
            10 => Val(DSS_A::_11BIT),
            11 => Val(DSS_A::_12BIT),
            12 => Val(DSS_A::_13BIT),
            13 => Val(DSS_A::_14BIT),
            14 => Val(DSS_A::_15BIT),
            15 => Val(DSS_A::_16BIT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_4BIT`"]
    #[inline(always)]
    pub fn is_4bit(&self) -> bool {
        *self == DSS_A::_4BIT
    }
    #[doc = "Checks if the value of the field is `_5BIT`"]
    #[inline(always)]
    pub fn is_5bit(&self) -> bool {
        *self == DSS_A::_5BIT
    }
    #[doc = "Checks if the value of the field is `_6BIT`"]
    #[inline(always)]
    pub fn is_6bit(&self) -> bool {
        *self == DSS_A::_6BIT
    }
    #[doc = "Checks if the value of the field is `_7BIT`"]
    #[inline(always)]
    pub fn is_7bit(&self) -> bool {
        *self == DSS_A::_7BIT
    }
    #[doc = "Checks if the value of the field is `_8BIT`"]
    #[inline(always)]
    pub fn is_8bit(&self) -> bool {
        *self == DSS_A::_8BIT
    }
    #[doc = "Checks if the value of the field is `_9BIT`"]
    #[inline(always)]
    pub fn is_9bit(&self) -> bool {
        *self == DSS_A::_9BIT
    }
    #[doc = "Checks if the value of the field is `_10BIT`"]
    #[inline(always)]
    pub fn is_10bit(&self) -> bool {
        *self == DSS_A::_10BIT
    }
    #[doc = "Checks if the value of the field is `_11BIT`"]
    #[inline(always)]
    pub fn is_11bit(&self) -> bool {
        *self == DSS_A::_11BIT
    }
    #[doc = "Checks if the value of the field is `_12BIT`"]
    #[inline(always)]
    pub fn is_12bit(&self) -> bool {
        *self == DSS_A::_12BIT
    }
    #[doc = "Checks if the value of the field is `_13BIT`"]
    #[inline(always)]
    pub fn is_13bit(&self) -> bool {
        *self == DSS_A::_13BIT
    }
    #[doc = "Checks if the value of the field is `_14BIT`"]
    #[inline(always)]
    pub fn is_14bit(&self) -> bool {
        *self == DSS_A::_14BIT
    }
    #[doc = "Checks if the value of the field is `_15BIT`"]
    #[inline(always)]
    pub fn is_15bit(&self) -> bool {
        *self == DSS_A::_15BIT
    }
    #[doc = "Checks if the value of the field is `_16BIT`"]
    #[inline(always)]
    pub fn is_16bit(&self) -> bool {
        *self == DSS_A::_16BIT
    }
}
#[doc = "Write proxy for field `DSS`"]
pub struct DSS_W<'a> {
    w: &'a mut W,
}
impl<'a> DSS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "data size 4 bit"]
    #[inline(always)]
    pub fn _4bit(self) -> &'a mut W {
        self.variant(DSS_A::_4BIT)
    }
    #[doc = "data size 5 bit"]
    #[inline(always)]
    pub fn _5bit(self) -> &'a mut W {
        self.variant(DSS_A::_5BIT)
    }
    #[doc = "data size 6 bit"]
    #[inline(always)]
    pub fn _6bit(self) -> &'a mut W {
        self.variant(DSS_A::_6BIT)
    }
    #[doc = "data size 7 bit"]
    #[inline(always)]
    pub fn _7bit(self) -> &'a mut W {
        self.variant(DSS_A::_7BIT)
    }
    #[doc = "data size 8 bit"]
    #[inline(always)]
    pub fn _8bit(self) -> &'a mut W {
        self.variant(DSS_A::_8BIT)
    }
    #[doc = "data size 9 bit"]
    #[inline(always)]
    pub fn _9bit(self) -> &'a mut W {
        self.variant(DSS_A::_9BIT)
    }
    #[doc = "data size 10 bit"]
    #[inline(always)]
    pub fn _10bit(self) -> &'a mut W {
        self.variant(DSS_A::_10BIT)
    }
    #[doc = "data size 11 bit"]
    #[inline(always)]
    pub fn _11bit(self) -> &'a mut W {
        self.variant(DSS_A::_11BIT)
    }
    #[doc = "data size 12 bit"]
    #[inline(always)]
    pub fn _12bit(self) -> &'a mut W {
        self.variant(DSS_A::_12BIT)
    }
    #[doc = "data size 13 bit"]
    #[inline(always)]
    pub fn _13bit(self) -> &'a mut W {
        self.variant(DSS_A::_13BIT)
    }
    #[doc = "data size 14 bit"]
    #[inline(always)]
    pub fn _14bit(self) -> &'a mut W {
        self.variant(DSS_A::_14BIT)
    }
    #[doc = "data size 15 bit"]
    #[inline(always)]
    pub fn _15bit(self) -> &'a mut W {
        self.variant(DSS_A::_15BIT)
    }
    #[doc = "data size 16 bit"]
    #[inline(always)]
    pub fn _16bit(self) -> &'a mut W {
        self.variant(DSS_A::_16BIT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRF_A {
    #[doc = "0: SPI of Motorola"]
    SPI,
    #[doc = "1: SSI of Texas Instruments"]
    SSI,
    #[doc = "2: Microwire of National Semiconductor"]
    MICROWIRE,
}
impl From<FRF_A> for u8 {
    #[inline(always)]
    fn from(variant: FRF_A) -> Self {
        match variant {
            FRF_A::SPI => 0,
            FRF_A::SSI => 1,
            FRF_A::MICROWIRE => 2,
        }
    }
}
#[doc = "Reader of field `FRF`"]
pub type FRF_R = crate::R<u8, FRF_A>;
impl FRF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FRF_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FRF_A::SPI),
            1 => Val(FRF_A::SSI),
            2 => Val(FRF_A::MICROWIRE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SPI`"]
    #[inline(always)]
    pub fn is_spi(&self) -> bool {
        *self == FRF_A::SPI
    }
    #[doc = "Checks if the value of the field is `SSI`"]
    #[inline(always)]
    pub fn is_ssi(&self) -> bool {
        *self == FRF_A::SSI
    }
    #[doc = "Checks if the value of the field is `MICROWIRE`"]
    #[inline(always)]
    pub fn is_microwire(&self) -> bool {
        *self == FRF_A::MICROWIRE
    }
}
#[doc = "Write proxy for field `FRF`"]
pub struct FRF_W<'a> {
    w: &'a mut W,
}
impl<'a> FRF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRF_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "SPI of Motorola"]
    #[inline(always)]
    pub fn spi(self) -> &'a mut W {
        self.variant(FRF_A::SPI)
    }
    #[doc = "SSI of Texas Instruments"]
    #[inline(always)]
    pub fn ssi(self) -> &'a mut W {
        self.variant(FRF_A::SSI)
    }
    #[doc = "Microwire of National Semiconductor"]
    #[inline(always)]
    pub fn microwire(self) -> &'a mut W {
        self.variant(FRF_A::MICROWIRE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `SPO`"]
pub type SPO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPO`"]
pub struct SPO_W<'a> {
    w: &'a mut W,
}
impl<'a> SPO_W<'a> {
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
#[doc = "Reader of field `SPH`"]
pub type SPH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPH`"]
pub struct SPH_W<'a> {
    w: &'a mut W,
}
impl<'a> SPH_W<'a> {
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
#[doc = "Reader of field `SCR`"]
pub type SCR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SCR`"]
pub struct SCR_W<'a> {
    w: &'a mut W,
}
impl<'a> SCR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn dss(&self) -> DSS_R {
        DSS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn frf(&self) -> FRF_R {
        FRF_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 6 - Polarity SSPCLKOUT"]
    #[inline(always)]
    pub fn spo(&self) -> SPO_R {
        SPO_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Phase SSPCLKOUT"]
    #[inline(always)]
    pub fn sph(&self) -> SPH_R {
        SPH_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn scr(&self) -> SCR_R {
        SCR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn dss(&mut self) -> DSS_W {
        DSS_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn frf(&mut self) -> FRF_W {
        FRF_W { w: self }
    }
    #[doc = "Bit 6 - Polarity SSPCLKOUT"]
    #[inline(always)]
    pub fn spo(&mut self) -> SPO_W {
        SPO_W { w: self }
    }
    #[doc = "Bit 7 - Phase SSPCLKOUT"]
    #[inline(always)]
    pub fn sph(&mut self) -> SPH_W {
        SPH_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn scr(&mut self) -> SCR_W {
        SCR_W { w: self }
    }
}
