#[doc = "Reader of register IRQ_ENB_L"]
pub type R = crate::R<u32, super::IRQ_ENB_L>;
#[doc = "Writer for register IRQ_ENB_L"]
pub type W = crate::W<u32, super::IRQ_ENB_L>;
#[doc = "Register IRQ_ENB_L `reset()`'s with value 0"]
impl crate::ResetValue for super::IRQ_ENB_L {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `USBBUSINTEN`"]
pub type USBBUSINTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USBBUSINTEN`"]
pub struct USBBUSINTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USBBUSINTEN_W<'a> {
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
#[doc = "Reader of field `CEP_INTEN`"]
pub type CEP_INTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CEP_INTEN`"]
pub struct CEP_INTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CEP_INTEN_W<'a> {
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
#[doc = "Reader of field `EP0_INTEN`"]
pub type EP0_INTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP0_INTEN`"]
pub struct EP0_INTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> EP0_INTEN_W<'a> {
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
#[doc = "Reader of field `EP1_INTEN`"]
pub type EP1_INTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP1_INTEN`"]
pub struct EP1_INTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> EP1_INTEN_W<'a> {
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
    #[doc = "Bit 0 - Enable interrupts from the events on the USB bus"]
    #[inline(always)]
    pub fn usbbusinten(&self) -> USBBUSINTEN_R {
        USBBUSINTEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Interrupt enable for Control EndPoint"]
    #[inline(always)]
    pub fn cep_inten(&self) -> CEP_INTEN_R {
        CEP_INTEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Interrupt enable for EndPoint 1"]
    #[inline(always)]
    pub fn ep0_inten(&self) -> EP0_INTEN_R {
        EP0_INTEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Interrupt enable for EndPoint 2"]
    #[inline(always)]
    pub fn ep1_inten(&self) -> EP1_INTEN_R {
        EP1_INTEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable interrupts from the events on the USB bus"]
    #[inline(always)]
    pub fn usbbusinten(&mut self) -> USBBUSINTEN_W {
        USBBUSINTEN_W { w: self }
    }
    #[doc = "Bit 1 - Interrupt enable for Control EndPoint"]
    #[inline(always)]
    pub fn cep_inten(&mut self) -> CEP_INTEN_W {
        CEP_INTEN_W { w: self }
    }
    #[doc = "Bit 2 - Interrupt enable for EndPoint 1"]
    #[inline(always)]
    pub fn ep0_inten(&mut self) -> EP0_INTEN_W {
        EP0_INTEN_W { w: self }
    }
    #[doc = "Bit 3 - Interrupt enable for EndPoint 2"]
    #[inline(always)]
    pub fn ep1_inten(&mut self) -> EP1_INTEN_W {
        EP1_INTEN_W { w: self }
    }
}
