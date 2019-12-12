#[doc = "Reader of register RICNT"]
pub type R = crate::R<u32, super::RICNT>;
#[doc = "Writer for register RICNT"]
pub type W = crate::W<u32, super::RICNT>;
#[doc = "Register RICNT `reset()`'s with value 0"]
impl crate::ResetValue for super::RICNT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "ADC8 average control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AVG8_A {
    #[doc = "0: Average disabled"]
    DISABLE,
    #[doc = "1: Average with 2 measures"]
    AVERAGE2,
    #[doc = "2: Average with 4 measures"]
    AVERAGE4,
    #[doc = "3: Average with 8 measures"]
    AVERAGE8,
    #[doc = "4: Average with 16 measures"]
    AVERAGE16,
    #[doc = "5: Average with 32 measures"]
    AVERAGE32,
    #[doc = "6: Average with 64 measures"]
    AVERAGE64,
}
impl From<AVG8_A> for u8 {
    #[inline(always)]
    fn from(variant: AVG8_A) -> Self {
        match variant {
            AVG8_A::DISABLE => 0,
            AVG8_A::AVERAGE2 => 1,
            AVG8_A::AVERAGE4 => 2,
            AVG8_A::AVERAGE8 => 3,
            AVG8_A::AVERAGE16 => 4,
            AVG8_A::AVERAGE32 => 5,
            AVG8_A::AVERAGE64 => 6,
        }
    }
}
#[doc = "Reader of field `AVG8`"]
pub type AVG8_R = crate::R<u8, AVG8_A>;
impl AVG8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, AVG8_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(AVG8_A::DISABLE),
            1 => Val(AVG8_A::AVERAGE2),
            2 => Val(AVG8_A::AVERAGE4),
            3 => Val(AVG8_A::AVERAGE8),
            4 => Val(AVG8_A::AVERAGE16),
            5 => Val(AVG8_A::AVERAGE32),
            6 => Val(AVG8_A::AVERAGE64),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == AVG8_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `AVERAGE2`"]
    #[inline(always)]
    pub fn is_average2(&self) -> bool {
        *self == AVG8_A::AVERAGE2
    }
    #[doc = "Checks if the value of the field is `AVERAGE4`"]
    #[inline(always)]
    pub fn is_average4(&self) -> bool {
        *self == AVG8_A::AVERAGE4
    }
    #[doc = "Checks if the value of the field is `AVERAGE8`"]
    #[inline(always)]
    pub fn is_average8(&self) -> bool {
        *self == AVG8_A::AVERAGE8
    }
    #[doc = "Checks if the value of the field is `AVERAGE16`"]
    #[inline(always)]
    pub fn is_average16(&self) -> bool {
        *self == AVG8_A::AVERAGE16
    }
    #[doc = "Checks if the value of the field is `AVERAGE32`"]
    #[inline(always)]
    pub fn is_average32(&self) -> bool {
        *self == AVG8_A::AVERAGE32
    }
    #[doc = "Checks if the value of the field is `AVERAGE64`"]
    #[inline(always)]
    pub fn is_average64(&self) -> bool {
        *self == AVG8_A::AVERAGE64
    }
}
#[doc = "Write proxy for field `AVG8`"]
pub struct AVG8_W<'a> {
    w: &'a mut W,
}
impl<'a> AVG8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AVG8_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Average disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(AVG8_A::DISABLE)
    }
    #[doc = "Average with 2 measures"]
    #[inline(always)]
    pub fn average2(self) -> &'a mut W {
        self.variant(AVG8_A::AVERAGE2)
    }
    #[doc = "Average with 4 measures"]
    #[inline(always)]
    pub fn average4(self) -> &'a mut W {
        self.variant(AVG8_A::AVERAGE4)
    }
    #[doc = "Average with 8 measures"]
    #[inline(always)]
    pub fn average8(self) -> &'a mut W {
        self.variant(AVG8_A::AVERAGE8)
    }
    #[doc = "Average with 16 measures"]
    #[inline(always)]
    pub fn average16(self) -> &'a mut W {
        self.variant(AVG8_A::AVERAGE16)
    }
    #[doc = "Average with 32 measures"]
    #[inline(always)]
    pub fn average32(self) -> &'a mut W {
        self.variant(AVG8_A::AVERAGE32)
    }
    #[doc = "Average with 64 measures"]
    #[inline(always)]
    pub fn average64(self) -> &'a mut W {
        self.variant(AVG8_A::AVERAGE64)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "ADC9 average control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AVG9_A {
    #[doc = "0: Average disabled"]
    DISABLE,
    #[doc = "1: Average with 2 measures"]
    AVERAGE2,
    #[doc = "2: Average with 4 measures"]
    AVERAGE4,
    #[doc = "3: Average with 8 measures"]
    AVERAGE8,
    #[doc = "4: Average with 16 measures"]
    AVERAGE16,
    #[doc = "5: Average with 32 measures"]
    AVERAGE32,
    #[doc = "6: Average with 64 measures"]
    AVERAGE64,
}
impl From<AVG9_A> for u8 {
    #[inline(always)]
    fn from(variant: AVG9_A) -> Self {
        match variant {
            AVG9_A::DISABLE => 0,
            AVG9_A::AVERAGE2 => 1,
            AVG9_A::AVERAGE4 => 2,
            AVG9_A::AVERAGE8 => 3,
            AVG9_A::AVERAGE16 => 4,
            AVG9_A::AVERAGE32 => 5,
            AVG9_A::AVERAGE64 => 6,
        }
    }
}
#[doc = "Reader of field `AVG9`"]
pub type AVG9_R = crate::R<u8, AVG9_A>;
impl AVG9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, AVG9_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(AVG9_A::DISABLE),
            1 => Val(AVG9_A::AVERAGE2),
            2 => Val(AVG9_A::AVERAGE4),
            3 => Val(AVG9_A::AVERAGE8),
            4 => Val(AVG9_A::AVERAGE16),
            5 => Val(AVG9_A::AVERAGE32),
            6 => Val(AVG9_A::AVERAGE64),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == AVG9_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `AVERAGE2`"]
    #[inline(always)]
    pub fn is_average2(&self) -> bool {
        *self == AVG9_A::AVERAGE2
    }
    #[doc = "Checks if the value of the field is `AVERAGE4`"]
    #[inline(always)]
    pub fn is_average4(&self) -> bool {
        *self == AVG9_A::AVERAGE4
    }
    #[doc = "Checks if the value of the field is `AVERAGE8`"]
    #[inline(always)]
    pub fn is_average8(&self) -> bool {
        *self == AVG9_A::AVERAGE8
    }
    #[doc = "Checks if the value of the field is `AVERAGE16`"]
    #[inline(always)]
    pub fn is_average16(&self) -> bool {
        *self == AVG9_A::AVERAGE16
    }
    #[doc = "Checks if the value of the field is `AVERAGE32`"]
    #[inline(always)]
    pub fn is_average32(&self) -> bool {
        *self == AVG9_A::AVERAGE32
    }
    #[doc = "Checks if the value of the field is `AVERAGE64`"]
    #[inline(always)]
    pub fn is_average64(&self) -> bool {
        *self == AVG9_A::AVERAGE64
    }
}
#[doc = "Write proxy for field `AVG9`"]
pub struct AVG9_W<'a> {
    w: &'a mut W,
}
impl<'a> AVG9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AVG9_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Average disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(AVG9_A::DISABLE)
    }
    #[doc = "Average with 2 measures"]
    #[inline(always)]
    pub fn average2(self) -> &'a mut W {
        self.variant(AVG9_A::AVERAGE2)
    }
    #[doc = "Average with 4 measures"]
    #[inline(always)]
    pub fn average4(self) -> &'a mut W {
        self.variant(AVG9_A::AVERAGE4)
    }
    #[doc = "Average with 8 measures"]
    #[inline(always)]
    pub fn average8(self) -> &'a mut W {
        self.variant(AVG9_A::AVERAGE8)
    }
    #[doc = "Average with 16 measures"]
    #[inline(always)]
    pub fn average16(self) -> &'a mut W {
        self.variant(AVG9_A::AVERAGE16)
    }
    #[doc = "Average with 32 measures"]
    #[inline(always)]
    pub fn average32(self) -> &'a mut W {
        self.variant(AVG9_A::AVERAGE32)
    }
    #[doc = "Average with 64 measures"]
    #[inline(always)]
    pub fn average64(self) -> &'a mut W {
        self.variant(AVG9_A::AVERAGE64)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "ADC10 average control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AVG10_A {
    #[doc = "0: Average disabled"]
    DISABLE,
    #[doc = "1: Average with 2 measures"]
    AVERAGE2,
    #[doc = "2: Average with 4 measures"]
    AVERAGE4,
    #[doc = "3: Average with 8 measures"]
    AVERAGE8,
    #[doc = "4: Average with 16 measures"]
    AVERAGE16,
    #[doc = "5: Average with 32 measures"]
    AVERAGE32,
    #[doc = "6: Average with 64 measures"]
    AVERAGE64,
}
impl From<AVG10_A> for u8 {
    #[inline(always)]
    fn from(variant: AVG10_A) -> Self {
        match variant {
            AVG10_A::DISABLE => 0,
            AVG10_A::AVERAGE2 => 1,
            AVG10_A::AVERAGE4 => 2,
            AVG10_A::AVERAGE8 => 3,
            AVG10_A::AVERAGE16 => 4,
            AVG10_A::AVERAGE32 => 5,
            AVG10_A::AVERAGE64 => 6,
        }
    }
}
#[doc = "Reader of field `AVG10`"]
pub type AVG10_R = crate::R<u8, AVG10_A>;
impl AVG10_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, AVG10_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(AVG10_A::DISABLE),
            1 => Val(AVG10_A::AVERAGE2),
            2 => Val(AVG10_A::AVERAGE4),
            3 => Val(AVG10_A::AVERAGE8),
            4 => Val(AVG10_A::AVERAGE16),
            5 => Val(AVG10_A::AVERAGE32),
            6 => Val(AVG10_A::AVERAGE64),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == AVG10_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `AVERAGE2`"]
    #[inline(always)]
    pub fn is_average2(&self) -> bool {
        *self == AVG10_A::AVERAGE2
    }
    #[doc = "Checks if the value of the field is `AVERAGE4`"]
    #[inline(always)]
    pub fn is_average4(&self) -> bool {
        *self == AVG10_A::AVERAGE4
    }
    #[doc = "Checks if the value of the field is `AVERAGE8`"]
    #[inline(always)]
    pub fn is_average8(&self) -> bool {
        *self == AVG10_A::AVERAGE8
    }
    #[doc = "Checks if the value of the field is `AVERAGE16`"]
    #[inline(always)]
    pub fn is_average16(&self) -> bool {
        *self == AVG10_A::AVERAGE16
    }
    #[doc = "Checks if the value of the field is `AVERAGE32`"]
    #[inline(always)]
    pub fn is_average32(&self) -> bool {
        *self == AVG10_A::AVERAGE32
    }
    #[doc = "Checks if the value of the field is `AVERAGE64`"]
    #[inline(always)]
    pub fn is_average64(&self) -> bool {
        *self == AVG10_A::AVERAGE64
    }
}
#[doc = "Write proxy for field `AVG10`"]
pub struct AVG10_W<'a> {
    w: &'a mut W,
}
impl<'a> AVG10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AVG10_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Average disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(AVG10_A::DISABLE)
    }
    #[doc = "Average with 2 measures"]
    #[inline(always)]
    pub fn average2(self) -> &'a mut W {
        self.variant(AVG10_A::AVERAGE2)
    }
    #[doc = "Average with 4 measures"]
    #[inline(always)]
    pub fn average4(self) -> &'a mut W {
        self.variant(AVG10_A::AVERAGE4)
    }
    #[doc = "Average with 8 measures"]
    #[inline(always)]
    pub fn average8(self) -> &'a mut W {
        self.variant(AVG10_A::AVERAGE8)
    }
    #[doc = "Average with 16 measures"]
    #[inline(always)]
    pub fn average16(self) -> &'a mut W {
        self.variant(AVG10_A::AVERAGE16)
    }
    #[doc = "Average with 32 measures"]
    #[inline(always)]
    pub fn average32(self) -> &'a mut W {
        self.variant(AVG10_A::AVERAGE32)
    }
    #[doc = "Average with 64 measures"]
    #[inline(always)]
    pub fn average64(self) -> &'a mut W {
        self.variant(AVG10_A::AVERAGE64)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "ADC11 average control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AVG11_A {
    #[doc = "0: Average disabled"]
    DISABLE,
    #[doc = "1: Average with 2 measures"]
    AVERAGE2,
    #[doc = "2: Average with 4 measures"]
    AVERAGE4,
    #[doc = "3: Average with 8 measures"]
    AVERAGE8,
    #[doc = "4: Average with 16 measures"]
    AVERAGE16,
    #[doc = "5: Average with 32 measures"]
    AVERAGE32,
    #[doc = "6: Average with 64 measures"]
    AVERAGE64,
}
impl From<AVG11_A> for u8 {
    #[inline(always)]
    fn from(variant: AVG11_A) -> Self {
        match variant {
            AVG11_A::DISABLE => 0,
            AVG11_A::AVERAGE2 => 1,
            AVG11_A::AVERAGE4 => 2,
            AVG11_A::AVERAGE8 => 3,
            AVG11_A::AVERAGE16 => 4,
            AVG11_A::AVERAGE32 => 5,
            AVG11_A::AVERAGE64 => 6,
        }
    }
}
#[doc = "Reader of field `AVG11`"]
pub type AVG11_R = crate::R<u8, AVG11_A>;
impl AVG11_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, AVG11_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(AVG11_A::DISABLE),
            1 => Val(AVG11_A::AVERAGE2),
            2 => Val(AVG11_A::AVERAGE4),
            3 => Val(AVG11_A::AVERAGE8),
            4 => Val(AVG11_A::AVERAGE16),
            5 => Val(AVG11_A::AVERAGE32),
            6 => Val(AVG11_A::AVERAGE64),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == AVG11_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `AVERAGE2`"]
    #[inline(always)]
    pub fn is_average2(&self) -> bool {
        *self == AVG11_A::AVERAGE2
    }
    #[doc = "Checks if the value of the field is `AVERAGE4`"]
    #[inline(always)]
    pub fn is_average4(&self) -> bool {
        *self == AVG11_A::AVERAGE4
    }
    #[doc = "Checks if the value of the field is `AVERAGE8`"]
    #[inline(always)]
    pub fn is_average8(&self) -> bool {
        *self == AVG11_A::AVERAGE8
    }
    #[doc = "Checks if the value of the field is `AVERAGE16`"]
    #[inline(always)]
    pub fn is_average16(&self) -> bool {
        *self == AVG11_A::AVERAGE16
    }
    #[doc = "Checks if the value of the field is `AVERAGE32`"]
    #[inline(always)]
    pub fn is_average32(&self) -> bool {
        *self == AVG11_A::AVERAGE32
    }
    #[doc = "Checks if the value of the field is `AVERAGE64`"]
    #[inline(always)]
    pub fn is_average64(&self) -> bool {
        *self == AVG11_A::AVERAGE64
    }
}
#[doc = "Write proxy for field `AVG11`"]
pub struct AVG11_W<'a> {
    w: &'a mut W,
}
impl<'a> AVG11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AVG11_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Average disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(AVG11_A::DISABLE)
    }
    #[doc = "Average with 2 measures"]
    #[inline(always)]
    pub fn average2(self) -> &'a mut W {
        self.variant(AVG11_A::AVERAGE2)
    }
    #[doc = "Average with 4 measures"]
    #[inline(always)]
    pub fn average4(self) -> &'a mut W {
        self.variant(AVG11_A::AVERAGE4)
    }
    #[doc = "Average with 8 measures"]
    #[inline(always)]
    pub fn average8(self) -> &'a mut W {
        self.variant(AVG11_A::AVERAGE8)
    }
    #[doc = "Average with 16 measures"]
    #[inline(always)]
    pub fn average16(self) -> &'a mut W {
        self.variant(AVG11_A::AVERAGE16)
    }
    #[doc = "Average with 32 measures"]
    #[inline(always)]
    pub fn average32(self) -> &'a mut W {
        self.variant(AVG11_A::AVERAGE32)
    }
    #[doc = "Average with 64 measures"]
    #[inline(always)]
    pub fn average64(self) -> &'a mut W {
        self.variant(AVG11_A::AVERAGE64)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "Reader of field `RICNT0`"]
