#[doc = "Reader of register GPIODEN2"]
pub type R = crate::R<u32, super::GPIODEN2>;
#[doc = "Writer for register GPIODEN2"]
pub type W = crate::W<u32, super::GPIODEN2>;
#[doc = "Register GPIODEN2 `reset()`'s with value 0"]
impl crate::ResetValue for super::GPIODEN2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PORTE`"]
pub type PORTE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PORTE`"]
pub struct PORTE_W<'a> {
    w: &'a mut W,
}
impl<'a> PORTE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `PORTF`"]
pub type PORTF_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PORTF`"]
pub struct PORTF_W<'a> {
    w: &'a mut W,
}
impl<'a> PORTF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Port E digital output (GPIO or ALTFUNC) enable"]
    #[inline(always)]
    pub fn porte(&self) -> PORTE_R {
        PORTE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Port F digital output (GPIO or ALTFUNC) enable"]
    #[inline(always)]
    pub fn portf(&self) -> PORTF_R {
        PORTF_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Port E digital output (GPIO or ALTFUNC) enable"]
    #[inline(always)]
    pub fn porte(&mut self) -> PORTE_W {
        PORTE_W { w: self }
    }
    #[doc = "Bits 16:31 - Port F digital output (GPIO or ALTFUNC) enable"]
    #[inline(always)]
    pub fn portf(&mut self) -> PORTF_W {
        PORTF_W { w: self }
    }
}
