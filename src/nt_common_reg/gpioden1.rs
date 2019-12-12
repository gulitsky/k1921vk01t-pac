#[doc = "Reader of register GPIODEN1"]
pub type R = crate::R<u32, super::GPIODEN1>;
#[doc = "Writer for register GPIODEN1"]
pub type W = crate::W<u32, super::GPIODEN1>;
#[doc = "Register GPIODEN1 `reset()`'s with value 0"]
impl crate::ResetValue for super::GPIODEN1 {
    type Type = u32;
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
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `PORTD`"]
pub type PORTD_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PORTD`"]
pub struct PORTD_W<'a> {
    w: &'a mut W,
}
impl<'a> PORTD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Port C digital output (GPIO or ALTFUNC) enable"]
    #[inline(always)]
    pub fn portc(&self) -> PORTC_R {
        PORTC_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Port D digital output (GPIO or ALTFUNC) enable"]
    #[inline(always)]
    pub fn portd(&self) -> PORTD_R {
        PORTD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Port C digital output (GPIO or ALTFUNC) enable"]
    #[inline(always)]
    pub fn portc(&mut self) -> PORTC_W {
        PORTC_W { w: self }
    }
    #[doc = "Bits 16:31 - Port D digital output (GPIO or ALTFUNC) enable"]
    #[inline(always)]
    pub fn portd(&mut self) -> PORTD_W {
        PORTD_W { w: self }
    }
}
