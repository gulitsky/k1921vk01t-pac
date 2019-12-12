#[doc = "Reader of register T_PROG"]
pub type R = crate::R<u32, super::T_PROG>;
#[doc = "Writer for register T_PROG"]
pub type W = crate::W<u32, super::T_PROG>;
#[doc = "Register T_PROG `reset()`'s with value 0"]
impl crate::ResetValue for super::T_PROG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `T_PROG`"]
pub type T_PROG_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `T_PROG`"]
pub struct T_PROG_W<'a> {
    w: &'a mut W,
}
impl<'a> T_PROG_W<'a> {
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
    pub fn t_prog(&self) -> T_PROG_R {
        T_PROG_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn t_prog(&mut self) -> T_PROG_W {
        T_PROG_W { w: self }
    }
}
