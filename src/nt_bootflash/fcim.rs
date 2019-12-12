#[doc = "Reader of register FCIM"]
pub type R = crate::R<u32, super::FCIM>;
#[doc = "Writer for register FCIM"]
pub type W = crate::W<u32, super::FCIM>;
#[doc = "Register FCIM `reset()`'s with value 0"]
impl crate::ResetValue for super::FCIM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MASK_OPCMLT`"]
pub type MASK_OPCMLT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MASK_OPCMLT`"]
pub struct MASK_OPCMLT_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK_OPCMLT_W<'a> {
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
    #[doc = "Bit 0 - Enables generation of an interrupt upon completion of a read or write, or erase"]
    #[inline(always)]
    pub fn mask_opcmlt(&self) -> MASK_OPCMLT_R {
        MASK_OPCMLT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables generation of an interrupt upon completion of a read or write, or erase"]
    #[inline(always)]
    pub fn mask_opcmlt(&mut self) -> MASK_OPCMLT_W {
        MASK_OPCMLT_W { w: self }
    }
}
