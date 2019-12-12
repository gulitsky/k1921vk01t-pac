#[doc = "Reader of register QUPRD"]
pub type R = crate::R<u32, super::QUPRD>;
#[doc = "Writer for register QUPRD"]
pub type W = crate::W<u32, super::QUPRD>;
#[doc = "Register QUPRD `reset()`'s with value 0"]
impl crate::ResetValue for super::QUPRD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `QUPRD`"]
pub type QUPRD_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `QUPRD`"]
pub struct QUPRD_W<'a> {
    w: &'a mut W,
}
impl<'a> QUPRD_W<'a> {
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
    pub fn quprd(&self) -> QUPRD_R {
        QUPRD_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn quprd(&mut self) -> QUPRD_W {
        QUPRD_W { w: self }
    }
}
