#[doc = "Reader of register AQSFRC"]
pub type R = crate::R<u32, super::AQSFRC>;
#[doc = "Writer for register AQSFRC"]
pub type W = crate::W<u32, super::AQSFRC>;
#[doc = "Register AQSFRC `reset()`'s with value 0"]
impl crate::ResetValue for super::AQSFRC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACTSFA_A {
    #[doc = "0: no action"]
    NOACTION,
    #[doc = "1: clear PWMA/PWMB"]
    CLEAR,
    #[doc = "2: set PWMA/PWMB"]
    SET,
    #[doc = "3: inverse PWMA/PWMB"]
    TOOGLE,
}
impl From<ACTSFA_A> for u8 {
    #[inline(always)]
    fn from(variant: ACTSFA_A) -> Self {
        match variant {
            ACTSFA_A::NOACTION => 0,
            ACTSFA_A::CLEAR => 1,
            ACTSFA_A::SET => 2,
            ACTSFA_A::TOOGLE => 3,
        }
    }
}
#[doc = "Reader of field `ACTSFA`"]
pub type ACTSFA_R = crate::R<u8, ACTSFA_A>;
impl ACTSFA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACTSFA_A {
        match self.bits {
            0 => ACTSFA_A::NOACTION,
            1 => ACTSFA_A::CLEAR,
            2 => ACTSFA_A::SET,
            3 => ACTSFA_A::TOOGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NOACTION`"]
    #[inline(always)]
    pub fn is_no_action(&self) -> bool {
        *self == ACTSFA_A::NOACTION
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == ACTSFA_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == ACTSFA_A::SET
    }
    #[doc = "Checks if the value of the field is `TOOGLE`"]
    #[inline(always)]
    pub fn is_toogle(&self) -> bool {
        *self == ACTSFA_A::TOOGLE
    }
}
#[doc = "Write proxy for field `ACTSFA`"]
pub struct ACTSFA_W<'a> {
    w: &'a mut W,
}
impl<'a> ACTSFA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACTSFA_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "no action"]
    #[inline(always)]
    pub fn no_action(self) -> &'a mut W {
        self.variant(ACTSFA_A::NOACTION)
    }
    #[doc = "clear PWMA/PWMB"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ACTSFA_A::CLEAR)
    }
    #[doc = "set PWMA/PWMB"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(ACTSFA_A::SET)
    }
    #[doc = "inverse PWMA/PWMB"]
    #[inline(always)]
    pub fn toogle(self) -> &'a mut W {
        self.variant(ACTSFA_A::TOOGLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `OTSFA`"]
pub type OTSFA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OTSFA`"]
pub struct OTSFA_W<'a> {
    w: &'a mut W,
}
impl<'a> OTSFA_W<'a> {
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
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACTSFB_A {
    #[doc = "0: no action"]
    NOACTION,
    #[doc = "1: clear PWMA/PWMB"]
    CLEAR,
    #[doc = "2: set PWMA/PWMB"]
    SET,
    #[doc = "3: inverse PWMA/PWMB"]
    TOOGLE,
}
impl From<ACTSFB_A> for u8 {
    #[inline(always)]
    fn from(variant: ACTSFB_A) -> Self {
        match variant {
            ACTSFB_A::NOACTION => 0,
            ACTSFB_A::CLEAR => 1,
            ACTSFB_A::SET => 2,
            ACTSFB_A::TOOGLE => 3,
        }
    }
}
#[doc = "Reader of field `ACTSFB`"]
pub type ACTSFB_R = crate::R<u8, ACTSFB_A>;
impl ACTSFB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACTSFB_A {
        match self.bits {
            0 => ACTSFB_A::NOACTION,
            1 => ACTSFB_A::CLEAR,
            2 => ACTSFB_A::SET,
            3 => ACTSFB_A::TOOGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NOACTION`"]
    #[inline(always)]
    pub fn is_no_action(&self) -> bool {
        *self == ACTSFB_A::NOACTION
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == ACTSFB_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == ACTSFB_A::SET
    }
    #[doc = "Checks if the value of the field is `TOOGLE`"]
    #[inline(always)]
    pub fn is_toogle(&self) -> bool {
        *self == ACTSFB_A::TOOGLE
    }
}
#[doc = "Write proxy for field `ACTSFB`"]
pub struct ACTSFB_W<'a> {
    w: &'a mut W,
}
impl<'a> ACTSFB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACTSFB_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "no action"]
    #[inline(always)]
    pub fn no_action(self) -> &'a mut W {
        self.variant(ACTSFB_A::NOACTION)
    }
    #[doc = "clear PWMA/PWMB"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ACTSFB_A::CLEAR)
    }
    #[doc = "set PWMA/PWMB"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(ACTSFB_A::SET)
    }
    #[doc = "inverse PWMA/PWMB"]
    #[inline(always)]
    pub fn toogle(self) -> &'a mut W {
        self.variant(ACTSFB_A::TOOGLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u32) & 0x03) << 3);
        self.w
    }
}
#[doc = "Reader of field `OTSFB`"]
pub type OTSFB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OTSFB`"]
pub struct OTSFB_W<'a> {
    w: &'a mut W,
}
impl<'a> OTSFB_W<'a> {
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
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RLDCSF_A {
    #[doc = "0: load when CTR = 0"]
    CTREQZERO,
    #[doc = "1: load when CTR = PRD"]
    CTREQPRD,
    #[doc = "2: load when CTR = 0 or CTR = PRD"]
    CTREQZEROPRD,
    #[doc = "3: load immediatelly"]
    NOSHADOW,
}
impl From<RLDCSF_A> for u8 {
    #[inline(always)]
    fn from(variant: RLDCSF_A) -> Self {
        match variant {
            RLDCSF_A::CTREQZERO => 0,
            RLDCSF_A::CTREQPRD => 1,
            RLDCSF_A::CTREQZEROPRD => 2,
            RLDCSF_A::NOSHADOW => 3,
        }
    }
}
#[doc = "Reader of field `RLDCSF`"]
pub type RLDCSF_R = crate::R<u8, RLDCSF_A>;
impl RLDCSF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RLDCSF_A {
        match self.bits {
            0 => RLDCSF_A::CTREQZERO,
            1 => RLDCSF_A::CTREQPRD,
            2 => RLDCSF_A::CTREQZEROPRD,
            3 => RLDCSF_A::NOSHADOW,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CTREQZERO`"]
    #[inline(always)]
    pub fn is_ctreq_zero(&self) -> bool {
        *self == RLDCSF_A::CTREQZERO
    }
    #[doc = "Checks if the value of the field is `CTREQPRD`"]
    #[inline(always)]
    pub fn is_ctreq_prd(&self) -> bool {
        *self == RLDCSF_A::CTREQPRD
    }
    #[doc = "Checks if the value of the field is `CTREQZEROPRD`"]
    #[inline(always)]
    pub fn is_ctreq_zero_prd(&self) -> bool {
        *self == RLDCSF_A::CTREQZEROPRD
    }
    #[doc = "Checks if the value of the field is `NOSHADOW`"]
    #[inline(always)]
    pub fn is_no_shadow(&self) -> bool {
        *self == RLDCSF_A::NOSHADOW
    }
}
#[doc = "Write proxy for field `RLDCSF`"]
pub struct RLDCSF_W<'a> {
    w: &'a mut W,
}
impl<'a> RLDCSF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RLDCSF_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "load when CTR = 0"]
    #[inline(always)]
    pub fn ctreq_zero(self) -> &'a mut W {
        self.variant(RLDCSF_A::CTREQZERO)
    }
    #[doc = "load when CTR = PRD"]
    #[inline(always)]
    pub fn ctreq_prd(self) -> &'a mut W {
        self.variant(RLDCSF_A::CTREQPRD)
    }
    #[doc = "load when CTR = 0 or CTR = PRD"]
    #[inline(always)]
    pub fn ctreq_zero_prd(self) -> &'a mut W {
        self.variant(RLDCSF_A::CTREQZEROPRD)
    }
    #[doc = "load immediatelly"]
    #[inline(always)]
    pub fn no_shadow(self) -> &'a mut W {
        self.variant(RLDCSF_A::NOSHADOW)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn actsfa(&self) -> ACTSFA_R {
        ACTSFA_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - Initialization single pulse"]
    #[inline(always)]
    pub fn otsfa(&self) -> OTSFA_R {
        OTSFA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn actsfb(&self) -> ACTSFB_R {
        ACTSFB_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bit 5 - Initialization single pulse"]
    #[inline(always)]
    pub fn otsfb(&self) -> OTSFB_R {
        OTSFB_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn rldcsf(&self) -> RLDCSF_R {
        RLDCSF_R::new(((self.bits >> 6) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn actsfa(&mut self) -> ACTSFA_W {
        ACTSFA_W { w: self }
    }
    #[doc = "Bit 2 - Initialization single pulse"]
    #[inline(always)]
    pub fn otsfa(&mut self) -> OTSFA_W {
        OTSFA_W { w: self }
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn actsfb(&mut self) -> ACTSFB_W {
        ACTSFB_W { w: self }
    }
    #[doc = "Bit 5 - Initialization single pulse"]
    #[inline(always)]
    pub fn otsfb(&mut self) -> OTSFB_W {
        OTSFB_W { w: self }
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn rldcsf(&mut self) -> RLDCSF_W {
        RLDCSF_W { w: self }
    }
}
