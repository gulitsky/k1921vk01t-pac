#[doc = "Reader of register CMPB"]
pub type R = crate::R<u32, super::CMPB>;
#[doc = "Writer for register CMPB"]
pub type W = crate::W<u32, super::CMPB>;
#[doc = "Register CMPB `reset()`'s with value 0"]
impl crate::ResetValue for super::CMPB {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CMPB`"]
pub type CMPB_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CMPB`"]
pub struct CMPB_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn cmpb(&self) -> CMPB_R {
        CMPB_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn cmpb(&mut self) -> CMPB_W {
        CMPB_W { w: self }
    }
}
