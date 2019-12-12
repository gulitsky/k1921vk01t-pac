#[doc = "Reader of register HDSEL"]
pub type R = crate::R<u32, super::HDSEL>;
#[doc = "Writer for register HDSEL"]
pub type W = crate::W<u32, super::HDSEL>;
#[doc = "Register HDSEL `reset()`'s with value 0"]
impl crate::ResetValue for super::HDSEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DCMP`"]
pub type DCMP_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DCMP`"]
pub struct DCMP_W<'a> {
    w: &'a mut W,
}
impl<'a> DCMP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
#[doc = "Reader of field `ACMP`"]
pub type ACMP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ACMP`"]
pub struct ACMP_W<'a> {
    w: &'a mut W,
}
impl<'a> ACMP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "Reader of field `CBC`"]
pub type CBC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CBC`"]
pub struct CBC_W<'a> {
    w: &'a mut W,
}
impl<'a> CBC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `OSHT`"]
pub type OSHT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OSHT`"]
pub struct OSHT_W<'a> {
    w: &'a mut W,
}
impl<'a> OSHT_W<'a> {
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
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn dcmp(&self) -> DCMP_R {
        DCMP_R::new((self.bits & 0x00ff_ffff) as u32)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn acmp(&self) -> ACMP_R {
        ACMP_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bit 28 - Enable source ACMP/DCMP in circle mode"]
    #[inline(always)]
    pub fn cbc(&self) -> CBC_R {
        CBC_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Enable source ACMP/DCMP in single mode"]
    #[inline(always)]
    pub fn osht(&self) -> OSHT_R {
        OSHT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn dcmp(&mut self) -> DCMP_W {
        DCMP_W { w: self }
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn acmp(&mut self) -> ACMP_W {
        ACMP_W { w: self }
    }
    #[doc = "Bit 28 - Enable source ACMP/DCMP in circle mode"]
    #[inline(always)]
    pub fn cbc(&mut self) -> CBC_W {
        CBC_W { w: self }
    }
    #[doc = "Bit 31 - Enable source ACMP/DCMP in single mode"]
    #[inline(always)]
    pub fn osht(&mut self) -> OSHT_W {
        OSHT_W { w: self }
    }
}
