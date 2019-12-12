#[doc = "Reader of register GPIOPUCTLD"]
pub type R = crate::R<u16, super::GPIOPUCTLD>;
#[doc = "Writer for register GPIOPUCTLD"]
pub type W = crate::W<u16, super::GPIOPUCTLD>;
#[doc = "Register GPIOPUCTLD `reset()`'s with value 0"]
impl crate::ResetValue for super::GPIOPUCTLD {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
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
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Port D pull-up enable"]
    #[inline(always)]
    pub fn portd(&self) -> PORTD_R {
        PORTD_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Port D pull-up enable"]
    #[inline(always)]
    pub fn portd(&mut self) -> PORTD_W {
        PORTD_W { w: self }
    }
}
