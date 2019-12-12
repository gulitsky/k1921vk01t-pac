#[doc = "Reader of register QCPRD"]
pub type R = crate::R<u32, super::QCPRD>;
#[doc = "Writer for register QCPRD"]
pub type W = crate::W<u32, super::QCPRD>;
#[doc = "Register QCPRD `reset()`'s with value 0"]
impl crate::ResetValue for super::QCPRD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `QCPRD`"]
pub type QCPRD_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `QCPRD`"]
pub struct QCPRD_W<'a> {
    w: &'a mut W,
}
impl<'a> QCPRD_W<'a> {
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
    pub fn qcprd(&self) -> QCPRD_R {
        QCPRD_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn qcprd(&mut self) -> QCPRD_W {
        QCPRD_W { w: self }
    }
}
