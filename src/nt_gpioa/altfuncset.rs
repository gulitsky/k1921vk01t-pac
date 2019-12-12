#[doc = "Reader of register ALTFUNCSET"]
pub type R = crate::R<u32, super::ALTFUNCSET>;
#[doc = "Writer for register ALTFUNCSET"]
pub type W = crate::W<u32, super::ALTFUNCSET>;
#[doc = "Register ALTFUNCSET `reset()`'s with value 0"]
impl crate::ResetValue for super::ALTFUNCSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ALTFUNCSET`"]
pub type ALTFUNCSET_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ALTFUNCSET`"]
pub struct ALTFUNCSET_W<'a> {
    w: &'a mut W,
}
impl<'a> ALTFUNCSET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn altfuncset(&self) -> ALTFUNCSET_R {
        ALTFUNCSET_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn altfuncset(&mut self) -> ALTFUNCSET_W {
        ALTFUNCSET_W { w: self }
    }
}
