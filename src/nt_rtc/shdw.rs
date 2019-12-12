#[doc = "Writer for register SHDW"]
pub type W = crate::W<u32, super::SHDW>;
#[doc = "Register SHDW `reset()`'s with value 0"]
impl crate::ResetValue for super::SHDW {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `UPDTEN`"]
pub struct UPDTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> UPDTEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
impl W {
    #[doc = "Bit 7 - Enable update shadow registers RTC"]
    #[inline(always)]
    pub fn updten(&mut self) -> UPDTEN_W {
        UPDTEN_W { w: self }
    }
}
