#[doc = "Reader of register OTG_IRQ_EN"]
pub type R = crate::R<u32, super::OTG_IRQ_EN>;
#[doc = "Writer for register OTG_IRQ_EN"]
pub type W = crate::W<u32, super::OTG_IRQ_EN>;
#[doc = "Register OTG_IRQ_EN `reset()`'s with value 0"]
impl crate::ResetValue for super::OTG_IRQ_EN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VBUSERREN`"]
pub type VBUSERREN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VBUSERREN`"]
pub struct VBUSERREN_W<'a> {
    w: &'a mut W,
}
impl<'a> VBUSERREN_W<'a> {
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
#[doc = "Reader of field `SPRDETECTEDEN`"]
pub type SPRDETECTEDEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPRDETECTEDEN`"]
pub struct SPRDETECTEDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPRDETECTEDEN_W<'a> {
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
#[doc = "Reader of field `TRANSINTREN`"]
pub type TRANSINTREN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRANSINTREN`"]
pub struct TRANSINTREN_W<'a> {
    w: &'a mut W,
}
impl<'a> TRANSINTREN_W<'a> {
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
#[doc = "Reader of field `SRPFAILEN`"]
pub type SRPFAILEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SRPFAILEN`"]
pub struct SRPFAILEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SRPFAILEN_W<'a> {
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
#[doc = "Reader of field `SESSIONFAILEN`"]
pub type SESSIONFAILEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SESSIONFAILEN`"]
pub struct SESSIONFAILEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SESSIONFAILEN_W<'a> {
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
#[doc = "Reader of field `A_DEVCHANGEEN`"]
pub type A_DEVCHANGEEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `A_DEVCHANGEEN`"]
pub struct A_DEVCHANGEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> A_DEVCHANGEEN_W<'a> {
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
#[doc = "Reader of field `B_DEVCHANGEEN`"]
pub type B_DEVCHANGEEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B_DEVCHANGEEN`"]
pub struct B_DEVCHANGEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> B_DEVCHANGEEN_W<'a> {
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
#[doc = "Reader of field `DEVSYNCEN`"]
pub type DEVSYNCEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DEVSYNCEN`"]
pub struct DEVSYNCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DEVSYNCEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `HOSTSYNCEN`"]
pub type HOSTSYNCEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HOSTSYNCEN`"]
pub struct HOSTSYNCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HOSTSYNCEN_W<'a> {
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
#[doc = "Reader of field `DISCONEN`"]
pub type DISCONEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DISCONEN`"]
pub struct DISCONEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DISCONEN_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Enable interrupt VBUSERR"]
    #[inline(always)]
    pub fn vbuserren(&self) -> VBUSERREN_R {
        VBUSERREN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable interrupt SPRDETECTED"]
    #[inline(always)]
    pub fn sprdetecteden(&self) -> SPRDETECTEDEN_R {
        SPRDETECTEDEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable interrupt TRANSINTR"]
    #[inline(always)]
    pub fn transintren(&self) -> TRANSINTREN_R {
        TRANSINTREN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable interrupt SRPFAIL"]
    #[inline(always)]
    pub fn srpfailen(&self) -> SRPFAILEN_R {
        SRPFAILEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable interrupt SESSIONFAIL"]
    #[inline(always)]
    pub fn sessionfailen(&self) -> SESSIONFAILEN_R {
        SESSIONFAILEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enable interrupt A_DEVCHANGE"]
    #[inline(always)]
    pub fn a_devchangeen(&self) -> A_DEVCHANGEEN_R {
        A_DEVCHANGEEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enable interrupt B_DEVCHANGE"]
    #[inline(always)]
    pub fn b_devchangeen(&self) -> B_DEVCHANGEEN_R {
        B_DEVCHANGEEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enable interrupt DEVSYNC"]
    #[inline(always)]
    pub fn devsyncen(&self) -> DEVSYNCEN_R {
        DEVSYNCEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enable interrupt HOSTSYNC (Resynchronization host)"]
    #[inline(always)]
    pub fn hostsyncen(&self) -> HOSTSYNCEN_R {
        HOSTSYNCEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Enable interrupt Disconnect"]
    #[inline(always)]
    pub fn disconen(&self) -> DISCONEN_R {
        DISCONEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable interrupt VBUSERR"]
    #[inline(always)]
    pub fn vbuserren(&mut self) -> VBUSERREN_W {
        VBUSERREN_W { w: self }
    }
    #[doc = "Bit 1 - Enable interrupt SPRDETECTED"]
    #[inline(always)]
    pub fn sprdetecteden(&mut self) -> SPRDETECTEDEN_W {
        SPRDETECTEDEN_W { w: self }
    }
    #[doc = "Bit 2 - Enable interrupt TRANSINTR"]
    #[inline(always)]
    pub fn transintren(&mut self) -> TRANSINTREN_W {
        TRANSINTREN_W { w: self }
    }
    #[doc = "Bit 3 - Enable interrupt SRPFAIL"]
    #[inline(always)]
    pub fn srpfailen(&mut self) -> SRPFAILEN_W {
        SRPFAILEN_W { w: self }
    }
    #[doc = "Bit 4 - Enable interrupt SESSIONFAIL"]
    #[inline(always)]
    pub fn sessionfailen(&mut self) -> SESSIONFAILEN_W {
        SESSIONFAILEN_W { w: self }
    }
    #[doc = "Bit 5 - Enable interrupt A_DEVCHANGE"]
    #[inline(always)]
    pub fn a_devchangeen(&mut self) -> A_DEVCHANGEEN_W {
        A_DEVCHANGEEN_W { w: self }
    }
    #[doc = "Bit 6 - Enable interrupt B_DEVCHANGE"]
    #[inline(always)]
    pub fn b_devchangeen(&mut self) -> B_DEVCHANGEEN_W {
        B_DEVCHANGEEN_W { w: self }
    }
    #[doc = "Bit 7 - Enable interrupt DEVSYNC"]
    #[inline(always)]
    pub fn devsyncen(&mut self) -> DEVSYNCEN_W {
        DEVSYNCEN_W { w: self }
    }
    #[doc = "Bit 8 - Enable interrupt HOSTSYNC (Resynchronization host)"]
    #[inline(always)]
    pub fn hostsyncen(&mut self) -> HOSTSYNCEN_W {
        HOSTSYNCEN_W { w: self }
    }
    #[doc = "Bit 9 - Enable interrupt Disconnect"]
    #[inline(always)]
    pub fn disconen(&mut self) -> DISCONEN_W {
        DISCONEN_W { w: self }
    }
}