pub type RICNT0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RICNT0`"]
pub struct RICNT0_W<'a> {
    w: &'a mut W,
}
impl<'a> RICNT0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `RICNT1`"]
pub type RICNT1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RICNT1`"]
pub struct RICNT1_W<'a> {
    w: &'a mut W,
}
impl<'a> RICNT1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `RICNT2`"]
pub type RICNT2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RICNT2`"]
pub struct RICNT2_W<'a> {
    w: &'a mut W,
}
impl<'a> RICNT2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `RICNT3`"]
pub type RICNT3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RICNT3`"]
pub struct RICNT3_W<'a> {
    w: &'a mut W,
}
impl<'a> RICNT3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `RICNT4`"]
pub type RICNT4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RICNT4`"]
pub struct RICNT4_W<'a> {
    w: &'a mut W,
}
impl<'a> RICNT4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `RICNT5`"]
pub type RICNT5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RICNT5`"]
pub struct RICNT5_W<'a> {
    w: &'a mut W,
}
impl<'a> RICNT5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `RICNT6`"]
pub type RICNT6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RICNT6`"]
pub struct RICNT6_W<'a> {
    w: &'a mut W,
}
impl<'a> RICNT6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `RICNT7`"]
pub type RICNT7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RICNT7`"]
pub struct RICNT7_W<'a> {
    w: &'a mut W,
}
impl<'a> RICNT7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - ADC8 average control"]
    #[inline(always)]
    pub fn avg8(&self) -> AVG8_R {
        AVG8_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - ADC9 average control"]
    #[inline(always)]
    pub fn avg9(&self) -> AVG9_R {
        AVG9_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 8:10 - ADC10 average control"]
    #[inline(always)]
    pub fn avg10(&self) -> AVG10_R {
        AVG10_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 12:14 - ADC11 average control"]
    #[inline(always)]
    pub fn avg11(&self) -> AVG11_R {
        AVG11_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bit 16 - Disable interrupt counter reset on sequencer 0 start"]
    #[inline(always)]
    pub fn ricnt0(&self) -> RICNT0_R {
        RICNT0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Disable interrupt counter reset on sequencer 1 start"]
    #[inline(always)]
    pub fn ricnt1(&self) -> RICNT1_R {
        RICNT1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Disable interrupt counter reset on sequencer 2 start"]
    #[inline(always)]
    pub fn ricnt2(&self) -> RICNT2_R {
        RICNT2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Disable interrupt counter reset on sequencer 3 start"]
    #[inline(always)]
    pub fn ricnt3(&self) -> RICNT3_R {
        RICNT3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Disable interrupt counter reset on sequencer 4 start"]
    #[inline(always)]
    pub fn ricnt4(&self) -> RICNT4_R {
        RICNT4_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Disable interrupt counter reset on sequencer 5 start"]
    #[inline(always)]
    pub fn ricnt5(&self) -> RICNT5_R {
        RICNT5_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Disable interrupt counter reset on sequencer 6 start"]
    #[inline(always)]
    pub fn ricnt6(&self) -> RICNT6_R {
        RICNT6_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Disable interrupt counter reset on sequencer 7 start"]
    #[inline(always)]
    pub fn ricnt7(&self) -> RICNT7_R {
        RICNT7_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - ADC8 average control"]
    #[inline(always)]
    pub fn avg8(&mut self) -> AVG8_W {
        AVG8_W { w: self }
    }
    #[doc = "Bits 4:6 - ADC9 average control"]
    #[inline(always)]
    pub fn avg9(&mut self) -> AVG9_W {
        AVG9_W { w: self }
    }
    #[doc = "Bits 8:10 - ADC10 average control"]
    #[inline(always)]
    pub fn avg10(&mut self) -> AVG10_W {
        AVG10_W { w: self }
    }
    #[doc = "Bits 12:14 - ADC11 average control"]
    #[inline(always)]
    pub fn avg11(&mut self) -> AVG11_W {
        AVG11_W { w: self }
    }
    #[doc = "Bit 16 - Disable interrupt counter reset on sequencer 0 start"]
    #[inline(always)]
    pub fn ricnt0(&mut self) -> RICNT0_W {
        RICNT0_W { w: self }
    }
    #[doc = "Bit 17 - Disable interrupt counter reset on sequencer 1 start"]
    #[inline(always)]
    pub fn ricnt1(&mut self) -> RICNT1_W {
        RICNT1_W { w: self }
    }
    #[doc = "Bit 18 - Disable interrupt counter reset on sequencer 2 start"]
    #[inline(always)]
    pub fn ricnt2(&mut self) -> RICNT2_W {
        RICNT2_W { w: self }
    }
    #[doc = "Bit 19 - Disable interrupt counter reset on sequencer 3 start"]
    #[inline(always)]
    pub fn ricnt3(&mut self) -> RICNT3_W {
        RICNT3_W { w: self }
    }
    #[doc = "Bit 20 - Disable interrupt counter reset on sequencer 4 start"]
    #[inline(always)]
    pub fn ricnt4(&mut self) -> RICNT4_W {
        RICNT4_W { w: self }
    }
    #[doc = "Bit 21 - Disable interrupt counter reset on sequencer 5 start"]
    #[inline(always)]
    pub fn ricnt5(&mut self) -> RICNT5_W {
        RICNT5_W { w: self }
    }
    #[doc = "Bit 22 - Disable interrupt counter reset on sequencer 6 start"]
    #[inline(always)]
    pub fn ricnt6(&mut self) -> RICNT6_W {
        RICNT6_W { w: self }
    }
    #[doc = "Bit 23 - Disable interrupt counter reset on sequencer 7 start"]
    #[inline(always)]
    pub fn ricnt7(&mut self) -> RICNT7_W {
        RICNT7_W { w: self }
    }
}
