#[doc = "Reader of register USB_PKT_FIELDS_A"]
pub type R = crate::R<u32, super::USB_PKT_FIELDS_A>;
#[doc = "Writer for register USB_PKT_FIELDS_A"]
pub type W = crate::W<u32, super::USB_PKT_FIELDS_A>;
#[doc = "Register USB_PKT_FIELDS_A `reset()`'s with value 0"]
impl crate::ResetValue for super::USB_PKT_FIELDS_A {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EP_ADDR`"]
pub type EP_ADDR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EP_ADDR`"]
pub struct EP_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PID_A {
    #[doc = "0: Out"]
    OUT,
    #[doc = "1: In"]
    IN,
    #[doc = "2: Setup"]
    SETUP,
    #[doc = "3: Ping"]
    PING,
}
impl From<PID_A> for u8 {
    #[inline(always)]
    fn from(variant: PID_A) -> Self {
        match variant {
            PID_A::OUT => 0,
            PID_A::IN => 1,
            PID_A::SETUP => 2,
            PID_A::PING => 3,
        }
    }
}
#[doc = "Reader of field `PID`"]
pub type PID_R = crate::R<u8, PID_A>;
impl PID_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PID_A {
        match self.bits {
            0 => PID_A::OUT,
            1 => PID_A::IN,
            2 => PID_A::SETUP,
            3 => PID_A::PING,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OUT`"]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        *self == PID_A::OUT
    }
    #[doc = "Checks if the value of the field is `IN`"]
    #[inline(always)]
    pub fn is_in_(&self) -> bool {
        *self == PID_A::IN
    }
    #[doc = "Checks if the value of the field is `SETUP`"]
    #[inline(always)]
    pub fn is_setup(&self) -> bool {
        *self == PID_A::SETUP
    }
    #[doc = "Checks if the value of the field is `PING`"]
    #[inline(always)]
    pub fn is_ping(&self) -> bool {
        *self == PID_A::PING
    }
}
#[doc = "Write proxy for field `PID`"]
pub struct PID_W<'a> {
    w: &'a mut W,
}
impl<'a> PID_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PID_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Out"]
    #[inline(always)]
    pub fn out(self) -> &'a mut W {
        self.variant(PID_A::OUT)
    }
    #[doc = "In"]
    #[inline(always)]
    pub fn in_(self) -> &'a mut W {
        self.variant(PID_A::IN)
    }
    #[doc = "Setup"]
    #[inline(always)]
    pub fn setup(self) -> &'a mut W {
        self.variant(PID_A::SETUP)
    }
    #[doc = "Ping"]
    #[inline(always)]
    pub fn ping(self) -> &'a mut W {
        self.variant(PID_A::PING)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `ENDPOINTTYPE`"]
pub type ENDPOINTTYPE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ENDPOINTTYPE`"]
pub struct ENDPOINTTYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDPOINTTYPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `DEV_ADDR`"]
pub type DEV_ADDR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DEV_ADDR`"]
pub struct DEV_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> DEV_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u32) & 0x7f) << 8);
        self.w
    }
}
#[doc = "Reader of field `IOC`"]
pub type IOC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IOC`"]
pub struct IOC_W<'a> {
    w: &'a mut W,
}
impl<'a> IOC_W<'a> {
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
#[doc = "Reader of field `PORT_NUMBER`"]
pub type PORT_NUMBER_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PORT_NUMBER`"]
pub struct PORT_NUMBER_W<'a> {
    w: &'a mut W,
}
impl<'a> PORT_NUMBER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | (((value as u32) & 0x7f) << 16);
        self.w
    }
}
#[doc = "Reader of field `DATATOGGLE`"]
pub type DATATOGGLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DATATOGGLE`"]
pub struct DATATOGGLE_W<'a> {
    w: &'a mut W,
}
impl<'a> DATATOGGLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `HUB_ADDRESS`"]
pub type HUB_ADDRESS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HUB_ADDRESS`"]
pub struct HUB_ADDRESS_W<'a> {
    w: &'a mut W,
}
impl<'a> HUB_ADDRESS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 24)) | (((value as u32) & 0x7f) << 24);
        self.w
    }
}
#[doc = "Reader of field `ENTRANSFER`"]
pub type ENTRANSFER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENTRANSFER`"]
pub struct ENTRANSFER_W<'a> {
    w: &'a mut W,
}
impl<'a> ENTRANSFER_W<'a> {
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
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn ep_addr(&self) -> EP_ADDR_R {
        EP_ADDR_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn pid(&self) -> PID_R {
        PID_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Buffer type"]
    #[inline(always)]
    pub fn endpointtype(&self) -> ENDPOINTTYPE_R {
        ENDPOINTTYPE_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:14"]
    #[inline(always)]
    pub fn dev_addr(&self) -> DEV_ADDR_R {
        DEV_ADDR_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - Bits specify the behavior of the host controller when transfer"]
    #[inline(always)]
    pub fn ioc(&self) -> IOC_R {
        IOC_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:22"]
    #[inline(always)]
    pub fn port_number(&self) -> PORT_NUMBER_R {
        PORT_NUMBER_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 23 - Identifier field data to be used in the current transmission"]
    #[inline(always)]
    pub fn datatoggle(&self) -> DATATOGGLE_R {
        DATATOGGLE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:30"]
    #[inline(always)]
    pub fn hub_address(&self) -> HUB_ADDRESS_R {
        HUB_ADDRESS_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
    #[doc = "Bit 31 - Beginning of the current permission bits of data"]
    #[inline(always)]
    pub fn entransfer(&self) -> ENTRANSFER_R {
        ENTRANSFER_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn ep_addr(&mut self) -> EP_ADDR_W {
        EP_ADDR_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn pid(&mut self) -> PID_W {
        PID_W { w: self }
    }
    #[doc = "Bits 6:7 - Buffer type"]
    #[inline(always)]
    pub fn endpointtype(&mut self) -> ENDPOINTTYPE_W {
        ENDPOINTTYPE_W { w: self }
    }
    #[doc = "Bits 8:14"]
    #[inline(always)]
    pub fn dev_addr(&mut self) -> DEV_ADDR_W {
        DEV_ADDR_W { w: self }
    }
    #[doc = "Bit 15 - Bits specify the behavior of the host controller when transfer"]
    #[inline(always)]
    pub fn ioc(&mut self) -> IOC_W {
        IOC_W { w: self }
    }
    #[doc = "Bits 16:22"]
    #[inline(always)]
    pub fn port_number(&mut self) -> PORT_NUMBER_W {
        PORT_NUMBER_W { w: self }
    }
    #[doc = "Bit 23 - Identifier field data to be used in the current transmission"]
    #[inline(always)]
    pub fn datatoggle(&mut self) -> DATATOGGLE_W {
        DATATOGGLE_W { w: self }
    }
    #[doc = "Bits 24:30"]
    #[inline(always)]
    pub fn hub_address(&mut self) -> HUB_ADDRESS_W {
        HUB_ADDRESS_W { w: self }
    }
    #[doc = "Bit 31 - Beginning of the current permission bits of data"]
    #[inline(always)]
    pub fn entransfer(&mut self) -> ENTRANSFER_W {
        ENTRANSFER_W { w: self }
    }
}
