#[doc = "Reader of register IRQ_ENB"]
pub type R = crate::R<u32, super::IRQ_ENB>;
#[doc = "Writer for register IRQ_ENB"]
pub type W = crate::W<u32, super::IRQ_ENB>;
#[doc = "Register IRQ_ENB `reset()`'s with value 0"]
impl crate::ResetValue for super::IRQ_ENB {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BUFFFULLINTEN`"]
pub type BUFFFULLINTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BUFFFULLINTEN`"]
pub struct BUFFFULLINTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFFFULLINTEN_W<'a> {
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
#[doc = "Reader of field `BUFFEMPTYINTEN`"]
pub type BUFFEMPTYINTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BUFFEMPTYINTEN`"]
pub struct BUFFEMPTYINTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFFEMPTYINTEN_W<'a> {
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
#[doc = "Reader of field `SHORTPKTTRINTEN`"]
pub type SHORTPKTTRINTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SHORTPKTTRINTEN`"]
pub struct SHORTPKTTRINTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SHORTPKTTRINTEN_W<'a> {
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
#[doc = "Reader of field `DATAPKTTRINTEN`"]
pub type DATAPKTTRINTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DATAPKTTRINTEN`"]
pub struct DATAPKTTRINTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DATAPKTTRINTEN_W<'a> {
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
#[doc = "Reader of field `DATAPKTRECINTEN`"]
pub type DATAPKTRECINTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DATAPKTRECINTEN`"]
pub struct DATAPKTRECINTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DATAPKTRECINTEN_W<'a> {
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
#[doc = "Reader of field `OUTTOKENINTEN`"]
pub type OUTTOKENINTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OUTTOKENINTEN`"]
pub struct OUTTOKENINTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTTOKENINTEN_W<'a> {
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
#[doc = "Reader of field `INTOKENINTEN`"]
pub type INTOKENINTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTOKENINTEN`"]
pub struct INTOKENINTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> INTOKENINTEN_W<'a> {
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
#[doc = "Reader of field `PINGTOKENINTEN`"]
pub type PINGTOKENINTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINGTOKENINTEN`"]
pub struct PINGTOKENINTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PINGTOKENINTEN_W<'a> {
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
#[doc = "Reader of field `NACKSENTINTEN`"]
pub type NACKSENTINTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NACKSENTINTEN`"]
pub struct NACKSENTINTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> NACKSENTINTEN_W<'a> {
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
#[doc = "Reader of field `STALLSENTINTEN`"]
pub type STALLSENTINTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STALLSENTINTEN`"]
pub struct STALLSENTINTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> STALLSENTINTEN_W<'a> {
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
#[doc = "Reader of field `NYETSENTINTEN`"]
pub type NYETSENTINTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NYETSENTINTEN`"]
pub struct NYETSENTINTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> NYETSENTINTEN_W<'a> {
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
#[doc = "Reader of field `ERRSENTINTEN`"]
pub type ERRSENTINTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERRSENTINTEN`"]
pub struct ERRSENTINTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ERRSENTINTEN_W<'a> {
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
    #[doc = "Bit 0 - Enable flag buffer fill"]
    #[inline(always)]
    pub fn bufffullinten(&self) -> BUFFFULLINTEN_R {
        BUFFFULLINTEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable buffer empty flag"]
    #[inline(always)]
    pub fn buffemptyinten(&self) -> BUFFEMPTYINTEN_R {
        BUFFEMPTYINTEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable flag size of the last packet"]
    #[inline(always)]
    pub fn shortpkttrinten(&self) -> SHORTPKTTRINTEN_R {
        SHORTPKTTRINTEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable flag to send a data packet from the buffer to the host"]
    #[inline(always)]
    pub fn datapkttrinten(&self) -> DATAPKTTRINTEN_R {
        DATAPKTTRINTEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable flag successful reception of a data packet from a host"]
    #[inline(always)]
    pub fn datapktrecinten(&self) -> DATAPKTRECINTEN_R {
        DATAPKTRECINTEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enable receiving flag tags OUT from the host"]
    #[inline(always)]
    pub fn outtokeninten(&self) -> OUTTOKENINTEN_R {
        OUTTOKENINTEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enable receiving flag tags IN from the host"]
    #[inline(always)]
    pub fn intokeninten(&self) -> INTOKENINTEN_R {
        INTOKENINTEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enable receiving flag tags PING from the host"]
    #[inline(always)]
    pub fn pingtokeninten(&self) -> PINGTOKENINTEN_R {
        PINGTOKENINTEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enable flag unsent last packet"]
    #[inline(always)]
    pub fn nacksentinten(&self) -> NACKSENTINTEN_R {
        NACKSENTINTEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Enable flag Uncommitted last packet"]
    #[inline(always)]
    pub fn stallsentinten(&self) -> STALLSENTINTEN_R {
        STALLSENTINTEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Enable flag lack of free memory to receive the next data packet"]
    #[inline(always)]
    pub fn nyetsentinten(&self) -> NYETSENTINTEN_R {
        NYETSENTINTEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Enable flag of any error during the operation"]
    #[inline(always)]
    pub fn errsentinten(&self) -> ERRSENTINTEN_R {
        ERRSENTINTEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable flag buffer fill"]
    #[inline(always)]
    pub fn bufffullinten(&mut self) -> BUFFFULLINTEN_W {
        BUFFFULLINTEN_W { w: self }
    }
    #[doc = "Bit 1 - Enable buffer empty flag"]
    #[inline(always)]
    pub fn buffemptyinten(&mut self) -> BUFFEMPTYINTEN_W {
        BUFFEMPTYINTEN_W { w: self }
    }
    #[doc = "Bit 2 - Enable flag size of the last packet"]
    #[inline(always)]
    pub fn shortpkttrinten(&mut self) -> SHORTPKTTRINTEN_W {
        SHORTPKTTRINTEN_W { w: self }
    }
    #[doc = "Bit 3 - Enable flag to send a data packet from the buffer to the host"]
    #[inline(always)]
    pub fn datapkttrinten(&mut self) -> DATAPKTTRINTEN_W {
        DATAPKTTRINTEN_W { w: self }
    }
    #[doc = "Bit 4 - Enable flag successful reception of a data packet from a host"]
    #[inline(always)]
    pub fn datapktrecinten(&mut self) -> DATAPKTRECINTEN_W {
        DATAPKTRECINTEN_W { w: self }
    }
    #[doc = "Bit 5 - Enable receiving flag tags OUT from the host"]
    #[inline(always)]
    pub fn outtokeninten(&mut self) -> OUTTOKENINTEN_W {
        OUTTOKENINTEN_W { w: self }
    }
    #[doc = "Bit 6 - Enable receiving flag tags IN from the host"]
    #[inline(always)]
    pub fn intokeninten(&mut self) -> INTOKENINTEN_W {
        INTOKENINTEN_W { w: self }
    }
    #[doc = "Bit 7 - Enable receiving flag tags PING from the host"]
    #[inline(always)]
    pub fn pingtokeninten(&mut self) -> PINGTOKENINTEN_W {
        PINGTOKENINTEN_W { w: self }
    }
    #[doc = "Bit 8 - Enable flag unsent last packet"]
    #[inline(always)]
    pub fn nacksentinten(&mut self) -> NACKSENTINTEN_W {
        NACKSENTINTEN_W { w: self }
    }
    #[doc = "Bit 9 - Enable flag Uncommitted last packet"]
    #[inline(always)]
    pub fn stallsentinten(&mut self) -> STALLSENTINTEN_W {
        STALLSENTINTEN_W { w: self }
    }
    #[doc = "Bit 10 - Enable flag lack of free memory to receive the next data packet"]
    #[inline(always)]
    pub fn nyetsentinten(&mut self) -> NYETSENTINTEN_W {
        NYETSENTINTEN_W { w: self }
    }
    #[doc = "Bit 11 - Enable flag of any error during the operation"]
    #[inline(always)]
    pub fn errsentinten(&mut self) -> ERRSENTINTEN_W {
        ERRSENTINTEN_W { w: self }
    }
}
