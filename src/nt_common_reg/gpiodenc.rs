#[doc = "Reader of register GPIODENC"]
pub type R = crate::R<u16, super::GPIODENC>;
#[doc = "Writer for register GPIODENC"]
pub type W = crate::W<u16, super::GPIODENC>;
#[doc = "Register GPIODENC `reset()`'s with value 0"]
impl crate::ResetValue for super::GPIODENC {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PORTC`"]
pub type PORTC_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PORTC`"]
pub struct PORTC_W<'a> {
    w: &'a mut W,
}
impl<'a> PORTC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Port C digital output (GPIO or ALTFUNC) enable"]
    #[inline(always)]
    pub fn portc(&self) -> PORTC_R {
        PORTC_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Port C digital output (GPIO or ALTFUNC) enable"]
    #[inline(always)]
    pub fn portc(&mut self) -> PORTC_W {
        PORTC_W { w: self }
    }
}
