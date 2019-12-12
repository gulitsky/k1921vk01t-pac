#[doc = "Reader of register BUFF_START_ADDR"]
pub type R = crate::R<u32, super::BUFF_START_ADDR>;
#[doc = "Writer for register BUFF_START_ADDR"]
pub type W = crate::W<u32, super::BUFF_START_ADDR>;
#[doc = "Register BUFF_START_ADDR `reset()`'s with value 0"]
impl crate::ResetValue for super::BUFF_START_ADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BUFFSTARTADDR`"]
pub type BUFFSTARTADDR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `BUFFSTARTADDR`"]
pub struct BUFFSTARTADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFFSTARTADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn buffstartaddr(&self) -> BUFFSTARTADDR_R {
        BUFFSTARTADDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn buffstartaddr(&mut self) -> BUFFSTARTADDR_W {
        BUFFSTARTADDR_W { w: self }
    }
}
