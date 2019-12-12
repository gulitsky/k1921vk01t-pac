#[doc = "Reader of register GPIOPUCTLF"]
pub type R = crate::R<u16, super::GPIOPUCTLF>;
#[doc = "Writer for register GPIOPUCTLF"]
pub type W = crate::W<u16, super::GPIOPUCTLF>;
#[doc = "Register GPIOPUCTLF `reset()`'s with value 0"]
impl crate::ResetValue for super::GPIOPUCTLF {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
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
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Port F pull-up enable"]
    #[inline(always)]
    pub fn portf(&self) -> PORTF_R {
        PORTF_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Port F pull-up enable"]
    #[inline(always)]
    pub fn portf(&mut self) -> PORTF_W {
        PORTF_W { w: self }
    }
}
