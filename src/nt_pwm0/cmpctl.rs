#[doc = "Reader of register CMPCTL"]
pub type R = crate::R<u32, super::CMPCTL>;
#[doc = "Writer for register CMPCTL"]
pub type W = crate::W<u32, super::CMPCTL>;
#[doc = "Register CMPCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::CMPCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOADAMODE_A {
    #[doc = "0: shadow load for CMPx (x=A,B) when CTR = 0"]
    CTREQZERO,
    #[doc = "1: shadow load for CMPx (x=A,B) when CTR = PRD"]
    CTREQPRD,
    #[doc = "2: shadow load for CMPx (x=A,B) when CTR = 0 or CTR = PRD"]
    CTREQZEROPRD,
    #[doc = "3: shadow load for CMPx (x=A,B) disabled"]
    DISABLE,
}
impl From<LOADAMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: LOADAMODE_A) -> Self {
        match variant {
            LOADAMODE_A::CTREQZERO => 0,
            LOADAMODE_A::CTREQPRD => 1,
            LOADAMODE_A::CTREQZEROPRD => 2,
            LOADAMODE_A::DISABLE => 3,
        }
    }
}
#[doc = "Reader of field `LOADAMODE`"]
pub type LOADAMODE_R = crate::R<u8, LOADAMODE_A>;
impl LOADAMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOADAMODE_A {
        match self.bits {
            0 => LOADAMODE_A::CTREQZERO,
            1 => LOADAMODE_A::CTREQPRD,
            2 => LOADAMODE_A::CTREQZEROPRD,
            3 => LOADAMODE_A::DISABLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CTREQZERO`"]
    #[inline(always)]
    pub fn is_ctreq_zero(&self) -> bool {
        *self == LOADAMODE_A::CTREQZERO
    }
    #[doc = "Checks if the value of the field is `CTREQPRD`"]
    #[inline(always)]
    pub fn is_ctreq_prd(&self) -> bool {
        *self == LOADAMODE_A::CTREQPRD
    }
    #[doc = "Checks if the value of the field is `CTREQZEROPRD`"]
    #[inline(always)]
    pub fn is_ctreq_zero_prd(&self) -> bool {
        *self == LOADAMODE_A::CTREQZEROPRD
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == LOADAMODE_A::DISABLE
    }
}
#[doc = "Write proxy for field `LOADAMODE`"]
pub struct LOADAMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> LOADAMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOADAMODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "shadow load for CMPx (x=A,B) when CTR = 0"]
    #[inline(always)]
    pub fn ctreq_zero(self) -> &'a mut W {
        self.variant(LOADAMODE_A::CTREQZERO)
    }
    #[doc = "shadow load for CMPx (x=A,B) when CTR = PRD"]
    #[inline(always)]
    pub fn ctreq_prd(self) -> &'a mut W {
        self.variant(LOADAMODE_A::CTREQPRD)
    }
    #[doc = "shadow load for CMPx (x=A,B) when CTR = 0 or CTR = PRD"]
    #[inline(always)]
    pub fn ctreq_zero_prd(self) -> &'a mut W {
        self.variant(LOADAMODE_A::CTREQZEROPRD)
    }
    #[doc = "shadow load for CMPx (x=A,B) disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(LOADAMODE_A::DISABLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOADBMODE_A {
    #[doc = "0: shadow load for CMPx (x=A,B) when CTR = 0"]
    CTREQZERO,
    #[doc = "1: shadow load for CMPx (x=A,B) when CTR = PRD"]
    CTREQPRD,
    #[doc = "2: shadow load for CMPx (x=A,B) when CTR = 0 or CTR = PRD"]
    CTREQZEROPRD,
    #[doc = "3: shadow load for CMPx (x=A,B) disabled"]
    DISABLE,
}
impl From<LOADBMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: LOADBMODE_A) -> Self {
        match variant {
            LOADBMODE_A::CTREQZERO => 0,
            LOADBMODE_A::CTREQPRD => 1,
            LOADBMODE_A::CTREQZEROPRD => 2,
            LOADBMODE_A::DISABLE => 3,
        }
    }
}
#[doc = "Reader of field `LOADBMODE`"]
pub type LOADBMODE_R = crate::R<u8, LOADBMODE_A>;
impl LOADBMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOADBMODE_A {
        match self.bits {
            0 => LOADBMODE_A::CTREQZERO,
            1 => LOADBMODE_A::CTREQPRD,
            2 => LOADBMODE_A::CTREQZEROPRD,
            3 => LOADBMODE_A::DISABLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CTREQZERO`"]
    #[inline(always)]
    pub fn is_ctreq_zero(&self) -> bool {
        *self == LOADBMODE_A::CTREQZERO
    }
    #[doc = "Checks if the value of the field is `CTREQPRD`"]
    #[inline(always)]
    pub fn is_ctreq_prd(&self) -> bool {
        *self == LOADBMODE_A::CTREQPRD
    }
    #[doc = "Checks if the value of the field is `CTREQZEROPRD`"]
    #[inline(always)]
    pub fn is_ctreq_zero_prd(&self) -> bool {
        *self == LOADBMODE_A::CTREQZEROPRD
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == LOADBMODE_A::DISABLE
    }
}
#[doc = "Write proxy for field `LOADBMODE`"]
pub struct LOADBMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> LOADBMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOADBMODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "shadow load for CMPx (x=A,B) when CTR = 0"]
    #[inline(always)]
    pub fn ctreq_zero(self) -> &'a mut W {
        self.variant(LOADBMODE_A::CTREQZERO)
    }
    #[doc = "shadow load for CMPx (x=A,B) when CTR = PRD"]
    #[inline(always)]
    pub fn ctreq_prd(self) -> &'a mut W {
        self.variant(LOADBMODE_A::CTREQPRD)
    }
    #[doc = "shadow load for CMPx (x=A,B) when CTR = 0 or CTR = PRD"]
    #[inline(always)]
    pub fn ctreq_zero_prd(self) -> &'a mut W {
        self.variant(LOADBMODE_A::CTREQZEROPRD)
    }
    #[doc = "shadow load for CMPx (x=A,B) disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(LOADBMODE_A::DISABLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `SHDWAMODE`"]
