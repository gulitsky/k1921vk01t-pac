#[doc = "Reader of register IRQ_STAT"]
pub type R = crate::R<u32, super::IRQ_STAT>;
#[doc = "Writer for register IRQ_STAT"]
pub type W = crate::W<u32, super::IRQ_STAT>;
#[doc = "Register IRQ_STAT `reset()`'s with value 0"]
impl crate::ResetValue for super::IRQ_STAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BUFFFULLINT`"]
pub type BUFFFULLINT_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUFFEMPTYINT`"]
pub type BUFFEMPTYINT_R = crate::R<bool, bool>;
#[doc = "Reader of field `SHORTPKTTRINT`"]
pub type SHORTPKTTRINT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SHORTPKTTRINT`"]
pub struct SHORTPKTTRINT_W<'a> {
    w: &'a mut W,
}
impl<'a> SHORTPKTTRINT_W<'a> {
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
#[doc = "Reader of field `DATAPKTTRINT`"]
pub type DATAPKTTRINT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DATAPKTTRINT`"]
pub struct DATAPKTTRINT_W<'a> {
    w: &'a mut W,
}
impl<'a> DATAPKTTRINT_W<'a> {
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
#[doc = "Reader of field `DATAPKTRECINT`"]
pub type DATAPKTRECINT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DATAPKTRECINT`"]
pub struct DATAPKTRECINT_W<'a> {
    w: &'a mut W,
}
impl<'a> DATAPKTRECINT_W<'a> {
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
#[doc = "Reader of field `OUTTOKENINT`"]
pub type OUTTOKENINT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OUTTOKENINT`"]
pub struct OUTTOKENINT_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTTOKENINT_W<'a> {
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
#[doc = "Reader of field `INTOKENINT`"]
pub type INTOKENINT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTOKENINT`"]
pub struct INTOKENINT_W<'a> {
    w: &'a mut W,
}
impl<'a> INTOKENINT_W<'a> {
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
#[doc = "Reader of field `PINGTOKENINT`"]
pub type PINGTOKENINT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINGTOKENINT`"]
pub struct PINGTOKENINT_W<'a> {
    w: &'a mut W,
}
impl<'a> PINGTOKENINT_W<'a> {
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
#[doc = "Reader of field `NACKSENTINT`"]
pub type NACKSENTINT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NACKSENTINT`"]
pub struct NACKSENTINT_W<'a> {
    w: &'a mut W,
}
impl<'a> NACKSENTINT_W<'a> {
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
#[doc = "Reader of field `STALLSENTINT`"]
pub type STALLSENTINT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STALLSENTINT`"]
pub struct STALLSENTINT_W<'a> {
    w: &'a mut W,
}
impl<'a> STALLSENTINT_W<'a> {
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
#[doc = "Reader of field `NYETSENTINT`"]
pub type NYETSENTINT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NYETSENTINT`"]
pub struct NYETSENTINT_W<'a> {
    w: &'a mut W,
}
impl<'a> NYETSENTINT_W<'a> {
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
#[doc = "Reader of field `ERRSENTINT`"]
pub type ERRSENTINT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERRSENTINT`"]
pub struct ERRSENTINT_W<'a> {
    w: &'a mut W,
}
impl<'a> ERRSENTINT_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Flag buffer fill"]
    #[inline(always)]
    pub fn bufffullint(&self) -> BUFFFULLINT_R {
        BUFFFULLINT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Buffer empty flag"]
    #[inline(always)]
    pub fn buffemptyint(&self) -> BUFFEMPTYINT_R {
        BUFFEMPTYINT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Flag size of the last packet"]
    #[inline(always)]
    pub fn shortpkttrint(&self) -> SHORTPKTTRINT_R {
        SHORTPKTTRINT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Flag to send a data packet from the buffer to the host"]
    #[inline(always)]
    pub fn datapkttrint(&self) -> DATAPKTTRINT_R {
        DATAPKTTRINT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Flag successful reception of a data packet from a host"]
    #[inline(always)]
    pub fn datapktrecint(&self) -> DATAPKTRECINT_R {
        DATAPKTRECINT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Receiving flag tags OUT from the host"]
    #[inline(always)]
    pub fn outtokenint(&self) -> OUTTOKENINT_R {
        OUTTOKENINT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Receiving flag tags IN from the host"]
    #[inline(always)]
    pub fn intokenint(&self) -> INTOKENINT_R {
        INTOKENINT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Receiving flag tags PING from the host"]
    #[inline(always)]
    pub fn pingtokenint(&self) -> PINGTOKENINT_R {
        PINGTOKENINT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Flag unsent last packet"]
    #[inline(always)]
    pub fn nacksentint(&self) -> NACKSENTINT_R {
        NACKSENTINT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Flag Uncommitted last packet"]
    #[inline(always)]
    pub fn stallsentint(&self) -> STALLSENTINT_R {
        STALLSENTINT_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Flag lack of free memory to receive the next data packet"]
    #[inline(always)]
    pub fn nyetsentint(&self) -> NYETSENTINT_R {
        NYETSENTINT_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Flag of any error during the operation"]
    #[inline(always)]
    pub fn errsentint(&self) -> ERRSENTINT_R {
        ERRSENTINT_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Flag size of the last packet"]
    #[inline(always)]
    pub fn shortpkttrint(&mut self) -> SHORTPKTTRINT_W {
        SHORTPKTTRINT_W { w: self }
    }
    #[doc = "Bit 3 - Flag to send a data packet from the buffer to the host"]
    #[inline(always)]
    pub fn datapkttrint(&mut self) -> DATAPKTTRINT_W {
        DATAPKTTRINT_W { w: self }
    }
    #[doc = "Bit 4 - Flag successful reception of a data packet from a host"]
    #[inline(always)]
    pub fn datapktrecint(&mut self) -> DATAPKTRECINT_W {
        DATAPKTRECINT_W { w: self }
    }
    #[doc = "Bit 5 - Receiving flag tags OUT from the host"]
    #[inline(always)]
    pub fn outtokenint(&mut self) -> OUTTOKENINT_W {
        OUTTOKENINT_W { w: self }
    }
    #[doc = "Bit 6 - Receiving flag tags IN from the host"]
    #[inline(always)]
    pub fn intokenint(&mut self) -> INTOKENINT_W {
        INTOKENINT_W { w: self }
    }
    #[doc = "Bit 7 - Receiving flag tags PING from the host"]
    #[inline(always)]
    pub fn pingtokenint(&mut self) -> PINGTOKENINT_W {
        PINGTOKENINT_W { w: self }
    }
    #[doc = "Bit 8 - Flag unsent last packet"]
    #[inline(always)]
    pub fn nacksentint(&mut self) -> NACKSENTINT_W {
        NACKSENTINT_W { w: self }
    }
    #[doc = "Bit 9 - Flag Uncommitted last packet"]
    #[inline(always)]
    pub fn stallsentint(&mut self) -> STALLSENTINT_W {
        STALLSENTINT_W { w: self }
    }
    #[doc = "Bit 10 - Flag lack of free memory to receive the next data packet"]
    #[inline(always)]
    pub fn nyetsentint(&mut self) -> NYETSENTINT_W {
        NYETSENTINT_W { w: self }
    }
    #[doc = "Bit 11 - Flag of any error during the operation"]
    #[inline(always)]
    pub fn errsentint(&mut self) -> ERRSENTINT_W {
        ERRSENTINT_W { w: self }
    }
}
