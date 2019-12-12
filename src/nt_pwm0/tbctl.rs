#[doc = "Reader of register TBCTL"]
pub type R = crate::R<u32, super::TBCTL>;
#[doc = "Writer for register TBCTL"]
pub type W = crate::W<u32, super::TBCTL>;
#[doc = "Register TBCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::TBCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTRMODE_A {
    #[doc = "0: count direction up"]
    UP,
    #[doc = "1: count direction down"]
    DOWN,
    #[doc = "2: count direction up-down"]
    UPDOWN,
    #[doc = "3: counter stopped"]
    STOP,
}
impl From<CTRMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: CTRMODE_A) -> Self {
        match variant {
            CTRMODE_A::UP => 0,
            CTRMODE_A::DOWN => 1,
            CTRMODE_A::UPDOWN => 2,
            CTRMODE_A::STOP => 3,
        }
    }
}
#[doc = "Reader of field `CTRMODE`"]
pub type CTRMODE_R = crate::R<u8, CTRMODE_A>;
impl CTRMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTRMODE_A {
        match self.bits {
            0 => CTRMODE_A::UP,
            1 => CTRMODE_A::DOWN,
            2 => CTRMODE_A::UPDOWN,
            3 => CTRMODE_A::STOP,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `UP`"]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        *self == CTRMODE_A::UP
    }
    #[doc = "Checks if the value of the field is `DOWN`"]
    #[inline(always)]
    pub fn is_down(&self) -> bool {
        *self == CTRMODE_A::DOWN
    }
    #[doc = "Checks if the value of the field is `UPDOWN`"]
    #[inline(always)]
    pub fn is_up_down(&self) -> bool {
        *self == CTRMODE_A::UPDOWN
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == CTRMODE_A::STOP
    }
}
#[doc = "Write proxy for field `CTRMODE`"]
pub struct CTRMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTRMODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "count direction up"]
    #[inline(always)]
    pub fn up(self) -> &'a mut W {
        self.variant(CTRMODE_A::UP)
    }
    #[doc = "count direction down"]
    #[inline(always)]
    pub fn down(self) -> &'a mut W {
        self.variant(CTRMODE_A::DOWN)
    }
    #[doc = "count direction up-down"]
    #[inline(always)]
    pub fn up_down(self) -> &'a mut W {
        self.variant(CTRMODE_A::UPDOWN)
    }
    #[doc = "counter stopped"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(CTRMODE_A::STOP)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `PHSEN`"]
