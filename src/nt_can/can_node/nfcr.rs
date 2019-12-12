#[doc = "Reader of register NFCR"]
pub type R = crate::R<u32, super::NFCR>;
#[doc = "Writer for register NFCR"]
pub type W = crate::W<u32, super::NFCR>;
#[doc = "Register NFCR `reset()`'s with value 0"]
impl crate::ResetValue for super::NFCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CFC`"]
pub type CFC_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CFC`"]
pub struct CFC_W<'a> {
    w: &'a mut W,
}
impl<'a> CFC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `CFSEL`"]
pub type CFSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CFSEL`"]
pub struct CFSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CFSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Reader of field `CFMOD`"]
pub type CFMOD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CFMOD`"]
pub struct CFMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> CFMOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 19)) | (((value as u32) & 0x03) << 19);
        self.w
    }
}
#[doc = "Reader of field `CFCIE`"]
pub type CFCIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CFCIE`"]
pub struct CFCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CFCIE_W<'a> {
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
#[doc = "Reader of field `CFCOV`"]
pub type CFCOV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CFCOV`"]
pub struct CFCOV_W<'a> {
    w: &'a mut W,
}
impl<'a> CFCOV_W<'a> {
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
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn cfc(&self) -> CFC_R {
        CFC_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn cfsel(&self) -> CFSEL_R {
        CFSEL_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 19:20"]
    #[inline(always)]
    pub fn cfmod(&self) -> CFMOD_R {
        CFMOD_R::new(((self.bits >> 19) & 0x03) as u8)
    }
    #[doc = "Bit 22 - Interrupt enable bit of the message counter"]
    #[inline(always)]
    pub fn cfcie(&self) -> CFCIE_R {
        CFCIE_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Counter overflow flag messages"]
    #[inline(always)]
    pub fn cfcov(&self) -> CFCOV_R {
        CFCOV_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn cfc(&mut self) -> CFC_W {
        CFC_W { w: self }
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn cfsel(&mut self) -> CFSEL_W {
        CFSEL_W { w: self }
    }
    #[doc = "Bits 19:20"]
    #[inline(always)]
    pub fn cfmod(&mut self) -> CFMOD_W {
        CFMOD_W { w: self }
    }
    #[doc = "Bit 22 - Interrupt enable bit of the message counter"]
    #[inline(always)]
    pub fn cfcie(&mut self) -> CFCIE_W {
        CFCIE_W { w: self }
    }
    #[doc = "Bit 23 - Counter overflow flag messages"]
    #[inline(always)]
    pub fn cfcov(&mut self) -> CFCOV_W {
        CFCOV_W { w: self }
    }
}
