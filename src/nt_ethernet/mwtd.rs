#[doc = "Reader of register MWTD"]
pub type R = crate::R<u32, super::MWTD>;
#[doc = "Writer for register MWTD"]
pub type W = crate::W<u32, super::MWTD>;
#[doc = "Register MWTD `reset()`'s with value 0"]
impl crate::ResetValue for super::MWTD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `WRITE_DATA`"]
pub struct WRITE_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> WRITE_DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn write_data(&mut self) -> WRITE_DATA_W {
        WRITE_DATA_W { w: self }
    }
}
