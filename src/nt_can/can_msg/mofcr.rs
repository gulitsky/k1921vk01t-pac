#[doc = "Reader of register MOFCR"]
pub type R = crate::R<u32, super::MOFCR>;
#[doc = "Writer for register MOFCR"]
pub type W = crate::W<u32, super::MOFCR>;
#[doc = "Register MOFCR `reset()`'s with value 0"]
impl crate::ResetValue for super::MOFCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MMC_A {
    #[doc = "0: message object"]
    MSGOBJ,
    #[doc = "1: receiver FIFO structure object"]
    RXOBJ,
    #[doc = "2: transmitter FIFO structure object"]
    TXOBJ,
    #[doc = "3: transmitter FIFO structure slave object"]
    SLAVETXOBJ,
    #[doc = "4: gateway source object"]
    SRCOBJ,
}
impl From<MMC_A> for u8 {
    #[inline(always)]
    fn from(variant: MMC_A) -> Self {
        match variant {
            MMC_A::MSGOBJ => 0,
            MMC_A::RXOBJ => 1,
            MMC_A::TXOBJ => 2,
            MMC_A::SLAVETXOBJ => 3,
            MMC_A::SRCOBJ => 4,
        }
    }
}
#[doc = "Reader of field `MMC`"]
pub type MMC_R = crate::R<u8, MMC_A>;
impl MMC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MMC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MMC_A::MSGOBJ),
            1 => Val(MMC_A::RXOBJ),
            2 => Val(MMC_A::TXOBJ),
            3 => Val(MMC_A::SLAVETXOBJ),
            4 => Val(MMC_A::SRCOBJ),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MSGOBJ`"]
    #[inline(always)]
    pub fn is_msg_obj(&self) -> bool {
        *self == MMC_A::MSGOBJ
    }
    #[doc = "Checks if the value of the field is `RXOBJ`"]
    #[inline(always)]
    pub fn is_rxobj(&self) -> bool {
        *self == MMC_A::RXOBJ
    }
    #[doc = "Checks if the value of the field is `TXOBJ`"]
    #[inline(always)]
    pub fn is_txobj(&self) -> bool {
        *self == MMC_A::TXOBJ
    }
    #[doc = "Checks if the value of the field is `SLAVETXOBJ`"]
    #[inline(always)]
    pub fn is_slave_txobj(&self) -> bool {
        *self == MMC_A::SLAVETXOBJ
    }
    #[doc = "Checks if the value of the field is `SRCOBJ`"]
    #[inline(always)]
    pub fn is_src_obj(&self) -> bool {
        *self == MMC_A::SRCOBJ
    }
}
#[doc = "Write proxy for field `MMC`"]
pub struct MMC_W<'a> {
    w: &'a mut W,
}
impl<'a> MMC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MMC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "message object"]
    #[inline(always)]
    pub fn msg_obj(self) -> &'a mut W {
        self.variant(MMC_A::MSGOBJ)
    }
    #[doc = "receiver FIFO structure object"]
    #[inline(always)]
    pub fn rxobj(self) -> &'a mut W {
        self.variant(MMC_A::RXOBJ)
    }
    #[doc = "transmitter FIFO structure object"]
    #[inline(always)]
    pub fn txobj(self) -> &'a mut W {
        self.variant(MMC_A::TXOBJ)
    }
    #[doc = "transmitter FIFO structure slave object"]
    #[inline(always)]
    pub fn slave_txobj(self) -> &'a mut W {
        self.variant(MMC_A::SLAVETXOBJ)
    }
    #[doc = "gateway source object"]
    #[inline(always)]
    pub fn src_obj(self) -> &'a mut W {
        self.variant(MMC_A::SRCOBJ)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `GDFS`"]
pub type GDFS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GDFS`"]
pub struct GDFS_W<'a> {
    w: &'a mut W,
}
impl<'a> GDFS_W<'a> {
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
#[doc = "Reader of field `IDC`"]
pub type IDC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IDC`"]
pub struct IDC_W<'a> {
    w: &'a mut W,
}
impl<'a> IDC_W<'a> {
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
#[doc = "Reader of field `DLCC`"]
pub type DLCC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DLCC`"]
pub struct DLCC_W<'a> {
    w: &'a mut W,
}
impl<'a> DLCC_W<'a> {
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
#[doc = "Reader of field `DATC`"]
pub type DATC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DATC`"]
pub struct DATC_W<'a> {
    w: &'a mut W,
}
impl<'a> DATC_W<'a> {
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
#[doc = "Reader of field `RXIE`"]
pub type RXIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXIE`"]
pub struct RXIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXIE_W<'a> {
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
#[doc = "Reader of field `TXIE`"]
pub type TXIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXIE`"]
pub struct TXIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXIE_W<'a> {
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
#[doc = "Reader of field `OVIE`"]
pub type OVIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OVIE`"]
pub struct OVIE_W<'a> {
    w: &'a mut W,
}
impl<'a> OVIE_W<'a> {
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
#[doc = "Reader of field `FRREN`"]
pub type FRREN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FRREN`"]
pub struct FRREN_W<'a> {
    w: &'a mut W,
}
impl<'a> FRREN_W<'a> {
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
#[doc = "Reader of field `RMM`"]
pub type RMM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMM`"]
pub struct RMM_W<'a> {
    w: &'a mut W,
}
impl<'a> RMM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `SDT`"]
pub type SDT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SDT`"]
pub struct SDT_W<'a> {
    w: &'a mut W,
}
impl<'a> SDT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `STT`"]
pub type STT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STT`"]
pub struct STT_W<'a> {
    w: &'a mut W,
}
impl<'a> STT_W<'a> {
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
#[doc = "Reader of field `DLC`"]
pub type DLC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DLC`"]
pub struct DLC_W<'a> {
    w: &'a mut W,
}
impl<'a> DLC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn mmc(&self) -> MMC_R {
        MMC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn gdfs(&self) -> GDFS_R {
        GDFS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn idc(&self) -> IDC_R {
        IDC_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn dlcc(&self) -> DLCC_R {
        DLCC_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn datc(&self) -> DATC_R {
        DATC_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Interrupt enable after taking the messages"]
    #[inline(always)]
    pub fn rxie(&self) -> RXIE_R {
        RXIE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Interrupt enable at the end of the message"]
    #[inline(always)]
    pub fn txie(&self) -> TXIE_R {
        TXIE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Interrupt enable FIFO to fill the message object 0"]
    #[inline(always)]
    pub fn ovie(&self) -> OVIE_R {
        OVIE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Enable remote request"]
    #[inline(always)]
    pub fn frren(&self) -> FRREN_R {
        FRREN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Enable remote monitoring of the communication object"]
    #[inline(always)]
    pub fn rmm(&self) -> RMM_R {
        RMM_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Bit single of the message object 0 participation in shipment"]
    #[inline(always)]
    pub fn sdt(&self) -> SDT_R {
        SDT_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Bit single data transfer"]
    #[inline(always)]
    pub fn stt(&self) -> STT_R {
        STT_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn dlc(&self) -> DLC_R {
        DLC_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn mmc(&mut self) -> MMC_W {
        MMC_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn gdfs(&mut self) -> GDFS_W {
        GDFS_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn idc(&mut self) -> IDC_W {
        IDC_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn dlcc(&mut self) -> DLCC_W {
        DLCC_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn datc(&mut self) -> DATC_W {
        DATC_W { w: self }
    }
    #[doc = "Bit 16 - Interrupt enable after taking the messages"]
    #[inline(always)]
    pub fn rxie(&mut self) -> RXIE_W {
        RXIE_W { w: self }
    }
    #[doc = "Bit 17 - Interrupt enable at the end of the message"]
    #[inline(always)]
    pub fn txie(&mut self) -> TXIE_W {
        TXIE_W { w: self }
    }
    #[doc = "Bit 18 - Interrupt enable FIFO to fill the message object 0"]
    #[inline(always)]
    pub fn ovie(&mut self) -> OVIE_W {
        OVIE_W { w: self }
    }
    #[doc = "Bit 20 - Enable remote request"]
    #[inline(always)]
    pub fn frren(&mut self) -> FRREN_W {
        FRREN_W { w: self }
    }
    #[doc = "Bit 21 - Enable remote monitoring of the communication object"]
    #[inline(always)]
    pub fn rmm(&mut self) -> RMM_W {
        RMM_W { w: self }
    }
    #[doc = "Bit 22 - Bit single of the message object 0 participation in shipment"]
    #[inline(always)]
    pub fn sdt(&mut self) -> SDT_W {
        SDT_W { w: self }
    }
    #[doc = "Bit 23 - Bit single data transfer"]
    #[inline(always)]
    pub fn stt(&mut self) -> STT_W {
        STT_W { w: self }
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn dlc(&mut self) -> DLC_W {
        DLC_W { w: self }
    }
}
