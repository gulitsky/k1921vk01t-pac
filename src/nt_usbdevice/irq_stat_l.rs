#[doc = "Reader of register IRQ_STAT_L"]
pub type R = crate::R<u32, super::IRQ_STAT_L>;
#[doc = "Writer for register IRQ_STAT_L"]
pub type W = crate::W<u32, super::IRQ_STAT_L>;
#[doc = "Register IRQ_STAT_L `reset()`'s with value 0"]
impl crate::ResetValue for super::IRQ_STAT_L {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `USBBUSINT`"]
pub type USBBUSINT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USBBUSINT`"]
pub struct USBBUSINT_W<'a> {
    w: &'a mut W,
}
impl<'a> USBBUSINT_W<'a> {
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
#[doc = "Reader of field `CEP_INT`"]
pub type CEP_INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CEP_INT`"]
pub struct CEP_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> CEP_INT_W<'a> {
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
#[doc = "Reader of field `EP0_INT`"]
pub type EP0_INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP0_INT`"]
pub struct EP0_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> EP0_INT_W<'a> {
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
#[doc = "Reader of field `EP1_INT`"]
pub type EP1_INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP1_INT`"]
pub struct EP1_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> EP1_INT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Interrupt flag events on the USB bus"]
    #[inline(always)]
    pub fn usbbusint(&self) -> USBBUSINT_R {
        USBBUSINT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Interrupt flag Control EndPoint"]
    #[inline(always)]
    pub fn cep_int(&self) -> CEP_INT_R {
        CEP_INT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Interrupt flags EndPoint 1"]
    #[inline(always)]
    pub fn ep0_int(&self) -> EP0_INT_R {
        EP0_INT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Interrupt flags EndPoint 2"]
    #[inline(always)]
    pub fn ep1_int(&self) -> EP1_INT_R {
        EP1_INT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt flag events on the USB bus"]
    #[inline(always)]
    pub fn usbbusint(&mut self) -> USBBUSINT_W {
        USBBUSINT_W { w: self }
    }
    #[doc = "Bit 1 - Interrupt flag Control EndPoint"]
    #[inline(always)]
    pub fn cep_int(&mut self) -> CEP_INT_W {
        CEP_INT_W { w: self }
    }
    #[doc = "Bit 2 - Interrupt flags EndPoint 1"]
    #[inline(always)]
    pub fn ep0_int(&mut self) -> EP0_INT_W {
        EP0_INT_W { w: self }
    }
    #[doc = "Bit 3 - Interrupt flags EndPoint 2"]
    #[inline(always)]
    pub fn ep1_int(&mut self) -> EP1_INT_W {
        EP1_INT_W { w: self }
    }
}
