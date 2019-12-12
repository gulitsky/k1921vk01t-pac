#[doc = "Reader of register HDCLR"]
pub type R = crate::R<u32, super::HDCLR>;
#[doc = "Writer for register HDCLR"]
pub type W = crate::W<u32, super::HDCLR>;
#[doc = "Register HDCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::HDCLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HDINT`"]
pub type HDINT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HDINT`"]
pub struct HDINT_W<'a> {
    w: &'a mut W,
}
impl<'a> HDINT_W<'a> {
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
#[doc = "Reader of field `CBC`"]
pub type CBC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CBC`"]
pub struct CBC_W<'a> {
    w: &'a mut W,
}
impl<'a> CBC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `OST`"]
pub type OST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OST`"]
pub struct OST_W<'a> {
    w: &'a mut W,
}
impl<'a> OST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Clear flag HDINT"]
    #[inline(always)]
    pub fn hdint(&self) -> HDINT_R {
        HDINT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Activation threshold trigger in cycle mode"]
    #[inline(always)]
    pub fn cbc(&self) -> CBC_R {
        CBC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Activation threshold trigger in single mode"]
    #[inline(always)]
    pub fn ost(&self) -> OST_R {
        OST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clear flag HDINT"]
    #[inline(always)]
    pub fn hdint(&mut self) -> HDINT_W {
        HDINT_W { w: self }
    }
    #[doc = "Bit 1 - Activation threshold trigger in cycle mode"]
    #[inline(always)]
    pub fn cbc(&mut self) -> CBC_W {
        CBC_W { w: self }
    }
    #[doc = "Bit 2 - Activation threshold trigger in single mode"]
    #[inline(always)]
    pub fn ost(&mut self) -> OST_W {
        OST_W { w: self }
    }
}
