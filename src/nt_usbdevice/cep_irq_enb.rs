#[doc = "Reader of register CEP_IRQ_ENB"]
pub type R = crate::R<u32, super::CEP_IRQ_ENB>;
#[doc = "Writer for register CEP_IRQ_ENB"]
pub type W = crate::W<u32, super::CEP_IRQ_ENB>;
#[doc = "Register CEP_IRQ_ENB `reset()`'s with value 0"]
impl crate::ResetValue for super::CEP_IRQ_ENB {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SETUPTOKENINT`"]
pub type SETUPTOKENINT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETUPTOKENINT`"]
pub struct SETUPTOKENINT_W<'a> {
    w: &'a mut W,
}
impl<'a> SETUPTOKENINT_W<'a> {
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
#[doc = "Reader of field `SETUPPKTINT`"]
pub type SETUPPKTINT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETUPPKTINT`"]
pub struct SETUPPKTINT_W<'a> {
    w: &'a mut W,
}
impl<'a> SETUPPKTINT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `NAKSENTINT`"]
pub type NAKSENTINT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NAKSENTINT`"]
pub struct NAKSENTINT_W<'a> {
    w: &'a mut W,
}
impl<'a> NAKSENTINT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `USBERRINT`"]
pub type USBERRINT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USBERRINT`"]
pub struct USBERRINT_W<'a> {
    w: &'a mut W,
}
impl<'a> USBERRINT_W<'a> {
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
#[doc = "Reader of field `STATCMPLNINT`"]
pub type STATCMPLNINT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STATCMPLNINT`"]
pub struct STATCMPLNINT_W<'a> {
    w: &'a mut W,
}
impl<'a> STATCMPLNINT_W<'a> {
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
#[doc = "Reader of field `BUFFFULLINT`"]
pub type BUFFFULLINT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BUFFFULLINT`"]
pub struct BUFFFULLINT_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFFFULLINT_W<'a> {
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
#[doc = "Reader of field `BUFFEMPTYINT`"]
pub type BUFFEMPTYINT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BUFFEMPTYINT`"]
pub struct BUFFEMPTYINT_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFFEMPTYINT_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Enable flag deadline tags SETUP from the host"]
    #[inline(always)]
    pub fn setuptokenint(&self) -> SETUPTOKENINT_R {
        SETUPTOKENINT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable flag deadline Setup package from the host"]
    #[inline(always)]
    pub fn setuppktint(&self) -> SETUPPKTINT_R {
        SETUPPKTINT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable reception end flag tags OUT from the host"]
    #[inline(always)]
    pub fn outtokenint(&self) -> OUTTOKENINT_R {
        OUTTOKENINT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable reception end flag tags IN from the host"]
    #[inline(always)]
    pub fn intokenint(&self) -> INTOKENINT_R {
        INTOKENINT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable reception end flag tags PING from host"]
    #[inline(always)]
    pub fn pingtokenint(&self) -> PINGTOKENINT_R {
        PINGTOKENINT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enable flag successful sending of the data packet in response to the label to obtain IN ACK confirming tags"]
    #[inline(always)]
    pub fn datapkttrint(&self) -> DATAPKTTRINT_R {
        DATAPKTTRINT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enable flag successful reception of the data packet following the label OUT, in response to which the mark has been sent ACK"]
    #[inline(always)]
    pub fn datapktrecint(&self) -> DATAPKTRECINT_R {
        DATAPKTRECINT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enable flag closure parcel labels NAK in response to mark IN or OUT"]
    #[inline(always)]
    pub fn naksentint(&self) -> NAKSENTINT_R {
        NAKSENTINT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enable flag closure parcel labels STALL in response to mark IN or OUT"]
    #[inline(always)]
    pub fn stallsentint(&self) -> STALLSENTINT_R {
        STALLSENTINT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Enable error flag during the operation"]
    #[inline(always)]
    pub fn usberrint(&self) -> USBERRINT_R {
        USBERRINT_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Enable flag successful completion stage 'Status' operations on the USB bus"]
    #[inline(always)]
    pub fn statcmplnint(&self) -> STATCMPLNINT_R {
        STATCMPLNINT_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Enable flag filling control buffer"]
    #[inline(always)]
    pub fn bufffullint(&self) -> BUFFFULLINT_R {
        BUFFFULLINT_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Enable flag emptying control buffer"]
    #[inline(always)]
    pub fn buffemptyint(&self) -> BUFFEMPTYINT_R {
        BUFFEMPTYINT_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable flag deadline tags SETUP from the host"]
    #[inline(always)]
    pub fn setuptokenint(&mut self) -> SETUPTOKENINT_W {
        SETUPTOKENINT_W { w: self }
    }
    #[doc = "Bit 1 - Enable flag deadline Setup package from the host"]
    #[inline(always)]
    pub fn setuppktint(&mut self) -> SETUPPKTINT_W {
        SETUPPKTINT_W { w: self }
    }
    #[doc = "Bit 2 - Enable reception end flag tags OUT from the host"]
    #[inline(always)]
    pub fn outtokenint(&mut self) -> OUTTOKENINT_W {
        OUTTOKENINT_W { w: self }
    }
    #[doc = "Bit 3 - Enable reception end flag tags IN from the host"]
    #[inline(always)]
    pub fn intokenint(&mut self) -> INTOKENINT_W {
        INTOKENINT_W { w: self }
    }
    #[doc = "Bit 4 - Enable reception end flag tags PING from host"]
    #[inline(always)]
    pub fn pingtokenint(&mut self) -> PINGTOKENINT_W {
        PINGTOKENINT_W { w: self }
    }
    #[doc = "Bit 5 - Enable flag successful sending of the data packet in response to the label to obtain IN ACK confirming tags"]
    #[inline(always)]
    pub fn datapkttrint(&mut self) -> DATAPKTTRINT_W {
        DATAPKTTRINT_W { w: self }
    }
    #[doc = "Bit 6 - Enable flag successful reception of the data packet following the label OUT, in response to which the mark has been sent ACK"]
    #[inline(always)]
    pub fn datapktrecint(&mut self) -> DATAPKTRECINT_W {
        DATAPKTRECINT_W { w: self }
    }
    #[doc = "Bit 7 - Enable flag closure parcel labels NAK in response to mark IN or OUT"]
    #[inline(always)]
    pub fn naksentint(&mut self) -> NAKSENTINT_W {
        NAKSENTINT_W { w: self }
    }
    #[doc = "Bit 8 - Enable flag closure parcel labels STALL in response to mark IN or OUT"]
    #[inline(always)]
    pub fn stallsentint(&mut self) -> STALLSENTINT_W {
        STALLSENTINT_W { w: self }
    }
    #[doc = "Bit 9 - Enable error flag during the operation"]
    #[inline(always)]
    pub fn usberrint(&mut self) -> USBERRINT_W {
        USBERRINT_W { w: self }
    }
    #[doc = "Bit 10 - Enable flag successful completion stage 'Status' operations on the USB bus"]
    #[inline(always)]
    pub fn statcmplnint(&mut self) -> STATCMPLNINT_W {
        STATCMPLNINT_W { w: self }
    }
    #[doc = "Bit 11 - Enable flag filling control buffer"]
    #[inline(always)]
    pub fn bufffullint(&mut self) -> BUFFFULLINT_W {
        BUFFFULLINT_W { w: self }
    }
    #[doc = "Bit 12 - Enable flag emptying control buffer"]
    #[inline(always)]
    pub fn buffemptyint(&mut self) -> BUFFEMPTYINT_W {
        BUFFEMPTYINT_W { w: self }
    }
}
