#[doc = "Reader of register CLRT"]
pub type R = crate::R<u32, super::CLRT>;
#[doc = "Writer for register CLRT"]
pub type W = crate::W<u32, super::CLRT>;
#[doc = "Register CLRT `reset()`'s with value 0"]
impl crate::ResetValue for super::CLRT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RETRANS_MAX`"]
pub type RETRANS_MAX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RETRANS_MAX`"]
pub struct RETRANS_MAX_W<'a> {
    w: &'a mut W,
}
impl<'a> RETRANS_MAX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `COLLISION_WINDOW`"]
pub type COLLISION_WINDOW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `COLLISION_WINDOW`"]
pub struct COLLISION_WINDOW_W<'a> {
    w: &'a mut W,
}
impl<'a> COLLISION_WINDOW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn retrans_max(&self) -> RETRANS_MAX_R {
        RETRANS_MAX_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    pub fn collision_window(&self) -> COLLISION_WINDOW_R {
        COLLISION_WINDOW_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn retrans_max(&mut self) -> RETRANS_MAX_W {
        RETRANS_MAX_W { w: self }
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    pub fn collision_window(&mut self) -> COLLISION_WINDOW_W {
        COLLISION_WINDOW_W { w: self }
    }
}
