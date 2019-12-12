#[doc = "Reader of register MAXF"]
pub type R = crate::R<u32, super::MAXF>;
#[doc = "Writer for register MAXF"]
pub type W = crate::W<u32, super::MAXF>;
#[doc = "Register MAXF `reset()`'s with value 0"]
impl crate::ResetValue for super::MAXF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MAXFRAME_LENGTH`"]
pub type MAXFRAME_LENGTH_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MAXFRAME_LENGTH`"]
pub struct MAXFRAME_LENGTH_W<'a> {
    w: &'a mut W,
}
impl<'a> MAXFRAME_LENGTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn maxframe_length(&self) -> MAXFRAME_LENGTH_R {
        MAXFRAME_LENGTH_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn maxframe_length(&mut self) -> MAXFRAME_LENGTH_W {
        MAXFRAME_LENGTH_W { w: self }
    }
}
