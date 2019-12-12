#[doc = "Reader of register TMR"]
pub type R = crate::R<u32, super::TMR>;
#[doc = "Writer for register TMR"]
pub type W = crate::W<u32, super::TMR>;
#[doc = "Register TMR `reset()`'s with value 0"]
impl crate::ResetValue for super::TMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TMR`"]
pub type TMR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `TMR`"]
pub struct TMR_W<'a> {
    w: &'a mut W,
}
impl<'a> TMR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - Sequencer ADC restart timer value"]
    #[inline(always)]
    pub fn tmr(&self) -> TMR_R {
        TMR_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - Sequencer ADC restart timer value"]
    #[inline(always)]
    pub fn tmr(&mut self) -> TMR_W {
        TMR_W { w: self }
    }
}
