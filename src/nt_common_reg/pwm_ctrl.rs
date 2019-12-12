#[doc = "Reader of register PWM_CTRL"]
pub type R = crate::R<u32, super::PWM_CTRL>;
#[doc = "Writer for register PWM_CTRL"]
pub type W = crate::W<u32, super::PWM_CTRL>;
#[doc = "Register PWM_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::PWM_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SYNCSELECT`"]
pub type SYNCSELECT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SYNCSELECT`"]
pub struct SYNCSELECT_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCSELECT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `CAPSYNCSEL`"]
pub type CAPSYNCSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAPSYNCSEL`"]
pub struct CAPSYNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPSYNCSEL_W<'a> {
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
#[doc = "Reader of field `SYNCREG`"]
pub type SYNCREG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYNCREG`"]
pub struct SYNCREG_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCREG_W<'a> {
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
impl R {
    #[doc = "Bits 0:1 - PWM sync control register"]
    #[inline(always)]
    pub fn syncselect(&self) -> SYNCSELECT_R {
        SYNCSELECT_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - CAP3 sync mode selection (from CAP2 or PWM0)"]
    #[inline(always)]
    pub fn capsyncsel(&self) -> CAPSYNCSEL_R {
        CAPSYNCSEL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SYNCI pulse generation on PWM0 input"]
    #[inline(always)]
    pub fn syncreg(&self) -> SYNCREG_R {
        SYNCREG_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - PWM sync control register"]
    #[inline(always)]
    pub fn syncselect(&mut self) -> SYNCSELECT_W {
        SYNCSELECT_W { w: self }
    }
    #[doc = "Bit 2 - CAP3 sync mode selection (from CAP2 or PWM0)"]
    #[inline(always)]
    pub fn capsyncsel(&mut self) -> CAPSYNCSEL_W {
        CAPSYNCSEL_W { w: self }
    }
    #[doc = "Bit 3 - SYNCI pulse generation on PWM0 input"]
    #[inline(always)]
    pub fn syncreg(&mut self) -> SYNCREG_W {
        SYNCREG_W { w: self }
    }
}
