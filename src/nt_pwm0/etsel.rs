#[doc = "Reader of register ETSEL"]
pub type R = crate::R<u32, super::ETSEL>;
#[doc = "Writer for register ETSEL"]
pub type W = crate::W<u32, super::ETSEL>;
#[doc = "Register ETSEL `reset()`'s with value 0"]
impl crate::ResetValue for super::ETSEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTSEL_A {
    #[doc = "1:  generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = 0"]
    CTREQZERO,
    #[doc = "2:  generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = PRD"]
    CTREQPRD,
    #[doc = "4:  generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPA when count up"]
    CTREQCMPA_ONUP,
    #[doc = "5:  generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPA when count down"]
    CTREQCMPA_ONDOWN,
    #[doc = "6:  generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPB when count up"]
    CTREQCMPB_ONUP,
    #[doc = "7:  generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPB when count down"]
    CTREQCMPB_ONDOWN,
}
impl From<INTSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: INTSEL_A) -> Self {
        match variant {
            INTSEL_A::CTREQZERO => 1,
            INTSEL_A::CTREQPRD => 2,
            INTSEL_A::CTREQCMPA_ONUP => 4,
            INTSEL_A::CTREQCMPA_ONDOWN => 5,
            INTSEL_A::CTREQCMPB_ONUP => 6,
            INTSEL_A::CTREQCMPB_ONDOWN => 7,
        }
    }
}
#[doc = "Reader of field `INTSEL`"]
pub type INTSEL_R = crate::R<u8, INTSEL_A>;
impl INTSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, INTSEL_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(INTSEL_A::CTREQZERO),
            2 => Val(INTSEL_A::CTREQPRD),
            4 => Val(INTSEL_A::CTREQCMPA_ONUP),
            5 => Val(INTSEL_A::CTREQCMPA_ONDOWN),
            6 => Val(INTSEL_A::CTREQCMPB_ONUP),
            7 => Val(INTSEL_A::CTREQCMPB_ONDOWN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CTREQZERO`"]
    #[inline(always)]
    pub fn is_ctreq_zero(&self) -> bool {
        *self == INTSEL_A::CTREQZERO
    }
    #[doc = "Checks if the value of the field is `CTREQPRD`"]
    #[inline(always)]
    pub fn is_ctreq_prd(&self) -> bool {
        *self == INTSEL_A::CTREQPRD
    }
    #[doc = "Checks if the value of the field is `CTREQCMPA_ONUP`"]
    #[inline(always)]
    pub fn is_ctreq_cmpa_on_up(&self) -> bool {
        *self == INTSEL_A::CTREQCMPA_ONUP
    }
    #[doc = "Checks if the value of the field is `CTREQCMPA_ONDOWN`"]
    #[inline(always)]
    pub fn is_ctreq_cmpa_on_down(&self) -> bool {
        *self == INTSEL_A::CTREQCMPA_ONDOWN
    }
    #[doc = "Checks if the value of the field is `CTREQCMPB_ONUP`"]
    #[inline(always)]
    pub fn is_ctreq_cmpb_on_up(&self) -> bool {
        *self == INTSEL_A::CTREQCMPB_ONUP
    }
    #[doc = "Checks if the value of the field is `CTREQCMPB_ONDOWN`"]
    #[inline(always)]
    pub fn is_ctreq_cmpb_on_down(&self) -> bool {
        *self == INTSEL_A::CTREQCMPB_ONDOWN
    }
}
#[doc = "Write proxy for field `INTSEL`"]
pub struct INTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> INTSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = 0"]
    #[inline(always)]
    pub fn ctreq_zero(self) -> &'a mut W {
        self.variant(INTSEL_A::CTREQZERO)
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = PRD"]
    #[inline(always)]
    pub fn ctreq_prd(self) -> &'a mut W {
        self.variant(INTSEL_A::CTREQPRD)
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPA when count up"]
    #[inline(always)]
    pub fn ctreq_cmpa_on_up(self) -> &'a mut W {
        self.variant(INTSEL_A::CTREQCMPA_ONUP)
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPA when count down"]
    #[inline(always)]
    pub fn ctreq_cmpa_on_down(self) -> &'a mut W {
        self.variant(INTSEL_A::CTREQCMPA_ONDOWN)
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPB when count up"]
    #[inline(always)]
    pub fn ctreq_cmpb_on_up(self) -> &'a mut W {
        self.variant(INTSEL_A::CTREQCMPB_ONUP)
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPB when count down"]
    #[inline(always)]
    pub fn ctreq_cmpb_on_down(self) -> &'a mut W {
        self.variant(INTSEL_A::CTREQCMPB_ONDOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `INTEN`"]
pub type INTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTEN`"]
pub struct INTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> INTEN_W<'a> {
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
pub enum SOCASEL_A {
    #[doc = "1:  generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = 0"]
    CTREQZERO,
    #[doc = "2:  generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = PRD"]
    CTREQPRD,
    #[doc = "4:  generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPA when count up"]
    CTREQCMPA_ONUP,
    #[doc = "5:  generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPA when count down"]
    CTREQCMPA_ONDOWN,
    #[doc = "6:  generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPB when count up"]
    CTREQCMPB_ONUP,
    #[doc = "7:  generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPB when count down"]
    CTREQCMPB_ONDOWN,
}
impl From<SOCASEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SOCASEL_A) -> Self {
        match variant {
            SOCASEL_A::CTREQZERO => 1,
            SOCASEL_A::CTREQPRD => 2,
            SOCASEL_A::CTREQCMPA_ONUP => 4,
            SOCASEL_A::CTREQCMPA_ONDOWN => 5,
            SOCASEL_A::CTREQCMPB_ONUP => 6,
            SOCASEL_A::CTREQCMPB_ONDOWN => 7,
        }
    }
}
#[doc = "Reader of field `SOCASEL`"]
pub type SOCASEL_R = crate::R<u8, SOCASEL_A>;
impl SOCASEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SOCASEL_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(SOCASEL_A::CTREQZERO),
            2 => Val(SOCASEL_A::CTREQPRD),
            4 => Val(SOCASEL_A::CTREQCMPA_ONUP),
            5 => Val(SOCASEL_A::CTREQCMPA_ONDOWN),
            6 => Val(SOCASEL_A::CTREQCMPB_ONUP),
            7 => Val(SOCASEL_A::CTREQCMPB_ONDOWN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CTREQZERO`"]
    #[inline(always)]
    pub fn is_ctreq_zero(&self) -> bool {
        *self == SOCASEL_A::CTREQZERO
    }
    #[doc = "Checks if the value of the field is `CTREQPRD`"]
    #[inline(always)]
    pub fn is_ctreq_prd(&self) -> bool {
        *self == SOCASEL_A::CTREQPRD
    }
    #[doc = "Checks if the value of the field is `CTREQCMPA_ONUP`"]
    #[inline(always)]
    pub fn is_ctreq_cmpa_on_up(&self) -> bool {
        *self == SOCASEL_A::CTREQCMPA_ONUP
    }
    #[doc = "Checks if the value of the field is `CTREQCMPA_ONDOWN`"]
    #[inline(always)]
    pub fn is_ctreq_cmpa_on_down(&self) -> bool {
        *self == SOCASEL_A::CTREQCMPA_ONDOWN
    }
    #[doc = "Checks if the value of the field is `CTREQCMPB_ONUP`"]
    #[inline(always)]
    pub fn is_ctreq_cmpb_on_up(&self) -> bool {
        *self == SOCASEL_A::CTREQCMPB_ONUP
    }
    #[doc = "Checks if the value of the field is `CTREQCMPB_ONDOWN`"]
    #[inline(always)]
    pub fn is_ctreq_cmpb_on_down(&self) -> bool {
        *self == SOCASEL_A::CTREQCMPB_ONDOWN
    }
}
#[doc = "Write proxy for field `SOCASEL`"]
pub struct SOCASEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SOCASEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SOCASEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = 0"]
    #[inline(always)]
    pub fn ctreq_zero(self) -> &'a mut W {
        self.variant(SOCASEL_A::CTREQZERO)
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = PRD"]
    #[inline(always)]
    pub fn ctreq_prd(self) -> &'a mut W {
        self.variant(SOCASEL_A::CTREQPRD)
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPA when count up"]
    #[inline(always)]
    pub fn ctreq_cmpa_on_up(self) -> &'a mut W {
        self.variant(SOCASEL_A::CTREQCMPA_ONUP)
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPA when count down"]
    #[inline(always)]
    pub fn ctreq_cmpa_on_down(self) -> &'a mut W {
        self.variant(SOCASEL_A::CTREQCMPA_ONDOWN)
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPB when count up"]
    #[inline(always)]
    pub fn ctreq_cmpb_on_up(self) -> &'a mut W {
        self.variant(SOCASEL_A::CTREQCMPB_ONUP)
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPB when count down"]
    #[inline(always)]
    pub fn ctreq_cmpb_on_down(self) -> &'a mut W {
        self.variant(SOCASEL_A::CTREQCMPB_ONDOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `SOCAEN`"]
pub type SOCAEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOCAEN`"]
pub struct SOCAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SOCAEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOCBSEL_A {
    #[doc = "1:  generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = 0"]
    CTREQZERO,
    #[doc = "2:  generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = PRD"]
    CTREQPRD,
    #[doc = "4:  generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPA when count up"]
    CTREQCMPA_ONUP,
    #[doc = "5:  generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPA when count down"]
    CTREQCMPA_ONDOWN,
    #[doc = "6:  generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPB when count up"]
    CTREQCMPB_ONUP,
    #[doc = "7:  generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPB when count down"]
    CTREQCMPB_ONDOWN,
}
impl From<SOCBSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SOCBSEL_A) -> Self {
        match variant {
            SOCBSEL_A::CTREQZERO => 1,
            SOCBSEL_A::CTREQPRD => 2,
            SOCBSEL_A::CTREQCMPA_ONUP => 4,
            SOCBSEL_A::CTREQCMPA_ONDOWN => 5,
            SOCBSEL_A::CTREQCMPB_ONUP => 6,
            SOCBSEL_A::CTREQCMPB_ONDOWN => 7,
        }
    }
}
#[doc = "Reader of field `SOCBSEL`"]
pub type SOCBSEL_R = crate::R<u8, SOCBSEL_A>;
impl SOCBSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SOCBSEL_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(SOCBSEL_A::CTREQZERO),
            2 => Val(SOCBSEL_A::CTREQPRD),
            4 => Val(SOCBSEL_A::CTREQCMPA_ONUP),
            5 => Val(SOCBSEL_A::CTREQCMPA_ONDOWN),
            6 => Val(SOCBSEL_A::CTREQCMPB_ONUP),
            7 => Val(SOCBSEL_A::CTREQCMPB_ONDOWN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CTREQZERO`"]
    #[inline(always)]
    pub fn is_ctreq_zero(&self) -> bool {
        *self == SOCBSEL_A::CTREQZERO
    }
    #[doc = "Checks if the value of the field is `CTREQPRD`"]
    #[inline(always)]
    pub fn is_ctreq_prd(&self) -> bool {
        *self == SOCBSEL_A::CTREQPRD
    }
    #[doc = "Checks if the value of the field is `CTREQCMPA_ONUP`"]
    #[inline(always)]
    pub fn is_ctreq_cmpa_on_up(&self) -> bool {
        *self == SOCBSEL_A::CTREQCMPA_ONUP
    }
    #[doc = "Checks if the value of the field is `CTREQCMPA_ONDOWN`"]
    #[inline(always)]
    pub fn is_ctreq_cmpa_on_down(&self) -> bool {
        *self == SOCBSEL_A::CTREQCMPA_ONDOWN
    }
    #[doc = "Checks if the value of the field is `CTREQCMPB_ONUP`"]
    #[inline(always)]
    pub fn is_ctreq_cmpb_on_up(&self) -> bool {
        *self == SOCBSEL_A::CTREQCMPB_ONUP
    }
    #[doc = "Checks if the value of the field is `CTREQCMPB_ONDOWN`"]
    #[inline(always)]
    pub fn is_ctreq_cmpb_on_down(&self) -> bool {
        *self == SOCBSEL_A::CTREQCMPB_ONDOWN
    }
}
#[doc = "Write proxy for field `SOCBSEL`"]
pub struct SOCBSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SOCBSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SOCBSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = 0"]
    #[inline(always)]
    pub fn ctreq_zero(self) -> &'a mut W {
        self.variant(SOCBSEL_A::CTREQZERO)
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = PRD"]
    #[inline(always)]
    pub fn ctreq_prd(self) -> &'a mut W {
        self.variant(SOCBSEL_A::CTREQPRD)
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPA when count up"]
    #[inline(always)]
    pub fn ctreq_cmpa_on_up(self) -> &'a mut W {
        self.variant(SOCBSEL_A::CTREQCMPA_ONUP)
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPA when count down"]
    #[inline(always)]
    pub fn ctreq_cmpa_on_down(self) -> &'a mut W {
        self.variant(SOCBSEL_A::CTREQCMPA_ONDOWN)
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPB when count up"]
    #[inline(always)]
    pub fn ctreq_cmpb_on_up(self) -> &'a mut W {
        self.variant(SOCBSEL_A::CTREQCMPB_ONUP)
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPB when count down"]
    #[inline(always)]
    pub fn ctreq_cmpb_on_down(self) -> &'a mut W {
        self.variant(SOCBSEL_A::CTREQCMPB_ONDOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "Reader of field `SOCBEN`"]
pub type SOCBEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOCBEN`"]
pub struct SOCBEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SOCBEN_W<'a> {
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
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn intsel(&self) -> INTSEL_R {
        INTSEL_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - Generation of external interrupt EPWMxINT"]
    #[inline(always)]
    pub fn inten(&self) -> INTEN_R {
        INTEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn socasel(&self) -> SOCASEL_R {
        SOCASEL_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 11 - generation of an external signal (EPWMxSOCA)"]
    #[inline(always)]
    pub fn socaen(&self) -> SOCAEN_R {
        SOCAEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn socbsel(&self) -> SOCBSEL_R {
        SOCBSEL_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bit 15 - generation of an external signal (EPWMxSOCB)"]
    #[inline(always)]
    pub fn socben(&self) -> SOCBEN_R {
        SOCBEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn intsel(&mut self) -> INTSEL_W {
        INTSEL_W { w: self }
    }
    #[doc = "Bit 3 - Generation of external interrupt EPWMxINT"]
    #[inline(always)]
    pub fn inten(&mut self) -> INTEN_W {
        INTEN_W { w: self }
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn socasel(&mut self) -> SOCASEL_W {
        SOCASEL_W { w: self }
    }
    #[doc = "Bit 11 - generation of an external signal (EPWMxSOCA)"]
    #[inline(always)]
    pub fn socaen(&mut self) -> SOCAEN_W {
        SOCAEN_W { w: self }
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn socbsel(&mut self) -> SOCBSEL_W {
        SOCBSEL_W { w: self }
    }
    #[doc = "Bit 15 - generation of an external signal (EPWMxSOCB)"]
    #[inline(always)]
    pub fn socben(&mut self) -> SOCBEN_W {
        SOCBEN_W { w: self }
    }
}
