#[doc = "Reader of register PORT_STS"]
pub type R = crate::R<u32, super::PORT_STS>;
#[doc = "Writer for register PORT_STS"]
pub type W = crate::W<u32, super::PORT_STS>;
#[doc = "Register PORT_STS `reset()`'s with value 0"]
impl crate::ResetValue for super::PORT_STS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PORTCONNECT`"]
pub type PORTCONNECT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PORTCONNECT`"]
pub struct PORTCONNECT_W<'a> {
    w: &'a mut W,
}
impl<'a> PORTCONNECT_W<'a> {
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
#[doc = "Reader of field `PORTCONCHNG`"]
pub type PORTCONCHNG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PORTCONCHNG`"]
pub struct PORTCONCHNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PORTCONCHNG_W<'a> {
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
#[doc = "Reader of field `PORTENABLE`"]
pub type PORTENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PORTENABLE`"]
pub struct PORTENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> PORTENABLE_W<'a> {
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
#[doc = "Reader of field `PORTEN_DISCHNG`"]
pub type PORTEN_DISCHNG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PORTEN_DISCHNG`"]
pub struct PORTEN_DISCHNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PORTEN_DISCHNG_W<'a> {
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
#[doc = "Reader of field `PORTRESET`"]
pub type PORTRESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PORTRESET`"]
pub struct PORTRESET_W<'a> {
    w: &'a mut W,
}
impl<'a> PORTRESET_W<'a> {
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
#[doc = "Reader of field `LINESTATE`"]
pub type LINESTATE_R = crate::R<u8, u8>;
#[doc = "Reader of field `PORTOWNER0`"]
pub type PORTOWNER0_R = crate::R<bool, bool>;
#[doc = "Reader of field `SUSPEND`"]
pub type SUSPEND_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SUSPEND`"]
pub struct SUSPEND_W<'a> {
    w: &'a mut W,
}
impl<'a> SUSPEND_W<'a> {
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
#[doc = "Reader of field `FORCEPORTRESUME`"]
pub type FORCEPORTRESUME_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FORCEPORTRESUME`"]
pub struct FORCEPORTRESUME_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCEPORTRESUME_W<'a> {
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
#[doc = "Reader of field `XCVRRESET`"]
pub type XCVRRESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XCVRRESET`"]
pub struct XCVRRESET_W<'a> {
    w: &'a mut W,
}
impl<'a> XCVRRESET_W<'a> {
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
#[doc = "Reader of field `XCVRRESLEVEL`"]
pub type XCVRRESLEVEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XCVRRESLEVEL`"]
pub struct XCVRRESLEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> XCVRRESLEVEL_W<'a> {
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
#[doc = "Reader of field `XCVRSUSPEND`"]
pub type XCVRSUSPEND_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XCVRSUSPEND`"]
pub struct XCVRSUSPEND_W<'a> {
    w: &'a mut W,
}
impl<'a> XCVRSUSPEND_W<'a> {
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
#[doc = "Reader of field `TESTJ`"]
pub type TESTJ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TESTJ`"]
pub struct TESTJ_W<'a> {
    w: &'a mut W,
}
impl<'a> TESTJ_W<'a> {
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
#[doc = "Reader of field `TESTK`"]
pub type TESTK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TESTK`"]
pub struct TESTK_W<'a> {
    w: &'a mut W,
}
impl<'a> TESTK_W<'a> {
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
#[doc = "Reader of field `TESTSEO`"]
pub type TESTSEO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TESTSEO`"]
pub struct TESTSEO_W<'a> {
    w: &'a mut W,
}
impl<'a> TESTSEO_W<'a> {
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
#[doc = "Reader of field `TESTFORCEEN`"]
pub type TESTFORCEEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TESTFORCEEN`"]
pub struct TESTFORCEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TESTFORCEEN_W<'a> {
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
#[doc = "Reader of field `TESTPKT`"]
pub type TESTPKT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TESTPKT`"]
pub struct TESTPKT_W<'a> {
    w: &'a mut W,
}
impl<'a> TESTPKT_W<'a> {
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
#[doc = "Reader of field `PORTOWNER1`"]
pub type PORTOWNER1_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Indicator device connection"]
    #[inline(always)]
    pub fn portconnect(&self) -> PORTCONNECT_R {
        PORTCONNECT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Indicator status change bit PORTCONNECT"]
    #[inline(always)]
    pub fn portconchng(&self) -> PORTCONCHNG_R {
        PORTCONCHNG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable bit port"]
    #[inline(always)]
    pub fn portenable(&self) -> PORTENABLE_R {
        PORTENABLE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Indicator status change bit PORTENABLE"]
    #[inline(always)]
    pub fn porten_dischng(&self) -> PORTEN_DISCHNG_R {
        PORTEN_DISCHNG_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Setting bit starts the reset procedure in accordance with USB2.0"]
    #[inline(always)]
    pub fn portreset(&self) -> PORTRESET_R {
        PORTRESET_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - Field reflect the current logic level signals D + and D-"]
    #[inline(always)]
    pub fn linestate(&self) -> LINESTATE_R {
        LINESTATE_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 7 - Bit with bit PORTOWNER1 assignment of the port speed"]
    #[inline(always)]
    pub fn portowner0(&self) -> PORTOWNER0_R {
        PORTOWNER0_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Bit suspension of work"]
    #[inline(always)]
    pub fn suspend(&self) -> SUSPEND_R {
        SUSPEND_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Flag resume"]
    #[inline(always)]
    pub fn forceportresume(&self) -> FORCEPORTRESUME_R {
        FORCEPORTRESUME_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Reset bit transceiver"]
    #[inline(always)]
    pub fn xcvrreset(&self) -> XCVRRESET_R {
        XCVRRESET_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Bit setting active level reset signal transceiver"]
    #[inline(always)]
    pub fn xcvrreslevel(&self) -> XCVRRESLEVEL_R {
        XCVRRESLEVEL_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn xcvrsuspend(&self) -> XCVRSUSPEND_R {
        XCVRSUSPEND_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - keep in line condition HS J"]
    #[inline(always)]
    pub fn testj(&self) -> TESTJ_R {
        TESTJ_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Enable keep in line condition HS K"]
    #[inline(always)]
    pub fn testk(&self) -> TESTK_R {
        TESTK_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Enable keep in line condition SEO"]
    #[inline(always)]
    pub fn testseo(&self) -> TESTSEO_R {
        TESTSEO_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Enable transmit packets looped"]
    #[inline(always)]
    pub fn testforceen(&self) -> TESTFORCEEN_R {
        TESTFORCEEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Bits select the standard USB"]
    #[inline(always)]
    pub fn testpkt(&self) -> TESTPKT_R {
        TESTPKT_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Indicator low-speed devices"]
    #[inline(always)]
    pub fn portowner1(&self) -> PORTOWNER1_R {
        PORTOWNER1_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Indicator device connection"]
    #[inline(always)]
    pub fn portconnect(&mut self) -> PORTCONNECT_W {
        PORTCONNECT_W { w: self }
    }
    #[doc = "Bit 1 - Indicator status change bit PORTCONNECT"]
    #[inline(always)]
    pub fn portconchng(&mut self) -> PORTCONCHNG_W {
        PORTCONCHNG_W { w: self }
    }
    #[doc = "Bit 2 - Enable bit port"]
    #[inline(always)]
    pub fn portenable(&mut self) -> PORTENABLE_W {
        PORTENABLE_W { w: self }
    }
    #[doc = "Bit 3 - Indicator status change bit PORTENABLE"]
    #[inline(always)]
    pub fn porten_dischng(&mut self) -> PORTEN_DISCHNG_W {
        PORTEN_DISCHNG_W { w: self }
    }
    #[doc = "Bit 4 - Setting bit starts the reset procedure in accordance with USB2.0"]
    #[inline(always)]
    pub fn portreset(&mut self) -> PORTRESET_W {
        PORTRESET_W { w: self }
    }
    #[doc = "Bit 8 - Bit suspension of work"]
    #[inline(always)]
    pub fn suspend(&mut self) -> SUSPEND_W {
        SUSPEND_W { w: self }
    }
    #[doc = "Bit 9 - Flag resume"]
    #[inline(always)]
    pub fn forceportresume(&mut self) -> FORCEPORTRESUME_W {
        FORCEPORTRESUME_W { w: self }
    }
    #[doc = "Bit 10 - Reset bit transceiver"]
    #[inline(always)]
    pub fn xcvrreset(&mut self) -> XCVRRESET_W {
        XCVRRESET_W { w: self }
    }
    #[doc = "Bit 11 - Bit setting active level reset signal transceiver"]
    #[inline(always)]
    pub fn xcvrreslevel(&mut self) -> XCVRRESLEVEL_W {
        XCVRRESLEVEL_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn xcvrsuspend(&mut self) -> XCVRSUSPEND_W {
        XCVRSUSPEND_W { w: self }
    }
    #[doc = "Bit 13 - keep in line condition HS J"]
    #[inline(always)]
    pub fn testj(&mut self) -> TESTJ_W {
        TESTJ_W { w: self }
    }
    #[doc = "Bit 14 - Enable keep in line condition HS K"]
    #[inline(always)]
    pub fn testk(&mut self) -> TESTK_W {
        TESTK_W { w: self }
    }
    #[doc = "Bit 15 - Enable keep in line condition SEO"]
    #[inline(always)]
    pub fn testseo(&mut self) -> TESTSEO_W {
        TESTSEO_W { w: self }
    }
    #[doc = "Bit 16 - Enable transmit packets looped"]
    #[inline(always)]
    pub fn testforceen(&mut self) -> TESTFORCEEN_W {
        TESTFORCEEN_W { w: self }
    }
    #[doc = "Bit 17 - Bits select the standard USB"]
    #[inline(always)]
    pub fn testpkt(&mut self) -> TESTPKT_W {
        TESTPKT_W { w: self }
    }
}
