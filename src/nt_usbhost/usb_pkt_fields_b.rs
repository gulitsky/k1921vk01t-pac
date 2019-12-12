#[doc = "Reader of register USB_PKT_FIELDS_B"]
pub type R = crate::R<u32, super::USB_PKT_FIELDS_B>;
#[doc = "Writer for register USB_PKT_FIELDS_B"]
pub type W = crate::W<u32, super::USB_PKT_FIELDS_B>;
#[doc = "Register USB_PKT_FIELDS_B `reset()`'s with value 0"]
impl crate::ResetValue for super::USB_PKT_FIELDS_B {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MPS`"]
pub type MPS_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MPS`"]
pub struct MPS_W<'a> {
    w: &'a mut W,
}
impl<'a> MPS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | ((value as u32) & 0x07ff);
        self.w
    }
}
#[doc = "Reader of field `ABORTTRANSFER`"]
pub type ABORTTRANSFER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ABORTTRANSFER`"]
pub struct ABORTTRANSFER_W<'a> {
    w: &'a mut W,
}
impl<'a> ABORTTRANSFER_W<'a> {
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
#[doc = "Reader of field `XFRLEVEL`"]
pub type XFRLEVEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XFRLEVEL`"]
pub struct XFRLEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> XFRLEVEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `IGNORSHORTPKT`"]
pub type IGNORSHORTPKT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IGNORSHORTPKT`"]
pub struct IGNORSHORTPKT_W<'a> {
    w: &'a mut W,
}
impl<'a> IGNORSHORTPKT_W<'a> {
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
#[doc = "Reader of field `EFIELD`"]
pub type EFIELD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EFIELD`"]
pub struct EFIELD_W<'a> {
    w: &'a mut W,
}
impl<'a> EFIELD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `SPEED`"]
pub type SPEED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPEED`"]
pub struct SPEED_W<'a> {
    w: &'a mut W,
}
impl<'a> SPEED_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `COMPLETESPLIT`"]
pub type COMPLETESPLIT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COMPLETESPLIT`"]
pub struct COMPLETESPLIT_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPLETESPLIT_W<'a> {
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
#[doc = "Reader of field `STARTSPLIT`"]
pub type STARTSPLIT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STARTSPLIT`"]
pub struct STARTSPLIT_W<'a> {
    w: &'a mut W,
}
impl<'a> STARTSPLIT_W<'a> {
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
#[doc = "Reader of field `MAX_ERR_CNT`"]
pub type MAX_ERR_CNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MAX_ERR_CNT`"]
pub struct MAX_ERR_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> MAX_ERR_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Reader of field `MAX_NAK_CNT`"]
pub type MAX_NAK_CNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MAX_NAK_CNT`"]
pub struct MAX_NAK_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> MAX_NAK_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `SLAVEMODE`"]
pub type SLAVEMODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLAVEMODE`"]
pub struct SLAVEMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAVEMODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `OHCIENABLE`"]
pub type OHCIENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OHCIENABLE`"]
pub struct OHCIENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> OHCIENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn mps(&self) -> MPS_R {
        MPS_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 11 - Bit the current operation"]
    #[inline(always)]
    pub fn aborttransfer(&self) -> ABORTTRANSFER_R {
        ABORTTRANSFER_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn xfrlevel(&self) -> XFRLEVEL_R {
        XFRLEVEL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Enable bit ignore short packets"]
    #[inline(always)]
    pub fn ignorshortpkt(&self) -> IGNORSHORTPKT_R {
        IGNORSHORTPKT_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Bit transmission control"]
    #[inline(always)]
    pub fn efield(&self) -> EFIELD_R {
        EFIELD_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Indicator the type of device connected to the router and port"]
    #[inline(always)]
    pub fn speed(&self) -> SPEED_R {
        SPEED_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Complete bit operations such as Split"]
    #[inline(always)]
    pub fn completesplit(&self) -> COMPLETESPLIT_R {
        COMPLETESPLIT_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Bit run operations such as Split"]
    #[inline(always)]
    pub fn startsplit(&self) -> STARTSPLIT_R {
        STARTSPLIT_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bits 20:23 - Field the maximum number of consecutive transmission errors"]
    #[inline(always)]
    pub fn max_err_cnt(&self) -> MAX_ERR_CNT_R {
        MAX_ERR_CNT_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Maximum number of labels NAK, which is allowed to accept the host."]
    #[inline(always)]
    pub fn max_nak_cnt(&self) -> MAX_NAK_CNT_R {
        MAX_NAK_CNT_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - Enable slave mode"]
    #[inline(always)]
    pub fn slavemode(&self) -> SLAVEMODE_R {
        SLAVEMODE_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Demand indicator"]
    #[inline(always)]
    pub fn ohcienable(&self) -> OHCIENABLE_R {
        OHCIENABLE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn mps(&mut self) -> MPS_W {
        MPS_W { w: self }
    }
    #[doc = "Bit 11 - Bit the current operation"]
    #[inline(always)]
    pub fn aborttransfer(&mut self) -> ABORTTRANSFER_W {
        ABORTTRANSFER_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn xfrlevel(&mut self) -> XFRLEVEL_W {
        XFRLEVEL_W { w: self }
    }
    #[doc = "Bit 13 - Enable bit ignore short packets"]
    #[inline(always)]
    pub fn ignorshortpkt(&mut self) -> IGNORSHORTPKT_W {
        IGNORSHORTPKT_W { w: self }
    }
    #[doc = "Bit 14 - Bit transmission control"]
    #[inline(always)]
    pub fn efield(&mut self) -> EFIELD_W {
        EFIELD_W { w: self }
    }
    #[doc = "Bit 15 - Indicator the type of device connected to the router and port"]
    #[inline(always)]
    pub fn speed(&mut self) -> SPEED_W {
        SPEED_W { w: self }
    }
    #[doc = "Bit 16 - Complete bit operations such as Split"]
    #[inline(always)]
    pub fn completesplit(&mut self) -> COMPLETESPLIT_W {
        COMPLETESPLIT_W { w: self }
    }
    #[doc = "Bit 17 - Bit run operations such as Split"]
    #[inline(always)]
    pub fn startsplit(&mut self) -> STARTSPLIT_W {
        STARTSPLIT_W { w: self }
    }
    #[doc = "Bits 20:23 - Field the maximum number of consecutive transmission errors"]
    #[inline(always)]
    pub fn max_err_cnt(&mut self) -> MAX_ERR_CNT_W {
        MAX_ERR_CNT_W { w: self }
    }
    #[doc = "Bits 24:27 - Maximum number of labels NAK, which is allowed to accept the host."]
    #[inline(always)]
    pub fn max_nak_cnt(&mut self) -> MAX_NAK_CNT_W {
        MAX_NAK_CNT_W { w: self }
    }
    #[doc = "Bit 28 - Enable slave mode"]
    #[inline(always)]
    pub fn slavemode(&mut self) -> SLAVEMODE_W {
        SLAVEMODE_W { w: self }
    }
    #[doc = "Bit 31 - Demand indicator"]
    #[inline(always)]
    pub fn ohcienable(&mut self) -> OHCIENABLE_W {
        OHCIENABLE_W { w: self }
    }
}
