#[doc = "Reader of register PEINT"]
pub type R = crate::R<u32, super::PEINT>;
#[doc = "Writer for register PEINT"]
pub type W = crate::W<u32, super::PEINT>;
#[doc = "Register PEINT `reset()`'s with value 0"]
impl crate::ResetValue for super::PEINT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PEINT`"]
pub type PEINT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PEINT`"]
pub struct PEINT_W<'a> {
    w: &'a mut W,
}
impl<'a> PEINT_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - active interrupt flag"]
    #[inline(always)]
    pub fn peint(&self) -> PEINT_R {
        PEINT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - active interrupt flag"]
    #[inline(always)]
    pub fn peint(&mut self) -> PEINT_W {
        PEINT_W { w: self }
    }
}
