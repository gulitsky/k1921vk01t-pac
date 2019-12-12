#[doc = "Reader of register SAC"]
pub type R = crate::R<u32, super::SAC>;
#[doc = "Writer for register SAC"]
pub type W = crate::W<u32, super::SAC>;
#[doc = "Register SAC `reset()`'s with value 0"]
impl crate::ResetValue for super::SAC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "ADC0 average control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AVG0_A {
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
impl From<AVG0_A> for u8 {
    #[inline(always)]
    fn from(variant: AVG0_A) -> Self {
        match variant {
            AVG0_A::DISABLE => 0,
            AVG0_A::AVERAGE2 => 1,
            AVG0_A::AVERAGE4 => 2,
            AVG0_A::AVERAGE8 => 3,
            AVG0_A::AVERAGE16 => 4,
            AVG0_A::AVERAGE32 => 5,
            AVG0_A::AVERAGE64 => 6,
        }
    }
}
#[doc = "Reader of field `AVG0`"]
pub type AVG0_R = crate::R<u8, AVG0_A>;
impl AVG0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, AVG0_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(AVG0_A::DISABLE),
            1 => Val(AVG0_A::AVERAGE2),
            2 => Val(AVG0_A::AVERAGE4),
            3 => Val(AVG0_A::AVERAGE8),
            4 => Val(AVG0_A::AVERAGE16),
            5 => Val(AVG0_A::AVERAGE32),
            6 => Val(AVG0_A::AVERAGE64),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == AVG0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `AVERAGE2`"]
    #[inline(always)]
    pub fn is_average2(&self) -> bool {
        *self == AVG0_A::AVERAGE2
    }
    #[doc = "Checks if the value of the field is `AVERAGE4`"]
    #[inline(always)]
    pub fn is_average4(&self) -> bool {
        *self == AVG0_A::AVERAGE4
    }
    #[doc = "Checks if the value of the field is `AVERAGE8`"]
    #[inline(always)]
    pub fn is_average8(&self) -> bool {
        *self == AVG0_A::AVERAGE8
    }
    #[doc = "Checks if the value of the field is `AVERAGE16`"]
    #[inline(always)]
    pub fn is_average16(&self) -> bool {
        *self == AVG0_A::AVERAGE16
    }
    #[doc = "Checks if the value of the field is `AVERAGE32`"]
    #[inline(always)]
    pub fn is_average32(&self) -> bool {
        *self == AVG0_A::AVERAGE32
    }
    #[doc = "Checks if the value of the field is `AVERAGE64`"]
    #[inline(always)]
    pub fn is_average64(&self) -> bool {
        *self == AVG0_A::AVERAGE64
    }
}
#[doc = "Write proxy for field `AVG0`"]
pub struct AVG0_W<'a> {
    w: &'a mut W,
}
impl<'a> AVG0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AVG0_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Average disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(AVG0_A::DISABLE)
    }
    #[doc = "Average with 2 measures"]
    #[inline(always)]
    pub fn average2(self) -> &'a mut W {
        self.variant(AVG0_A::AVERAGE2)
    }
    #[doc = "Average with 4 measures"]
    #[inline(always)]
    pub fn average4(self) -> &'a mut W {
        self.variant(AVG0_A::AVERAGE4)
    }
    #[doc = "Average with 8 measures"]
    #[inline(always)]
    pub fn average8(self) -> &'a mut W {
        self.variant(AVG0_A::AVERAGE8)
    }
    #[doc = "Average with 16 measures"]
    #[inline(always)]
    pub fn average16(self) -> &'a mut W {
        self.variant(AVG0_A::AVERAGE16)
    }
    #[doc = "Average with 32 measures"]
    #[inline(always)]
    pub fn average32(self) -> &'a mut W {
        self.variant(AVG0_A::AVERAGE32)
    }
    #[doc = "Average with 64 measures"]
    #[inline(always)]
    pub fn average64(self) -> &'a mut W {
        self.variant(AVG0_A::AVERAGE64)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "ADC1 average control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AVG1_A {
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
impl From<AVG1_A> for u8 {
    #[inline(always)]
    fn from(variant: AVG1_A) -> Self {
        match variant {
            AVG1_A::DISABLE => 0,
            AVG1_A::AVERAGE2 => 1,
            AVG1_A::AVERAGE4 => 2,
            AVG1_A::AVERAGE8 => 3,
            AVG1_A::AVERAGE16 => 4,
            AVG1_A::AVERAGE32 => 5,
            AVG1_A::AVERAGE64 => 6,
        }
    }
}
#[doc = "Reader of field `AVG1`"]
pub type AVG1_R = crate::R<u8, AVG1_A>;
impl AVG1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, AVG1_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(AVG1_A::DISABLE),
            1 => Val(AVG1_A::AVERAGE2),
            2 => Val(AVG1_A::AVERAGE4),
            3 => Val(AVG1_A::AVERAGE8),
            4 => Val(AVG1_A::AVERAGE16),
            5 => Val(AVG1_A::AVERAGE32),
            6 => Val(AVG1_A::AVERAGE64),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == AVG1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `AVERAGE2`"]
    #[inline(always)]
    pub fn is_average2(&self) -> bool {
        *self == AVG1_A::AVERAGE2
    }
    #[doc = "Checks if the value of the field is `AVERAGE4`"]
    #[inline(always)]
    pub fn is_average4(&self) -> bool {
        *self == AVG1_A::AVERAGE4
    }
    #[doc = "Checks if the value of the field is `AVERAGE8`"]
    #[inline(always)]
    pub fn is_average8(&self) -> bool {
        *self == AVG1_A::AVERAGE8
    }
    #[doc = "Checks if the value of the field is `AVERAGE16`"]
    #[inline(always)]
    pub fn is_average16(&self) -> bool {
        *self == AVG1_A::AVERAGE16
    }
    #[doc = "Checks if the value of the field is `AVERAGE32`"]
    #[inline(always)]
    pub fn is_average32(&self) -> bool {
        *self == AVG1_A::AVERAGE32
    }
    #[doc = "Checks if the value of the field is `AVERAGE64`"]
    #[inline(always)]
    pub fn is_average64(&self) -> bool {
        *self == AVG1_A::AVERAGE64
    }
}
#[doc = "Write proxy for field `AVG1`"]
pub struct AVG1_W<'a> {
    w: &'a mut W,
}
impl<'a> AVG1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AVG1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Average disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(AVG1_A::DISABLE)
    }
    #[doc = "Average with 2 measures"]
    #[inline(always)]
    pub fn average2(self) -> &'a mut W {
        self.variant(AVG1_A::AVERAGE2)
    }
    #[doc = "Average with 4 measures"]
    #[inline(always)]
    pub fn average4(self) -> &'a mut W {
        self.variant(AVG1_A::AVERAGE4)
    }
    #[doc = "Average with 8 measures"]
    #[inline(always)]
    pub fn average8(self) -> &'a mut W {
        self.variant(AVG1_A::AVERAGE8)
    }
    #[doc = "Average with 16 measures"]
    #[inline(always)]
    pub fn average16(self) -> &'a mut W {
        self.variant(AVG1_A::AVERAGE16)
    }
    #[doc = "Average with 32 measures"]
    #[inline(always)]
    pub fn average32(self) -> &'a mut W {
        self.variant(AVG1_A::AVERAGE32)
    }
    #[doc = "Average with 64 measures"]
    #[inline(always)]
    pub fn average64(self) -> &'a mut W {
        self.variant(AVG1_A::AVERAGE64)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "ADC2 average control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AVG2_A {
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
impl From<AVG2_A> for u8 {
    #[inline(always)]
    fn from(variant: AVG2_A) -> Self {
        match variant {
            AVG2_A::DISABLE => 0,
            AVG2_A::AVERAGE2 => 1,
            AVG2_A::AVERAGE4 => 2,
            AVG2_A::AVERAGE8 => 3,
            AVG2_A::AVERAGE16 => 4,
            AVG2_A::AVERAGE32 => 5,
            AVG2_A::AVERAGE64 => 6,
        }
    }
}
#[doc = "Reader of field `AVG2`"]
pub type AVG2_R = crate::R<u8, AVG2_A>;
impl AVG2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, AVG2_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(AVG2_A::DISABLE),
            1 => Val(AVG2_A::AVERAGE2),
            2 => Val(AVG2_A::AVERAGE4),
            3 => Val(AVG2_A::AVERAGE8),
            4 => Val(AVG2_A::AVERAGE16),
            5 => Val(AVG2_A::AVERAGE32),
            6 => Val(AVG2_A::AVERAGE64),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == AVG2_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `AVERAGE2`"]
    #[inline(always)]
    pub fn is_average2(&self) -> bool {
        *self == AVG2_A::AVERAGE2
    }
    #[doc = "Checks if the value of the field is `AVERAGE4`"]
    #[inline(always)]
    pub fn is_average4(&self) -> bool {
        *self == AVG2_A::AVERAGE4
    }
    #[doc = "Checks if the value of the field is `AVERAGE8`"]
    #[inline(always)]
    pub fn is_average8(&self) -> bool {
        *self == AVG2_A::AVERAGE8
    }
    #[doc = "Checks if the value of the field is `AVERAGE16`"]
    #[inline(always)]
    pub fn is_average16(&self) -> bool {
        *self == AVG2_A::AVERAGE16
    }
    #[doc = "Checks if the value of the field is `AVERAGE32`"]
    #[inline(always)]
    pub fn is_average32(&self) -> bool {
        *self == AVG2_A::AVERAGE32
    }
    #[doc = "Checks if the value of the field is `AVERAGE64`"]
    #[inline(always)]
    pub fn is_average64(&self) -> bool {
        *self == AVG2_A::AVERAGE64
    }
}
#[doc = "Write proxy for field `AVG2`"]
pub struct AVG2_W<'a> {
    w: &'a mut W,
}
impl<'a> AVG2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AVG2_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Average disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(AVG2_A::DISABLE)
    }
    #[doc = "Average with 2 measures"]
    #[inline(always)]
    pub fn average2(self) -> &'a mut W {
        self.variant(AVG2_A::AVERAGE2)
    }
    #[doc = "Average with 4 measures"]
    #[inline(always)]
    pub fn average4(self) -> &'a mut W {
        self.variant(AVG2_A::AVERAGE4)
    }
    #[doc = "Average with 8 measures"]
    #[inline(always)]
    pub fn average8(self) -> &'a mut W {
        self.variant(AVG2_A::AVERAGE8)
    }
    #[doc = "Average with 16 measures"]
    #[inline(always)]
    pub fn average16(self) -> &'a mut W {
        self.variant(AVG2_A::AVERAGE16)
    }
    #[doc = "Average with 32 measures"]
    #[inline(always)]
    pub fn average32(self) -> &'a mut W {
        self.variant(AVG2_A::AVERAGE32)
    }
    #[doc = "Average with 64 measures"]
    #[inline(always)]
    pub fn average64(self) -> &'a mut W {
        self.variant(AVG2_A::AVERAGE64)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "ADC3 average control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AVG3_A {
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
impl From<AVG3_A> for u8 {
    #[inline(always)]
    fn from(variant: AVG3_A) -> Self {
        match variant {
            AVG3_A::DISABLE => 0,
            AVG3_A::AVERAGE2 => 1,
            AVG3_A::AVERAGE4 => 2,
            AVG3_A::AVERAGE8 => 3,
            AVG3_A::AVERAGE16 => 4,
            AVG3_A::AVERAGE32 => 5,
            AVG3_A::AVERAGE64 => 6,
        }
    }
}
#[doc = "Reader of field `AVG3`"]
pub type AVG3_R = crate::R<u8, AVG3_A>;
impl AVG3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, AVG3_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(AVG3_A::DISABLE),
            1 => Val(AVG3_A::AVERAGE2),
            2 => Val(AVG3_A::AVERAGE4),
            3 => Val(AVG3_A::AVERAGE8),
            4 => Val(AVG3_A::AVERAGE16),
            5 => Val(AVG3_A::AVERAGE32),
            6 => Val(AVG3_A::AVERAGE64),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == AVG3_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `AVERAGE2`"]
    #[inline(always)]
    pub fn is_average2(&self) -> bool {
        *self == AVG3_A::AVERAGE2
    }
    #[doc = "Checks if the value of the field is `AVERAGE4`"]
    #[inline(always)]
    pub fn is_average4(&self) -> bool {
        *self == AVG3_A::AVERAGE4
    }
    #[doc = "Checks if the value of the field is `AVERAGE8`"]
    #[inline(always)]
    pub fn is_average8(&self) -> bool {
        *self == AVG3_A::AVERAGE8
    }
    #[doc = "Checks if the value of the field is `AVERAGE16`"]
    #[inline(always)]
    pub fn is_average16(&self) -> bool {
        *self == AVG3_A::AVERAGE16
    }
    #[doc = "Checks if the value of the field is `AVERAGE32`"]
    #[inline(always)]
    pub fn is_average32(&self) -> bool {
        *self == AVG3_A::AVERAGE32
    }
    #[doc = "Checks if the value of the field is `AVERAGE64`"]
    #[inline(always)]
    pub fn is_average64(&self) -> bool {
        *self == AVG3_A::AVERAGE64
    }
}
#[doc = "Write proxy for field `AVG3`"]
pub struct AVG3_W<'a> {
    w: &'a mut W,
}
impl<'a> AVG3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AVG3_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Average disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(AVG3_A::DISABLE)
    }
    #[doc = "Average with 2 measures"]
    #[inline(always)]
    pub fn average2(self) -> &'a mut W {
        self.variant(AVG3_A::AVERAGE2)
    }
    #[doc = "Average with 4 measures"]
    #[inline(always)]
    pub fn average4(self) -> &'a mut W {
        self.variant(AVG3_A::AVERAGE4)
    }
    #[doc = "Average with 8 measures"]
    #[inline(always)]
    pub fn average8(self) -> &'a mut W {
        self.variant(AVG3_A::AVERAGE8)
    }
    #[doc = "Average with 16 measures"]
    #[inline(always)]
    pub fn average16(self) -> &'a mut W {
        self.variant(AVG3_A::AVERAGE16)
    }
    #[doc = "Average with 32 measures"]
    #[inline(always)]
    pub fn average32(self) -> &'a mut W {
        self.variant(AVG3_A::AVERAGE32)
    }
    #[doc = "Average with 64 measures"]
    #[inline(always)]
    pub fn average64(self) -> &'a mut W {
        self.variant(AVG3_A::AVERAGE64)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "ADC4 average control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AVG4_A {
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
impl From<AVG4_A> for u8 {
    #[inline(always)]
    fn from(variant: AVG4_A) -> Self {
        match variant {
            AVG4_A::DISABLE => 0,
            AVG4_A::AVERAGE2 => 1,
            AVG4_A::AVERAGE4 => 2,
            AVG4_A::AVERAGE8 => 3,
            AVG4_A::AVERAGE16 => 4,
            AVG4_A::AVERAGE32 => 5,
            AVG4_A::AVERAGE64 => 6,
        }
    }
}
#[doc = "Reader of field `AVG4`"]
pub type AVG4_R = crate::R<u8, AVG4_A>;
impl AVG4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, AVG4_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(AVG4_A::DISABLE),
            1 => Val(AVG4_A::AVERAGE2),
            2 => Val(AVG4_A::AVERAGE4),
            3 => Val(AVG4_A::AVERAGE8),
            4 => Val(AVG4_A::AVERAGE16),
            5 => Val(AVG4_A::AVERAGE32),
            6 => Val(AVG4_A::AVERAGE64),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == AVG4_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `AVERAGE2`"]
    #[inline(always)]
    pub fn is_average2(&self) -> bool {
        *self == AVG4_A::AVERAGE2
    }
    #[doc = "Checks if the value of the field is `AVERAGE4`"]
    #[inline(always)]
    pub fn is_average4(&self) -> bool {
        *self == AVG4_A::AVERAGE4
    }
    #[doc = "Checks if the value of the field is `AVERAGE8`"]
    #[inline(always)]
    pub fn is_average8(&self) -> bool {
        *self == AVG4_A::AVERAGE8
    }
    #[doc = "Checks if the value of the field is `AVERAGE16`"]
    #[inline(always)]
    pub fn is_average16(&self) -> bool {
        *self == AVG4_A::AVERAGE16
    }
    #[doc = "Checks if the value of the field is `AVERAGE32`"]
    #[inline(always)]
    pub fn is_average32(&self) -> bool {
        *self == AVG4_A::AVERAGE32
    }
    #[doc = "Checks if the value of the field is `AVERAGE64`"]
    #[inline(always)]
    pub fn is_average64(&self) -> bool {
        *self == AVG4_A::AVERAGE64
    }
}
#[doc = "Write proxy for field `AVG4`"]
pub struct AVG4_W<'a> {
    w: &'a mut W,
}
impl<'a> AVG4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AVG4_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Average disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(AVG4_A::DISABLE)
    }
    #[doc = "Average with 2 measures"]
    #[inline(always)]
    pub fn average2(self) -> &'a mut W {
        self.variant(AVG4_A::AVERAGE2)
    }
    #[doc = "Average with 4 measures"]
    #[inline(always)]
    pub fn average4(self) -> &'a mut W {
        self.variant(AVG4_A::AVERAGE4)
    }
    #[doc = "Average with 8 measures"]
    #[inline(always)]
    pub fn average8(self) -> &'a mut W {
        self.variant(AVG4_A::AVERAGE8)
    }
    #[doc = "Average with 16 measures"]
    #[inline(always)]
    pub fn average16(self) -> &'a mut W {
        self.variant(AVG4_A::AVERAGE16)
    }
    #[doc = "Average with 32 measures"]
    #[inline(always)]
    pub fn average32(self) -> &'a mut W {
        self.variant(AVG4_A::AVERAGE32)
    }
    #[doc = "Average with 64 measures"]
    #[inline(always)]
    pub fn average64(self) -> &'a mut W {
        self.variant(AVG4_A::AVERAGE64)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "ADC5 average control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AVG5_A {
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
impl From<AVG5_A> for u8 {
    #[inline(always)]
    fn from(variant: AVG5_A) -> Self {
        match variant {
            AVG5_A::DISABLE => 0,
            AVG5_A::AVERAGE2 => 1,
            AVG5_A::AVERAGE4 => 2,
            AVG5_A::AVERAGE8 => 3,
            AVG5_A::AVERAGE16 => 4,
            AVG5_A::AVERAGE32 => 5,
            AVG5_A::AVERAGE64 => 6,
        }
    }
}
#[doc = "Reader of field `AVG5`"]
pub type AVG5_R = crate::R<u8, AVG5_A>;
impl AVG5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, AVG5_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(AVG5_A::DISABLE),
            1 => Val(AVG5_A::AVERAGE2),
            2 => Val(AVG5_A::AVERAGE4),
            3 => Val(AVG5_A::AVERAGE8),
            4 => Val(AVG5_A::AVERAGE16),
            5 => Val(AVG5_A::AVERAGE32),
            6 => Val(AVG5_A::AVERAGE64),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == AVG5_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `AVERAGE2`"]
    #[inline(always)]
    pub fn is_average2(&self) -> bool {
        *self == AVG5_A::AVERAGE2
    }
    #[doc = "Checks if the value of the field is `AVERAGE4`"]
    #[inline(always)]
    pub fn is_average4(&self) -> bool {
        *self == AVG5_A::AVERAGE4
    }
    #[doc = "Checks if the value of the field is `AVERAGE8`"]
    #[inline(always)]
    pub fn is_average8(&self) -> bool {
        *self == AVG5_A::AVERAGE8
    }
    #[doc = "Checks if the value of the field is `AVERAGE16`"]
    #[inline(always)]
    pub fn is_average16(&self) -> bool {
        *self == AVG5_A::AVERAGE16
    }
    #[doc = "Checks if the value of the field is `AVERAGE32`"]
    #[inline(always)]
    pub fn is_average32(&self) -> bool {
        *self == AVG5_A::AVERAGE32
    }
    #[doc = "Checks if the value of the field is `AVERAGE64`"]
    #[inline(always)]
    pub fn is_average64(&self) -> bool {
        *self == AVG5_A::AVERAGE64
    }
}
#[doc = "Write proxy for field `AVG5`"]
pub struct AVG5_W<'a> {
    w: &'a mut W,
}
impl<'a> AVG5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AVG5_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Average disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(AVG5_A::DISABLE)
    }
    #[doc = "Average with 2 measures"]
    #[inline(always)]
    pub fn average2(self) -> &'a mut W {
        self.variant(AVG5_A::AVERAGE2)
    }
    #[doc = "Average with 4 measures"]
    #[inline(always)]
    pub fn average4(self) -> &'a mut W {
        self.variant(AVG5_A::AVERAGE4)
    }
    #[doc = "Average with 8 measures"]
    #[inline(always)]
    pub fn average8(self) -> &'a mut W {
        self.variant(AVG5_A::AVERAGE8)
    }
    #[doc = "Average with 16 measures"]
    #[inline(always)]
    pub fn average16(self) -> &'a mut W {
        self.variant(AVG5_A::AVERAGE16)
    }
    #[doc = "Average with 32 measures"]
    #[inline(always)]
    pub fn average32(self) -> &'a mut W {
        self.variant(AVG5_A::AVERAGE32)
    }
    #[doc = "Average with 64 measures"]
    #[inline(always)]
    pub fn average64(self) -> &'a mut W {
        self.variant(AVG5_A::AVERAGE64)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | (((value as u32) & 0x07) << 20);
        self.w
    }
}
#[doc = "ADC6 average control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AVG6_A {
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
impl From<AVG6_A> for u8 {
    #[inline(always)]
    fn from(variant: AVG6_A) -> Self {
        match variant {
            AVG6_A::DISABLE => 0,
            AVG6_A::AVERAGE2 => 1,
            AVG6_A::AVERAGE4 => 2,
            AVG6_A::AVERAGE8 => 3,
            AVG6_A::AVERAGE16 => 4,
            AVG6_A::AVERAGE32 => 5,
            AVG6_A::AVERAGE64 => 6,
        }
    }
}
#[doc = "Reader of field `AVG6`"]
pub type AVG6_R = crate::R<u8, AVG6_A>;
impl AVG6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, AVG6_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(AVG6_A::DISABLE),
            1 => Val(AVG6_A::AVERAGE2),
            2 => Val(AVG6_A::AVERAGE4),
            3 => Val(AVG6_A::AVERAGE8),
            4 => Val(AVG6_A::AVERAGE16),
            5 => Val(AVG6_A::AVERAGE32),
            6 => Val(AVG6_A::AVERAGE64),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == AVG6_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `AVERAGE2`"]
    #[inline(always)]
    pub fn is_average2(&self) -> bool {
        *self == AVG6_A::AVERAGE2
    }
    #[doc = "Checks if the value of the field is `AVERAGE4`"]
    #[inline(always)]
    pub fn is_average4(&self) -> bool {
        *self == AVG6_A::AVERAGE4
    }
    #[doc = "Checks if the value of the field is `AVERAGE8`"]
    #[inline(always)]
    pub fn is_average8(&self) -> bool {
        *self == AVG6_A::AVERAGE8
    }
    #[doc = "Checks if the value of the field is `AVERAGE16`"]
    #[inline(always)]
    pub fn is_average16(&self) -> bool {
        *self == AVG6_A::AVERAGE16
    }
    #[doc = "Checks if the value of the field is `AVERAGE32`"]
    #[inline(always)]
    pub fn is_average32(&self) -> bool {
        *self == AVG6_A::AVERAGE32
    }
    #[doc = "Checks if the value of the field is `AVERAGE64`"]
    #[inline(always)]
    pub fn is_average64(&self) -> bool {
        *self == AVG6_A::AVERAGE64
    }
}
#[doc = "Write proxy for field `AVG6`"]
pub struct AVG6_W<'a> {
    w: &'a mut W,
}
impl<'a> AVG6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AVG6_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Average disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(AVG6_A::DISABLE)
    }
    #[doc = "Average with 2 measures"]
    #[inline(always)]
    pub fn average2(self) -> &'a mut W {
        self.variant(AVG6_A::AVERAGE2)
    }
    #[doc = "Average with 4 measures"]
    #[inline(always)]
    pub fn average4(self) -> &'a mut W {
        self.variant(AVG6_A::AVERAGE4)
    }
    #[doc = "Average with 8 measures"]
    #[inline(always)]
    pub fn average8(self) -> &'a mut W {
        self.variant(AVG6_A::AVERAGE8)
    }
    #[doc = "Average with 16 measures"]
    #[inline(always)]
    pub fn average16(self) -> &'a mut W {
        self.variant(AVG6_A::AVERAGE16)
    }
    #[doc = "Average with 32 measures"]
    #[inline(always)]
    pub fn average32(self) -> &'a mut W {
        self.variant(AVG6_A::AVERAGE32)
    }
    #[doc = "Average with 64 measures"]
    #[inline(always)]
    pub fn average64(self) -> &'a mut W {
        self.variant(AVG6_A::AVERAGE64)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "ADC7 average control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AVG7_A {
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
impl From<AVG7_A> for u8 {
    #[inline(always)]
    fn from(variant: AVG7_A) -> Self {
        match variant {
            AVG7_A::DISABLE => 0,
            AVG7_A::AVERAGE2 => 1,
            AVG7_A::AVERAGE4 => 2,
            AVG7_A::AVERAGE8 => 3,
            AVG7_A::AVERAGE16 => 4,
            AVG7_A::AVERAGE32 => 5,
            AVG7_A::AVERAGE64 => 6,
        }
    }
}
#[doc = "Reader of field `AVG7`"]
pub type AVG7_R = crate::R<u8, AVG7_A>;
impl AVG7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, AVG7_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(AVG7_A::DISABLE),
            1 => Val(AVG7_A::AVERAGE2),
            2 => Val(AVG7_A::AVERAGE4),
            3 => Val(AVG7_A::AVERAGE8),
            4 => Val(AVG7_A::AVERAGE16),
            5 => Val(AVG7_A::AVERAGE32),
            6 => Val(AVG7_A::AVERAGE64),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == AVG7_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `AVERAGE2`"]
    #[inline(always)]
    pub fn is_average2(&self) -> bool {
        *self == AVG7_A::AVERAGE2
    }
    #[doc = "Checks if the value of the field is `AVERAGE4`"]
    #[inline(always)]
    pub fn is_average4(&self) -> bool {
        *self == AVG7_A::AVERAGE4
    }
    #[doc = "Checks if the value of the field is `AVERAGE8`"]
    #[inline(always)]
    pub fn is_average8(&self) -> bool {
        *self == AVG7_A::AVERAGE8
    }
    #[doc = "Checks if the value of the field is `AVERAGE16`"]
    #[inline(always)]
    pub fn is_average16(&self) -> bool {
        *self == AVG7_A::AVERAGE16
    }
    #[doc = "Checks if the value of the field is `AVERAGE32`"]
    #[inline(always)]
    pub fn is_average32(&self) -> bool {
        *self == AVG7_A::AVERAGE32
    }
    #[doc = "Checks if the value of the field is `AVERAGE64`"]
    #[inline(always)]
    pub fn is_average64(&self) -> bool {
        *self == AVG7_A::AVERAGE64
    }
}
#[doc = "Write proxy for field `AVG7`"]
pub struct AVG7_W<'a> {
    w: &'a mut W,
}
impl<'a> AVG7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AVG7_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Average disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(AVG7_A::DISABLE)
    }
    #[doc = "Average with 2 measures"]
    #[inline(always)]
    pub fn average2(self) -> &'a mut W {
        self.variant(AVG7_A::AVERAGE2)
    }
    #[doc = "Average with 4 measures"]
    #[inline(always)]
    pub fn average4(self) -> &'a mut W {
        self.variant(AVG7_A::AVERAGE4)
    }
    #[doc = "Average with 8 measures"]
    #[inline(always)]
    pub fn average8(self) -> &'a mut W {
        self.variant(AVG7_A::AVERAGE8)
    }
    #[doc = "Average with 16 measures"]
    #[inline(always)]
    pub fn average16(self) -> &'a mut W {
        self.variant(AVG7_A::AVERAGE16)
    }
    #[doc = "Average with 32 measures"]
    #[inline(always)]
    pub fn average32(self) -> &'a mut W {
        self.variant(AVG7_A::AVERAGE32)
    }
    #[doc = "Average with 64 measures"]
    #[inline(always)]
    pub fn average64(self) -> &'a mut W {
        self.variant(AVG7_A::AVERAGE64)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | (((value as u32) & 0x07) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - ADC0 average control"]
    #[inline(always)]
    pub fn avg0(&self) -> AVG0_R {
        AVG0_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - ADC1 average control"]
    #[inline(always)]
    pub fn avg1(&self) -> AVG1_R {
        AVG1_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 8:10 - ADC2 average control"]
    #[inline(always)]
    pub fn avg2(&self) -> AVG2_R {
        AVG2_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 12:14 - ADC3 average control"]
    #[inline(always)]
    pub fn avg3(&self) -> AVG3_R {
        AVG3_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 16:18 - ADC4 average control"]
    #[inline(always)]
    pub fn avg4(&self) -> AVG4_R {
        AVG4_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 20:22 - ADC5 average control"]
    #[inline(always)]
    pub fn avg5(&self) -> AVG5_R {
        AVG5_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 24:26 - ADC6 average control"]
    #[inline(always)]
    pub fn avg6(&self) -> AVG6_R {
        AVG6_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 28:30 - ADC7 average control"]
    #[inline(always)]
    pub fn avg7(&self) -> AVG7_R {
        AVG7_R::new(((self.bits >> 28) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - ADC0 average control"]
    #[inline(always)]
    pub fn avg0(&mut self) -> AVG0_W {
        AVG0_W { w: self }
    }
    #[doc = "Bits 4:6 - ADC1 average control"]
    #[inline(always)]
    pub fn avg1(&mut self) -> AVG1_W {
        AVG1_W { w: self }
    }
    #[doc = "Bits 8:10 - ADC2 average control"]
    #[inline(always)]
    pub fn avg2(&mut self) -> AVG2_W {
        AVG2_W { w: self }
    }
    #[doc = "Bits 12:14 - ADC3 average control"]
    #[inline(always)]
    pub fn avg3(&mut self) -> AVG3_W {
        AVG3_W { w: self }
    }
    #[doc = "Bits 16:18 - ADC4 average control"]
    #[inline(always)]
    pub fn avg4(&mut self) -> AVG4_W {
        AVG4_W { w: self }
    }
    #[doc = "Bits 20:22 - ADC5 average control"]
    #[inline(always)]
    pub fn avg5(&mut self) -> AVG5_W {
        AVG5_W { w: self }
    }
    #[doc = "Bits 24:26 - ADC6 average control"]
    #[inline(always)]
    pub fn avg6(&mut self) -> AVG6_W {
        AVG6_W { w: self }
    }
    #[doc = "Bits 28:30 - ADC7 average control"]
    #[inline(always)]
    pub fn avg7(&mut self) -> AVG7_W {
        AVG7_W { w: self }
    }
}
