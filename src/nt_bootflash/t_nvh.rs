#[doc = "Reader of register T_NVH"]
pub type R = crate::R<u32, super::T_NVH>;
#[doc = "Writer for register T_NVH"]
pub type W = crate::W<u32, super::T_NVH>;
#[doc = "Register T_NVH `reset()`'s with value 0"]
impl crate::ResetValue for super::T_NVH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `T_NVH`"]
pub type T_NVH_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `T_NVH`"]
pub struct T_NVH_W<'a> {
    w: &'a mut W,
}
impl<'a> T_NVH_W<'a> {
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
    pub fn t_nvh(&self) -> T_NVH_R {
        T_NVH_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn t_nvh(&mut self) -> T_NVH_W {
        T_NVH_W { w: self }
    }
}
