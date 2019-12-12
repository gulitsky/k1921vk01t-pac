#[doc = "Reader of register T_PGS"]
pub type R = crate::R<u32, super::T_PGS>;
#[doc = "Writer for register T_PGS"]
pub type W = crate::W<u32, super::T_PGS>;
#[doc = "Register T_PGS `reset()`'s with value 0"]
impl crate::ResetValue for super::T_PGS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `T_PGS`"]
pub type T_PGS_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `T_PGS`"]
pub struct T_PGS_W<'a> {
    w: &'a mut W,
}
impl<'a> T_PGS_W<'a> {
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
    pub fn t_pgs(&self) -> T_PGS_R {
        T_PGS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn t_pgs(&mut self) -> T_PGS_W {
        T_PGS_W { w: self }
    }
}
