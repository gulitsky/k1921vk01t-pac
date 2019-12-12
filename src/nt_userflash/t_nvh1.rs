#[doc = "Reader of register T_NVH1"]
pub type R = crate::R<u32, super::T_NVH1>;
#[doc = "Writer for register T_NVH1"]
pub type W = crate::W<u32, super::T_NVH1>;
#[doc = "Register T_NVH1 `reset()`'s with value 0"]
impl crate::ResetValue for super::T_NVH1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `T_NVH1`"]
pub type T_NVH1_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `T_NVH1`"]
pub struct T_NVH1_W<'a> {
    w: &'a mut W,
}
impl<'a> T_NVH1_W<'a> {
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
    pub fn t_nvh1(&self) -> T_NVH1_R {
        T_NVH1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn t_nvh1(&mut self) -> T_NVH1_W {
        T_NVH1_W { w: self }
    }
}
