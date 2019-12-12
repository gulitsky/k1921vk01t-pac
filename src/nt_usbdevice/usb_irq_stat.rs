#[doc = "Reader of register USB_IRQ_STAT"]
pub type R = crate::R<u32, super::USB_IRQ_STAT>;
#[doc = "Writer for register USB_IRQ_STAT"]
pub type W = crate::W<u32, super::USB_IRQ_STAT>;
#[doc = "Register USB_IRQ_STAT `reset()`'s with value 0"]
impl crate::ResetValue for super::USB_IRQ_STAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SOF`"]
pub type SOF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOF`"]
pub struct SOF_W<'a> {
    w: &'a mut W,
}
impl<'a> SOF_W<'a> {
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
#[doc = "Reader of field `RESSTATUS`"]
pub type RESSTATUS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESSTATUS`"]
pub struct RESSTATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> RESSTATUS_W<'a> {
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
#[doc = "Reader of field `RESUME`"]
pub type RESUME_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESUME`"]
pub struct RESUME_W<'a> {
    w: &'a mut W,
}
impl<'a> RESUME_W<'a> {
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
#[doc = "Reader of field `SUSPENDREQ`"]
pub type SUSPENDREQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SUSPENDREQ`"]
pub struct SUSPENDREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> SUSPENDREQ_W<'a> {
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
#[doc = "Reader of field `HIGHSPEEDSETTLE`"]
pub type HIGHSPEEDSETTLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HIGHSPEEDSETTLE`"]
pub struct HIGHSPEEDSETTLE_W<'a> {
    w: &'a mut W,
}
impl<'a> HIGHSPEEDSETTLE_W<'a> {
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
#[doc = "Reader of field `DMACMPLINT`"]
pub type DMACMPLINT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMACMPLINT`"]
pub struct DMACMPLINT_W<'a> {
    w: &'a mut W,
}
impl<'a> DMACMPLINT_W<'a> {
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
#[doc = "Reader of field `CLKUNSTBLINT`"]
pub type CLKUNSTBLINT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLKUNSTBLINT`"]
pub struct CLKUNSTBLINT_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKUNSTBLINT_W<'a> {
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
    #[doc = "Bit 0 - Flag SOF packet reception"]
    #[inline(always)]
    pub fn sof(&self) -> SOF_R {
        SOF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Completion flag reset the root port"]
    #[inline(always)]
    pub fn resstatus(&self) -> RESSTATUS_R {
        RESSTATUS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Flag reactivate the device"]
    #[inline(always)]
    pub fn resume(&self) -> RESUME_R {
        RESUME_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SUSPEND mode request bit"]
    #[inline(always)]
    pub fn suspendreq(&self) -> SUSPENDREQ_R {
        SUSPENDREQ_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Completion flag reset and switching devices in high-speed mode"]
    #[inline(always)]
    pub fn highspeedsettle(&self) -> HIGHSPEEDSETTLE_R {
        HIGHSPEEDSETTLE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Completion flag DMA data transfer"]
    #[inline(always)]
    pub fn dmacmplint(&self) -> DMACMPLINT_R {
        DMACMPLINT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Interrupt flag, signaling an unstable clock signal UTMI and to initialize the device controller registers"]
    #[inline(always)]
    pub fn clkunstblint(&self) -> CLKUNSTBLINT_R {
        CLKUNSTBLINT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Flag SOF packet reception"]
    #[inline(always)]
    pub fn sof(&mut self) -> SOF_W {
        SOF_W { w: self }
    }
    #[doc = "Bit 1 - Completion flag reset the root port"]
    #[inline(always)]
    pub fn resstatus(&mut self) -> RESSTATUS_W {
        RESSTATUS_W { w: self }
    }
    #[doc = "Bit 2 - Flag reactivate the device"]
    #[inline(always)]
    pub fn resume(&mut self) -> RESUME_W {
        RESUME_W { w: self }
    }
    #[doc = "Bit 3 - SUSPEND mode request bit"]
    #[inline(always)]
    pub fn suspendreq(&mut self) -> SUSPENDREQ_W {
        SUSPENDREQ_W { w: self }
    }
    #[doc = "Bit 4 - Completion flag reset and switching devices in high-speed mode"]
    #[inline(always)]
    pub fn highspeedsettle(&mut self) -> HIGHSPEEDSETTLE_W {
        HIGHSPEEDSETTLE_W { w: self }
    }
    #[doc = "Bit 5 - Completion flag DMA data transfer"]
    #[inline(always)]
    pub fn dmacmplint(&mut self) -> DMACMPLINT_W {
        DMACMPLINT_W { w: self }
    }
    #[doc = "Bit 6 - Interrupt flag, signaling an unstable clock signal UTMI and to initialize the device controller registers"]
    #[inline(always)]
    pub fn clkunstblint(&mut self) -> CLKUNSTBLINT_W {
        CLKUNSTBLINT_W { w: self }
    }
}
