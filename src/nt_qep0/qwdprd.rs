#[doc = "Reader of register QWDPRD"]
pub type R = crate::R<u32, super::QWDPRD>;
#[doc = "Writer for register QWDPRD"]
pub type W = crate::W<u32, super::QWDPRD>;
#[doc = "Register QWDPRD `reset()`'s with value 0"]
impl crate::ResetValue for super::QWDPRD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `QWDPRD`"]
pub type QWDPRD_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `QWDPRD`"]
pub struct QWDPRD_W<'a> {
    w: &'a mut W,
}
impl<'a> QWDPRD_W<'a> {
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
    pub fn qwdprd(&self) -> QWDPRD_R {
        QWDPRD_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn qwdprd(&mut self) -> QWDPRD_W {
        QWDPRD_W { w: self }
    }
}
