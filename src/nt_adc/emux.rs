#[doc = "Reader of register EMUX"]
pub type R = crate::R<u32, super::EMUX>;
#[doc = "Writer for register EMUX"]
pub type W = crate::W<u32, super::EMUX>;
#[doc = "Register EMUX `reset()`'s with value 0"]
impl crate::ResetValue for super::EMUX {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Select start event for sequencer 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EM0_A {
    #[doc = "0: software request by GSYNC bit"]
    SWREQ,
    #[doc = "1: Analog comparator 0 request"]
    CMP0,
    #[doc = "2: Analog comparator 1 request"]
    CMP1,
    #[doc = "3: Analog comparator 2 request"]
    CMP2,
    #[doc = "4:  Any GPIO interrupt"]
    ITGPIO,
    #[doc = "5: Timer request"]
    TIMER,
    #[doc = "6: PWM 0 request"]
    PWM0,
    #[doc = "7: PWM 1 request"]
    PWM1,
    #[doc = "8: PWM 2 request"]
    PWM2,
    #[doc = "9: PWM 3 request"]
    PWM3,
    #[doc = "10: PWM 4 request"]
    PWM4,
    #[doc = "11: PWM 5 request"]
    PWM5,
    #[doc = "15: Cycle mode"]
    CYCLE,
}
impl From<EM0_A> for u8 {
    #[inline(always)]
    fn from(variant: EM0_A) -> Self {
        match variant {
            EM0_A::SWREQ => 0,
            EM0_A::CMP0 => 1,
            EM0_A::CMP1 => 2,
            EM0_A::CMP2 => 3,
            EM0_A::ITGPIO => 4,
            EM0_A::TIMER => 5,
            EM0_A::PWM0 => 6,
            EM0_A::PWM1 => 7,
            EM0_A::PWM2 => 8,
            EM0_A::PWM3 => 9,
            EM0_A::PWM4 => 10,
            EM0_A::PWM5 => 11,
            EM0_A::CYCLE => 15,
        }
    }
}
#[doc = "Reader of field `EM0`"]
pub type EM0_R = crate::R<u8, EM0_A>;
impl EM0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, EM0_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(EM0_A::SWREQ),
            1 => Val(EM0_A::CMP0),
            2 => Val(EM0_A::CMP1),
            3 => Val(EM0_A::CMP2),
            4 => Val(EM0_A::ITGPIO),
            5 => Val(EM0_A::TIMER),
            6 => Val(EM0_A::PWM0),
            7 => Val(EM0_A::PWM1),
            8 => Val(EM0_A::PWM2),
            9 => Val(EM0_A::PWM3),
            10 => Val(EM0_A::PWM4),
            11 => Val(EM0_A::PWM5),
            15 => Val(EM0_A::CYCLE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SWREQ`"]
    #[inline(always)]
    pub fn is_sw_req(&self) -> bool {
        *self == EM0_A::SWREQ
    }
    #[doc = "Checks if the value of the field is `CMP0`"]
    #[inline(always)]
    pub fn is_cmp0(&self) -> bool {
        *self == EM0_A::CMP0
    }
    #[doc = "Checks if the value of the field is `CMP1`"]
    #[inline(always)]
    pub fn is_cmp1(&self) -> bool {
        *self == EM0_A::CMP1
    }
    #[doc = "Checks if the value of the field is `CMP2`"]
    #[inline(always)]
    pub fn is_cmp2(&self) -> bool {
        *self == EM0_A::CMP2
    }
    #[doc = "Checks if the value of the field is `ITGPIO`"]
    #[inline(always)]
    pub fn is_itgpio(&self) -> bool {
        *self == EM0_A::ITGPIO
    }
    #[doc = "Checks if the value of the field is `TIMER`"]
    #[inline(always)]
    pub fn is_timer(&self) -> bool {
        *self == EM0_A::TIMER
    }
    #[doc = "Checks if the value of the field is `PWM0`"]
    #[inline(always)]
    pub fn is_pwm0(&self) -> bool {
        *self == EM0_A::PWM0
    }
    #[doc = "Checks if the value of the field is `PWM1`"]
    #[inline(always)]
    pub fn is_pwm1(&self) -> bool {
        *self == EM0_A::PWM1
    }
    #[doc = "Checks if the value of the field is `PWM2`"]
    #[inline(always)]
    pub fn is_pwm2(&self) -> bool {
        *self == EM0_A::PWM2
    }
    #[doc = "Checks if the value of the field is `PWM3`"]
    #[inline(always)]
    pub fn is_pwm3(&self) -> bool {
        *self == EM0_A::PWM3
    }
    #[doc = "Checks if the value of the field is `PWM4`"]
    #[inline(always)]
    pub fn is_pwm4(&self) -> bool {
        *self == EM0_A::PWM4
    }
    #[doc = "Checks if the value of the field is `PWM5`"]
    #[inline(always)]
    pub fn is_pwm5(&self) -> bool {
        *self == EM0_A::PWM5
    }
    #[doc = "Checks if the value of the field is `CYCLE`"]
    #[inline(always)]
    pub fn is_cycle(&self) -> bool {
        *self == EM0_A::CYCLE
    }
}
#[doc = "Write proxy for field `EM0`"]
pub struct EM0_W<'a> {
    w: &'a mut W,
}
impl<'a> EM0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EM0_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "software request by GSYNC bit"]
    #[inline(always)]
    pub fn sw_req(self) -> &'a mut W {
        self.variant(EM0_A::SWREQ)
    }
    #[doc = "Analog comparator 0 request"]
    #[inline(always)]
    pub fn cmp0(self) -> &'a mut W {
        self.variant(EM0_A::CMP0)
    }
    #[doc = "Analog comparator 1 request"]
    #[inline(always)]
    pub fn cmp1(self) -> &'a mut W {
        self.variant(EM0_A::CMP1)
    }
    #[doc = "Analog comparator 2 request"]
    #[inline(always)]
    pub fn cmp2(self) -> &'a mut W {
        self.variant(EM0_A::CMP2)
    }
    #[doc = "Any GPIO interrupt"]
    #[inline(always)]
    pub fn itgpio(self) -> &'a mut W {
        self.variant(EM0_A::ITGPIO)
    }
    #[doc = "Timer request"]
    #[inline(always)]
    pub fn timer(self) -> &'a mut W {
        self.variant(EM0_A::TIMER)
    }
    #[doc = "PWM 0 request"]
    #[inline(always)]
    pub fn pwm0(self) -> &'a mut W {
        self.variant(EM0_A::PWM0)
    }
    #[doc = "PWM 1 request"]
    #[inline(always)]
    pub fn pwm1(self) -> &'a mut W {
        self.variant(EM0_A::PWM1)
    }
    #[doc = "PWM 2 request"]
    #[inline(always)]
    pub fn pwm2(self) -> &'a mut W {
        self.variant(EM0_A::PWM2)
    }
    #[doc = "PWM 3 request"]
    #[inline(always)]
    pub fn pwm3(self) -> &'a mut W {
        self.variant(EM0_A::PWM3)
    }
    #[doc = "PWM 4 request"]
    #[inline(always)]
    pub fn pwm4(self) -> &'a mut W {
        self.variant(EM0_A::PWM4)
    }
    #[doc = "PWM 5 request"]
    #[inline(always)]
    pub fn pwm5(self) -> &'a mut W {
        self.variant(EM0_A::PWM5)
    }
    #[doc = "Cycle mode"]
    #[inline(always)]
    pub fn cycle(self) -> &'a mut W {
        self.variant(EM0_A::CYCLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Select start event for sequencer 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EM1_A {
    #[doc = "0: software request by GSYNC bit"]
    SWREQ,
    #[doc = "1: Analog comparator 0 request"]
    CMP0,
    #[doc = "2: Analog comparator 1 request"]
    CMP1,
    #[doc = "3: Analog comparator 2 request"]
    CMP2,
    #[doc = "4:  Any GPIO interrupt"]
    ITGPIO,
    #[doc = "5: Timer request"]
    TIMER,
    #[doc = "6: PWM 0 request"]
    PWM0,
    #[doc = "7: PWM 1 request"]
    PWM1,
    #[doc = "8: PWM 2 request"]
    PWM2,
    #[doc = "9: PWM 3 request"]
    PWM3,
    #[doc = "10: PWM 4 request"]
    PWM4,
    #[doc = "11: PWM 5 request"]
    PWM5,
    #[doc = "15: Cycle mode"]
    CYCLE,
}
impl From<EM1_A> for u8 {
    #[inline(always)]
    fn from(variant: EM1_A) -> Self {
        match variant {
            EM1_A::SWREQ => 0,
            EM1_A::CMP0 => 1,
            EM1_A::CMP1 => 2,
            EM1_A::CMP2 => 3,
            EM1_A::ITGPIO => 4,
            EM1_A::TIMER => 5,
            EM1_A::PWM0 => 6,
            EM1_A::PWM1 => 7,
            EM1_A::PWM2 => 8,
            EM1_A::PWM3 => 9,
            EM1_A::PWM4 => 10,
            EM1_A::PWM5 => 11,
            EM1_A::CYCLE => 15,
        }
    }
}
#[doc = "Reader of field `EM1`"]
pub type EM1_R = crate::R<u8, EM1_A>;
impl EM1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, EM1_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(EM1_A::SWREQ),
            1 => Val(EM1_A::CMP0),
            2 => Val(EM1_A::CMP1),
            3 => Val(EM1_A::CMP2),
            4 => Val(EM1_A::ITGPIO),
            5 => Val(EM1_A::TIMER),
            6 => Val(EM1_A::PWM0),
            7 => Val(EM1_A::PWM1),
            8 => Val(EM1_A::PWM2),
            9 => Val(EM1_A::PWM3),
            10 => Val(EM1_A::PWM4),
            11 => Val(EM1_A::PWM5),
            15 => Val(EM1_A::CYCLE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SWREQ`"]
    #[inline(always)]
    pub fn is_sw_req(&self) -> bool {
        *self == EM1_A::SWREQ
    }
    #[doc = "Checks if the value of the field is `CMP0`"]
    #[inline(always)]
    pub fn is_cmp0(&self) -> bool {
        *self == EM1_A::CMP0
    }
    #[doc = "Checks if the value of the field is `CMP1`"]
    #[inline(always)]
    pub fn is_cmp1(&self) -> bool {
        *self == EM1_A::CMP1
    }
    #[doc = "Checks if the value of the field is `CMP2`"]
    #[inline(always)]
    pub fn is_cmp2(&self) -> bool {
        *self == EM1_A::CMP2
    }
    #[doc = "Checks if the value of the field is `ITGPIO`"]
    #[inline(always)]
    pub fn is_itgpio(&self) -> bool {
        *self == EM1_A::ITGPIO
    }
    #[doc = "Checks if the value of the field is `TIMER`"]
    #[inline(always)]
    pub fn is_timer(&self) -> bool {
        *self == EM1_A::TIMER
    }
    #[doc = "Checks if the value of the field is `PWM0`"]
    #[inline(always)]
    pub fn is_pwm0(&self) -> bool {
        *self == EM1_A::PWM0
    }
    #[doc = "Checks if the value of the field is `PWM1`"]
    #[inline(always)]
    pub fn is_pwm1(&self) -> bool {
        *self == EM1_A::PWM1
    }
    #[doc = "Checks if the value of the field is `PWM2`"]
    #[inline(always)]
    pub fn is_pwm2(&self) -> bool {
        *self == EM1_A::PWM2
    }
    #[doc = "Checks if the value of the field is `PWM3`"]
    #[inline(always)]
    pub fn is_pwm3(&self) -> bool {
        *self == EM1_A::PWM3
    }
    #[doc = "Checks if the value of the field is `PWM4`"]
    #[inline(always)]
    pub fn is_pwm4(&self) -> bool {
        *self == EM1_A::PWM4
    }
    #[doc = "Checks if the value of the field is `PWM5`"]
    #[inline(always)]
    pub fn is_pwm5(&self) -> bool {
        *self == EM1_A::PWM5
    }
    #[doc = "Checks if the value of the field is `CYCLE`"]
    #[inline(always)]
    pub fn is_cycle(&self) -> bool {
        *self == EM1_A::CYCLE
    }
}
#[doc = "Write proxy for field `EM1`"]
pub struct EM1_W<'a> {
    w: &'a mut W,
}
impl<'a> EM1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EM1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "software request by GSYNC bit"]
    #[inline(always)]
    pub fn sw_req(self) -> &'a mut W {
        self.variant(EM1_A::SWREQ)
    }
    #[doc = "Analog comparator 0 request"]
    #[inline(always)]
    pub fn cmp0(self) -> &'a mut W {
        self.variant(EM1_A::CMP0)
    }
    #[doc = "Analog comparator 1 request"]
    #[inline(always)]
    pub fn cmp1(self) -> &'a mut W {
        self.variant(EM1_A::CMP1)
    }
    #[doc = "Analog comparator 2 request"]
    #[inline(always)]
    pub fn cmp2(self) -> &'a mut W {
        self.variant(EM1_A::CMP2)
    }
    #[doc = "Any GPIO interrupt"]
    #[inline(always)]
    pub fn itgpio(self) -> &'a mut W {
        self.variant(EM1_A::ITGPIO)
    }
    #[doc = "Timer request"]
    #[inline(always)]
    pub fn timer(self) -> &'a mut W {
        self.variant(EM1_A::TIMER)
    }
    #[doc = "PWM 0 request"]
    #[inline(always)]
    pub fn pwm0(self) -> &'a mut W {
        self.variant(EM1_A::PWM0)
    }
    #[doc = "PWM 1 request"]
    #[inline(always)]
    pub fn pwm1(self) -> &'a mut W {
        self.variant(EM1_A::PWM1)
    }
    #[doc = "PWM 2 request"]
    #[inline(always)]
    pub fn pwm2(self) -> &'a mut W {
        self.variant(EM1_A::PWM2)
    }
    #[doc = "PWM 3 request"]
    #[inline(always)]
    pub fn pwm3(self) -> &'a mut W {
        self.variant(EM1_A::PWM3)
    }
    #[doc = "PWM 4 request"]
    #[inline(always)]
    pub fn pwm4(self) -> &'a mut W {
        self.variant(EM1_A::PWM4)
    }
    #[doc = "PWM 5 request"]
    #[inline(always)]
    pub fn pwm5(self) -> &'a mut W {
        self.variant(EM1_A::PWM5)
    }
    #[doc = "Cycle mode"]
    #[inline(always)]
    pub fn cycle(self) -> &'a mut W {
        self.variant(EM1_A::CYCLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Select start event for sequencer 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EM2_A {
    #[doc = "0: software request by GSYNC bit"]
    SWREQ,
    #[doc = "1: Analog comparator 0 request"]
    CMP0,
    #[doc = "2: Analog comparator 1 request"]
    CMP1,
    #[doc = "3: Analog comparator 2 request"]
    CMP2,
    #[doc = "4:  Any GPIO interrupt"]
    ITGPIO,
    #[doc = "5: Timer request"]
    TIMER,
    #[doc = "6: PWM 0 request"]
    PWM0,
    #[doc = "7: PWM 1 request"]
    PWM1,
    #[doc = "8: PWM 2 request"]
    PWM2,
    #[doc = "9: PWM 3 request"]
    PWM3,
    #[doc = "10: PWM 4 request"]
    PWM4,
    #[doc = "11: PWM 5 request"]
    PWM5,
    #[doc = "15: Cycle mode"]
    CYCLE,
}
impl From<EM2_A> for u8 {
    #[inline(always)]
    fn from(variant: EM2_A) -> Self {
        match variant {
            EM2_A::SWREQ => 0,
            EM2_A::CMP0 => 1,
            EM2_A::CMP1 => 2,
            EM2_A::CMP2 => 3,
            EM2_A::ITGPIO => 4,
            EM2_A::TIMER => 5,
            EM2_A::PWM0 => 6,
            EM2_A::PWM1 => 7,
            EM2_A::PWM2 => 8,
            EM2_A::PWM3 => 9,
            EM2_A::PWM4 => 10,
            EM2_A::PWM5 => 11,
            EM2_A::CYCLE => 15,
        }
    }
}
#[doc = "Reader of field `EM2`"]
pub type EM2_R = crate::R<u8, EM2_A>;
impl EM2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, EM2_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(EM2_A::SWREQ),
            1 => Val(EM2_A::CMP0),
            2 => Val(EM2_A::CMP1),
            3 => Val(EM2_A::CMP2),
            4 => Val(EM2_A::ITGPIO),
            5 => Val(EM2_A::TIMER),
            6 => Val(EM2_A::PWM0),
            7 => Val(EM2_A::PWM1),
            8 => Val(EM2_A::PWM2),
            9 => Val(EM2_A::PWM3),
            10 => Val(EM2_A::PWM4),
            11 => Val(EM2_A::PWM5),
            15 => Val(EM2_A::CYCLE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SWREQ`"]
    #[inline(always)]
    pub fn is_sw_req(&self) -> bool {
        *self == EM2_A::SWREQ
    }
    #[doc = "Checks if the value of the field is `CMP0`"]
    #[inline(always)]
    pub fn is_cmp0(&self) -> bool {
        *self == EM2_A::CMP0
    }
    #[doc = "Checks if the value of the field is `CMP1`"]
    #[inline(always)]
    pub fn is_cmp1(&self) -> bool {
        *self == EM2_A::CMP1
    }
    #[doc = "Checks if the value of the field is `CMP2`"]
    #[inline(always)]
    pub fn is_cmp2(&self) -> bool {
        *self == EM2_A::CMP2
    }
    #[doc = "Checks if the value of the field is `ITGPIO`"]
    #[inline(always)]
    pub fn is_itgpio(&self) -> bool {
        *self == EM2_A::ITGPIO
    }
    #[doc = "Checks if the value of the field is `TIMER`"]
    #[inline(always)]
    pub fn is_timer(&self) -> bool {
        *self == EM2_A::TIMER
    }
    #[doc = "Checks if the value of the field is `PWM0`"]
    #[inline(always)]
    pub fn is_pwm0(&self) -> bool {
        *self == EM2_A::PWM0
    }
    #[doc = "Checks if the value of the field is `PWM1`"]
    #[inline(always)]
    pub fn is_pwm1(&self) -> bool {
        *self == EM2_A::PWM1
    }
    #[doc = "Checks if the value of the field is `PWM2`"]
    #[inline(always)]
    pub fn is_pwm2(&self) -> bool {
        *self == EM2_A::PWM2
    }
    #[doc = "Checks if the value of the field is `PWM3`"]
    #[inline(always)]
    pub fn is_pwm3(&self) -> bool {
        *self == EM2_A::PWM3
    }
    #[doc = "Checks if the value of the field is `PWM4`"]
    #[inline(always)]
    pub fn is_pwm4(&self) -> bool {
        *self == EM2_A::PWM4
    }
    #[doc = "Checks if the value of the field is `PWM5`"]
    #[inline(always)]
    pub fn is_pwm5(&self) -> bool {
        *self == EM2_A::PWM5
    }
    #[doc = "Checks if the value of the field is `CYCLE`"]
    #[inline(always)]
    pub fn is_cycle(&self) -> bool {
        *self == EM2_A::CYCLE
    }
}
#[doc = "Write proxy for field `EM2`"]
pub struct EM2_W<'a> {
    w: &'a mut W,
}
impl<'a> EM2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EM2_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "software request by GSYNC bit"]
    #[inline(always)]
    pub fn sw_req(self) -> &'a mut W {
        self.variant(EM2_A::SWREQ)
    }
    #[doc = "Analog comparator 0 request"]
    #[inline(always)]
    pub fn cmp0(self) -> &'a mut W {
        self.variant(EM2_A::CMP0)
    }
    #[doc = "Analog comparator 1 request"]
    #[inline(always)]
    pub fn cmp1(self) -> &'a mut W {
        self.variant(EM2_A::CMP1)
    }
    #[doc = "Analog comparator 2 request"]
    #[inline(always)]
    pub fn cmp2(self) -> &'a mut W {
        self.variant(EM2_A::CMP2)
    }
    #[doc = "Any GPIO interrupt"]
    #[inline(always)]
    pub fn itgpio(self) -> &'a mut W {
        self.variant(EM2_A::ITGPIO)
    }
    #[doc = "Timer request"]
    #[inline(always)]
    pub fn timer(self) -> &'a mut W {
        self.variant(EM2_A::TIMER)
    }
    #[doc = "PWM 0 request"]
    #[inline(always)]
    pub fn pwm0(self) -> &'a mut W {
        self.variant(EM2_A::PWM0)
    }
    #[doc = "PWM 1 request"]
    #[inline(always)]
    pub fn pwm1(self) -> &'a mut W {
        self.variant(EM2_A::PWM1)
    }
    #[doc = "PWM 2 request"]
    #[inline(always)]
    pub fn pwm2(self) -> &'a mut W {
        self.variant(EM2_A::PWM2)
    }
    #[doc = "PWM 3 request"]
    #[inline(always)]
    pub fn pwm3(self) -> &'a mut W {
        self.variant(EM2_A::PWM3)
    }
    #[doc = "PWM 4 request"]
    #[inline(always)]
    pub fn pwm4(self) -> &'a mut W {
        self.variant(EM2_A::PWM4)
    }
    #[doc = "PWM 5 request"]
    #[inline(always)]
    pub fn pwm5(self) -> &'a mut W {
        self.variant(EM2_A::PWM5)
    }
    #[doc = "Cycle mode"]
    #[inline(always)]
    pub fn cycle(self) -> &'a mut W {
        self.variant(EM2_A::CYCLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Select start event for sequencer 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EM3_A {
    #[doc = "0: software request by GSYNC bit"]
    SWREQ,
    #[doc = "1: Analog comparator 0 request"]
    CMP0,
    #[doc = "2: Analog comparator 1 request"]
    CMP1,
    #[doc = "3: Analog comparator 2 request"]
    CMP2,
    #[doc = "4:  Any GPIO interrupt"]
    ITGPIO,
    #[doc = "5: Timer request"]
    TIMER,
    #[doc = "6: PWM 0 request"]
    PWM0,
    #[doc = "7: PWM 1 request"]
    PWM1,
    #[doc = "8: PWM 2 request"]
    PWM2,
    #[doc = "9: PWM 3 request"]
    PWM3,
    #[doc = "10: PWM 4 request"]
    PWM4,
    #[doc = "11: PWM 5 request"]
    PWM5,
    #[doc = "15: Cycle mode"]
    CYCLE,
}
impl From<EM3_A> for u8 {
    #[inline(always)]
    fn from(variant: EM3_A) -> Self {
        match variant {
            EM3_A::SWREQ => 0,
            EM3_A::CMP0 => 1,
            EM3_A::CMP1 => 2,
            EM3_A::CMP2 => 3,
            EM3_A::ITGPIO => 4,
            EM3_A::TIMER => 5,
            EM3_A::PWM0 => 6,
            EM3_A::PWM1 => 7,
            EM3_A::PWM2 => 8,
            EM3_A::PWM3 => 9,
            EM3_A::PWM4 => 10,
            EM3_A::PWM5 => 11,
            EM3_A::CYCLE => 15,
        }
    }
}
#[doc = "Reader of field `EM3`"]
pub type EM3_R = crate::R<u8, EM3_A>;
impl EM3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, EM3_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(EM3_A::SWREQ),
            1 => Val(EM3_A::CMP0),
            2 => Val(EM3_A::CMP1),
            3 => Val(EM3_A::CMP2),
            4 => Val(EM3_A::ITGPIO),
            5 => Val(EM3_A::TIMER),
            6 => Val(EM3_A::PWM0),
            7 => Val(EM3_A::PWM1),
            8 => Val(EM3_A::PWM2),
            9 => Val(EM3_A::PWM3),
            10 => Val(EM3_A::PWM4),
            11 => Val(EM3_A::PWM5),
            15 => Val(EM3_A::CYCLE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SWREQ`"]
    #[inline(always)]
    pub fn is_sw_req(&self) -> bool {
        *self == EM3_A::SWREQ
    }
    #[doc = "Checks if the value of the field is `CMP0`"]
    #[inline(always)]
    pub fn is_cmp0(&self) -> bool {
        *self == EM3_A::CMP0
    }
    #[doc = "Checks if the value of the field is `CMP1`"]
    #[inline(always)]
    pub fn is_cmp1(&self) -> bool {
        *self == EM3_A::CMP1
    }
    #[doc = "Checks if the value of the field is `CMP2`"]
    #[inline(always)]
    pub fn is_cmp2(&self) -> bool {
        *self == EM3_A::CMP2
    }
    #[doc = "Checks if the value of the field is `ITGPIO`"]
    #[inline(always)]
    pub fn is_itgpio(&self) -> bool {
        *self == EM3_A::ITGPIO
    }
    #[doc = "Checks if the value of the field is `TIMER`"]
    #[inline(always)]
    pub fn is_timer(&self) -> bool {
        *self == EM3_A::TIMER
    }
    #[doc = "Checks if the value of the field is `PWM0`"]
    #[inline(always)]
    pub fn is_pwm0(&self) -> bool {
        *self == EM3_A::PWM0
    }
    #[doc = "Checks if the value of the field is `PWM1`"]
    #[inline(always)]
    pub fn is_pwm1(&self) -> bool {
        *self == EM3_A::PWM1
    }
    #[doc = "Checks if the value of the field is `PWM2`"]
    #[inline(always)]
    pub fn is_pwm2(&self) -> bool {
        *self == EM3_A::PWM2
    }
    #[doc = "Checks if the value of the field is `PWM3`"]
    #[inline(always)]
    pub fn is_pwm3(&self) -> bool {
        *self == EM3_A::PWM3
    }
    #[doc = "Checks if the value of the field is `PWM4`"]
    #[inline(always)]
    pub fn is_pwm4(&self) -> bool {
        *self == EM3_A::PWM4
    }
    #[doc = "Checks if the value of the field is `PWM5`"]
    #[inline(always)]
    pub fn is_pwm5(&self) -> bool {
        *self == EM3_A::PWM5
    }
    #[doc = "Checks if the value of the field is `CYCLE`"]
    #[inline(always)]
    pub fn is_cycle(&self) -> bool {
        *self == EM3_A::CYCLE
    }
}
#[doc = "Write proxy for field `EM3`"]
pub struct EM3_W<'a> {
    w: &'a mut W,
}
impl<'a> EM3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EM3_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "software request by GSYNC bit"]
    #[inline(always)]
    pub fn sw_req(self) -> &'a mut W {
        self.variant(EM3_A::SWREQ)
    }
    #[doc = "Analog comparator 0 request"]
    #[inline(always)]
    pub fn cmp0(self) -> &'a mut W {
        self.variant(EM3_A::CMP0)
    }
    #[doc = "Analog comparator 1 request"]
    #[inline(always)]
    pub fn cmp1(self) -> &'a mut W {
        self.variant(EM3_A::CMP1)
    }
    #[doc = "Analog comparator 2 request"]
    #[inline(always)]
    pub fn cmp2(self) -> &'a mut W {
        self.variant(EM3_A::CMP2)
    }
    #[doc = "Any GPIO interrupt"]
    #[inline(always)]
    pub fn itgpio(self) -> &'a mut W {
        self.variant(EM3_A::ITGPIO)
    }
    #[doc = "Timer request"]
    #[inline(always)]
    pub fn timer(self) -> &'a mut W {
        self.variant(EM3_A::TIMER)
    }
    #[doc = "PWM 0 request"]
    #[inline(always)]
    pub fn pwm0(self) -> &'a mut W {
        self.variant(EM3_A::PWM0)
    }
    #[doc = "PWM 1 request"]
    #[inline(always)]
    pub fn pwm1(self) -> &'a mut W {
        self.variant(EM3_A::PWM1)
    }
    #[doc = "PWM 2 request"]
    #[inline(always)]
    pub fn pwm2(self) -> &'a mut W {
        self.variant(EM3_A::PWM2)
    }
    #[doc = "PWM 3 request"]
    #[inline(always)]
    pub fn pwm3(self) -> &'a mut W {
        self.variant(EM3_A::PWM3)
    }
    #[doc = "PWM 4 request"]
    #[inline(always)]
    pub fn pwm4(self) -> &'a mut W {
        self.variant(EM3_A::PWM4)
    }
    #[doc = "PWM 5 request"]
    #[inline(always)]
    pub fn pwm5(self) -> &'a mut W {
        self.variant(EM3_A::PWM5)
    }
    #[doc = "Cycle mode"]
    #[inline(always)]
    pub fn cycle(self) -> &'a mut W {
        self.variant(EM3_A::CYCLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Select start event for sequencer 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EM4_A {
    #[doc = "0: software request by GSYNC bit"]
    SWREQ,
    #[doc = "1: Analog comparator 0 request"]
    CMP0,
    #[doc = "2: Analog comparator 1 request"]
    CMP1,
    #[doc = "3: Analog comparator 2 request"]
    CMP2,
    #[doc = "4:  Any GPIO interrupt"]
    ITGPIO,
    #[doc = "5: Timer request"]
    TIMER,
    #[doc = "6: PWM 0 request"]
    PWM0,
    #[doc = "7: PWM 1 request"]
    PWM1,
    #[doc = "8: PWM 2 request"]
    PWM2,
    #[doc = "9: PWM 3 request"]
    PWM3,
    #[doc = "10: PWM 4 request"]
    PWM4,
    #[doc = "11: PWM 5 request"]
    PWM5,
    #[doc = "15: Cycle mode"]
    CYCLE,
}
impl From<EM4_A> for u8 {
    #[inline(always)]
    fn from(variant: EM4_A) -> Self {
        match variant {
            EM4_A::SWREQ => 0,
            EM4_A::CMP0 => 1,
            EM4_A::CMP1 => 2,
            EM4_A::CMP2 => 3,
            EM4_A::ITGPIO => 4,
            EM4_A::TIMER => 5,
            EM4_A::PWM0 => 6,
            EM4_A::PWM1 => 7,
            EM4_A::PWM2 => 8,
            EM4_A::PWM3 => 9,
            EM4_A::PWM4 => 10,
            EM4_A::PWM5 => 11,
            EM4_A::CYCLE => 15,
        }
    }
}
#[doc = "Reader of field `EM4`"]
pub type EM4_R = crate::R<u8, EM4_A>;
impl EM4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, EM4_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(EM4_A::SWREQ),
            1 => Val(EM4_A::CMP0),
            2 => Val(EM4_A::CMP1),
            3 => Val(EM4_A::CMP2),
            4 => Val(EM4_A::ITGPIO),
            5 => Val(EM4_A::TIMER),
            6 => Val(EM4_A::PWM0),
            7 => Val(EM4_A::PWM1),
            8 => Val(EM4_A::PWM2),
            9 => Val(EM4_A::PWM3),
            10 => Val(EM4_A::PWM4),
            11 => Val(EM4_A::PWM5),
            15 => Val(EM4_A::CYCLE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SWREQ`"]
    #[inline(always)]
    pub fn is_sw_req(&self) -> bool {
        *self == EM4_A::SWREQ
    }
    #[doc = "Checks if the value of the field is `CMP0`"]
    #[inline(always)]
    pub fn is_cmp0(&self) -> bool {
        *self == EM4_A::CMP0
    }
    #[doc = "Checks if the value of the field is `CMP1`"]
    #[inline(always)]
    pub fn is_cmp1(&self) -> bool {
        *self == EM4_A::CMP1
    }
    #[doc = "Checks if the value of the field is `CMP2`"]
    #[inline(always)]
    pub fn is_cmp2(&self) -> bool {
        *self == EM4_A::CMP2
    }
    #[doc = "Checks if the value of the field is `ITGPIO`"]
    #[inline(always)]
    pub fn is_itgpio(&self) -> bool {
        *self == EM4_A::ITGPIO
    }
    #[doc = "Checks if the value of the field is `TIMER`"]
    #[inline(always)]
    pub fn is_timer(&self) -> bool {
        *self == EM4_A::TIMER
    }
    #[doc = "Checks if the value of the field is `PWM0`"]
    #[inline(always)]
    pub fn is_pwm0(&self) -> bool {
        *self == EM4_A::PWM0
    }
    #[doc = "Checks if the value of the field is `PWM1`"]
    #[inline(always)]
    pub fn is_pwm1(&self) -> bool {
        *self == EM4_A::PWM1
    }
    #[doc = "Checks if the value of the field is `PWM2`"]
    #[inline(always)]
    pub fn is_pwm2(&self) -> bool {
        *self == EM4_A::PWM2
    }
    #[doc = "Checks if the value of the field is `PWM3`"]
    #[inline(always)]
    pub fn is_pwm3(&self) -> bool {
        *self == EM4_A::PWM3
    }
    #[doc = "Checks if the value of the field is `PWM4`"]
    #[inline(always)]
    pub fn is_pwm4(&self) -> bool {
        *self == EM4_A::PWM4
    }
    #[doc = "Checks if the value of the field is `PWM5`"]
    #[inline(always)]
    pub fn is_pwm5(&self) -> bool {
        *self == EM4_A::PWM5
    }
    #[doc = "Checks if the value of the field is `CYCLE`"]
    #[inline(always)]
    pub fn is_cycle(&self) -> bool {
        *self == EM4_A::CYCLE
    }
}
#[doc = "Write proxy for field `EM4`"]
pub struct EM4_W<'a> {
    w: &'a mut W,
}
impl<'a> EM4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EM4_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "software request by GSYNC bit"]
    #[inline(always)]
    pub fn sw_req(self) -> &'a mut W {
        self.variant(EM4_A::SWREQ)
    }
    #[doc = "Analog comparator 0 request"]
    #[inline(always)]
    pub fn cmp0(self) -> &'a mut W {
        self.variant(EM4_A::CMP0)
    }
    #[doc = "Analog comparator 1 request"]
    #[inline(always)]
    pub fn cmp1(self) -> &'a mut W {
        self.variant(EM4_A::CMP1)
    }
    #[doc = "Analog comparator 2 request"]
    #[inline(always)]
    pub fn cmp2(self) -> &'a mut W {
        self.variant(EM4_A::CMP2)
    }
    #[doc = "Any GPIO interrupt"]
    #[inline(always)]
    pub fn itgpio(self) -> &'a mut W {
        self.variant(EM4_A::ITGPIO)
    }
    #[doc = "Timer request"]
    #[inline(always)]
    pub fn timer(self) -> &'a mut W {
        self.variant(EM4_A::TIMER)
    }
    #[doc = "PWM 0 request"]
    #[inline(always)]
    pub fn pwm0(self) -> &'a mut W {
        self.variant(EM4_A::PWM0)
    }
    #[doc = "PWM 1 request"]
    #[inline(always)]
    pub fn pwm1(self) -> &'a mut W {
        self.variant(EM4_A::PWM1)
    }
    #[doc = "PWM 2 request"]
    #[inline(always)]
    pub fn pwm2(self) -> &'a mut W {
        self.variant(EM4_A::PWM2)
    }
    #[doc = "PWM 3 request"]
    #[inline(always)]
    pub fn pwm3(self) -> &'a mut W {
        self.variant(EM4_A::PWM3)
    }
    #[doc = "PWM 4 request"]
    #[inline(always)]
    pub fn pwm4(self) -> &'a mut W {
        self.variant(EM4_A::PWM4)
    }
    #[doc = "PWM 5 request"]
    #[inline(always)]
    pub fn pwm5(self) -> &'a mut W {
        self.variant(EM4_A::PWM5)
    }
    #[doc = "Cycle mode"]
    #[inline(always)]
    pub fn cycle(self) -> &'a mut W {
        self.variant(EM4_A::CYCLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Select start event for sequencer 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EM5_A {
    #[doc = "0: software request by GSYNC bit"]
    SWREQ,
    #[doc = "1: Analog comparator 0 request"]
    CMP0,
    #[doc = "2: Analog comparator 1 request"]
    CMP1,
    #[doc = "3: Analog comparator 2 request"]
    CMP2,
    #[doc = "4:  Any GPIO interrupt"]
    ITGPIO,
    #[doc = "5: Timer request"]
    TIMER,
    #[doc = "6: PWM 0 request"]
    PWM0,
    #[doc = "7: PWM 1 request"]
    PWM1,
    #[doc = "8: PWM 2 request"]
    PWM2,
    #[doc = "9: PWM 3 request"]
    PWM3,
    #[doc = "10: PWM 4 request"]
    PWM4,
    #[doc = "11: PWM 5 request"]
    PWM5,
    #[doc = "15: Cycle mode"]
    CYCLE,
}
impl From<EM5_A> for u8 {
    #[inline(always)]
    fn from(variant: EM5_A) -> Self {
        match variant {
            EM5_A::SWREQ => 0,
            EM5_A::CMP0 => 1,
            EM5_A::CMP1 => 2,
            EM5_A::CMP2 => 3,
            EM5_A::ITGPIO => 4,
            EM5_A::TIMER => 5,
            EM5_A::PWM0 => 6,
            EM5_A::PWM1 => 7,
            EM5_A::PWM2 => 8,
            EM5_A::PWM3 => 9,
            EM5_A::PWM4 => 10,
            EM5_A::PWM5 => 11,
            EM5_A::CYCLE => 15,
        }
    }
}
#[doc = "Reader of field `EM5`"]
pub type EM5_R = crate::R<u8, EM5_A>;
impl EM5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, EM5_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(EM5_A::SWREQ),
            1 => Val(EM5_A::CMP0),
            2 => Val(EM5_A::CMP1),
            3 => Val(EM5_A::CMP2),
            4 => Val(EM5_A::ITGPIO),
            5 => Val(EM5_A::TIMER),
            6 => Val(EM5_A::PWM0),
            7 => Val(EM5_A::PWM1),
            8 => Val(EM5_A::PWM2),
            9 => Val(EM5_A::PWM3),
            10 => Val(EM5_A::PWM4),
            11 => Val(EM5_A::PWM5),
            15 => Val(EM5_A::CYCLE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SWREQ`"]
    #[inline(always)]
    pub fn is_sw_req(&self) -> bool {
        *self == EM5_A::SWREQ
    }
    #[doc = "Checks if the value of the field is `CMP0`"]
    #[inline(always)]
    pub fn is_cmp0(&self) -> bool {
        *self == EM5_A::CMP0
    }
    #[doc = "Checks if the value of the field is `CMP1`"]
    #[inline(always)]
    pub fn is_cmp1(&self) -> bool {
        *self == EM5_A::CMP1
    }
    #[doc = "Checks if the value of the field is `CMP2`"]
    #[inline(always)]
    pub fn is_cmp2(&self) -> bool {
        *self == EM5_A::CMP2
    }
    #[doc = "Checks if the value of the field is `ITGPIO`"]
    #[inline(always)]
    pub fn is_itgpio(&self) -> bool {
        *self == EM5_A::ITGPIO
    }
    #[doc = "Checks if the value of the field is `TIMER`"]
    #[inline(always)]
    pub fn is_timer(&self) -> bool {
        *self == EM5_A::TIMER
    }
    #[doc = "Checks if the value of the field is `PWM0`"]
    #[inline(always)]
    pub fn is_pwm0(&self) -> bool {
        *self == EM5_A::PWM0
    }
    #[doc = "Checks if the value of the field is `PWM1`"]
    #[inline(always)]
    pub fn is_pwm1(&self) -> bool {
        *self == EM5_A::PWM1
    }
    #[doc = "Checks if the value of the field is `PWM2`"]
    #[inline(always)]
    pub fn is_pwm2(&self) -> bool {
        *self == EM5_A::PWM2
    }
    #[doc = "Checks if the value of the field is `PWM3`"]
    #[inline(always)]
    pub fn is_pwm3(&self) -> bool {
        *self == EM5_A::PWM3
    }
    #[doc = "Checks if the value of the field is `PWM4`"]
    #[inline(always)]
    pub fn is_pwm4(&self) -> bool {
        *self == EM5_A::PWM4
    }
    #[doc = "Checks if the value of the field is `PWM5`"]
    #[inline(always)]
    pub fn is_pwm5(&self) -> bool {
        *self == EM5_A::PWM5
    }
    #[doc = "Checks if the value of the field is `CYCLE`"]
    #[inline(always)]
    pub fn is_cycle(&self) -> bool {
        *self == EM5_A::CYCLE
    }
}
#[doc = "Write proxy for field `EM5`"]
pub struct EM5_W<'a> {
    w: &'a mut W,
}
impl<'a> EM5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EM5_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "software request by GSYNC bit"]
    #[inline(always)]
    pub fn sw_req(self) -> &'a mut W {
        self.variant(EM5_A::SWREQ)
    }
    #[doc = "Analog comparator 0 request"]
    #[inline(always)]
    pub fn cmp0(self) -> &'a mut W {
        self.variant(EM5_A::CMP0)
    }
    #[doc = "Analog comparator 1 request"]
    #[inline(always)]
    pub fn cmp1(self) -> &'a mut W {
        self.variant(EM5_A::CMP1)
    }
    #[doc = "Analog comparator 2 request"]
    #[inline(always)]
    pub fn cmp2(self) -> &'a mut W {
        self.variant(EM5_A::CMP2)
    }
    #[doc = "Any GPIO interrupt"]
    #[inline(always)]
    pub fn itgpio(self) -> &'a mut W {
        self.variant(EM5_A::ITGPIO)
    }
    #[doc = "Timer request"]
    #[inline(always)]
    pub fn timer(self) -> &'a mut W {
        self.variant(EM5_A::TIMER)
    }
    #[doc = "PWM 0 request"]
    #[inline(always)]
    pub fn pwm0(self) -> &'a mut W {
        self.variant(EM5_A::PWM0)
    }
    #[doc = "PWM 1 request"]
    #[inline(always)]
    pub fn pwm1(self) -> &'a mut W {
        self.variant(EM5_A::PWM1)
    }
    #[doc = "PWM 2 request"]
    #[inline(always)]
    pub fn pwm2(self) -> &'a mut W {
        self.variant(EM5_A::PWM2)
    }
    #[doc = "PWM 3 request"]
    #[inline(always)]
    pub fn pwm3(self) -> &'a mut W {
        self.variant(EM5_A::PWM3)
    }
    #[doc = "PWM 4 request"]
    #[inline(always)]
    pub fn pwm4(self) -> &'a mut W {
        self.variant(EM5_A::PWM4)
    }
    #[doc = "PWM 5 request"]
    #[inline(always)]
    pub fn pwm5(self) -> &'a mut W {
        self.variant(EM5_A::PWM5)
    }
    #[doc = "Cycle mode"]
    #[inline(always)]
    pub fn cycle(self) -> &'a mut W {
        self.variant(EM5_A::CYCLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Select start event for sequencer 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EM6_A {
    #[doc = "0: software request by GSYNC bit"]
    SWREQ,
    #[doc = "1: Analog comparator 0 request"]
    CMP0,
    #[doc = "2: Analog comparator 1 request"]
    CMP1,
    #[doc = "3: Analog comparator 2 request"]
    CMP2,
    #[doc = "4:  Any GPIO interrupt"]
    ITGPIO,
    #[doc = "5: Timer request"]
    TIMER,
    #[doc = "6: PWM 0 request"]
    PWM0,
    #[doc = "7: PWM 1 request"]
    PWM1,
    #[doc = "8: PWM 2 request"]
    PWM2,
    #[doc = "9: PWM 3 request"]
    PWM3,
    #[doc = "10: PWM 4 request"]
    PWM4,
    #[doc = "11: PWM 5 request"]
    PWM5,
    #[doc = "15: Cycle mode"]
    CYCLE,
}
impl From<EM6_A> for u8 {
    #[inline(always)]
    fn from(variant: EM6_A) -> Self {
        match variant {
            EM6_A::SWREQ => 0,
            EM6_A::CMP0 => 1,
            EM6_A::CMP1 => 2,
            EM6_A::CMP2 => 3,
            EM6_A::ITGPIO => 4,
            EM6_A::TIMER => 5,
            EM6_A::PWM0 => 6,
            EM6_A::PWM1 => 7,
            EM6_A::PWM2 => 8,
            EM6_A::PWM3 => 9,
            EM6_A::PWM4 => 10,
            EM6_A::PWM5 => 11,
            EM6_A::CYCLE => 15,
        }
    }
}
#[doc = "Reader of field `EM6`"]
pub type EM6_R = crate::R<u8, EM6_A>;
impl EM6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, EM6_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(EM6_A::SWREQ),
            1 => Val(EM6_A::CMP0),
            2 => Val(EM6_A::CMP1),
            3 => Val(EM6_A::CMP2),
            4 => Val(EM6_A::ITGPIO),
            5 => Val(EM6_A::TIMER),
            6 => Val(EM6_A::PWM0),
            7 => Val(EM6_A::PWM1),
            8 => Val(EM6_A::PWM2),
            9 => Val(EM6_A::PWM3),
            10 => Val(EM6_A::PWM4),
            11 => Val(EM6_A::PWM5),
            15 => Val(EM6_A::CYCLE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SWREQ`"]
    #[inline(always)]
    pub fn is_sw_req(&self) -> bool {
        *self == EM6_A::SWREQ
    }
    #[doc = "Checks if the value of the field is `CMP0`"]
    #[inline(always)]
    pub fn is_cmp0(&self) -> bool {
        *self == EM6_A::CMP0
    }
    #[doc = "Checks if the value of the field is `CMP1`"]
    #[inline(always)]
    pub fn is_cmp1(&self) -> bool {
        *self == EM6_A::CMP1
    }
    #[doc = "Checks if the value of the field is `CMP2`"]
    #[inline(always)]
    pub fn is_cmp2(&self) -> bool {
        *self == EM6_A::CMP2
    }
    #[doc = "Checks if the value of the field is `ITGPIO`"]
    #[inline(always)]
    pub fn is_itgpio(&self) -> bool {
        *self == EM6_A::ITGPIO
    }
    #[doc = "Checks if the value of the field is `TIMER`"]
    #[inline(always)]
    pub fn is_timer(&self) -> bool {
        *self == EM6_A::TIMER
    }
    #[doc = "Checks if the value of the field is `PWM0`"]
    #[inline(always)]
    pub fn is_pwm0(&self) -> bool {
        *self == EM6_A::PWM0
    }
    #[doc = "Checks if the value of the field is `PWM1`"]
    #[inline(always)]
    pub fn is_pwm1(&self) -> bool {
        *self == EM6_A::PWM1
    }
    #[doc = "Checks if the value of the field is `PWM2`"]
    #[inline(always)]
    pub fn is_pwm2(&self) -> bool {
        *self == EM6_A::PWM2
    }
    #[doc = "Checks if the value of the field is `PWM3`"]
    #[inline(always)]
    pub fn is_pwm3(&self) -> bool {
        *self == EM6_A::PWM3
    }
    #[doc = "Checks if the value of the field is `PWM4`"]
    #[inline(always)]
    pub fn is_pwm4(&self) -> bool {
        *self == EM6_A::PWM4
    }
    #[doc = "Checks if the value of the field is `PWM5`"]
    #[inline(always)]
    pub fn is_pwm5(&self) -> bool {
        *self == EM6_A::PWM5
    }
    #[doc = "Checks if the value of the field is `CYCLE`"]
    #[inline(always)]
    pub fn is_cycle(&self) -> bool {
        *self == EM6_A::CYCLE
    }
}
#[doc = "Write proxy for field `EM6`"]
pub struct EM6_W<'a> {
    w: &'a mut W,
}
impl<'a> EM6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EM6_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "software request by GSYNC bit"]
    #[inline(always)]
    pub fn sw_req(self) -> &'a mut W {
        self.variant(EM6_A::SWREQ)
    }
    #[doc = "Analog comparator 0 request"]
    #[inline(always)]
    pub fn cmp0(self) -> &'a mut W {
        self.variant(EM6_A::CMP0)
    }
    #[doc = "Analog comparator 1 request"]
    #[inline(always)]
    pub fn cmp1(self) -> &'a mut W {
        self.variant(EM6_A::CMP1)
    }
    #[doc = "Analog comparator 2 request"]
    #[inline(always)]
    pub fn cmp2(self) -> &'a mut W {
        self.variant(EM6_A::CMP2)
    }
    #[doc = "Any GPIO interrupt"]
    #[inline(always)]
    pub fn itgpio(self) -> &'a mut W {
        self.variant(EM6_A::ITGPIO)
    }
    #[doc = "Timer request"]
    #[inline(always)]
    pub fn timer(self) -> &'a mut W {
        self.variant(EM6_A::TIMER)
    }
    #[doc = "PWM 0 request"]
    #[inline(always)]
    pub fn pwm0(self) -> &'a mut W {
        self.variant(EM6_A::PWM0)
    }
    #[doc = "PWM 1 request"]
    #[inline(always)]
    pub fn pwm1(self) -> &'a mut W {
        self.variant(EM6_A::PWM1)
    }
    #[doc = "PWM 2 request"]
    #[inline(always)]
    pub fn pwm2(self) -> &'a mut W {
        self.variant(EM6_A::PWM2)
    }
    #[doc = "PWM 3 request"]
    #[inline(always)]
    pub fn pwm3(self) -> &'a mut W {
        self.variant(EM6_A::PWM3)
    }
    #[doc = "PWM 4 request"]
    #[inline(always)]
    pub fn pwm4(self) -> &'a mut W {
        self.variant(EM6_A::PWM4)
    }
    #[doc = "PWM 5 request"]
    #[inline(always)]
    pub fn pwm5(self) -> &'a mut W {
        self.variant(EM6_A::PWM5)
    }
    #[doc = "Cycle mode"]
    #[inline(always)]
    pub fn cycle(self) -> &'a mut W {
        self.variant(EM6_A::CYCLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Select start event for sequencer 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EM7_A {
    #[doc = "0: software request by GSYNC bit"]
    SWREQ,
    #[doc = "1: Analog comparator 0 request"]
    CMP0,
    #[doc = "2: Analog comparator 1 request"]
    CMP1,
    #[doc = "3: Analog comparator 2 request"]
    CMP2,
    #[doc = "4:  Any GPIO interrupt"]
    ITGPIO,
    #[doc = "5: Timer request"]
    TIMER,
    #[doc = "6: PWM 0 request"]
    PWM0,
    #[doc = "7: PWM 1 request"]
    PWM1,
    #[doc = "8: PWM 2 request"]
    PWM2,
    #[doc = "9: PWM 3 request"]
    PWM3,
    #[doc = "10: PWM 4 request"]
    PWM4,
    #[doc = "11: PWM 5 request"]
    PWM5,
    #[doc = "15: Cycle mode"]
    CYCLE,
}
impl From<EM7_A> for u8 {
    #[inline(always)]
    fn from(variant: EM7_A) -> Self {
        match variant {
            EM7_A::SWREQ => 0,
            EM7_A::CMP0 => 1,
            EM7_A::CMP1 => 2,
            EM7_A::CMP2 => 3,
            EM7_A::ITGPIO => 4,
            EM7_A::TIMER => 5,
            EM7_A::PWM0 => 6,
            EM7_A::PWM1 => 7,
            EM7_A::PWM2 => 8,
            EM7_A::PWM3 => 9,
            EM7_A::PWM4 => 10,
            EM7_A::PWM5 => 11,
            EM7_A::CYCLE => 15,
        }
    }
}
#[doc = "Reader of field `EM7`"]
pub type EM7_R = crate::R<u8, EM7_A>;
impl EM7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, EM7_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(EM7_A::SWREQ),
            1 => Val(EM7_A::CMP0),
            2 => Val(EM7_A::CMP1),
            3 => Val(EM7_A::CMP2),
            4 => Val(EM7_A::ITGPIO),
            5 => Val(EM7_A::TIMER),
            6 => Val(EM7_A::PWM0),
            7 => Val(EM7_A::PWM1),
            8 => Val(EM7_A::PWM2),
            9 => Val(EM7_A::PWM3),
            10 => Val(EM7_A::PWM4),
            11 => Val(EM7_A::PWM5),
            15 => Val(EM7_A::CYCLE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SWREQ`"]
    #[inline(always)]
    pub fn is_sw_req(&self) -> bool {
        *self == EM7_A::SWREQ
    }
    #[doc = "Checks if the value of the field is `CMP0`"]
    #[inline(always)]
    pub fn is_cmp0(&self) -> bool {
        *self == EM7_A::CMP0
    }
    #[doc = "Checks if the value of the field is `CMP1`"]
    #[inline(always)]
    pub fn is_cmp1(&self) -> bool {
        *self == EM7_A::CMP1
    }
    #[doc = "Checks if the value of the field is `CMP2`"]
    #[inline(always)]
    pub fn is_cmp2(&self) -> bool {
        *self == EM7_A::CMP2
    }
    #[doc = "Checks if the value of the field is `ITGPIO`"]
    #[inline(always)]
    pub fn is_itgpio(&self) -> bool {
        *self == EM7_A::ITGPIO
    }
    #[doc = "Checks if the value of the field is `TIMER`"]
    #[inline(always)]
    pub fn is_timer(&self) -> bool {
        *self == EM7_A::TIMER
    }
    #[doc = "Checks if the value of the field is `PWM0`"]
    #[inline(always)]
    pub fn is_pwm0(&self) -> bool {
        *self == EM7_A::PWM0
    }
    #[doc = "Checks if the value of the field is `PWM1`"]
    #[inline(always)]
    pub fn is_pwm1(&self) -> bool {
        *self == EM7_A::PWM1
    }
    #[doc = "Checks if the value of the field is `PWM2`"]
    #[inline(always)]
    pub fn is_pwm2(&self) -> bool {
        *self == EM7_A::PWM2
    }
    #[doc = "Checks if the value of the field is `PWM3`"]
    #[inline(always)]
    pub fn is_pwm3(&self) -> bool {
        *self == EM7_A::PWM3
    }
    #[doc = "Checks if the value of the field is `PWM4`"]
    #[inline(always)]
    pub fn is_pwm4(&self) -> bool {
        *self == EM7_A::PWM4
    }
    #[doc = "Checks if the value of the field is `PWM5`"]
    #[inline(always)]
    pub fn is_pwm5(&self) -> bool {
        *self == EM7_A::PWM5
    }
    #[doc = "Checks if the value of the field is `CYCLE`"]
    #[inline(always)]
    pub fn is_cycle(&self) -> bool {
        *self == EM7_A::CYCLE
    }
}
#[doc = "Write proxy for field `EM7`"]
pub struct EM7_W<'a> {
    w: &'a mut W,
}
impl<'a> EM7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EM7_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "software request by GSYNC bit"]
    #[inline(always)]
    pub fn sw_req(self) -> &'a mut W {
        self.variant(EM7_A::SWREQ)
    }
    #[doc = "Analog comparator 0 request"]
    #[inline(always)]
    pub fn cmp0(self) -> &'a mut W {
        self.variant(EM7_A::CMP0)
    }
    #[doc = "Analog comparator 1 request"]
    #[inline(always)]
    pub fn cmp1(self) -> &'a mut W {
        self.variant(EM7_A::CMP1)
    }
    #[doc = "Analog comparator 2 request"]
    #[inline(always)]
    pub fn cmp2(self) -> &'a mut W {
        self.variant(EM7_A::CMP2)
    }
    #[doc = "Any GPIO interrupt"]
    #[inline(always)]
    pub fn itgpio(self) -> &'a mut W {
        self.variant(EM7_A::ITGPIO)
    }
    #[doc = "Timer request"]
    #[inline(always)]
    pub fn timer(self) -> &'a mut W {
        self.variant(EM7_A::TIMER)
    }
    #[doc = "PWM 0 request"]
    #[inline(always)]
    pub fn pwm0(self) -> &'a mut W {
        self.variant(EM7_A::PWM0)
    }
    #[doc = "PWM 1 request"]
    #[inline(always)]
    pub fn pwm1(self) -> &'a mut W {
        self.variant(EM7_A::PWM1)
    }
    #[doc = "PWM 2 request"]
    #[inline(always)]
    pub fn pwm2(self) -> &'a mut W {
        self.variant(EM7_A::PWM2)
    }
    #[doc = "PWM 3 request"]
    #[inline(always)]
    pub fn pwm3(self) -> &'a mut W {
        self.variant(EM7_A::PWM3)
    }
    #[doc = "PWM 4 request"]
    #[inline(always)]
    pub fn pwm4(self) -> &'a mut W {
        self.variant(EM7_A::PWM4)
    }
    #[doc = "PWM 5 request"]
    #[inline(always)]
    pub fn pwm5(self) -> &'a mut W {
        self.variant(EM7_A::PWM5)
    }
    #[doc = "Cycle mode"]
    #[inline(always)]
    pub fn cycle(self) -> &'a mut W {
        self.variant(EM7_A::CYCLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Select start event for sequencer 0"]
    #[inline(always)]
    pub fn em0(&self) -> EM0_R {
        EM0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Select start event for sequencer 1"]
    #[inline(always)]
    pub fn em1(&self) -> EM1_R {
        EM1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Select start event for sequencer 2"]
    #[inline(always)]
    pub fn em2(&self) -> EM2_R {
        EM2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Select start event for sequencer 3"]
    #[inline(always)]
    pub fn em3(&self) -> EM3_R {
        EM3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Select start event for sequencer 4"]
    #[inline(always)]
    pub fn em4(&self) -> EM4_R {
        EM4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Select start event for sequencer 5"]
    #[inline(always)]
    pub fn em5(&self) -> EM5_R {
        EM5_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Select start event for sequencer 6"]
    #[inline(always)]
    pub fn em6(&self) -> EM6_R {
        EM6_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Select start event for sequencer 7"]
    #[inline(always)]
    pub fn em7(&self) -> EM7_R {
        EM7_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Select start event for sequencer 0"]
    #[inline(always)]
    pub fn em0(&mut self) -> EM0_W {
        EM0_W { w: self }
    }
    #[doc = "Bits 4:7 - Select start event for sequencer 1"]
    #[inline(always)]
    pub fn em1(&mut self) -> EM1_W {
        EM1_W { w: self }
    }
    #[doc = "Bits 8:11 - Select start event for sequencer 2"]
    #[inline(always)]
    pub fn em2(&mut self) -> EM2_W {
        EM2_W { w: self }
    }
    #[doc = "Bits 12:15 - Select start event for sequencer 3"]
    #[inline(always)]
    pub fn em3(&mut self) -> EM3_W {
        EM3_W { w: self }
    }
    #[doc = "Bits 16:19 - Select start event for sequencer 4"]
    #[inline(always)]
    pub fn em4(&mut self) -> EM4_W {
        EM4_W { w: self }
    }
    #[doc = "Bits 20:23 - Select start event for sequencer 5"]
    #[inline(always)]
    pub fn em5(&mut self) -> EM5_W {
        EM5_W { w: self }
    }
    #[doc = "Bits 24:27 - Select start event for sequencer 6"]
    #[inline(always)]
    pub fn em6(&mut self) -> EM6_W {
        EM6_W { w: self }
    }
    #[doc = "Bits 28:31 - Select start event for sequencer 7"]
    #[inline(always)]
    pub fn em7(&mut self) -> EM7_W {
        EM7_W { w: self }
    }
}
