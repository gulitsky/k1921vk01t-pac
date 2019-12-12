#[doc = "Reader of register PER_RST0"]
pub type R = crate::R<u32, super::PER_RST0>;
#[doc = "Writer for register PER_RST0"]
pub type W = crate::W<u32, super::PER_RST0>;
#[doc = "Register PER_RST0 `reset()`'s with value 0"]
impl crate::ResetValue for super::PER_RST0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WDRST`"]
pub type WDRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WDRST`"]
pub struct WDRST_W<'a> {
    w: &'a mut W,
}
impl<'a> WDRST_W<'a> {
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
#[doc = "Reader of field `I2CRST0`"]
pub type I2CRST0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2CRST0`"]
pub struct I2CRST0_W<'a> {
    w: &'a mut W,
}
impl<'a> I2CRST0_W<'a> {
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
#[doc = "Reader of field `I2CRST1`"]
pub type I2CRST1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2CRST1`"]
pub struct I2CRST1_W<'a> {
    w: &'a mut W,
}
impl<'a> I2CRST1_W<'a> {
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
#[doc = "Reader of field `USBPHYRST`"]
pub type USBPHYRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USBPHYRST`"]
pub struct USBPHYRST_W<'a> {
    w: &'a mut W,
}
impl<'a> USBPHYRST_W<'a> {
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
#[doc = "Reader of field `TIMERRST0`"]
pub type TIMERRST0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMERRST0`"]
pub struct TIMERRST0_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMERRST0_W<'a> {
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
#[doc = "Reader of field `TIMERRST1`"]
pub type TIMERRST1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMERRST1`"]
pub struct TIMERRST1_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMERRST1_W<'a> {
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
#[doc = "Reader of field `TIMERRST2`"]
pub type TIMERRST2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMERRST2`"]
pub struct TIMERRST2_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMERRST2_W<'a> {
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
#[doc = "Reader of field `UARTRST0`"]
pub type UARTRST0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UARTRST0`"]
pub struct UARTRST0_W<'a> {
    w: &'a mut W,
}
impl<'a> UARTRST0_W<'a> {
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
#[doc = "Reader of field `UARTRST1`"]
pub type UARTRST1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UARTRST1`"]
pub struct UARTRST1_W<'a> {
    w: &'a mut W,
}
impl<'a> UARTRST1_W<'a> {
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
#[doc = "Reader of field `UARTRST2`"]
pub type UARTRST2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UARTRST2`"]
pub struct UARTRST2_W<'a> {
    w: &'a mut W,
}
impl<'a> UARTRST2_W<'a> {
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
#[doc = "Reader of field `UARTRST3`"]
pub type UARTRST3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UARTRST3`"]
pub struct UARTRST3_W<'a> {
    w: &'a mut W,
}
impl<'a> UARTRST3_W<'a> {
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
#[doc = "Reader of field `SPIRST0`"]
pub type SPIRST0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPIRST0`"]
pub struct SPIRST0_W<'a> {
    w: &'a mut W,
}
impl<'a> SPIRST0_W<'a> {
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
#[doc = "Reader of field `SPIRST1`"]
pub type SPIRST1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPIRST1`"]
pub struct SPIRST1_W<'a> {
    w: &'a mut W,
}
impl<'a> SPIRST1_W<'a> {
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
#[doc = "Reader of field `SPIRST2`"]
pub type SPIRST2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPIRST2`"]
pub struct SPIRST2_W<'a> {
    w: &'a mut W,
}
impl<'a> SPIRST2_W<'a> {
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
#[doc = "Reader of field `SPIRST3`"]
pub type SPIRST3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPIRST3`"]
pub struct SPIRST3_W<'a> {
    w: &'a mut W,
}
impl<'a> SPIRST3_W<'a> {
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
#[doc = "Reader of field `ETHRST`"]
pub type ETHRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ETHRST`"]
pub struct ETHRST_W<'a> {
    w: &'a mut W,
}
impl<'a> ETHRST_W<'a> {
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
    #[doc = "Bit 0 - Reset WatchDog"]
    #[inline(always)]
    pub fn wdrst(&self) -> WDRST_R {
        WDRST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Reset I2C0"]
    #[inline(always)]
    pub fn i2crst0(&self) -> I2CRST0_R {
        I2CRST0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Reset I2C1"]
    #[inline(always)]
    pub fn i2crst1(&self) -> I2CRST1_R {
        I2CRST1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Reset USB_PHY"]
    #[inline(always)]
    pub fn usbphyrst(&self) -> USBPHYRST_R {
        USBPHYRST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Reset TIMER0"]
    #[inline(always)]
    pub fn timerrst0(&self) -> TIMERRST0_R {
        TIMERRST0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Reset TIMER1"]
    #[inline(always)]
    pub fn timerrst1(&self) -> TIMERRST1_R {
        TIMERRST1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Reset TIMER2"]
    #[inline(always)]
    pub fn timerrst2(&self) -> TIMERRST2_R {
        TIMERRST2_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Reset UART0"]
    #[inline(always)]
    pub fn uartrst0(&self) -> UARTRST0_R {
        UARTRST0_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Reset UART1"]
    #[inline(always)]
    pub fn uartrst1(&self) -> UARTRST1_R {
        UARTRST1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Reset UART2"]
    #[inline(always)]
    pub fn uartrst2(&self) -> UARTRST2_R {
        UARTRST2_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Reset UART3"]
    #[inline(always)]
    pub fn uartrst3(&self) -> UARTRST3_R {
        UARTRST3_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Reset SPI0"]
    #[inline(always)]
    pub fn spirst0(&self) -> SPIRST0_R {
        SPIRST0_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Reset SPI1"]
    #[inline(always)]
    pub fn spirst1(&self) -> SPIRST1_R {
        SPIRST1_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Reset SPI2"]
    #[inline(always)]
    pub fn spirst2(&self) -> SPIRST2_R {
        SPIRST2_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Reset SPI3"]
    #[inline(always)]
    pub fn spirst3(&self) -> SPIRST3_R {
        SPIRST3_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Reset Ethernet"]
    #[inline(always)]
    pub fn ethrst(&self) -> ETHRST_R {
        ETHRST_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reset WatchDog"]
    #[inline(always)]
    pub fn wdrst(&mut self) -> WDRST_W {
        WDRST_W { w: self }
    }
    #[doc = "Bit 1 - Reset I2C0"]
    #[inline(always)]
    pub fn i2crst0(&mut self) -> I2CRST0_W {
        I2CRST0_W { w: self }
    }
    #[doc = "Bit 2 - Reset I2C1"]
    #[inline(always)]
    pub fn i2crst1(&mut self) -> I2CRST1_W {
        I2CRST1_W { w: self }
    }
    #[doc = "Bit 3 - Reset USB_PHY"]
    #[inline(always)]
    pub fn usbphyrst(&mut self) -> USBPHYRST_W {
        USBPHYRST_W { w: self }
    }
    #[doc = "Bit 4 - Reset TIMER0"]
    #[inline(always)]
    pub fn timerrst0(&mut self) -> TIMERRST0_W {
        TIMERRST0_W { w: self }
    }
    #[doc = "Bit 5 - Reset TIMER1"]
    #[inline(always)]
    pub fn timerrst1(&mut self) -> TIMERRST1_W {
        TIMERRST1_W { w: self }
    }
    #[doc = "Bit 6 - Reset TIMER2"]
    #[inline(always)]
    pub fn timerrst2(&mut self) -> TIMERRST2_W {
        TIMERRST2_W { w: self }
    }
    #[doc = "Bit 7 - Reset UART0"]
    #[inline(always)]
    pub fn uartrst0(&mut self) -> UARTRST0_W {
        UARTRST0_W { w: self }
    }
    #[doc = "Bit 8 - Reset UART1"]
    #[inline(always)]
    pub fn uartrst1(&mut self) -> UARTRST1_W {
        UARTRST1_W { w: self }
    }
    #[doc = "Bit 9 - Reset UART2"]
    #[inline(always)]
    pub fn uartrst2(&mut self) -> UARTRST2_W {
        UARTRST2_W { w: self }
    }
    #[doc = "Bit 10 - Reset UART3"]
    #[inline(always)]
    pub fn uartrst3(&mut self) -> UARTRST3_W {
        UARTRST3_W { w: self }
    }
    #[doc = "Bit 11 - Reset SPI0"]
    #[inline(always)]
    pub fn spirst0(&mut self) -> SPIRST0_W {
        SPIRST0_W { w: self }
    }
    #[doc = "Bit 12 - Reset SPI1"]
    #[inline(always)]
    pub fn spirst1(&mut self) -> SPIRST1_W {
        SPIRST1_W { w: self }
    }
    #[doc = "Bit 13 - Reset SPI2"]
    #[inline(always)]
    pub fn spirst2(&mut self) -> SPIRST2_W {
        SPIRST2_W { w: self }
    }
    #[doc = "Bit 14 - Reset SPI3"]
    #[inline(always)]
    pub fn spirst3(&mut self) -> SPIRST3_W {
        SPIRST3_W { w: self }
    }
    #[doc = "Bit 15 - Reset Ethernet"]
    #[inline(always)]
    pub fn ethrst(&mut self) -> ETHRST_W {
        ETHRST_W { w: self }
    }
}
