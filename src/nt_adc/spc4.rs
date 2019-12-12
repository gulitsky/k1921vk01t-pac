#[doc = "Reader of register SPC4"]
pub type R = crate::R<u32, super::SPC4>;
#[doc = "Writer for register SPC4"]
pub type W = crate::W<u32, super::SPC4>;
#[doc = "Register SPC4 `reset()`'s with value 0"]
impl crate::ResetValue for super::SPC4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PHASE8`"]
pub type PHASE8_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PHASE8`"]
pub struct PHASE8_W<'a> {
    w: &'a mut W,
}
impl<'a> PHASE8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
#[doc = "Reader of field `PHASE9`"]
pub type PHASE9_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PHASE9`"]
pub struct PHASE9_W<'a> {
    w: &'a mut W,
}
impl<'a> PHASE9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | (((value as u32) & 0x0fff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - ADC8 start delay (channel 16,17)"]
    #[inline(always)]
    pub fn phase8(&self) -> PHASE8_R {
        PHASE8_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - ADC9 start delay (channel 18,19)"]
    #[inline(always)]
    pub fn phase9(&self) -> PHASE9_R {
        PHASE9_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - ADC8 start delay (channel 16,17)"]
    #[inline(always)]
    pub fn phase8(&mut self) -> PHASE8_W {
        PHASE8_W { w: self }
    }
    #[doc = "Bits 16:27 - ADC9 start delay (channel 18,19)"]
    #[inline(always)]
    pub fn phase9(&mut self) -> PHASE9_W {
        PHASE9_W { w: self }
    }
}
