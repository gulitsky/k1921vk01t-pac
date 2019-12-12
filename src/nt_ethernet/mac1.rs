#[doc = "Reader of register MAC1"]
pub type R = crate::R<u32, super::MAC1>;
#[doc = "Writer for register MAC1"]
pub type W = crate::W<u32, super::MAC1>;
#[doc = "Register MAC1 `reset()`'s with value 0"]
impl crate::ResetValue for super::MAC1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RXENABLE`"]
pub type RXENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXENABLE`"]
pub struct RXENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXENABLE_W<'a> {
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
#[doc = "Reader of field `PASSALL`"]
pub type PASSALL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PASSALL`"]
pub struct PASSALL_W<'a> {
    w: &'a mut W,
}
impl<'a> PASSALL_W<'a> {
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
#[doc = "Reader of field `RXPAUSE`"]
pub type RXPAUSE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXPAUSE`"]
pub struct RXPAUSE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXPAUSE_W<'a> {
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
#[doc = "Reader of field `TXPAUSE`"]
pub type TXPAUSE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXPAUSE`"]
pub struct TXPAUSE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXPAUSE_W<'a> {
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
#[doc = "Reader of field `LOOPBACK`"]
pub type LOOPBACK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOOPBACK`"]
pub struct LOOPBACK_W<'a> {
    w: &'a mut W,
}
impl<'a> LOOPBACK_W<'a> {
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
#[doc = "Reader of field `RESETTFUN`"]
pub type RESETTFUN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESETTFUN`"]
pub struct RESETTFUN_W<'a> {
    w: &'a mut W,
}
impl<'a> RESETTFUN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `RESETTMCS`"]
pub type RESETTMCS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESETTMCS`"]
pub struct RESETTMCS_W<'a> {
    w: &'a mut W,
}
impl<'a> RESETTMCS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `RESETRFUN`"]
pub type RESETRFUN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESETRFUN`"]
pub struct RESETRFUN_W<'a> {
    w: &'a mut W,
}
impl<'a> RESETRFUN_W<'a> {
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
#[doc = "Reader of field `RESETRMCS`"]
pub type RESETRMCS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESETRMCS`"]
pub struct RESETRMCS_W<'a> {
    w: &'a mut W,
}
impl<'a> RESETRMCS_W<'a> {
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
#[doc = "Reader of field `SIMRESET`"]
pub type SIMRESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SIMRESET`"]
pub struct SIMRESET_W<'a> {
    w: &'a mut W,
}
impl<'a> SIMRESET_W<'a> {
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
#[doc = "Reader of field `SOFTRESET`"]
pub type SOFTRESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFTRESET`"]
pub struct SOFTRESET_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTRESET_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Frame reception enable bit"]
    #[inline(always)]
    pub fn rxenable(&self) -> RXENABLE_R {
        RXENABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Bit control PASS"]
    #[inline(always)]
    pub fn passall(&self) -> PASSALL_R {
        PASSALL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable receiving a pause as part of Frame"]
    #[inline(always)]
    pub fn rxpause(&self) -> RXPAUSE_R {
        RXPAUSE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable transmission pauses Frame"]
    #[inline(always)]
    pub fn txpause(&self) -> TXPAUSE_R {
        TXPAUSE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Bit activate reception of packets transmitted back through MACReceive-interface"]
    #[inline(always)]
    pub fn loopback(&self) -> LOOPBACK_R {
        LOOPBACK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Reset bit logic devices packet"]
    #[inline(always)]
    pub fn resettfun(&self) -> RESETTFUN_R {
        RESETTFUN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Reset bit device MAC layer, responsible for managing addresses in information transmission Reset bit device MAC layer, responsible for managing addresses in information transmission"]
    #[inline(always)]
    pub fn resettmcs(&self) -> RESETTMCS_R {
        RESETTMCS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Reset bit logic devices receive packets"]
    #[inline(always)]
    pub fn resetrfun(&self) -> RESETRFUN_R {
        RESETRFUN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Reset bit device MAC layer, responsible for managing the filtering addresses of packets received"]
    #[inline(always)]
    pub fn resetrmcs(&self) -> RESETRMCS_R {
        RESETRMCS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Reset bit random number generator transmitting device"]
    #[inline(always)]
    pub fn simreset(&self) -> SIMRESET_R {
        SIMRESET_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Reset bit block MAC Ethernet Controller"]
    #[inline(always)]
    pub fn softreset(&self) -> SOFTRESET_R {
        SOFTRESET_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Frame reception enable bit"]
    #[inline(always)]
    pub fn rxenable(&mut self) -> RXENABLE_W {
        RXENABLE_W { w: self }
    }
    #[doc = "Bit 1 - Bit control PASS"]
    #[inline(always)]
    pub fn passall(&mut self) -> PASSALL_W {
        PASSALL_W { w: self }
    }
    #[doc = "Bit 2 - Enable receiving a pause as part of Frame"]
    #[inline(always)]
    pub fn rxpause(&mut self) -> RXPAUSE_W {
        RXPAUSE_W { w: self }
    }
    #[doc = "Bit 3 - Enable transmission pauses Frame"]
    #[inline(always)]
    pub fn txpause(&mut self) -> TXPAUSE_W {
        TXPAUSE_W { w: self }
    }
    #[doc = "Bit 4 - Bit activate reception of packets transmitted back through MACReceive-interface"]
    #[inline(always)]
    pub fn loopback(&mut self) -> LOOPBACK_W {
        LOOPBACK_W { w: self }
    }
    #[doc = "Bit 8 - Reset bit logic devices packet"]
    #[inline(always)]
    pub fn resettfun(&mut self) -> RESETTFUN_W {
        RESETTFUN_W { w: self }
    }
    #[doc = "Bit 9 - Reset bit device MAC layer, responsible for managing addresses in information transmission Reset bit device MAC layer, responsible for managing addresses in information transmission"]
    #[inline(always)]
    pub fn resettmcs(&mut self) -> RESETTMCS_W {
        RESETTMCS_W { w: self }
    }
    #[doc = "Bit 10 - Reset bit logic devices receive packets"]
    #[inline(always)]
    pub fn resetrfun(&mut self) -> RESETRFUN_W {
        RESETRFUN_W { w: self }
    }
    #[doc = "Bit 11 - Reset bit device MAC layer, responsible for managing the filtering addresses of packets received"]
    #[inline(always)]
    pub fn resetrmcs(&mut self) -> RESETRMCS_W {
        RESETRMCS_W { w: self }
    }
    #[doc = "Bit 14 - Reset bit random number generator transmitting device"]
    #[inline(always)]
    pub fn simreset(&mut self) -> SIMRESET_W {
        SIMRESET_W { w: self }
    }
    #[doc = "Bit 15 - Reset bit block MAC Ethernet Controller"]
    #[inline(always)]
    pub fn softreset(&mut self) -> SOFTRESET_W {
        SOFTRESET_W { w: self }
    }
}
