#[doc = "Reader of register CEP_CTRL_STAT"]
pub type R = crate::R<u32, super::CEP_CTRL_STAT>;
#[doc = "Writer for register CEP_CTRL_STAT"]
pub type W = crate::W<u32, super::CEP_CTRL_STAT>;
#[doc = "Register CEP_CTRL_STAT `reset()`'s with value 0"]
impl crate::ResetValue for super::CEP_CTRL_STAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `NAKCLEAR`"]
pub type NAKCLEAR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NAKCLEAR`"]
pub struct NAKCLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> NAKCLEAR_W<'a> {
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
#[doc = "Reader of field `STALL`"]
pub type STALL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STALL`"]
pub struct STALL_W<'a> {
    w: &'a mut W,
}
impl<'a> STALL_W<'a> {
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
#[doc = "Reader of field `ZEROLEN`"]
pub type ZEROLEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ZEROLEN`"]
pub struct ZEROLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ZEROLEN_W<'a> {
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
#[doc = "Reader of field `CEPFLUSH`"]
pub type CEPFLUSH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CEPFLUSH`"]
pub struct CEPFLUSH_W<'a> {
    w: &'a mut W,
}
impl<'a> CEPFLUSH_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Receiving flag tags SETIP"]
    #[inline(always)]
    pub fn nakclear(&self) -> NAKCLEAR_R {
        NAKCLEAR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable sending 'Stall'"]
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable transmission zero length packet"]
    #[inline(always)]
    pub fn zerolen(&self) -> ZEROLEN_R {
        ZEROLEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Reset buffer"]
    #[inline(always)]
    pub fn cepflush(&self) -> CEPFLUSH_R {
        CEPFLUSH_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receiving flag tags SETIP"]
    #[inline(always)]
    pub fn nakclear(&mut self) -> NAKCLEAR_W {
        NAKCLEAR_W { w: self }
    }
    #[doc = "Bit 1 - Enable sending 'Stall'"]
    #[inline(always)]
    pub fn stall(&mut self) -> STALL_W {
        STALL_W { w: self }
    }
    #[doc = "Bit 2 - Enable transmission zero length packet"]
    #[inline(always)]
    pub fn zerolen(&mut self) -> ZEROLEN_W {
        ZEROLEN_W { w: self }
    }
    #[doc = "Bit 3 - Reset buffer"]
    #[inline(always)]
    pub fn cepflush(&mut self) -> CEPFLUSH_W {
        CEPFLUSH_W { w: self }
    }
}
