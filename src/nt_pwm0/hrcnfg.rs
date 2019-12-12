#[doc = "Reader of register HRCNFG"]
pub type R = crate::R<u32, super::HRCNFG>;
#[doc = "Writer for register HRCNFG"]
pub type W = crate::W<u32, super::HRCNFG>;
#[doc = "Register HRCNFG `reset()`'s with value 0"]
impl crate::ResetValue for super::HRCNFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDGMODE_A {
    #[doc = "1: posedge will be delayed"]
    POSEDGE,
    #[doc = "2: negedge will be delayed"]
    NEGEDGE,
    #[doc = "3: posedge and negedge will be delayed"]
    POSNEG,
}
impl From<EDGMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: EDGMODE_A) -> Self {
        match variant {
            EDGMODE_A::POSEDGE => 1,
            EDGMODE_A::NEGEDGE => 2,
            EDGMODE_A::POSNEG => 3,
        }
    }
}
#[doc = "Reader of field `EDGMODE`"]
pub type EDGMODE_R = crate::R<u8, EDGMODE_A>;
impl EDGMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, EDGMODE_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(EDGMODE_A::POSEDGE),
            2 => Val(EDGMODE_A::NEGEDGE),
            3 => Val(EDGMODE_A::POSNEG),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `POSEDGE`"]
    #[inline(always)]
    pub fn is_pos_edge(&self) -> bool {
        *self == EDGMODE_A::POSEDGE
    }
    #[doc = "Checks if the value of the field is `NEGEDGE`"]
    #[inline(always)]
    pub fn is_neg_edge(&self) -> bool {
        *self == EDGMODE_A::NEGEDGE
    }
    #[doc = "Checks if the value of the field is `POSNEG`"]
    #[inline(always)]
    pub fn is_pos_neg(&self) -> bool {
        *self == EDGMODE_A::POSNEG
    }
}
#[doc = "Write proxy for field `EDGMODE`"]
pub struct EDGMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> EDGMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EDGMODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "posedge will be delayed"]
    #[inline(always)]
    pub fn pos_edge(self) -> &'a mut W {
        self.variant(EDGMODE_A::POSEDGE)
    }
    #[doc = "negedge will be delayed"]
    #[inline(always)]
    pub fn neg_edge(self) -> &'a mut W {
        self.variant(EDGMODE_A::NEGEDGE)
    }
    #[doc = "posedge and negedge will be delayed"]
    #[inline(always)]
    pub fn pos_neg(self) -> &'a mut W {
        self.variant(EDGMODE_A::POSNEG)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `CTLMODE`"]
pub type CTLMODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTLMODE`"]
pub struct CTLMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CTLMODE_W<'a> {
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
#[doc = "Reader of field `HR_LOAD`"]
pub type HR_LOAD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HR_LOAD`"]
pub struct HR_LOAD_W<'a> {
    w: &'a mut W,
}
impl<'a> HR_LOAD_W<'a> {
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
#[doc = "Reader of field `MEP_SCALEFACTOR`"]
pub type MEP_SCALEFACTOR_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn edgmode(&self) -> EDGMODE_R {
        EDGMODE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - register specifies the delay"]
    #[inline(always)]
    pub fn ctlmode(&self) -> CTLMODE_R {
        CTLMODE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - load event of deferred value to the active register"]
    #[inline(always)]
    pub fn hr_load(&self) -> HR_LOAD_R {
        HR_LOAD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn mep_scalefactor(&self) -> MEP_SCALEFACTOR_R {
        MEP_SCALEFACTOR_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn edgmode(&mut self) -> EDGMODE_W {
        EDGMODE_W { w: self }
    }
    #[doc = "Bit 2 - register specifies the delay"]
    #[inline(always)]
    pub fn ctlmode(&mut self) -> CTLMODE_W {
        CTLMODE_W { w: self }
    }
    #[doc = "Bit 3 - load event of deferred value to the active register"]
    #[inline(always)]
    pub fn hr_load(&mut self) -> HR_LOAD_W {
        HR_LOAD_W { w: self }
    }
}
