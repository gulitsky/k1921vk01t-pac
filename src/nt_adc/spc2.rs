#[doc = "Reader of register SPC2"]
pub type R = crate::R<u32, super::SPC2>;
#[doc = "Writer for register SPC2"]
pub type W = crate::W<u32, super::SPC2>;
#[doc = "Register SPC2 `reset()`'s with value 0"]
impl crate::ResetValue for super::SPC2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PHASE4`"]
pub type PHASE4_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PHASE4`"]
pub struct PHASE4_W<'a> {
    w: &'a mut W,
}
impl<'a> PHASE4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
#[doc = "Reader of field `PHASE5`"]
pub type PHASE5_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PHASE5`"]
pub struct PHASE5_W<'a> {
    w: &'a mut W,
}
impl<'a> PHASE5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | (((value as u32) & 0x0fff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - ADC4 start delay (channel 8,9)"]
    #[inline(always)]
    pub fn phase4(&self) -> PHASE4_R {
        PHASE4_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - ADC5 start delay (channel 10,11)"]
    #[inline(always)]
    pub fn phase5(&self) -> PHASE5_R {
        PHASE5_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - ADC4 start delay (channel 8,9)"]
    #[inline(always)]
    pub fn phase4(&mut self) -> PHASE4_W {
        PHASE4_W { w: self }
    }
    #[doc = "Bits 16:27 - ADC5 start delay (channel 10,11)"]
    #[inline(always)]
    pub fn phase5(&mut self) -> PHASE5_W {
        PHASE5_W { w: self }
    }
}
