#[doc = "Reader of register DBFED"]
pub type R = crate::R<u32, super::DBFED>;
#[doc = "Writer for register DBFED"]
pub type W = crate::W<u32, super::DBFED>;
#[doc = "Register DBFED `reset()`'s with value 0"]
impl crate::ResetValue for super::DBFED {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DEL`"]
pub type DEL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DEL`"]
pub struct DEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn del(&self) -> DEL_R {
        DEL_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn del(&mut self) -> DEL_W {
        DEL_W { w: self }
    }
}
