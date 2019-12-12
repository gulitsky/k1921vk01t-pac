#[doc = "Reader of register CEP_START_ADDR"]
pub type R = crate::R<u32, super::CEP_START_ADDR>;
#[doc = "Writer for register CEP_START_ADDR"]
pub type W = crate::W<u32, super::CEP_START_ADDR>;
#[doc = "Register CEP_START_ADDR `reset()`'s with value 0"]
impl crate::ResetValue for super::CEP_START_ADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `STARTADDR`"]
pub type STARTADDR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `STARTADDR`"]
pub struct STARTADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> STARTADDR_W<'a> {
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
    pub fn startaddr(&self) -> STARTADDR_R {
        STARTADDR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn startaddr(&mut self) -> STARTADDR_W {
        STARTADDR_W { w: self }
    }
}
