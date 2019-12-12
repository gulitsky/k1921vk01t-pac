#[doc = "Reader of register T_ACC"]
pub type R = crate::R<u32, super::T_ACC>;
#[doc = "Writer for register T_ACC"]
pub type W = crate::W<u32, super::T_ACC>;
#[doc = "Register T_ACC `reset()`'s with value 0"]
impl crate::ResetValue for super::T_ACC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `T_ACC`"]
pub type T_ACC_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `T_ACC`"]
pub struct T_ACC_W<'a> {
    w: &'a mut W,
}
impl<'a> T_ACC_W<'a> {
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
    pub fn t_acc(&self) -> T_ACC_R {
        T_ACC_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn t_acc(&mut self) -> T_ACC_W {
        T_ACC_W { w: self }
    }
}
