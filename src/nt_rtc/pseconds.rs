#[doc = "Reader of register PSECONDS"]
pub type R = crate::R<u32, super::PSECONDS>;
#[doc = "Writer for register PSECONDS"]
pub type W = crate::W<u32, super::PSECONDS>;
#[doc = "Register PSECONDS `reset()`'s with value 0"]
impl crate::ResetValue for super::PSECONDS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PART_SEC`"]
pub type PART_SEC_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PART_SEC`"]
pub struct PART_SEC_W<'a> {
    w: &'a mut W,
}
impl<'a> PART_SEC_W<'a> {
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
    pub fn part_sec(&self) -> PART_SEC_R {
        PART_SEC_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn part_sec(&mut self) -> PART_SEC_W {
        PART_SEC_W { w: self }
    }
}