pub type SHDWAMODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SHDWAMODE`"]
pub struct SHDWAMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SHDWAMODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `SHDWBMODE`"]
pub type SHDWBMODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SHDWBMODE`"]
pub struct SHDWBMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SHDWBMODE_W<'a> {
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
#[doc = "Reader of field `SHDWAFULL`"]
pub type SHDWAFULL_R = crate::R<bool, bool>;
#[doc = "Reader of field `SHDWBFULL`"]
pub type SHDWBFULL_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn loadamode(&self) -> LOADAMODE_R {
        LOADAMODE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn loadbmode(&self) -> LOADBMODE_R {
        LOADBMODE_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 4 - Write-permit mode"]
    #[inline(always)]
    pub fn shdwamode(&self) -> SHDWAMODE_R {
        SHDWAMODE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Write-permit mode"]
    #[inline(always)]
    pub fn shdwbmode(&self) -> SHDWBMODE_R {
        SHDWBMODE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Status lazy loading in CMPA"]
    #[inline(always)]
    pub fn shdwafull(&self) -> SHDWAFULL_R {
        SHDWAFULL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Status lazy loading in CMPB"]
    #[inline(always)]
    pub fn shdwbfull(&self) -> SHDWBFULL_R {
        SHDWBFULL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn loadamode(&mut self) -> LOADAMODE_W {
        LOADAMODE_W { w: self }
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn loadbmode(&mut self) -> LOADBMODE_W {
        LOADBMODE_W { w: self }
    }
    #[doc = "Bit 4 - Write-permit mode"]
    #[inline(always)]
    pub fn shdwamode(&mut self) -> SHDWAMODE_W {
        SHDWAMODE_W { w: self }
    }
    #[doc = "Bit 6 - Write-permit mode"]
    #[inline(always)]
    pub fn shdwbmode(&mut self) -> SHDWBMODE_W {
        SHDWBMODE_W { w: self }
    }
}
