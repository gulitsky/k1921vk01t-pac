#[doc = "Reader of register T_NVS"]
pub type R = crate::R<u32, super::T_NVS>;
#[doc = "Writer for register T_NVS"]
pub type W = crate::W<u32, super::T_NVS>;
#[doc = "Register T_NVS `reset()`'s with value 0"]
impl crate::ResetValue for super::T_NVS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `T_NVS`"]
pub type T_NVS_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `T_NVS`"]
pub struct T_NVS_W<'a> {
    w: &'a mut W,
}
impl<'a> T_NVS_W<'a> {
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
    pub fn t_nvs(&self) -> T_NVS_R {
        T_NVS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn t_nvs(&mut self) -> T_NVS_W {
        T_NVS_W { w: self }
    }
}
