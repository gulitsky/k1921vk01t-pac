#[doc = "Reader of register DMARxCtrl"]
pub type R = crate::R<u32, super::DMARXCTRL>;
#[doc = "Writer for register DMARxCtrl"]
pub type W = crate::W<u32, super::DMARXCTRL>;
#[doc = "Register DMARxCtrl `reset()`'s with value 0"]
impl crate::ResetValue for super::DMARXCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RXENABLE`"]
pub type RXENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXENABLE`"]
pub struct RXENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXENABLE_W<'a> {
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
    #[doc = "Bit 0 - Enable receive"]
    #[inline(always)]
    pub fn rxenable(&self) -> RXENABLE_R {
        RXENABLE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable receive"]
    #[inline(always)]
    pub fn rxenable(&mut self) -> RXENABLE_W {
        RXENABLE_W { w: self }
    }
}
