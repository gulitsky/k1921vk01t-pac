#[doc = "Reader of register EXT_MEM_CFG"]
pub type R = crate::R<u32, super::EXT_MEM_CFG>;
#[doc = "Writer for register EXT_MEM_CFG"]
pub type W = crate::W<u32, super::EXT_MEM_CFG>;
#[doc = "Register EXT_MEM_CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::EXT_MEM_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ReadWS`"]
pub type READWS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ReadWS`"]
pub struct READWS_W<'a> {
    w: &'a mut W,
}
impl<'a> READWS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `WriteWS`"]
pub type WRITEWS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WriteWS`"]
pub struct WRITEWS_W<'a> {
    w: &'a mut W,
}
impl<'a> WRITEWS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | (((value as u32) & 0x07) << 3);
        self.w
    }
}
#[doc = "Reader of field `RWWS`"]
pub type RWWS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RWWS`"]
pub struct RWWS_W<'a> {
    w: &'a mut W,
}
impl<'a> RWWS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 6)) | (((value as u32) & 0x07) << 6);
        self.w
    }
}
#[doc = "Reader of field `CE_MASK`"]
pub type CE_MASK_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CE_MASK`"]
pub struct CE_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> CE_MASK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 9)) | (((value as u32) & 0x01ff) << 9);
        self.w
    }
}
#[doc = "Reader of field `BW`"]
pub type BW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BW`"]
pub struct BW_W<'a> {
    w: &'a mut W,
}
impl<'a> BW_W<'a> {
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
    #[doc = "Bits 0:2 - Read wait state cycles value"]
    #[inline(always)]
    pub fn read_ws(&self) -> READWS_R {
        READWS_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:5 - Write wait state cycles value"]
    #[inline(always)]
    pub fn write_ws(&self) -> WRITEWS_R {
        WRITEWS_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bits 6:8 - Pause wait state cycles value"]
    #[inline(always)]
    pub fn rwws(&self) -> RWWS_R {
        RWWS_R::new(((self.bits >> 6) & 0x07) as u8)
    }
    #[doc = "Bits 9:17 - CE1, OE1 address mask"]
    #[inline(always)]
    pub fn ce_mask(&self) -> CE_MASK_R {
        CE_MASK_R::new(((self.bits >> 9) & 0x01ff) as u16)
    }
    #[doc = "Bit 31 - External memory bus width (8/16bit)"]
    #[inline(always)]
    pub fn bw(&self) -> BW_R {
        BW_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Read wait state cycles value"]
    #[inline(always)]
    pub fn read_ws(&mut self) -> READWS_W {
        READWS_W { w: self }
    }
    #[doc = "Bits 3:5 - Write wait state cycles value"]
    #[inline(always)]
    pub fn write_ws(&mut self) -> WRITEWS_W {
        WRITEWS_W { w: self }
    }
    #[doc = "Bits 6:8 - Pause wait state cycles value"]
    #[inline(always)]
    pub fn rwws(&mut self) -> RWWS_W {
        RWWS_W { w: self }
    }
    #[doc = "Bits 9:17 - CE1, OE1 address mask"]
    #[inline(always)]
    pub fn ce_mask(&mut self) -> CE_MASK_W {
        CE_MASK_W { w: self }
    }
    #[doc = "Bit 31 - External memory bus width (8/16bit)"]
    #[inline(always)]
    pub fn bw(&mut self) -> BW_W {
        BW_W { w: self }
    }
}
