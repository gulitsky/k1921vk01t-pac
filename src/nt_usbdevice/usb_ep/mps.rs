#[doc = "Reader of register MPS"]
pub type R = crate::R<u32, super::MPS>;
#[doc = "Writer for register MPS"]
pub type W = crate::W<u32, super::MPS>;
#[doc = "Register MPS `reset()`'s with value 0"]
impl crate::ResetValue for super::MPS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MAXPKTSIZE`"]
pub type MAXPKTSIZE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MAXPKTSIZE`"]
pub struct MAXPKTSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> MAXPKTSIZE_W<'a> {
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
    pub fn maxpktsize(&self) -> MAXPKTSIZE_R {
        MAXPKTSIZE_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn maxpktsize(&mut self) -> MAXPKTSIZE_W {
        MAXPKTSIZE_W { w: self }
    }
}
