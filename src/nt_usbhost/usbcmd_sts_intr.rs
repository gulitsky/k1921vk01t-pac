#[doc = "Reader of register USBCMD_STS_INTR"]
pub type R = crate::R<u32, super::USBCMD_STS_INTR>;
#[doc = "Writer for register USBCMD_STS_INTR"]
pub type W = crate::W<u32, super::USBCMD_STS_INTR>;
#[doc = "Register USBCMD_STS_INTR `reset()`'s with value 0"]
impl crate::ResetValue for super::USBCMD_STS_INTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OUTSPACEINTEN`"]
pub type OUTSPACEINTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INDATAREADYINTEN`"]
pub struct INDATAREADYINTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> INDATAREADYINTEN_W<'a> {
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
#[doc = "Write proxy for field `FRAMESOFINTEN`"]
pub struct FRAMESOFINTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAMESOFINTEN_W<'a> {
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
#[doc = "Write proxy for field `MICROFRAMEINTEN`"]
pub struct MICROFRAMEINTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MICROFRAMEINTEN_W<'a> {
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
#[doc = "Write proxy for field `PORTCHANGINTEN`"]
pub struct PORTCHANGINTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PORTCHANGINTEN_W<'a> {
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
#[doc = "Reader of field `FRAMESOFINT`"]
pub type FRAMESOFINT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FRAMESOFINT`"]
pub struct FRAMESOFINT_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAMESOFINT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `MICROFRAMEINT`"]
pub type MICROFRAMEINT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MICROFRAMEINT`"]
pub struct MICROFRAMEINT_W<'a> {
    w: &'a mut W,
}
impl<'a> MICROFRAMEINT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `PORTCHANGINT`"]
pub type PORTCHANGINT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PORTCHANGINT`"]
pub struct PORTCHANGINT_W<'a> {
    w: &'a mut W,
}
impl<'a> PORTCHANGINT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `HCHALTED`"]
pub type HCHALTED_R = crate::R<bool, bool>;
#[doc = "Reader of field `RUN_STOP`"]
pub type RUN_STOP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RUN_STOP`"]
pub struct RUN_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> RUN_STOP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `HOSTRESET`"]
pub type HOSTRESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HOSTRESET`"]
pub struct HOSTRESET_W<'a> {
    w: &'a mut W,
}
impl<'a> HOSTRESET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `CONFIGFLAG`"]
pub type CONFIGFLAG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CONFIGFLAG`"]
pub struct CONFIGFLAG_W<'a> {
    w: &'a mut W,
}
impl<'a> CONFIGFLAG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `INT_SPACE`"]
pub type INT_SPACE_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 1 - Hardware interrupt enable bit"]
    #[inline(always)]
    pub fn outspaceinten(&self) -> OUTSPACEINTEN_R {
        OUTSPACEINTEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Interrupt flag Framesof"]
    #[inline(always)]
    pub fn framesofint(&self) -> FRAMESOFINT_R {
        FRAMESOFINT_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Interrupt flag Microframesof"]
    #[inline(always)]
    pub fn microframeint(&self) -> MICROFRAMEINT_R {
        MICROFRAMEINT_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Indicate the any port has a change bit transmission from 0 to 1"]
    #[inline(always)]
    pub fn portchangint(&self) -> PORTCHANGINT_R {
        PORTCHANGINT_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Indicator controller stop"]
    #[inline(always)]
    pub fn hchalted(&self) -> HCHALTED_R {
        HCHALTED_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Perfoms data"]
    #[inline(always)]
    pub fn run_stop(&self) -> RUN_STOP_R {
        RUN_STOP_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Initial state of USB-controller"]
    #[inline(always)]
    pub fn hostreset(&self) -> HOSTRESET_R {
        HOSTRESET_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Configurate flag"]
    #[inline(always)]
    pub fn configflag(&self) -> CONFIGFLAG_R {
        CONFIGFLAG_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn int_space(&self) -> INT_SPACE_R {
        INT_SPACE_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 2 - Enable interrupt InDataReady"]
    #[inline(always)]
    pub fn indatareadyinten(&mut self) -> INDATAREADYINTEN_W {
        INDATAREADYINTEN_W { w: self }
    }
    #[doc = "Bit 3 - Enable interrupt Frame sof"]
    #[inline(always)]
    pub fn framesofinten(&mut self) -> FRAMESOFINTEN_W {
        FRAMESOFINTEN_W { w: self }
    }
    #[doc = "Bit 4 - Enable interrupt Microframe soa"]
    #[inline(always)]
    pub fn microframeinten(&mut self) -> MICROFRAMEINTEN_W {
        MICROFRAMEINTEN_W { w: self }
    }
    #[doc = "Bit 6 - Enable interrupt Port change detect"]
    #[inline(always)]
    pub fn portchanginten(&mut self) -> PORTCHANGINTEN_W {
        PORTCHANGINTEN_W { w: self }
    }
    #[doc = "Bit 10 - Interrupt flag Framesof"]
    #[inline(always)]
    pub fn framesofint(&mut self) -> FRAMESOFINT_W {
        FRAMESOFINT_W { w: self }
    }
    #[doc = "Bit 11 - Interrupt flag Microframesof"]
    #[inline(always)]
    pub fn microframeint(&mut self) -> MICROFRAMEINT_W {
        MICROFRAMEINT_W { w: self }
    }
    #[doc = "Bit 13 - Indicate the any port has a change bit transmission from 0 to 1"]
    #[inline(always)]
    pub fn portchangint(&mut self) -> PORTCHANGINT_W {
        PORTCHANGINT_W { w: self }
    }
    #[doc = "Bit 16 - Perfoms data"]
    #[inline(always)]
    pub fn run_stop(&mut self) -> RUN_STOP_W {
        RUN_STOP_W { w: self }
    }
    #[doc = "Bit 17 - Initial state of USB-controller"]
    #[inline(always)]
    pub fn hostreset(&mut self) -> HOSTRESET_W {
        HOSTRESET_W { w: self }
    }
    #[doc = "Bit 18 - Configurate flag"]
    #[inline(always)]
    pub fn configflag(&mut self) -> CONFIGFLAG_W {
        CONFIGFLAG_W { w: self }
    }
}
