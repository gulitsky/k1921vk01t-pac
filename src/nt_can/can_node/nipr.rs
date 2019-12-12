#[doc = "Reader of register NIPR"]
pub type R = crate::R<u32, super::NIPR>;
#[doc = "Writer for register NIPR"]
pub type W = crate::W<u32, super::NIPR>;
#[doc = "Register NIPR `reset()`'s with value 0"]
impl crate::ResetValue for super::NIPR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ALINP`"]
pub type ALINP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ALINP`"]
pub struct ALINP_W<'a> {
    w: &'a mut W,
}
impl<'a> ALINP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `LECINP`"]
pub type LECINP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LECINP`"]
pub struct LECINP_W<'a> {
    w: &'a mut W,
}
impl<'a> LECINP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `TRINP`"]
pub type TRINP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRINP`"]
pub struct TRINP_W<'a> {
    w: &'a mut W,
}
impl<'a> TRINP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `CFCINP`"]
pub type CFCINP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CFCINP`"]
pub struct CFCINP_W<'a> {
    w: &'a mut W,
}
impl<'a> CFCINP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn alinp(&self) -> ALINP_R {
        ALINP_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn lecinp(&self) -> LECINP_R {
        LECINP_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn trinp(&self) -> TRINP_R {
        TRINP_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn cfcinp(&self) -> CFCINP_R {
        CFCINP_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn alinp(&mut self) -> ALINP_W {
        ALINP_W { w: self }
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn lecinp(&mut self) -> LECINP_W {
        LECINP_W { w: self }
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn trinp(&mut self) -> TRINP_W {
        TRINP_W { w: self }
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn cfcinp(&mut self) -> CFCINP_W {
        CFCINP_W { w: self }
    }
}
