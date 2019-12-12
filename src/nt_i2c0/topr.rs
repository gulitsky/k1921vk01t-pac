#[doc = "Reader of register TOPR"]
pub type R = crate::R<u32, super::TOPR>;
#[doc = "Writer for register TOPR"]
pub type W = crate::W<u32, super::TOPR>;
#[doc = "Register TOPR `reset()`'s with value 0"]
impl crate::ResetValue for super::TOPR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SMBTOPR`"]
pub type SMBTOPR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SMBTOPR`"]
pub struct SMBTOPR_W<'a> {
    w: &'a mut W,
}
impl<'a> SMBTOPR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn smbtopr(&self) -> SMBTOPR_R {
        SMBTOPR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn smbtopr(&mut self) -> SMBTOPR_W {
        SMBTOPR_W { w: self }
    }
}
