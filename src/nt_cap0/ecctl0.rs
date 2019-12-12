#[doc = "Reader of register ECCTL0"]
pub type R = crate::R<u32, super::ECCTL0>;
#[doc = "Writer for register ECCTL0"]
pub type W = crate::W<u32, super::ECCTL0>;
#[doc = "Register ECCTL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::ECCTL0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CAP0_POL`"]
pub type CAP0_POL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAP0_POL`"]
pub struct CAP0_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP0_POL_W<'a> {
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
#[doc = "Reader of field `CTR_RST0`"]
pub type CTR_RST0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTR_RST0`"]
pub struct CTR_RST0_W<'a> {
    w: &'a mut W,
}
impl<'a> CTR_RST0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `CAP1_POL`"]
pub type CAP1_POL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAP1_POL`"]
pub struct CAP1_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP1_POL_W<'a> {
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
#[doc = "Reader of field `CTR_RST1`"]
pub type CTR_RST1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTR_RST1`"]
pub struct CTR_RST1_W<'a> {
    w: &'a mut W,
}
impl<'a> CTR_RST1_W<'a> {
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
#[doc = "Reader of field `CAP2_POL`"]
pub type CAP2_POL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAP2_POL`"]
pub struct CAP2_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP2_POL_W<'a> {
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
#[doc = "Reader of field `CTR_RST2`"]
pub type CTR_RST2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTR_RST2`"]
pub struct CTR_RST2_W<'a> {
    w: &'a mut W,
}
impl<'a> CTR_RST2_W<'a> {
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
#[doc = "Reader of field `CAP3_POL`"]
pub type CAP3_POL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAP3_POL`"]
pub struct CAP3_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP3_POL_W<'a> {
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
#[doc = "Reader of field `CTR_RST3`"]
pub type CTR_RST3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTR_RST3`"]
pub struct CTR_RST3_W<'a> {
    w: &'a mut W,
}
impl<'a> CTR_RST3_W<'a> {
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
#[doc = "Reader of field `CAPLDEN`"]
pub type CAPLDEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAPLDEN`"]
pub struct CAPLDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPLDEN_W<'a> {
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
#[doc = "Reader of field `PRESCALE`"]
pub type PRESCALE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRESCALE`"]
pub struct PRESCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> PRESCALE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 9)) | (((value as u32) & 0x1f) << 9);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FREE_SOFT_A {
    #[doc = "0: stop timer immedeatelly"]
    STOP,
    #[doc = "1: stop timer when reach zero"]
    STOPATZERO,
}
impl From<FREE_SOFT_A> for u8 {
    #[inline(always)]
    fn from(variant: FREE_SOFT_A) -> Self {
        match variant {
            FREE_SOFT_A::STOP => 0,
            FREE_SOFT_A::STOPATZERO => 1,
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
            0 => Val(FREE_SOFT_A::STOP),
            1 => Val(FREE_SOFT_A::STOPATZERO),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == FREE_SOFT_A::STOP
    }
    #[doc = "Checks if the value of the field is `STOPATZERO`"]
    #[inline(always)]
    pub fn is_stop_at_zero(&self) -> bool {
        *self == FREE_SOFT_A::STOPATZERO
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
    #[doc = "stop timer immedeatelly"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(FREE_SOFT_A::STOP)
    }
    #[doc = "stop timer when reach zero"]
    #[inline(always)]
    pub fn stop_at_zero(self) -> &'a mut W {
        self.variant(FREE_SOFT_A::STOPATZERO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - pos/negedge capture 0"]
    #[inline(always)]
    pub fn cap0_pol(&self) -> CAP0_POL_R {
        CAP0_POL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - reset later event 0"]
    #[inline(always)]
    pub fn ctr_rst0(&self) -> CTR_RST0_R {
        CTR_RST0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - pos/negedge capture 1"]
    #[inline(always)]
    pub fn cap1_pol(&self) -> CAP1_POL_R {
        CAP1_POL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - reset later event 1"]
    #[inline(always)]
    pub fn ctr_rst1(&self) -> CTR_RST1_R {
        CTR_RST1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - pos/negedge capture 2"]
    #[inline(always)]
    pub fn cap2_pol(&self) -> CAP2_POL_R {
        CAP2_POL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - reset later event 2"]
    #[inline(always)]
    pub fn ctr_rst2(&self) -> CTR_RST2_R {
        CTR_RST2_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - pos/negedge capture 3"]
    #[inline(always)]
    pub fn cap3_pol(&self) -> CAP3_POL_R {
        CAP3_POL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - reset later event 3"]
    #[inline(always)]
    pub fn ctr_rst3(&self) -> CTR_RST3_R {
        CTR_RST3_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - enable capture"]
    #[inline(always)]
    pub fn caplden(&self) -> CAPLDEN_R {
        CAPLDEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 9:13"]
    #[inline(always)]
    pub fn prescale(&self) -> PRESCALE_R {
        PRESCALE_R::new(((self.bits >> 9) & 0x1f) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn free_soft(&self) -> FREE_SOFT_R {
        FREE_SOFT_R::new(((self.bits >> 14) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - pos/negedge capture 0"]
    #[inline(always)]
    pub fn cap0_pol(&mut self) -> CAP0_POL_W {
        CAP0_POL_W { w: self }
    }
    #[doc = "Bit 1 - reset later event 0"]
    #[inline(always)]
    pub fn ctr_rst0(&mut self) -> CTR_RST0_W {
        CTR_RST0_W { w: self }
    }
    #[doc = "Bit 2 - pos/negedge capture 1"]
    #[inline(always)]
    pub fn cap1_pol(&mut self) -> CAP1_POL_W {
        CAP1_POL_W { w: self }
    }
    #[doc = "Bit 3 - reset later event 1"]
    #[inline(always)]
    pub fn ctr_rst1(&mut self) -> CTR_RST1_W {
        CTR_RST1_W { w: self }
    }
    #[doc = "Bit 4 - pos/negedge capture 2"]
    #[inline(always)]
    pub fn cap2_pol(&mut self) -> CAP2_POL_W {
        CAP2_POL_W { w: self }
    }
    #[doc = "Bit 5 - reset later event 2"]
    #[inline(always)]
    pub fn ctr_rst2(&mut self) -> CTR_RST2_W {
        CTR_RST2_W { w: self }
    }
    #[doc = "Bit 6 - pos/negedge capture 3"]
    #[inline(always)]
    pub fn cap3_pol(&mut self) -> CAP3_POL_W {
        CAP3_POL_W { w: self }
    }
    #[doc = "Bit 7 - reset later event 3"]
    #[inline(always)]
    pub fn ctr_rst3(&mut self) -> CTR_RST3_W {
        CTR_RST3_W { w: self }
    }
    #[doc = "Bit 8 - enable capture"]
    #[inline(always)]
    pub fn caplden(&mut self) -> CAPLDEN_W {
        CAPLDEN_W { w: self }
    }
    #[doc = "Bits 9:13"]
    #[inline(always)]
    pub fn prescale(&mut self) -> PRESCALE_W {
        PRESCALE_W { w: self }
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn free_soft(&mut self) -> FREE_SOFT_W {
        FREE_SOFT_W { w: self }
    }
}
