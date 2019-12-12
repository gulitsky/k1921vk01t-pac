#[doc = "Reader of register USB_IRQ_ENB"]
pub type R = crate::R<u32, super::USB_IRQ_ENB>;
#[doc = "Writer for register USB_IRQ_ENB"]
pub type W = crate::W<u32, super::USB_IRQ_ENB>;
#[doc = "Register USB_IRQ_ENB `reset()`'s with value 0"]
impl crate::ResetValue for super::USB_IRQ_ENB {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SOFINTEN`"]
pub type SOFINTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFINTEN`"]
pub struct SOFINTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFINTEN_W<'a> {
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
#[doc = "Reader of field `RESSTATUSINTEN`"]
pub type RESSTATUSINTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESSTATUSINTEN`"]
pub struct RESSTATUSINTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RESSTATUSINTEN_W<'a> {
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
#[doc = "Reader of field `RESUMEINTEN`"]
pub type RESUMEINTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESUMEINTEN`"]
pub struct RESUMEINTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RESUMEINTEN_W<'a> {
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
#[doc = "Reader of field `SUSPENDINTEN`"]
pub type SUSPENDINTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SUSPENDINTEN`"]
pub struct SUSPENDINTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SUSPENDINTEN_W<'a> {
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
#[doc = "Reader of field `HISPEEDINTEN`"]
pub type HISPEEDINTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HISPEEDINTEN`"]
pub struct HISPEEDINTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HISPEEDINTEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `DMACMPLINTEN`"]
pub type DMACMPLINTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMACMPLINTEN`"]
pub struct DMACMPLINTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMACMPLINTEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `CLKUNSTBLINTEN`"]
pub type CLKUNSTBLINTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLKUNSTBLINTEN`"]
pub struct CLKUNSTBLINTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKUNSTBLINTEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable SOF packet reception"]
    #[inline(always)]
    pub fn sofinten(&self) -> SOFINTEN_R {
        SOFINTEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable completion reset the root port"]
    #[inline(always)]
    pub fn resstatusinten(&self) -> RESSTATUSINTEN_R {
        RESSTATUSINTEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable reactivate the device"]
    #[inline(always)]
    pub fn resumeinten(&self) -> RESUMEINTEN_R {
        RESUMEINTEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable SUSPEND mode request bit"]
    #[inline(always)]
    pub fn suspendinten(&self) -> SUSPENDINTEN_R {
        SUSPENDINTEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable completion reset and switching devices in high-speed mode"]
    #[inline(always)]
    pub fn hispeedinten(&self) -> HISPEEDINTEN_R {
        HISPEEDINTEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enable completion DMA data transfer"]
    #[inline(always)]
    pub fn dmacmplinten(&self) -> DMACMPLINTEN_R {
        DMACMPLINTEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enable interrupt, signaling an unstable clock signal UTMI and to initialize the device controller registers"]
    #[inline(always)]
    pub fn clkunstblinten(&self) -> CLKUNSTBLINTEN_R {
        CLKUNSTBLINTEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable SOF packet reception"]
    #[inline(always)]
    pub fn sofinten(&mut self) -> SOFINTEN_W {
        SOFINTEN_W { w: self }
    }
    #[doc = "Bit 1 - Enable completion reset the root port"]
    #[inline(always)]
    pub fn resstatusinten(&mut self) -> RESSTATUSINTEN_W {
        RESSTATUSINTEN_W { w: self }
    }
    #[doc = "Bit 2 - Enable reactivate the device"]
    #[inline(always)]
    pub fn resumeinten(&mut self) -> RESUMEINTEN_W {
        RESUMEINTEN_W { w: self }
    }
    #[doc = "Bit 3 - Enable SUSPEND mode request bit"]
    #[inline(always)]
    pub fn suspendinten(&mut self) -> SUSPENDINTEN_W {
        SUSPENDINTEN_W { w: self }
    }
    #[doc = "Bit 4 - Enable completion reset and switching devices in high-speed mode"]
    #[inline(always)]
    pub fn hispeedinten(&mut self) -> HISPEEDINTEN_W {
        HISPEEDINTEN_W { w: self }
    }
    #[doc = "Bit 5 - Enable completion DMA data transfer"]
    #[inline(always)]
    pub fn dmacmplinten(&mut self) -> DMACMPLINTEN_W {
        DMACMPLINTEN_W { w: self }
    }
    #[doc = "Bit 6 - Enable interrupt, signaling an unstable clock signal UTMI and to initialize the device controller registers"]
    #[inline(always)]
    pub fn clkunstblinten(&mut self) -> CLKUNSTBLINTEN_W {
        CLKUNSTBLINTEN_W { w: self }
    }
}
