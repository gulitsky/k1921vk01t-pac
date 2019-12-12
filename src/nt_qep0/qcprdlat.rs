#[doc = "Reader of register QCPRDLAT"]
pub type R = crate::R<u32, super::QCPRDLAT>;
#[doc = "Writer for register QCPRDLAT"]
pub type W = crate::W<u32, super::QCPRDLAT>;
#[doc = "Register QCPRDLAT `reset()`'s with value 0"]
impl crate::ResetValue for super::QCPRDLAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `QCPRDLAT`"]
pub type QCPRDLAT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `QCPRDLAT`"]
pub struct QCPRDLAT_W<'a> {
    w: &'a mut W,
}
impl<'a> QCPRDLAT_W<'a> {
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
    pub fn qcprdlat(&self) -> QCPRDLAT_R {
        QCPRDLAT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn qcprdlat(&mut self) -> QCPRDLAT_W {
        QCPRDLAT_W { w: self }
    }
}
