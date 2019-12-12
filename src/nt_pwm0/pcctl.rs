#[doc = "Reader of register PCCTL"]
pub type R = crate::R<u32, super::PCCTL>;
#[doc = "Writer for register PCCTL"]
pub type W = crate::W<u32, super::PCCTL>;
#[doc = "Register PCCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::PCCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CHPEN`"]
pub type CHPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CHPEN`"]
pub struct CHPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CHPEN_W<'a> {
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
#[doc = "Reader of field `OSHTWTH`"]
pub type OSHTWTH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OSHTWTH`"]
pub struct OSHTWTH_W<'a> {
    w: &'a mut W,
}
impl<'a> OSHTWTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 1)) | (((value as u32) & 0x0f) << 1);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SHRFREQ_A {
    #[doc = "0: sync frequency divide by 1"]
    DIV1,
    #[doc = "1: sync frequency divide by 2"]
    DIV2,
    #[doc = "2: sync frequency divide by 3"]
    DIV3,
    #[doc = "3: sync frequency divide by 4"]
    DIV4,
    #[doc = "4: sync frequency divide by 5"]
    DIV5,
    #[doc = "5: sync frequency divide by 6"]
    DIV6,
    #[doc = "6: sync frequency divide by 7"]
    DIV7,
    #[doc = "7: sync frequency divide by 8"]
    DIV8,
}
impl From<SHRFREQ_A> for u8 {
    #[inline(always)]
    fn from(variant: SHRFREQ_A) -> Self {
        match variant {
            SHRFREQ_A::DIV1 => 0,
            SHRFREQ_A::DIV2 => 1,
            SHRFREQ_A::DIV3 => 2,
            SHRFREQ_A::DIV4 => 3,
            SHRFREQ_A::DIV5 => 4,
            SHRFREQ_A::DIV6 => 5,
            SHRFREQ_A::DIV7 => 6,
            SHRFREQ_A::DIV8 => 7,
        }
    }
}
#[doc = "Reader of field `SHRFREQ`"]
pub type SHRFREQ_R = crate::R<u8, SHRFREQ_A>;
impl SHRFREQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SHRFREQ_A {
        match self.bits {
            0 => SHRFREQ_A::DIV1,
            1 => SHRFREQ_A::DIV2,
            2 => SHRFREQ_A::DIV3,
            3 => SHRFREQ_A::DIV4,
            4 => SHRFREQ_A::DIV5,
            5 => SHRFREQ_A::DIV6,
            6 => SHRFREQ_A::DIV7,
            7 => SHRFREQ_A::DIV8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == SHRFREQ_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == SHRFREQ_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV3`"]
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        *self == SHRFREQ_A::DIV3
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == SHRFREQ_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV5`"]
    #[inline(always)]
    pub fn is_div5(&self) -> bool {
        *self == SHRFREQ_A::DIV5
    }
    #[doc = "Checks if the value of the field is `DIV6`"]
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == SHRFREQ_A::DIV6
    }
    #[doc = "Checks if the value of the field is `DIV7`"]
    #[inline(always)]
    pub fn is_div7(&self) -> bool {
        *self == SHRFREQ_A::DIV7
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == SHRFREQ_A::DIV8
    }
}
#[doc = "Write proxy for field `SHRFREQ`"]
pub struct SHRFREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> SHRFREQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SHRFREQ_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "sync frequency divide by 1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(SHRFREQ_A::DIV1)
    }
    #[doc = "sync frequency divide by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(SHRFREQ_A::DIV2)
    }
    #[doc = "sync frequency divide by 3"]
    #[inline(always)]
    pub fn div3(self) -> &'a mut W {
        self.variant(SHRFREQ_A::DIV3)
    }
    #[doc = "sync frequency divide by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(SHRFREQ_A::DIV4)
    }
    #[doc = "sync frequency divide by 5"]
    #[inline(always)]
    pub fn div5(self) -> &'a mut W {
        self.variant(SHRFREQ_A::DIV5)
    }
    #[doc = "sync frequency divide by 6"]
    #[inline(always)]
    pub fn div6(self) -> &'a mut W {
        self.variant(SHRFREQ_A::DIV6)
    }
    #[doc = "sync frequency divide by 7"]
    #[inline(always)]
    pub fn div7(self) -> &'a mut W {
        self.variant(SHRFREQ_A::DIV7)
    }
    #[doc = "sync frequency divide by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(SHRFREQ_A::DIV8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | (((value as u32) & 0x07) << 5);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHPDUTY_A {
    #[doc = "0: duty 1/8"]
    DUTY_1_8,
    #[doc = "1: duty 2/8"]
    DUTY_2_8,
    #[doc = "2: duty 3/8"]
    DUTY_3_8,
    #[doc = "3: duty 4/8"]
    DUTY_4_8,
    #[doc = "4: duty 5/8"]
    DUTY_5_8,
    #[doc = "5: duty 6/8"]
    DUTY_6_8,
    #[doc = "6: duty 7/8"]
    DUTY_7_8,
}
impl From<CHPDUTY_A> for u8 {
    #[inline(always)]
    fn from(variant: CHPDUTY_A) -> Self {
        match variant {
            CHPDUTY_A::DUTY_1_8 => 0,
            CHPDUTY_A::DUTY_2_8 => 1,
            CHPDUTY_A::DUTY_3_8 => 2,
            CHPDUTY_A::DUTY_4_8 => 3,
            CHPDUTY_A::DUTY_5_8 => 4,
            CHPDUTY_A::DUTY_6_8 => 5,
            CHPDUTY_A::DUTY_7_8 => 6,
        }
    }
}
#[doc = "Reader of field `CHPDUTY`"]
pub type CHPDUTY_R = crate::R<u8, CHPDUTY_A>;
impl CHPDUTY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CHPDUTY_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CHPDUTY_A::DUTY_1_8),
            1 => Val(CHPDUTY_A::DUTY_2_8),
            2 => Val(CHPDUTY_A::DUTY_3_8),
            3 => Val(CHPDUTY_A::DUTY_4_8),
            4 => Val(CHPDUTY_A::DUTY_5_8),
            5 => Val(CHPDUTY_A::DUTY_6_8),
            6 => Val(CHPDUTY_A::DUTY_7_8),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DUTY_1_8`"]
    #[inline(always)]
    pub fn is_duty_1_8(&self) -> bool {
        *self == CHPDUTY_A::DUTY_1_8
    }
    #[doc = "Checks if the value of the field is `DUTY_2_8`"]
    #[inline(always)]
    pub fn is_duty_2_8(&self) -> bool {
        *self == CHPDUTY_A::DUTY_2_8
    }
    #[doc = "Checks if the value of the field is `DUTY_3_8`"]
    #[inline(always)]
    pub fn is_duty_3_8(&self) -> bool {
        *self == CHPDUTY_A::DUTY_3_8
    }
    #[doc = "Checks if the value of the field is `DUTY_4_8`"]
    #[inline(always)]
    pub fn is_duty_4_8(&self) -> bool {
        *self == CHPDUTY_A::DUTY_4_8
    }
    #[doc = "Checks if the value of the field is `DUTY_5_8`"]
    #[inline(always)]
    pub fn is_duty_5_8(&self) -> bool {
        *self == CHPDUTY_A::DUTY_5_8
    }
    #[doc = "Checks if the value of the field is `DUTY_6_8`"]
    #[inline(always)]
    pub fn is_duty_6_8(&self) -> bool {
        *self == CHPDUTY_A::DUTY_6_8
    }
    #[doc = "Checks if the value of the field is `DUTY_7_8`"]
    #[inline(always)]
    pub fn is_duty_7_8(&self) -> bool {
        *self == CHPDUTY_A::DUTY_7_8
    }
}
#[doc = "Write proxy for field `CHPDUTY`"]
pub struct CHPDUTY_W<'a> {
    w: &'a mut W,
}
impl<'a> CHPDUTY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHPDUTY_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "duty 1/8"]
    #[inline(always)]
    pub fn duty_1_8(self) -> &'a mut W {
        self.variant(CHPDUTY_A::DUTY_1_8)
    }
    #[doc = "duty 2/8"]
    #[inline(always)]
    pub fn duty_2_8(self) -> &'a mut W {
        self.variant(CHPDUTY_A::DUTY_2_8)
    }
    #[doc = "duty 3/8"]
    #[inline(always)]
    pub fn duty_3_8(self) -> &'a mut W {
        self.variant(CHPDUTY_A::DUTY_3_8)
    }
    #[doc = "duty 4/8"]
    #[inline(always)]
    pub fn duty_4_8(self) -> &'a mut W {
        self.variant(CHPDUTY_A::DUTY_4_8)
    }
    #[doc = "duty 5/8"]
    #[inline(always)]
    pub fn duty_5_8(self) -> &'a mut W {
        self.variant(CHPDUTY_A::DUTY_5_8)
    }
    #[doc = "duty 6/8"]
    #[inline(always)]
    pub fn duty_6_8(self) -> &'a mut W {
        self.variant(CHPDUTY_A::DUTY_6_8)
    }
    #[doc = "duty 7/8"]
    #[inline(always)]
    pub fn duty_7_8(self) -> &'a mut W {
        self.variant(CHPDUTY_A::DUTY_7_8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - enables the Modulator"]
    #[inline(always)]
    pub fn chpen(&self) -> CHPEN_R {
        CHPEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:4"]
    #[inline(always)]
    pub fn oshtwth(&self) -> OSHTWTH_R {
        OSHTWTH_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bits 5:7"]
    #[inline(always)]
    pub fn shrfreq(&self) -> SHRFREQ_R {
        SHRFREQ_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn chpduty(&self) -> CHPDUTY_R {
        CHPDUTY_R::new(((self.bits >> 8) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - enables the Modulator"]
    #[inline(always)]
    pub fn chpen(&mut self) -> CHPEN_W {
        CHPEN_W { w: self }
    }
    #[doc = "Bits 1:4"]
    #[inline(always)]
    pub fn oshtwth(&mut self) -> OSHTWTH_W {
        OSHTWTH_W { w: self }
    }
    #[doc = "Bits 5:7"]
    #[inline(always)]
    pub fn shrfreq(&mut self) -> SHRFREQ_W {
        SHRFREQ_W { w: self }
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn chpduty(&mut self) -> CHPDUTY_W {
        CHPDUTY_W { w: self }
    }
}
