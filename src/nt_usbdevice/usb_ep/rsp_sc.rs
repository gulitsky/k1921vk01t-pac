#[doc = "Reader of register RSP_SC"]
pub type R = crate::R<u32, super::RSP_SC>;
#[doc = "Writer for register RSP_SC"]
pub type W = crate::W<u32, super::RSP_SC>;
#[doc = "Register RSP_SC `reset()`'s with value 0"]
impl crate::ResetValue for super::RSP_SC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BUFFFLUSH`"]
pub type BUFFFLUSH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BUFFFLUSH`"]
pub struct BUFFFLUSH_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFFFLUSH_W<'a> {
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
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MODE`"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u32) & 0x03) << 1);
        self.w
    }
}
#[doc = "Reader of field `ENDPOINTTOGGL`"]
pub type ENDPOINTTOGGL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENDPOINTTOGGL`"]
pub struct ENDPOINTTOGGL_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDPOINTTOGGL_W<'a> {
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
#[doc = "Reader of field `ENDPOINTHALT`"]
pub type ENDPOINTHALT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENDPOINTHALT`"]
pub struct ENDPOINTHALT_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDPOINTHALT_W<'a> {
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
#[doc = "Reader of field `ZEROLENIN`"]
pub type ZEROLENIN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ZEROLENIN`"]
pub struct ZEROLENIN_W<'a> {
    w: &'a mut W,
}
impl<'a> ZEROLENIN_W<'a> {
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
#[doc = "Reader of field `PKTEND`"]
pub type PKTEND_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PKTEND`"]
pub struct PKTEND_W<'a> {
    w: &'a mut W,
}
impl<'a> PKTEND_W<'a> {
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
    #[doc = "Bit 0 - Reset bit in the buffer and register EP_AVAIL_CNT"]
    #[inline(always)]
    pub fn buffflush(&self) -> BUFFFLUSH_R {
        BUFFFLUSH_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 3 - Control bit by bit DATATOGGLE"]
    #[inline(always)]
    pub fn endpointtoggl(&self) -> ENDPOINTTOGGL_R {
        ENDPOINTTOGGL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Select bit answer to any label from the host"]
    #[inline(always)]
    pub fn endpointhalt(&self) -> ENDPOINTHALT_R {
        ENDPOINTHALT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Select bit answer to mark IN"]
    #[inline(always)]
    pub fn zerolenin(&self) -> ZEROLENIN_R {
        ZEROLENIN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Indicator number of bytes"]
    #[inline(always)]
    pub fn pktend(&self) -> PKTEND_R {
        PKTEND_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reset bit in the buffer and register EP_AVAIL_CNT"]
    #[inline(always)]
    pub fn buffflush(&mut self) -> BUFFFLUSH_W {
        BUFFFLUSH_W { w: self }
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bit 3 - Control bit by bit DATATOGGLE"]
    #[inline(always)]
    pub fn endpointtoggl(&mut self) -> ENDPOINTTOGGL_W {
        ENDPOINTTOGGL_W { w: self }
    }
    #[doc = "Bit 4 - Select bit answer to any label from the host"]
    #[inline(always)]
    pub fn endpointhalt(&mut self) -> ENDPOINTHALT_W {
        ENDPOINTHALT_W { w: self }
    }
    #[doc = "Bit 5 - Select bit answer to mark IN"]
    #[inline(always)]
    pub fn zerolenin(&mut self) -> ZEROLENIN_W {
        ZEROLENIN_W { w: self }
    }
    #[doc = "Bit 6 - Indicator number of bytes"]
    #[inline(always)]
    pub fn pktend(&mut self) -> PKTEND_W {
        PKTEND_W { w: self }
    }
}
