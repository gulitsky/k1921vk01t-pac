#[doc = "Reader of register OTG_IRQ_STAT"]
pub type R = crate::R<u32, super::OTG_IRQ_STAT>;
#[doc = "Writer for register OTG_IRQ_STAT"]
pub type W = crate::W<u32, super::OTG_IRQ_STAT>;
#[doc = "Register OTG_IRQ_STAT `reset()`'s with value 0"]
impl crate::ResetValue for super::OTG_IRQ_STAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VBUSERR`"]
pub type VBUSERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VBUSERR`"]
pub struct VBUSERR_W<'a> {
    w: &'a mut W,
}
impl<'a> VBUSERR_W<'a> {
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
#[doc = "Reader of field `SPRDETECTED`"]
pub type SPRDETECTED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPRDETECTED`"]
pub struct SPRDETECTED_W<'a> {
    w: &'a mut W,
}
impl<'a> SPRDETECTED_W<'a> {
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
#[doc = "Reader of field `HANDOFF_INTR`"]
pub type HANDOFF_INTR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HANDOFF_INTR`"]
pub struct HANDOFF_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> HANDOFF_INTR_W<'a> {
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
#[doc = "Reader of field `SPR_FAIL`"]
pub type SPR_FAIL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPR_FAIL`"]
pub struct SPR_FAIL_W<'a> {
    w: &'a mut W,
}
impl<'a> SPR_FAIL_W<'a> {
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
#[doc = "Reader of field `DEVICE`"]
pub type DEVICE_R = crate::R<bool, bool>;
#[doc = "Reader of field `HOST`"]
pub type HOST_R = crate::R<bool, bool>;
#[doc = "Reader of field `A_BUSREQ`"]
pub type A_BUSREQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `A_BUSREQ`"]
pub struct A_BUSREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> A_BUSREQ_W<'a> {
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
#[doc = "Reader of field `A_BUSDROP`"]
pub type A_BUSDROP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `A_BUSDROP`"]
pub struct A_BUSDROP_W<'a> {
    w: &'a mut W,
}
impl<'a> A_BUSDROP_W<'a> {
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
#[doc = "Reader of field `A_HNPEN`"]
pub type A_HNPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `A_HNPEN`"]
pub struct A_HNPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> A_HNPEN_W<'a> {
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
#[doc = "Reader of field `B_HNPEN`"]
pub type B_HNPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B_HNPEN`"]
pub struct B_HNPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> B_HNPEN_W<'a> {
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
#[doc = "Reader of field `B_BUSREQ`"]
pub type B_BUSREQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B_BUSREQ`"]
pub struct B_BUSREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> B_BUSREQ_W<'a> {
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
#[doc = "Reader of field `A_SUSPENDREQ`"]
pub type A_SUSPENDREQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `A_SUSPENDREQ`"]
pub struct A_SUSPENDREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> A_SUSPENDREQ_W<'a> {
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
#[doc = "Reader of field `A_DEVICE`"]
pub type A_DEVICE_R = crate::R<bool, bool>;
#[doc = "Reader of field `B_DEVICE`"]
pub type B_DEVICE_R = crate::R<bool, bool>;
#[doc = "Reader of field `SESSION_FAIL`"]
pub type SESSION_FAIL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SESSION_FAIL`"]
pub struct SESSION_FAIL_W<'a> {
    w: &'a mut W,
}
impl<'a> SESSION_FAIL_W<'a> {
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
#[doc = "Reader of field `SUSPENDEN`"]
pub type SUSPENDEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SUSPENDEN`"]
pub struct SUSPENDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SUSPENDEN_W<'a> {
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
#[doc = "Reader of field `A_DEVCHANGE`"]
pub type A_DEVCHANGE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `A_DEVCHANGE`"]
pub struct A_DEVCHANGE_W<'a> {
    w: &'a mut W,
}
impl<'a> A_DEVCHANGE_W<'a> {
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
#[doc = "Reader of field `B_DEVCHANGE`"]
pub type B_DEVCHANGE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B_DEVCHANGE`"]
pub struct B_DEVCHANGE_W<'a> {
    w: &'a mut W,
}
impl<'a> B_DEVCHANGE_W<'a> {
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
#[doc = "Reader of field `DEVSYNC`"]
pub type DEVSYNC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DEVSYNC`"]
pub struct DEVSYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> DEVSYNC_W<'a> {
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
#[doc = "Reader of field `HOSTSYNC`"]
pub type HOSTSYNC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HOSTSYNC`"]
pub struct HOSTSYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> HOSTSYNC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `DISCON`"]
pub type DISCON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DISCON`"]
pub struct DISCON_W<'a> {
    w: &'a mut W,
}
impl<'a> DISCON_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable interrupt a_clr_err"]
    #[inline(always)]
    pub fn vbuserr(&self) -> VBUSERR_R {
        VBUSERR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable interrupt srp_detected"]
    #[inline(always)]
    pub fn sprdetected(&self) -> SPRDETECTED_R {
        SPRDETECTED_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Interrupt when the output state Id"]
    #[inline(always)]
    pub fn handoff_intr(&self) -> HANDOFF_INTR_R {
        HANDOFF_INTR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Indication of lack of response from the host mode operation Device"]
    #[inline(always)]
    pub fn spr_fail(&self) -> SPR_FAIL_R {
        SPR_FAIL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Set when the kernel is running as a device controller"]
    #[inline(always)]
    pub fn device(&self) -> DEVICE_R {
        DEVICE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Set when the kernel is running as a host controller"]
    #[inline(always)]
    pub fn host(&self) -> HOST_R {
        HOST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Request A device control bus"]
    #[inline(always)]
    pub fn a_busreq(&self) -> A_BUSREQ_R {
        A_BUSREQ_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Set when the device A, you must disconnect bus"]
    #[inline(always)]
    pub fn a_busdrop(&self) -> A_BUSDROP_R {
        A_BUSDROP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Resolution run HNP device A"]
    #[inline(always)]
    pub fn a_hnpen(&self) -> A_HNPEN_R {
        A_HNPEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Resolution run HNP device A"]
    #[inline(always)]
    pub fn b_hnpen(&self) -> B_HNPEN_R {
        B_HNPEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Request bus control device B"]
    #[inline(always)]
    pub fn b_busreq(&self) -> B_BUSREQ_R {
        B_BUSREQ_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Request A suspension device"]
    #[inline(always)]
    pub fn a_suspendreq(&self) -> A_SUSPENDREQ_R {
        A_SUSPENDREQ_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Display device A connections"]
    #[inline(always)]
    pub fn a_device(&self) -> A_DEVICE_R {
        A_DEVICE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Display device B connections"]
    #[inline(always)]
    pub fn b_device(&self) -> B_DEVICE_R {
        B_DEVICE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Bit is device B, A when the device interrupts the signal Vbus."]
    #[inline(always)]
    pub fn session_fail(&self) -> SESSION_FAIL_R {
        SESSION_FAIL_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Request"]
    #[inline(always)]
    pub fn suspenden(&self) -> SUSPENDEN_R {
        SUSPENDEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Bit is set when there is a change device A. At the same time, the appropriate interrupt enable bit."]
    #[inline(always)]
    pub fn a_devchange(&self) -> A_DEVCHANGE_R {
        A_DEVCHANGE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Bit is set when you change the display B. Simultaneously, the appropriate interrupt enable bit"]
    #[inline(always)]
    pub fn b_devchange(&self) -> B_DEVCHANGE_R {
        B_DEVCHANGE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - When resynchronization device this bit is set and the corresponding interrupt enable bit"]
    #[inline(always)]
    pub fn devsync(&self) -> DEVSYNC_R {
        DEVSYNC_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - When resynchronization host this bit is set and the corresponding interrupt enable bit"]
    #[inline(always)]
    pub fn hostsync(&self) -> HOSTSYNC_R {
        HOSTSYNC_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Bit is set when disconnected not in suspend mode"]
    #[inline(always)]
    pub fn discon(&self) -> DISCON_R {
        DISCON_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable interrupt a_clr_err"]
    #[inline(always)]
    pub fn vbuserr(&mut self) -> VBUSERR_W {
        VBUSERR_W { w: self }
    }
    #[doc = "Bit 1 - Enable interrupt srp_detected"]
    #[inline(always)]
    pub fn sprdetected(&mut self) -> SPRDETECTED_W {
        SPRDETECTED_W { w: self }
    }
    #[doc = "Bit 2 - Interrupt when the output state Id"]
    #[inline(always)]
    pub fn handoff_intr(&mut self) -> HANDOFF_INTR_W {
        HANDOFF_INTR_W { w: self }
    }
    #[doc = "Bit 3 - Indication of lack of response from the host mode operation Device"]
    #[inline(always)]
    pub fn spr_fail(&mut self) -> SPR_FAIL_W {
        SPR_FAIL_W { w: self }
    }
    #[doc = "Bit 6 - Request A device control bus"]
    #[inline(always)]
    pub fn a_busreq(&mut self) -> A_BUSREQ_W {
        A_BUSREQ_W { w: self }
    }
    #[doc = "Bit 7 - Set when the device A, you must disconnect bus"]
    #[inline(always)]
    pub fn a_busdrop(&mut self) -> A_BUSDROP_W {
        A_BUSDROP_W { w: self }
    }
    #[doc = "Bit 8 - Resolution run HNP device A"]
    #[inline(always)]
    pub fn a_hnpen(&mut self) -> A_HNPEN_W {
        A_HNPEN_W { w: self }
    }
    #[doc = "Bit 9 - Resolution run HNP device A"]
    #[inline(always)]
    pub fn b_hnpen(&mut self) -> B_HNPEN_W {
        B_HNPEN_W { w: self }
    }
    #[doc = "Bit 10 - Request bus control device B"]
    #[inline(always)]
    pub fn b_busreq(&mut self) -> B_BUSREQ_W {
        B_BUSREQ_W { w: self }
    }
    #[doc = "Bit 11 - Request A suspension device"]
    #[inline(always)]
    pub fn a_suspendreq(&mut self) -> A_SUSPENDREQ_W {
        A_SUSPENDREQ_W { w: self }
    }
    #[doc = "Bit 14 - Bit is device B, A when the device interrupts the signal Vbus."]
    #[inline(always)]
    pub fn session_fail(&mut self) -> SESSION_FAIL_W {
        SESSION_FAIL_W { w: self }
    }
    #[doc = "Bit 15 - Request"]
    #[inline(always)]
    pub fn suspenden(&mut self) -> SUSPENDEN_W {
        SUSPENDEN_W { w: self }
    }
    #[doc = "Bit 16 - Bit is set when there is a change device A. At the same time, the appropriate interrupt enable bit."]
    #[inline(always)]
    pub fn a_devchange(&mut self) -> A_DEVCHANGE_W {
        A_DEVCHANGE_W { w: self }
    }
    #[doc = "Bit 17 - Bit is set when you change the display B. Simultaneously, the appropriate interrupt enable bit"]
    #[inline(always)]
    pub fn b_devchange(&mut self) -> B_DEVCHANGE_W {
        B_DEVCHANGE_W { w: self }
    }
    #[doc = "Bit 18 - When resynchronization device this bit is set and the corresponding interrupt enable bit"]
    #[inline(always)]
    pub fn devsync(&mut self) -> DEVSYNC_W {
        DEVSYNC_W { w: self }
    }
    #[doc = "Bit 19 - When resynchronization host this bit is set and the corresponding interrupt enable bit"]
    #[inline(always)]
    pub fn hostsync(&mut self) -> HOSTSYNC_W {
        HOSTSYNC_W { w: self }
    }
    #[doc = "Bit 20 - Bit is set when disconnected not in suspend mode"]
    #[inline(always)]
    pub fn discon(&mut self) -> DISCON_W {
        DISCON_W { w: self }
    }
}
