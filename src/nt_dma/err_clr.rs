#[doc = "Reader of register ERR_CLR"]
pub type R = crate::R<u32, super::ERR_CLR>;
#[doc = "Writer for register ERR_CLR"]
pub type W = crate::W<u32, super::ERR_CLR>;
#[doc = "Register ERR_CLR `reset()`'s with value 0"]
impl crate::ResetValue for super::ERR_CLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ERR_CLR`"]
pub type ERR_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERR_CLR`"]
pub struct ERR_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR_CLR_W<'a> {
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
    #[doc = "Bit 0 - Indicate Error on bus AHB-Lite / Clear flag error"]
    #[inline(always)]
    pub fn err_clr(&self) -> ERR_CLR_R {
        ERR_CLR_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Indicate Error on bus AHB-Lite / Clear flag error"]
    #[inline(always)]
    pub fn err_clr(&mut self) -> ERR_CLR_W {
        ERR_CLR_W { w: self }
    }
}
