#[doc = "Reader of register USB_OPER"]
pub type R = crate::R<u32, super::USB_OPER>;
#[doc = "Writer for register USB_OPER"]
pub type W = crate::W<u32, super::USB_OPER>;
#[doc = "Register USB_OPER `reset()`'s with value 0"]
impl crate::ResetValue for super::USB_OPER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GEN_RESUME`"]
pub type GEN_RESUME_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GEN_RESUME`"]
pub struct GEN_RESUME_W<'a> {
    w: &'a mut W,
}
impl<'a> GEN_RESUME_W<'a> {
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
#[doc = "Reader of field `HIGHSPEED`"]
pub type HIGHSPEED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HIGHSPEED`"]
pub struct HIGHSPEED_W<'a> {
    w: &'a mut W,
}
impl<'a> HIGHSPEED_W<'a> {
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
#[doc = "Reader of field `CURRENTSPEED`"]
pub type CURRENTSPEED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CURRENTSPEED`"]
pub struct CURRENTSPEED_W<'a> {
    w: &'a mut W,
}
impl<'a> CURRENTSPEED_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Bit start recovery work"]
    #[inline(always)]
    pub fn gen_resume(&self) -> GEN_RESUME_R {
        GEN_RESUME_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Trigger bit protocol 'Chirp'"]
    #[inline(always)]
    pub fn highspeed(&self) -> HIGHSPEED_R {
        HIGHSPEED_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Speed indication device controller 1-High speed; 0 - Full speed"]
    #[inline(always)]
    pub fn currentspeed(&self) -> CURRENTSPEED_R {
        CURRENTSPEED_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bit start recovery work"]
    #[inline(always)]
    pub fn gen_resume(&mut self) -> GEN_RESUME_W {
        GEN_RESUME_W { w: self }
    }
    #[doc = "Bit 1 - Trigger bit protocol 'Chirp'"]
    #[inline(always)]
    pub fn highspeed(&mut self) -> HIGHSPEED_W {
        HIGHSPEED_W { w: self }
    }
    #[doc = "Bit 2 - Speed indication device controller 1-High speed; 0 - Full speed"]
    #[inline(always)]
    pub fn currentspeed(&mut self) -> CURRENTSPEED_W {
        CURRENTSPEED_W { w: self }
    }
}
