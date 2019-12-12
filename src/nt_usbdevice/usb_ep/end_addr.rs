#[doc = "Reader of register END_ADDR"]
pub type R = crate::R<u32, super::END_ADDR>;
#[doc = "Writer for register END_ADDR"]
pub type W = crate::W<u32, super::END_ADDR>;
#[doc = "Register END_ADDR `reset()`'s with value 0"]
impl crate::ResetValue for super::END_ADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ENDADDR`"]
pub type ENDADDR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ENDADDR`"]
pub struct ENDADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDADDR_W<'a> {
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
    pub fn endaddr(&self) -> ENDADDR_R {
        ENDADDR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn endaddr(&mut self) -> ENDADDR_W {
        ENDADDR_W { w: self }
    }
}
