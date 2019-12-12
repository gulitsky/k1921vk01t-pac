#[doc = "Reader of register SPC5"]
pub type R = crate::R<u32, super::SPC5>;
#[doc = "Writer for register SPC5"]
pub type W = crate::W<u32, super::SPC5>;
#[doc = "Register SPC5 `reset()`'s with value 0"]
impl crate::ResetValue for super::SPC5 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PHASE10`"]
pub type PHASE10_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PHASE10`"]
pub struct PHASE10_W<'a> {
    w: &'a mut W,
}
impl<'a> PHASE10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
#[doc = "Reader of field `PHASE11`"]
pub type PHASE11_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PHASE11`"]
pub struct PHASE11_W<'a> {
    w: &'a mut W,
}
impl<'a> PHASE11_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | (((value as u32) & 0x0fff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - ADC10 start delay (channel 20,21)"]
    #[inline(always)]
    pub fn phase10(&self) -> PHASE10_R {
        PHASE10_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - ADC11 start delay (channel 22,23)"]
    #[inline(always)]
    pub fn phase11(&self) -> PHASE11_R {
        PHASE11_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - ADC10 start delay (channel 20,21)"]
    #[inline(always)]
    pub fn phase10(&mut self) -> PHASE10_W {
        PHASE10_W { w: self }
    }
    #[doc = "Bits 16:27 - ADC11 start delay (channel 22,23)"]
    #[inline(always)]
    pub fn phase11(&mut self) -> PHASE11_W {
        PHASE11_W { w: self }
    }
}
