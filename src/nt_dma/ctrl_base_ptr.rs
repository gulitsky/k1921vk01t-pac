#[doc = "Reader of register CTRL_BASE_PTR"]
pub type R = crate::R<u32, super::CTRL_BASE_PTR>;
#[doc = "Writer for register CTRL_BASE_PTR"]
pub type W = crate::W<u32, super::CTRL_BASE_PTR>;
#[doc = "Register CTRL_BASE_PTR `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRL_BASE_PTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CTRL_BASE_PTR`"]
pub type CTRL_BASE_PTR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CTRL_BASE_PTR`"]
pub struct CTRL_BASE_PTR_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRL_BASE_PTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x003f_ffff << 10)) | (((value as u32) & 0x003f_ffff) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bits 10:31 - Pointer to base address of the primary structure of the control data"]
    #[inline(always)]
    pub fn ctrl_base_ptr(&self) -> CTRL_BASE_PTR_R {
        CTRL_BASE_PTR_R::new(((self.bits >> 10) & 0x003f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 10:31 - Pointer to base address of the primary structure of the control data"]
    #[inline(always)]
    pub fn ctrl_base_ptr(&mut self) -> CTRL_BASE_PTR_W {
        CTRL_BASE_PTR_W { w: self }
    }
}
