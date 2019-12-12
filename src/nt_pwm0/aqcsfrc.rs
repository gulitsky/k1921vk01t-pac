#[doc = "Reader of register AQCSFRC"]
pub type R = crate::R<u32, super::AQCSFRC>;
#[doc = "Writer for register AQCSFRC"]
pub type W = crate::W<u32, super::AQCSFRC>;
#[doc = "Register AQCSFRC `reset()`'s with value 0"]
impl crate::ResetValue for super::AQCSFRC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSFA_A {
    #[doc = "0: no action"]
    NOACTION,
    #[doc = "1: clear PWMA/PWMB"]
    CLEAR,
    #[doc = "2: set PWMA/PWMB"]
    SET,
}
impl From<CSFA_A> for u8 {
    #[inline(always)]
    fn from(variant: CSFA_A) -> Self {
        match variant {
            CSFA_A::NOACTION => 0,
            CSFA_A::CLEAR => 1,
            CSFA_A::SET => 2,
        }
    }
}
#[doc = "Reader of field `CSFA`"]
pub type CSFA_R = crate::R<u8, CSFA_A>;
impl CSFA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CSFA_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CSFA_A::NOACTION),
            1 => Val(CSFA_A::CLEAR),
            2 => Val(CSFA_A::SET),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NOACTION`"]
    #[inline(always)]
    pub fn is_no_action(&self) -> bool {
        *self == CSFA_A::NOACTION
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CSFA_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == CSFA_A::SET
    }
}
#[doc = "Write proxy for field `CSFA`"]
pub struct CSFA_W<'a> {
    w: &'a mut W,
}
impl<'a> CSFA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSFA_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "no action"]
    #[inline(always)]
    pub fn no_action(self) -> &'a mut W {
        self.variant(CSFA_A::NOACTION)
    }
    #[doc = "clear PWMA/PWMB"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CSFA_A::CLEAR)
    }
    #[doc = "set PWMA/PWMB"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CSFA_A::SET)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSFB_A {
    #[doc = "0: no action"]
    NOACTION,
    #[doc = "1: clear PWMA/PWMB"]
    CLEAR,
    #[doc = "2: set PWMA/PWMB"]
    SET,
}
impl From<CSFB_A> for u8 {
    #[inline(always)]
    fn from(variant: CSFB_A) -> Self {
        match variant {
            CSFB_A::NOACTION => 0,
            CSFB_A::CLEAR => 1,
            CSFB_A::SET => 2,
        }
    }
}
#[doc = "Reader of field `CSFB`"]
pub type CSFB_R = crate::R<u8, CSFB_A>;
impl CSFB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CSFB_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CSFB_A::NOACTION),
            1 => Val(CSFB_A::CLEAR),
            2 => Val(CSFB_A::SET),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NOACTION`"]
    #[inline(always)]
    pub fn is_no_action(&self) -> bool {
        *self == CSFB_A::NOACTION
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CSFB_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == CSFB_A::SET
    }
}
#[doc = "Write proxy for field `CSFB`"]
pub struct CSFB_W<'a> {
    w: &'a mut W,
}
impl<'a> CSFB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSFB_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "no action"]
    #[inline(always)]
    pub fn no_action(self) -> &'a mut W {
        self.variant(CSFB_A::NOACTION)
    }
    #[doc = "clear PWMA/PWMB"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CSFB_A::CLEAR)
    }
    #[doc = "set PWMA/PWMB"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CSFB_A::SET)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn csfa(&self) -> CSFA_R {
        CSFA_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn csfb(&self) -> CSFB_R {
        CSFB_R::new(((self.bits >> 2) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn csfa(&mut self) -> CSFA_W {
        CSFA_W { w: self }
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn csfb(&mut self) -> CSFB_W {
        CSFB_W { w: self }
    }
}
