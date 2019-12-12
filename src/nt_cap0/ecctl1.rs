#[doc = "Reader of register ECCTL1"]
pub type R = crate::R<u32, super::ECCTL1>;
#[doc = "Writer for register ECCTL1"]
pub type W = crate::W<u32, super::ECCTL1>;
#[doc = "Register ECCTL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::ECCTL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CONT_ONESHT`"]
pub type CONT_ONESHT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CONT_ONESHT`"]
pub struct CONT_ONESHT_W<'a> {
    w: &'a mut W,
}
impl<'a> CONT_ONESHT_W<'a> {
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
#[doc = "Reader of field `STOP_WRAP`"]
pub type STOP_WRAP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `STOP_WRAP`"]
pub struct STOP_WRAP_W<'a> {
    w: &'a mut W,
}
impl<'a> STOP_WRAP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u32) & 0x03) << 1);
        self.w
    }
}
#[doc = "Reader of field `REARM`"]
pub type REARM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `REARM`"]
pub struct REARM_W<'a> {
    w: &'a mut W,
}
impl<'a> REARM_W<'a> {
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
#[doc = "Reader of field `TSCTRSTOP`"]
pub type TSCTRSTOP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSCTRSTOP`"]
pub struct TSCTRSTOP_W<'a> {
    w: &'a mut W,
}
impl<'a> TSCTRSTOP_W<'a> {
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
#[doc = "Reader of field `SYNCI_EN`"]
pub type SYNCI_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYNCI_EN`"]
pub struct SYNCI_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCI_EN_W<'a> {
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
pub enum SYNCO_SEL_A {
    #[doc = "0: sync in connected with sync out"]
    BYPASS,
    #[doc = "1:  sync out generated when CTR = PRD"]
    CTREQPRD,
    #[doc = "2:  sync out generate disabled"]
    DISABLE,
}
impl From<SYNCO_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SYNCO_SEL_A) -> Self {
        match variant {
            SYNCO_SEL_A::BYPASS => 0,
            SYNCO_SEL_A::CTREQPRD => 1,
            SYNCO_SEL_A::DISABLE => 2,
        }
    }
}
#[doc = "Reader of field `SYNCO_SEL`"]
pub type SYNCO_SEL_R = crate::R<u8, SYNCO_SEL_A>;
impl SYNCO_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SYNCO_SEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SYNCO_SEL_A::BYPASS),
            1 => Val(SYNCO_SEL_A::CTREQPRD),
            2 => Val(SYNCO_SEL_A::DISABLE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BYPASS`"]
    #[inline(always)]
    pub fn is_bypass(&self) -> bool {
        *self == SYNCO_SEL_A::BYPASS
    }
    #[doc = "Checks if the value of the field is `CTREQPRD`"]
    #[inline(always)]
    pub fn is_ctreq_prd(&self) -> bool {
        *self == SYNCO_SEL_A::CTREQPRD
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SYNCO_SEL_A::DISABLE
    }
}
#[doc = "Write proxy for field `SYNCO_SEL`"]
pub struct SYNCO_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCO_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYNCO_SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "sync in connected with sync out"]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut W {
        self.variant(SYNCO_SEL_A::BYPASS)
    }
    #[doc = "sync out generated when CTR = PRD"]
    #[inline(always)]
    pub fn ctreq_prd(self) -> &'a mut W {
        self.variant(SYNCO_SEL_A::CTREQPRD)
    }
    #[doc = "sync out generate disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SYNCO_SEL_A::DISABLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `SWSYNC`"]
pub type SWSYNC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWSYNC`"]
pub struct SWSYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> SWSYNC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `CAP_APWM`"]
pub type CAP_APWM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAP_APWM`"]
pub struct CAP_APWM_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP_APWM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `APWM_POL`"]
pub type APWM_POL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APWM_POL`"]
pub struct APWM_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> APWM_POL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Capture mode"]
    #[inline(always)]
    pub fn cont_onesht(&self) -> CONT_ONESHT_R {
        CONT_ONESHT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    pub fn stop_wrap(&self) -> STOP_WRAP_R {
        STOP_WRAP_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 3 - reset and enable controller, load reg capt"]
    #[inline(always)]
    pub fn rearm(&self) -> REARM_R {
        REARM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - enable Timer"]
    #[inline(always)]
    pub fn tsctrstop(&self) -> TSCTRSTOP_R {
        TSCTRSTOP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - sync outer impulse"]
    #[inline(always)]
    pub fn synci_en(&self) -> SYNCI_EN_R {
        SYNCI_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn synco_sel(&self) -> SYNCO_SEL_R {
        SYNCO_SEL_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 8 - Timers synchr"]
    #[inline(always)]
    pub fn swsync(&self) -> SWSYNC_R {
        SWSYNC_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Capture mode or APWM"]
    #[inline(always)]
    pub fn cap_apwm(&self) -> CAP_APWM_R {
        CAP_APWM_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - High/low level APWM"]
    #[inline(always)]
    pub fn apwm_pol(&self) -> APWM_POL_R {
        APWM_POL_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Capture mode"]
    #[inline(always)]
    pub fn cont_onesht(&mut self) -> CONT_ONESHT_W {
        CONT_ONESHT_W { w: self }
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    pub fn stop_wrap(&mut self) -> STOP_WRAP_W {
        STOP_WRAP_W { w: self }
    }
    #[doc = "Bit 3 - reset and enable controller, load reg capt"]
    #[inline(always)]
    pub fn rearm(&mut self) -> REARM_W {
        REARM_W { w: self }
    }
    #[doc = "Bit 4 - enable Timer"]
    #[inline(always)]
    pub fn tsctrstop(&mut self) -> TSCTRSTOP_W {
        TSCTRSTOP_W { w: self }
    }
    #[doc = "Bit 5 - sync outer impulse"]
    #[inline(always)]
    pub fn synci_en(&mut self) -> SYNCI_EN_W {
        SYNCI_EN_W { w: self }
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn synco_sel(&mut self) -> SYNCO_SEL_W {
        SYNCO_SEL_W { w: self }
    }
    #[doc = "Bit 8 - Timers synchr"]
    #[inline(always)]
    pub fn swsync(&mut self) -> SWSYNC_W {
        SWSYNC_W { w: self }
    }
    #[doc = "Bit 9 - Capture mode or APWM"]
    #[inline(always)]
    pub fn cap_apwm(&mut self) -> CAP_APWM_W {
        CAP_APWM_W { w: self }
    }
    #[doc = "Bit 10 - High/low level APWM"]
    #[inline(always)]
    pub fn apwm_pol(&mut self) -> APWM_POL_W {
        APWM_POL_W { w: self }
    }
}
