#[doc = "Reader of register SUPP"]
pub type R = crate::R<u32, super::SUPP>;
#[doc = "Writer for register SUPP"]
pub type W = crate::W<u32, super::SUPP>;
#[doc = "Register SUPP `reset()`'s with value 0"]
impl crate::ResetValue for super::SUPP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BITMODE`"]
pub type BITMODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BITMODE`"]
pub struct BITMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> BITMODE_W<'a> {
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
#[doc = "Reader of field `ENBLJABBER`"]
pub type ENBLJABBER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENBLJABBER`"]
pub struct ENBLJABBER_W<'a> {
    w: &'a mut W,
}
impl<'a> ENBLJABBER_W<'a> {
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
#[doc = "Reader of field `RESET10T`"]
pub type RESET10T_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESET10T`"]
pub struct RESET10T_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET10T_W<'a> {
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
#[doc = "Reader of field `LINKFAIL`"]
pub type LINKFAIL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LINKFAIL`"]
pub struct LINKFAIL_W<'a> {
    w: &'a mut W,
}
impl<'a> LINKFAIL_W<'a> {
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
#[doc = "Reader of field `NOCIPHER`"]
pub type NOCIPHER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NOCIPHER`"]
pub struct NOCIPHER_W<'a> {
    w: &'a mut W,
}
impl<'a> NOCIPHER_W<'a> {
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
#[doc = "Reader of field `FORCEQUIET`"]
pub type FORCEQUIET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FORCEQUIET`"]
pub struct FORCEQUIET_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCEQUIET_W<'a> {
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
#[doc = "Reader of field `RESET100X`"]
pub type RESET100X_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESET100X`"]
pub struct RESET100X_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET100X_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `RESETRMII`"]
pub type RESETRMII_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESETRMII`"]
pub struct RESETRMII_W<'a> {
    w: &'a mut W,
}
impl<'a> RESETRMII_W<'a> {
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
#[doc = "Reader of field `PHYMODE`"]
pub type PHYMODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PHYMODE`"]
pub struct PHYMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> PHYMODE_W<'a> {
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
#[doc = "Reader of field `RESETINT`"]
pub type RESETINT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESETINT`"]
pub struct RESETINT_W<'a> {
    w: &'a mut W,
}
impl<'a> RESETINT_W<'a> {
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
    #[doc = "Bit 0 - Activated mode 10BASE-T ENDEC"]
    #[inline(always)]
    pub fn bitmode(&self) -> BITMODE_R {
        BITMODE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable bit of protection against incorrect data transmission mode 10T ENDEC"]
    #[inline(always)]
    pub fn enbljabber(&self) -> ENBLJABBER_R {
        ENBLJABBER_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Reset bit module, which converts the MII nibble streams into a serial bit stream mode transceiver 10T"]
    #[inline(always)]
    pub fn reset10t(&self) -> RESET10T_R {
        RESET10T_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Bit resolution modeling"]
    #[inline(always)]
    pub fn linkfail(&self) -> LINKFAIL_R {
        LINKFAIL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Select encryption"]
    #[inline(always)]
    pub fn nocipher(&self) -> NOCIPHER_R {
        NOCIPHER_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - enable encryption"]
    #[inline(always)]
    pub fn forcequiet(&self) -> FORCEQUIET_R {
        FORCEQUIET_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Reset bit of the module that contains the logic of the encoder / decoder bit characters 4/5 bits"]
    #[inline(always)]
    pub fn reset100x(&self) -> RESET100X_R {
        RESET100X_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Bits set the operating speed simplified MII"]
    #[inline(always)]
    pub fn speed(&self) -> SPEED_R {
        SPEED_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Reset bit simplified logic MII"]
    #[inline(always)]
    pub fn resetrmii(&self) -> RESETRMII_R {
        RESETRMII_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Bit configuration consistent with MI SMII-connected devices"]
    #[inline(always)]
    pub fn phymode(&self) -> PHYMODE_R {
        PHYMODE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Reset bit of the physical interface"]
    #[inline(always)]
    pub fn resetint(&self) -> RESETINT_R {
        RESETINT_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Activated mode 10BASE-T ENDEC"]
    #[inline(always)]
    pub fn bitmode(&mut self) -> BITMODE_W {
        BITMODE_W { w: self }
    }
    #[doc = "Bit 1 - Enable bit of protection against incorrect data transmission mode 10T ENDEC"]
    #[inline(always)]
    pub fn enbljabber(&mut self) -> ENBLJABBER_W {
        ENBLJABBER_W { w: self }
    }
    #[doc = "Bit 3 - Reset bit module, which converts the MII nibble streams into a serial bit stream mode transceiver 10T"]
    #[inline(always)]
    pub fn reset10t(&mut self) -> RESET10T_W {
        RESET10T_W { w: self }
    }
    #[doc = "Bit 4 - Bit resolution modeling"]
    #[inline(always)]
    pub fn linkfail(&mut self) -> LINKFAIL_W {
        LINKFAIL_W { w: self }
    }
    #[doc = "Bit 5 - Select encryption"]
    #[inline(always)]
    pub fn nocipher(&mut self) -> NOCIPHER_W {
        NOCIPHER_W { w: self }
    }
    #[doc = "Bit 6 - enable encryption"]
    #[inline(always)]
    pub fn forcequiet(&mut self) -> FORCEQUIET_W {
        FORCEQUIET_W { w: self }
    }
    #[doc = "Bit 7 - Reset bit of the module that contains the logic of the encoder / decoder bit characters 4/5 bits"]
    #[inline(always)]
    pub fn reset100x(&mut self) -> RESET100X_W {
        RESET100X_W { w: self }
    }
    #[doc = "Bit 8 - Bits set the operating speed simplified MII"]
    #[inline(always)]
    pub fn speed(&mut self) -> SPEED_W {
        SPEED_W { w: self }
    }
    #[doc = "Bit 11 - Reset bit simplified logic MII"]
    #[inline(always)]
    pub fn resetrmii(&mut self) -> RESETRMII_W {
        RESETRMII_W { w: self }
    }
    #[doc = "Bit 12 - Bit configuration consistent with MI SMII-connected devices"]
    #[inline(always)]
    pub fn phymode(&mut self) -> PHYMODE_W {
        PHYMODE_W { w: self }
    }
    #[doc = "Bit 15 - Reset bit of the physical interface"]
    #[inline(always)]
    pub fn resetint(&mut self) -> RESETINT_W {
        RESETINT_W { w: self }
    }
}
