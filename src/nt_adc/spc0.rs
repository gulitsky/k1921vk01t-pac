#[doc = "Reader of register SPC0"]
pub type R = crate::R<u32, super::SPC0>;
#[doc = "Writer for register SPC0"]
pub type W = crate::W<u32, super::SPC0>;
#[doc = "Register SPC0 `reset()`'s with value 0"]
impl crate::ResetValue for super::SPC0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PHASE0`"]
pub type PHASE0_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PHASE0`"]
pub struct PHASE0_W<'a> {
    w: &'a mut W,
}
impl<'a> PHASE0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
#[doc = "Reader of field `PHASE1`"]
pub type PHASE1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PHASE1`"]
pub struct PHASE1_W<'a> {
    w: &'a mut W,
}
impl<'a> PHASE1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | (((value as u32) & 0x0fff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - ADC0 start delay (channel 0,1)"]
    #[inline(always)]
    pub fn phase0(&self) -> PHASE0_R {
        PHASE0_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - ADC1 start delay (channel 2,3)"]
    #[inline(always)]
    pub fn phase1(&self) -> PHASE1_R {
        PHASE1_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - ADC0 start delay (channel 0,1)"]
    #[inline(always)]
    pub fn phase0(&mut self) -> PHASE0_W {
        PHASE0_W { w: self }
    }
    #[doc = "Bits 16:27 - ADC1 start delay (channel 2,3)"]
    #[inline(always)]
    pub fn phase1(&mut self) -> PHASE1_W {
        PHASE1_W { w: self }
    }
}
