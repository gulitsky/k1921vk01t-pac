#[doc = "Reader of register QCAPCTL"]
pub type R = crate::R<u32, super::QCAPCTL>;
#[doc = "Writer for register QCAPCTL"]
pub type W = crate::W<u32, super::QCAPCTL>;
#[doc = "Register QCAPCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::QCAPCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UPPS_A {
    #[doc = "0: quad signal not divided"]
    DISABLE,
    #[doc = "1: quad signal divided by 2"]
    DIV2,
    #[doc = "2: quad signal divided by 4"]
    DIV4,
    #[doc = "3: quad signal divided by 8"]
    DIV8,
    #[doc = "4: quad signal divided by 16"]
    DIV16,
    #[doc = "5: quad signal divided by 32"]
    DIV32,
    #[doc = "6: quad signal divided by 64"]
    DIV64,
    #[doc = "7: quad signal divided by 128"]
    DIV128,
    #[doc = "8: quad signal divided by 256"]
    DIV256,
    #[doc = "9: quad signal divided by 512"]
    DIV512,
    #[doc = "10: quad signal divided by 1024"]
    DIV1024,
    #[doc = "11: quad signal divided by 2048"]
    DIV2048,
}
impl From<UPPS_A> for u8 {
    #[inline(always)]
    fn from(variant: UPPS_A) -> Self {
        match variant {
            UPPS_A::DISABLE => 0,
            UPPS_A::DIV2 => 1,
            UPPS_A::DIV4 => 2,
            UPPS_A::DIV8 => 3,
            UPPS_A::DIV16 => 4,
            UPPS_A::DIV32 => 5,
            UPPS_A::DIV64 => 6,
            UPPS_A::DIV128 => 7,
            UPPS_A::DIV256 => 8,
            UPPS_A::DIV512 => 9,
            UPPS_A::DIV1024 => 10,
            UPPS_A::DIV2048 => 11,
        }
    }
}
#[doc = "Reader of field `UPPS`"]
pub type UPPS_R = crate::R<u8, UPPS_A>;
impl UPPS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, UPPS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(UPPS_A::DISABLE),
            1 => Val(UPPS_A::DIV2),
            2 => Val(UPPS_A::DIV4),
            3 => Val(UPPS_A::DIV8),
            4 => Val(UPPS_A::DIV16),
            5 => Val(UPPS_A::DIV32),
            6 => Val(UPPS_A::DIV64),
            7 => Val(UPPS_A::DIV128),
            8 => Val(UPPS_A::DIV256),
            9 => Val(UPPS_A::DIV512),
            10 => Val(UPPS_A::DIV1024),
            11 => Val(UPPS_A::DIV2048),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == UPPS_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == UPPS_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == UPPS_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == UPPS_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == UPPS_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == UPPS_A::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == UPPS_A::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == UPPS_A::DIV128
    }
    #[doc = "Checks if the value of the field is `DIV256`"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == UPPS_A::DIV256
    }
    #[doc = "Checks if the value of the field is `DIV512`"]
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        *self == UPPS_A::DIV512
    }
    #[doc = "Checks if the value of the field is `DIV1024`"]
    #[inline(always)]
    pub fn is_div1024(&self) -> bool {
        *self == UPPS_A::DIV1024
    }
    #[doc = "Checks if the value of the field is `DIV2048`"]
    #[inline(always)]
    pub fn is_div2048(&self) -> bool {
        *self == UPPS_A::DIV2048
    }
}
#[doc = "Write proxy for field `UPPS`"]
pub struct UPPS_W<'a> {
    w: &'a mut W,
}
impl<'a> UPPS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UPPS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "quad signal not divided"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(UPPS_A::DISABLE)
    }
    #[doc = "quad signal divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(UPPS_A::DIV2)
    }
    #[doc = "quad signal divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(UPPS_A::DIV4)
    }
    #[doc = "quad signal divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(UPPS_A::DIV8)
    }
    #[doc = "quad signal divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(UPPS_A::DIV16)
    }
    #[doc = "quad signal divided by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(UPPS_A::DIV32)
    }
    #[doc = "quad signal divided by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(UPPS_A::DIV64)
    }
    #[doc = "quad signal divided by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(UPPS_A::DIV128)
    }
    #[doc = "quad signal divided by 256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut W {
        self.variant(UPPS_A::DIV256)
    }
    #[doc = "quad signal divided by 512"]
    #[inline(always)]
    pub fn div512(self) -> &'a mut W {
        self.variant(UPPS_A::DIV512)
    }
    #[doc = "quad signal divided by 1024"]
    #[inline(always)]
    pub fn div1024(self) -> &'a mut W {
        self.variant(UPPS_A::DIV1024)
    }
    #[doc = "quad signal divided by 2048"]
    #[inline(always)]
    pub fn div2048(self) -> &'a mut W {
        self.variant(UPPS_A::DIV2048)
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
pub enum CCPS_A {
    #[doc = "0: no divider"]
    DISABLE,
    #[doc = "1: sysclk divided by 2"]
    DIV2,
    #[doc = "2: sysclk divided by 4"]
    DIV4,
    #[doc = "3: sysclk divided by 8"]
    DIV8,
    #[doc = "4: sysclk divided by 16"]
    DIV16,
    #[doc = "5: sysclk divided by 32"]
    DIV32,
    #[doc = "6: sysclk divided by 64"]
    DIV64,
    #[doc = "7: sysclk divided by 128"]
    DIV128,
}
impl From<CCPS_A> for u8 {
    #[inline(always)]
    fn from(variant: CCPS_A) -> Self {
        match variant {
            CCPS_A::DISABLE => 0,
            CCPS_A::DIV2 => 1,
            CCPS_A::DIV4 => 2,
            CCPS_A::DIV8 => 3,
            CCPS_A::DIV16 => 4,
            CCPS_A::DIV32 => 5,
            CCPS_A::DIV64 => 6,
            CCPS_A::DIV128 => 7,
        }
    }
}
#[doc = "Reader of field `CCPS`"]
pub type CCPS_R = crate::R<u8, CCPS_A>;
impl CCPS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCPS_A {
        match self.bits {
            0 => CCPS_A::DISABLE,
            1 => CCPS_A::DIV2,
            2 => CCPS_A::DIV4,
            3 => CCPS_A::DIV8,
            4 => CCPS_A::DIV16,
            5 => CCPS_A::DIV32,
            6 => CCPS_A::DIV64,
            7 => CCPS_A::DIV128,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CCPS_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == CCPS_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == CCPS_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == CCPS_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == CCPS_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == CCPS_A::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == CCPS_A::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == CCPS_A::DIV128
    }
}
#[doc = "Write proxy for field `CCPS`"]
pub struct CCPS_W<'a> {
    w: &'a mut W,
}
impl<'a> CCPS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCPS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "no divider"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CCPS_A::DISABLE)
    }
    #[doc = "sysclk divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(CCPS_A::DIV2)
    }
    #[doc = "sysclk divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(CCPS_A::DIV4)
    }
    #[doc = "sysclk divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(CCPS_A::DIV8)
    }
    #[doc = "sysclk divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(CCPS_A::DIV16)
    }
    #[doc = "sysclk divided by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(CCPS_A::DIV32)
    }
    #[doc = "sysclk divided by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(CCPS_A::DIV64)
    }
    #[doc = "sysclk divided by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(CCPS_A::DIV128)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `SELEVENT`"]
pub type SELEVENT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SELEVENT`"]
pub struct SELEVENT_W<'a> {
    w: &'a mut W,
}
impl<'a> SELEVENT_W<'a> {
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
#[doc = "Reader of field `CEN`"]
pub type CEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CEN`"]
pub struct CEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CEN_W<'a> {
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
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn upps(&self) -> UPPS_R {
        UPPS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn ccps(&self) -> CCPS_R {
        CCPS_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 7 - Reset timer"]
    #[inline(always)]
    pub fn selevent(&self) -> SELEVENT_R {
        SELEVENT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 15 - enable decoder"]
    #[inline(always)]
    pub fn cen(&self) -> CEN_R {
        CEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn upps(&mut self) -> UPPS_W {
        UPPS_W { w: self }
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn ccps(&mut self) -> CCPS_W {
        CCPS_W { w: self }
    }
    #[doc = "Bit 7 - Reset timer"]
    #[inline(always)]
    pub fn selevent(&mut self) -> SELEVENT_W {
        SELEVENT_W { w: self }
    }
    #[doc = "Bit 15 - enable decoder"]
    #[inline(always)]
    pub fn cen(&mut self) -> CEN_W {
        CEN_W { w: self }
    }
}
