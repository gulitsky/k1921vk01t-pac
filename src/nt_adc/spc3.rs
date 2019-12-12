#[doc = "Reader of register SPC3"]
pub type R = crate::R<u32, super::SPC3>;
#[doc = "Writer for register SPC3"]
pub type W = crate::W<u32, super::SPC3>;
#[doc = "Register SPC3 `reset()`'s with value 0"]
impl crate::ResetValue for super::SPC3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PHASE6`"]
pub type PHASE6_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PHASE6`"]
pub struct PHASE6_W<'a> {
    w: &'a mut W,
}
impl<'a> PHASE6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
#[doc = "Reader of field `PHASE7`"]
pub type PHASE7_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PHASE7`"]
pub struct PHASE7_W<'a> {
    w: &'a mut W,
}
impl<'a> PHASE7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | (((value as u32) & 0x0fff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - ADC6 start delay (channel 12,13)"]
    #[inline(always)]
    pub fn phase6(&self) -> PHASE6_R {
        PHASE6_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - ADC7 start delay (channel 14,15)"]
    #[inline(always)]
    pub fn phase7(&self) -> PHASE7_R {
        PHASE7_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - ADC6 start delay (channel 12,13)"]
    #[inline(always)]
    pub fn phase6(&mut self) -> PHASE6_W {
        PHASE6_W { w: self }
    }
    #[doc = "Bits 16:27 - ADC7 start delay (channel 14,15)"]
    #[inline(always)]
    pub fn phase7(&mut self) -> PHASE7_W {
        PHASE7_W { w: self }
    }
}
