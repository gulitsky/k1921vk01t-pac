#[doc = "Reader of register MOIPR"]
pub type R = crate::R<u32, super::MOIPR>;
#[doc = "Writer for register MOIPR"]
pub type W = crate::W<u32, super::MOIPR>;
#[doc = "Register MOIPR `reset()`'s with value 0"]
impl crate::ResetValue for super::MOIPR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RXINP`"]
pub type RXINP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RXINP`"]
pub struct RXINP_W<'a> {
    w: &'a mut W,
}
impl<'a> RXINP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `TXINP`"]
pub type TXINP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TXINP`"]
pub struct TXINP_W<'a> {
    w: &'a mut W,
}
impl<'a> TXINP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `MPN`"]
pub type MPN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MPN`"]
pub struct MPN_W<'a> {
    w: &'a mut W,
}
impl<'a> MPN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `CFCVAL`"]
pub type CFCVAL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CFCVAL`"]
pub struct CFCVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> CFCVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn rxinp(&self) -> RXINP_R {
        RXINP_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn txinp(&self) -> TXINP_R {
        TXINP_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn mpn(&self) -> MPN_R {
        MPN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn cfcval(&self) -> CFCVAL_R {
        CFCVAL_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn rxinp(&mut self) -> RXINP_W {
        RXINP_W { w: self }
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn txinp(&mut self) -> TXINP_W {
        TXINP_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn mpn(&mut self) -> MPN_W {
        MPN_W { w: self }
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn cfcval(&mut self) -> CFCVAL_W {
        CFCVAL_W { w: self }
    }
}
