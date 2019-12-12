#[doc = "Writer for register FCIC"]
pub type W = crate::W<u32, super::FCIC>;
#[doc = "Register FCIC `reset()`'s with value 0"]
impl crate::ResetValue for super::FCIC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `CLR_OPCMLT`"]
pub struct CLR_OPCMLT_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR_OPCMLT_W<'a> {
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
#[doc = "Write proxy for field `CLR_OPERROR`"]
pub struct CLR_OPERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR_OPERROR_W<'a> {
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
impl W {
    #[doc = "Bit 0 - Clear flag operate complit"]
    #[inline(always)]
    pub fn clr_opcmlt(&mut self) -> CLR_OPCMLT_W {
        CLR_OPCMLT_W { w: self }
    }
    #[doc = "Bit 1 - Clear flag operate Error"]
    #[inline(always)]
    pub fn clr_operror(&mut self) -> CLR_OPERROR_W {
        CLR_OPERROR_W { w: self }
    }
}
