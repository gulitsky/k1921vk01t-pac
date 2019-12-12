#[doc = "Reader of register QEPSTS"]
pub type R = crate::R<u32, super::QEPSTS>;
#[doc = "Writer for register QEPSTS"]
pub type W = crate::W<u32, super::QEPSTS>;
#[doc = "Register QEPSTS `reset()`'s with value 0"]
impl crate::ResetValue for super::QEPSTS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PCEF`"]
pub type PCEF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCEF`"]
pub struct PCEF_W<'a> {
    w: &'a mut W,
}
impl<'a> PCEF_W<'a> {
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
#[doc = "Reader of field `FIMF`"]
pub type FIMF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FIMF`"]
pub struct FIMF_W<'a> {
    w: &'a mut W,
}
impl<'a> FIMF_W<'a> {
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
#[doc = "Reader of field `CDEF`"]
pub type CDEF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CDEF`"]
pub struct CDEF_W<'a> {
    w: &'a mut W,
}
impl<'a> CDEF_W<'a> {
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
#[doc = "Reader of field `COEF`"]
pub type COEF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COEF`"]
pub struct COEF_W<'a> {
    w: &'a mut W,
}
impl<'a> COEF_W<'a> {
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
#[doc = "Reader of field `QDLF`"]
pub type QDLF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `QDLF`"]
pub struct QDLF_W<'a> {
    w: &'a mut W,
}
impl<'a> QDLF_W<'a> {
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
#[doc = "Reader of field `QDF`"]
pub type QDF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `QDF`"]
pub struct QDF_W<'a> {
    w: &'a mut W,
}
impl<'a> QDF_W<'a> {
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
#[doc = "Reader of field `FIDF`"]
pub type FIDF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FIDF`"]
pub struct FIDF_W<'a> {
    w: &'a mut W,
}
impl<'a> FIDF_W<'a> {
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
#[doc = "Reader of field `UPEVNT`"]
pub type UPEVNT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UPEVNT`"]
pub struct UPEVNT_W<'a> {
    w: &'a mut W,
}
impl<'a> UPEVNT_W<'a> {
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
impl R {
    #[doc = "Bit 0 - error position flag"]
    #[inline(always)]
    pub fn pcef(&self) -> PCEF_R {
        PCEF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - receiving impulse"]
    #[inline(always)]
    pub fn fimf(&self) -> FIMF_R {
        FIMF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - change rotation"]
    #[inline(always)]
    pub fn cdef(&self) -> CDEF_R {
        CDEF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Flag Counter Overflow quadrature"]
    #[inline(always)]
    pub fn coef(&self) -> COEF_R {
        COEF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - rotation of the rotor shaft"]
    #[inline(always)]
    pub fn qdlf(&self) -> QDLF_R {
        QDLF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - rotation of the rotor shaft"]
    #[inline(always)]
    pub fn qdf(&self) -> QDF_R {
        QDF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - rotation of the rotor shaft"]
    #[inline(always)]
    pub fn fidf(&self) -> FIDF_R {
        FIDF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Flag timer time samples"]
    #[inline(always)]
    pub fn upevnt(&self) -> UPEVNT_R {
        UPEVNT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - error position flag"]
    #[inline(always)]
    pub fn pcef(&mut self) -> PCEF_W {
        PCEF_W { w: self }
    }
    #[doc = "Bit 1 - receiving impulse"]
    #[inline(always)]
    pub fn fimf(&mut self) -> FIMF_W {
        FIMF_W { w: self }
    }
    #[doc = "Bit 2 - change rotation"]
    #[inline(always)]
    pub fn cdef(&mut self) -> CDEF_W {
        CDEF_W { w: self }
    }
    #[doc = "Bit 3 - Flag Counter Overflow quadrature"]
    #[inline(always)]
    pub fn coef(&mut self) -> COEF_W {
        COEF_W { w: self }
    }
    #[doc = "Bit 4 - rotation of the rotor shaft"]
    #[inline(always)]
    pub fn qdlf(&mut self) -> QDLF_W {
        QDLF_W { w: self }
    }
    #[doc = "Bit 5 - rotation of the rotor shaft"]
    #[inline(always)]
    pub fn qdf(&mut self) -> QDF_W {
        QDF_W { w: self }
    }
    #[doc = "Bit 6 - rotation of the rotor shaft"]
    #[inline(always)]
    pub fn fidf(&mut self) -> FIDF_W {
        FIDF_W { w: self }
    }
    #[doc = "Bit 7 - Flag timer time samples"]
    #[inline(always)]
    pub fn upevnt(&mut self) -> UPEVNT_W {
        UPEVNT_W { w: self }
    }
}
