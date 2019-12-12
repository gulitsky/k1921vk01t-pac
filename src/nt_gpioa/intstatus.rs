#[doc = "Reader of register INTSTATUS"]
pub type R = crate::R<u32, super::INTSTATUS>;
#[doc = "Writer for register INTSTATUS"]
pub type W = crate::W<u32, super::INTSTATUS>;
#[doc = "Register INTSTATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::INTSTATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INTSTATUS`"]
pub type INTSTATUS_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `INTSTATUS`"]
pub struct INTSTATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> INTSTATUS_W<'a> {
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
    pub fn intstatus(&self) -> INTSTATUS_R {
        INTSTATUS_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn intstatus(&mut self) -> INTSTATUS_W {
        INTSTATUS_W { w: self }
    }
}
