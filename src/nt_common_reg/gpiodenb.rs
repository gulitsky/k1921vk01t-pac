#[doc = "Reader of register GPIODENB"]
pub type R = crate::R<u16, super::GPIODENB>;
#[doc = "Writer for register GPIODENB"]
pub type W = crate::W<u16, super::GPIODENB>;
#[doc = "Register GPIODENB `reset()`'s with value 0"]
impl crate::ResetValue for super::GPIODENB {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PORTB`"]
pub type PORTB_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PORTB`"]
pub struct PORTB_W<'a> {
    w: &'a mut W,
}
impl<'a> PORTB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Port B digital output (GPIO or ALTFUNC) enable"]
    #[inline(always)]
    pub fn portb(&self) -> PORTB_R {
        PORTB_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Port B digital output (GPIO or ALTFUNC) enable"]
    #[inline(always)]
    pub fn portb(&mut self) -> PORTB_W {
        PORTB_W { w: self }
    }
}
