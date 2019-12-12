#[doc = "Reader of register SPC1"]
pub type R = crate::R<u32, super::SPC1>;
#[doc = "Writer for register SPC1"]
pub type W = crate::W<u32, super::SPC1>;
#[doc = "Register SPC1 `reset()`'s with value 0"]
impl crate::ResetValue for super::SPC1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PHASE2`"]
pub type PHASE2_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PHASE2`"]
pub struct PHASE2_W<'a> {
    w: &'a mut W,
}
impl<'a> PHASE2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
#[doc = "Reader of field `PHASE3`"]
pub type PHASE3_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PHASE3`"]
pub struct PHASE3_W<'a> {
    w: &'a mut W,
}
impl<'a> PHASE3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | (((value as u32) & 0x0fff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - ADC2 start delay (channel 4,5)"]
    #[inline(always)]
    pub fn phase2(&self) -> PHASE2_R {
        PHASE2_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - ADC3 start delay (channel 6,7)"]
    #[inline(always)]
    pub fn phase3(&self) -> PHASE3_R {
        PHASE3_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - ADC2 start delay (channel 4,5)"]
    #[inline(always)]
    pub fn phase2(&mut self) -> PHASE2_W {
        PHASE2_W { w: self }
    }
    #[doc = "Bits 16:27 - ADC3 start delay (channel 6,7)"]
    #[inline(always)]
    pub fn phase3(&mut self) -> PHASE3_W {
        PHASE3_W { w: self }
    }
}
