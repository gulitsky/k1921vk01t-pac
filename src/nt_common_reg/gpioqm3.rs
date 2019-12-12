#[doc = "Reader of register GPIOQM3"]
pub type R = crate::R<u32, super::GPIOQM3>;
#[doc = "Writer for register GPIOQM3"]
pub type W = crate::W<u32, super::GPIOQM3>;
#[doc = "Register GPIOQM3 `reset()`'s with value 0"]
impl crate::ResetValue for super::GPIOQM3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PORTG`"]
pub type PORTG_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PORTG`"]
pub struct PORTG_W<'a> {
    w: &'a mut W,
}
impl<'a> PORTG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `PORTH`"]
pub type PORTH_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PORTH`"]
pub struct PORTH_W<'a> {
    w: &'a mut W,
}
impl<'a> PORTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn portg(&self) -> PORTG_R {
        PORTG_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn porth(&self) -> PORTH_R {
        PORTH_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn portg(&mut self) -> PORTG_W {
        PORTG_W { w: self }
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn porth(&mut self) -> PORTH_W {
        PORTH_W { w: self }
    }
}