pub type PHSEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PHSEN`"]
pub struct PHSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PHSEN_W<'a> {
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
#[doc = "Reader of field `PRDLD`"]
pub type PRDLD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRDLD`"]
pub struct PRDLD_W<'a> {
    w: &'a mut W,
}
impl<'a> PRDLD_W<'a> {
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
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNCOSEL_A {
    #[doc = "0: PWM_SYNCI is source for PWM_SYNCO"]
    SYNCI,
    #[doc = "1:  CTR = 0000h is source for PWM_SYNCO"]
    CTREQZERO,
    #[doc = "2: CTR = CMPB is source for PWM_SYNCO"]
    CTREQCMPB,
    #[doc = "3: PWM_SYNCO generation disabled"]
    DISABLE,
}
impl From<SYNCOSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SYNCOSEL_A) -> Self {
        match variant {
            SYNCOSEL_A::SYNCI => 0,
            SYNCOSEL_A::CTREQZERO => 1,
            SYNCOSEL_A::CTREQCMPB => 2,
            SYNCOSEL_A::DISABLE => 3,
        }
    }
}
#[doc = "Reader of field `SYNCOSEL`"]
pub type SYNCOSEL_R = crate::R<u8, SYNCOSEL_A>;
impl SYNCOSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNCOSEL_A {
        match self.bits {
            0 => SYNCOSEL_A::SYNCI,
            1 => SYNCOSEL_A::CTREQZERO,
            2 => SYNCOSEL_A::CTREQCMPB,
            3 => SYNCOSEL_A::DISABLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SYNCI`"]
    #[inline(always)]
    pub fn is_synci(&self) -> bool {
        *self == SYNCOSEL_A::SYNCI
    }
    #[doc = "Checks if the value of the field is `CTREQZERO`"]
    #[inline(always)]
    pub fn is_ctreq_zero(&self) -> bool {
        *self == SYNCOSEL_A::CTREQZERO
    }
    #[doc = "Checks if the value of the field is `CTREQCMPB`"]
    #[inline(always)]
    pub fn is_ctreq_cmpb(&self) -> bool {
        *self == SYNCOSEL_A::CTREQCMPB
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SYNCOSEL_A::DISABLE
    }
}
#[doc = "Write proxy for field `SYNCOSEL`"]
pub struct SYNCOSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCOSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYNCOSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "PWM_SYNCI is source for PWM_SYNCO"]
    #[inline(always)]
    pub fn synci(self) -> &'a mut W {
        self.variant(SYNCOSEL_A::SYNCI)
    }
    #[doc = "CTR = 0000h is source for PWM_SYNCO"]
    #[inline(always)]
    pub fn ctreq_zero(self) -> &'a mut W {
        self.variant(SYNCOSEL_A::CTREQZERO)
    }
    #[doc = "CTR = CMPB is source for PWM_SYNCO"]
    #[inline(always)]
    pub fn ctreq_cmpb(self) -> &'a mut W {
        self.variant(SYNCOSEL_A::CTREQCMPB)
    }
    #[doc = "PWM_SYNCO generation disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SYNCOSEL_A::DISABLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `SWFSYNC`"]
pub type SWFSYNC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWFSYNC`"]
pub struct SWFSYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> SWFSYNC_W<'a> {
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
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSPCLKDIV_A {
    #[doc = "0: clock not divided"]
    DIV1,
    #[doc = "1: clock divided by 2"]
    DIV2,
    #[doc = "2: clock divided by 4"]
    DIV4,
    #[doc = "3: clock divided by 6"]
    DIV6,
    #[doc = "4: clock divided by 8"]
    DIV8,
    #[doc = "5: clock divided by 10"]
    DIV10,
    #[doc = "6: clock divided by 12"]
    DIV12,
    #[doc = "7: clock divided by 14"]
    DIV14,
}
impl From<HSPCLKDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: HSPCLKDIV_A) -> Self {
        match variant {
            HSPCLKDIV_A::DIV1 => 0,
            HSPCLKDIV_A::DIV2 => 1,
            HSPCLKDIV_A::DIV4 => 2,
            HSPCLKDIV_A::DIV6 => 3,
            HSPCLKDIV_A::DIV8 => 4,
            HSPCLKDIV_A::DIV10 => 5,
            HSPCLKDIV_A::DIV12 => 6,
            HSPCLKDIV_A::DIV14 => 7,
        }
    }
}
#[doc = "Reader of field `HSPCLKDIV`"]
pub type HSPCLKDIV_R = crate::R<u8, HSPCLKDIV_A>;
impl HSPCLKDIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSPCLKDIV_A {
        match self.bits {
            0 => HSPCLKDIV_A::DIV1,
            1 => HSPCLKDIV_A::DIV2,
            2 => HSPCLKDIV_A::DIV4,
            3 => HSPCLKDIV_A::DIV6,
            4 => HSPCLKDIV_A::DIV8,
            5 => HSPCLKDIV_A::DIV10,
            6 => HSPCLKDIV_A::DIV12,
            7 => HSPCLKDIV_A::DIV14,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == HSPCLKDIV_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == HSPCLKDIV_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == HSPCLKDIV_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV6`"]
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == HSPCLKDIV_A::DIV6
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == HSPCLKDIV_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV10`"]
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        *self == HSPCLKDIV_A::DIV10
    }
    #[doc = "Checks if the value of the field is `DIV12`"]
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        *self == HSPCLKDIV_A::DIV12
    }
    #[doc = "Checks if the value of the field is `DIV14`"]
    #[inline(always)]
    pub fn is_div14(&self) -> bool {
        *self == HSPCLKDIV_A::DIV14
    }
}
#[doc = "Write proxy for field `HSPCLKDIV`"]
pub struct HSPCLKDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> HSPCLKDIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HSPCLKDIV_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "clock not divided"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(HSPCLKDIV_A::DIV1)
    }
    #[doc = "clock divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(HSPCLKDIV_A::DIV2)
    }
    #[doc = "clock divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(HSPCLKDIV_A::DIV4)
    }
    #[doc = "clock divided by 6"]
    #[inline(always)]
    pub fn div6(self) -> &'a mut W {
        self.variant(HSPCLKDIV_A::DIV6)
    }
    #[doc = "clock divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(HSPCLKDIV_A::DIV8)
    }
    #[doc = "clock divided by 10"]
    #[inline(always)]
    pub fn div10(self) -> &'a mut W {
        self.variant(HSPCLKDIV_A::DIV10)
    }
    #[doc = "clock divided by 12"]
    #[inline(always)]
    pub fn div12(self) -> &'a mut W {
        self.variant(HSPCLKDIV_A::DIV12)
    }
    #[doc = "clock divided by 14"]
    #[inline(always)]
    pub fn div14(self) -> &'a mut W {
        self.variant(HSPCLKDIV_A::DIV14)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 7)) | (((value as u32) & 0x07) << 7);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKDIV_A {
    #[doc = "0: clock not divided"]
    DIV1,
    #[doc = "1: clock divided by 2"]
    DIV2,
    #[doc = "2: clock divided by 4"]
    DIV4,
    #[doc = "3: clock divided by 8"]
    DIV8,
    #[doc = "4: clock divided by 16"]
    DIV16,
    #[doc = "5: clock divided by 32"]
    DIV32,
    #[doc = "6: clock divided by 64"]
    DIV64,
    #[doc = "7: clock divided by 128"]
    DIV128,
}
impl From<CLKDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKDIV_A) -> Self {
        match variant {
            CLKDIV_A::DIV1 => 0,
            CLKDIV_A::DIV2 => 1,
            CLKDIV_A::DIV4 => 2,
            CLKDIV_A::DIV8 => 3,
            CLKDIV_A::DIV16 => 4,
            CLKDIV_A::DIV32 => 5,
            CLKDIV_A::DIV64 => 6,
            CLKDIV_A::DIV128 => 7,
        }
    }
}
#[doc = "Reader of field `CLKDIV`"]
pub type CLKDIV_R = crate::R<u8, CLKDIV_A>;
impl CLKDIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKDIV_A {
        match self.bits {
            0 => CLKDIV_A::DIV1,
            1 => CLKDIV_A::DIV2,
            2 => CLKDIV_A::DIV4,
            3 => CLKDIV_A::DIV8,
            4 => CLKDIV_A::DIV16,
            5 => CLKDIV_A::DIV32,
            6 => CLKDIV_A::DIV64,
            7 => CLKDIV_A::DIV128,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == CLKDIV_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == CLKDIV_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == CLKDIV_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == CLKDIV_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == CLKDIV_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == CLKDIV_A::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == CLKDIV_A::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == CLKDIV_A::DIV128
    }
}
#[doc = "Write proxy for field `CLKDIV`"]
pub struct CLKDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKDIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKDIV_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "clock not divided"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(CLKDIV_A::DIV1)
    }
    #[doc = "clock divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(CLKDIV_A::DIV2)
    }
    #[doc = "clock divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(CLKDIV_A::DIV4)
    }
    #[doc = "clock divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(CLKDIV_A::DIV8)
    }
    #[doc = "clock divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(CLKDIV_A::DIV16)
    }
    #[doc = "clock divided by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(CLKDIV_A::DIV32)
    }
    #[doc = "clock divided by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(CLKDIV_A::DIV64)
    }
    #[doc = "clock divided by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(CLKDIV_A::DIV128)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 10)) | (((value as u32) & 0x07) << 10);
        self.w
    }
}
#[doc = "Reader of field `PHSDIR`"]
pub type PHSDIR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PHSDIR`"]
pub struct PHSDIR_W<'a> {
    w: &'a mut W,
}
impl<'a> PHSDIR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FREE_SOFT_A {
    #[doc = "0: stop timer at next TBCLK tact"]
    STOPATTBCLK,
    #[doc = "1: stop timer when period ends"]
    STOPATPERIOD,
}
impl From<FREE_SOFT_A> for u8 {
    #[inline(always)]
    fn from(variant: FREE_SOFT_A) -> Self {
        match variant {
            FREE_SOFT_A::STOPATTBCLK => 0,
            FREE_SOFT_A::STOPATPERIOD => 1,
        }
    }
}
#[doc = "Reader of field `FREE_SOFT`"]
pub type FREE_SOFT_R = crate::R<u8, FREE_SOFT_A>;
impl FREE_SOFT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FREE_SOFT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FREE_SOFT_A::STOPATTBCLK),
            1 => Val(FREE_SOFT_A::STOPATPERIOD),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `STOPATTBCLK`"]
    #[inline(always)]
    pub fn is_stop_at_tbclk(&self) -> bool {
        *self == FREE_SOFT_A::STOPATTBCLK
    }
    #[doc = "Checks if the value of the field is `STOPATPERIOD`"]
    #[inline(always)]
    pub fn is_stop_at_period(&self) -> bool {
        *self == FREE_SOFT_A::STOPATPERIOD
    }
}
#[doc = "Write proxy for field `FREE_SOFT`"]
pub struct FREE_SOFT_W<'a> {
    w: &'a mut W,
}
impl<'a> FREE_SOFT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FREE_SOFT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "stop timer at next TBCLK tact"]
    #[inline(always)]
    pub fn stop_at_tbclk(self) -> &'a mut W {
        self.variant(FREE_SOFT_A::STOPATTBCLK)
    }
    #[doc = "stop timer when period ends"]
    #[inline(always)]
    pub fn stop_at_period(self) -> &'a mut W {
        self.variant(FREE_SOFT_A::STOPATPERIOD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn ctrmode(&self) -> CTRMODE_R {
        CTRMODE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - load timer counter"]
    #[inline(always)]
    pub fn phsen(&self) -> PHSEN_R {
        PHSEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - lazy loading mode register"]
    #[inline(always)]
    pub fn prdld(&self) -> PRDLD_R {
        PRDLD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn syncosel(&self) -> SYNCOSEL_R {
        SYNCOSEL_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 6 - Software emulation clock"]
    #[inline(always)]
    pub fn swfsync(&self) -> SWFSYNC_R {
        SWFSYNC_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 7:9"]
    #[inline(always)]
    pub fn hspclkdiv(&self) -> HSPCLKDIV_R {
        HSPCLKDIV_R::new(((self.bits >> 7) & 0x07) as u8)
    }
    #[doc = "Bits 10:12"]
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new(((self.bits >> 10) & 0x07) as u8)
    }
    #[doc = "Bit 13 - The phase direction"]
    #[inline(always)]
    pub fn phsdir(&self) -> PHSDIR_R {
        PHSDIR_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn free_soft(&self) -> FREE_SOFT_R {
        FREE_SOFT_R::new(((self.bits >> 14) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn ctrmode(&mut self) -> CTRMODE_W {
        CTRMODE_W { w: self }
    }
    #[doc = "Bit 2 - load timer counter"]
    #[inline(always)]
    pub fn phsen(&mut self) -> PHSEN_W {
        PHSEN_W { w: self }
    }
    #[doc = "Bit 3 - lazy loading mode register"]
    #[inline(always)]
    pub fn prdld(&mut self) -> PRDLD_W {
        PRDLD_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn syncosel(&mut self) -> SYNCOSEL_W {
        SYNCOSEL_W { w: self }
    }
    #[doc = "Bit 6 - Software emulation clock"]
    #[inline(always)]
    pub fn swfsync(&mut self) -> SWFSYNC_W {
        SWFSYNC_W { w: self }
    }
    #[doc = "Bits 7:9"]
    #[inline(always)]
    pub fn hspclkdiv(&mut self) -> HSPCLKDIV_W {
        HSPCLKDIV_W { w: self }
    }
    #[doc = "Bits 10:12"]
    #[inline(always)]
    pub fn clkdiv(&mut self) -> CLKDIV_W {
        CLKDIV_W { w: self }
    }
    #[doc = "Bit 13 - The phase direction"]
    #[inline(always)]
    pub fn phsdir(&mut self) -> PHSDIR_W {
        PHSDIR_W { w: self }
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn free_soft(&mut self) -> FREE_SOFT_W {
        FREE_SOFT_W { w: self }
    }
}
