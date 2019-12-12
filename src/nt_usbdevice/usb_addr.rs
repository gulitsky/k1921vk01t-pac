#[doc = "Reader of register USB_ADDR"]
pub type R = crate::R<u32, super::USB_ADDR>;
#[doc = "Writer for register USB_ADDR"]
pub type W = crate::W<u32, super::USB_ADDR>;
#[doc = "Register USB_ADDR `reset()`'s with value 0"]
impl crate::ResetValue for super::USB_ADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CURRENT_ADDR`"]
pub type CURRENT_ADDR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CURRENT_ADDR`"]
pub struct CURRENT_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> CURRENT_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Reader of field `USBADDR`"]
pub type USBADDR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USBADDR`"]
pub struct USBADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> USBADDR_W<'a> {
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
    #[doc = "Bits 0:5 - Field of the current address of the device"]
    #[inline(always)]
    pub fn current_addr(&self) -> CURRENT_ADDR_R {
        CURRENT_ADDR_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - Field of the current address of the device"]
    #[inline(always)]
    pub fn usbaddr(&self) -> USBADDR_R {
        USBADDR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Field of the current address of the device"]
    #[inline(always)]
    pub fn current_addr(&mut self) -> CURRENT_ADDR_W {
        CURRENT_ADDR_W { w: self }
    }
    #[doc = "Bit 6 - Field of the current address of the device"]
    #[inline(always)]
    pub fn usbaddr(&mut self) -> USBADDR_W {
        USBADDR_W { w: self }
    }
}
