#[doc = "Reader of register MAC2"]
pub type R = crate::R<u32, super::MAC2>;
#[doc = "Writer for register MAC2"]
pub type W = crate::W<u32, super::MAC2>;
#[doc = "Register MAC2 `reset()`'s with value 0"]
impl crate::ResetValue for super::MAC2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FULLDUPLEX`"]
pub type FULLDUPLEX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FULLDUPLEX`"]
pub struct FULLDUPLEX_W<'a> {
    w: &'a mut W,
}
impl<'a> FULLDUPLEX_W<'a> {
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
#[doc = "Reader of field `LENGTHCHECK`"]
pub type LENGTHCHECK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LENGTHCHECK`"]
pub struct LENGTHCHECK_W<'a> {
    w: &'a mut W,
}
impl<'a> LENGTHCHECK_W<'a> {
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
#[doc = "Reader of field `HUGEFRAME`"]
pub type HUGEFRAME_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HUGEFRAME`"]
pub struct HUGEFRAME_W<'a> {
    w: &'a mut W,
}
impl<'a> HUGEFRAME_W<'a> {
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
#[doc = "Reader of field `DALAYCRC`"]
pub type DALAYCRC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DALAYCRC`"]
pub struct DALAYCRC_W<'a> {
    w: &'a mut W,
}
impl<'a> DALAYCRC_W<'a> {
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
#[doc = "Reader of field `CRCENABLE`"]
pub type CRCENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CRCENABLE`"]
pub struct CRCENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCENABLE_W<'a> {
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
#[doc = "Reader of field `PADENABLE`"]
pub type PADENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PADENABLE`"]
pub struct PADENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> PADENABLE_W<'a> {
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
#[doc = "Reader of field `VLANPAD`"]
pub type VLANPAD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VLANPAD`"]
pub struct VLANPAD_W<'a> {
    w: &'a mut W,
}
impl<'a> VLANPAD_W<'a> {
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
#[doc = "Reader of field `AUTOPAD`"]
pub type AUTOPAD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUTOPAD`"]
pub struct AUTOPAD_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTOPAD_W<'a> {
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
#[doc = "Reader of field `PUREPRE`"]
pub type PUREPRE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PUREPRE`"]
pub struct PUREPRE_W<'a> {
    w: &'a mut W,
}
impl<'a> PUREPRE_W<'a> {
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
#[doc = "Reader of field `LONGPRE`"]
pub type LONGPRE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LONGPRE`"]
pub struct LONGPRE_W<'a> {
    w: &'a mut W,
}
impl<'a> LONGPRE_W<'a> {
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
#[doc = "Reader of field `NOBACKOFF`"]
pub type NOBACKOFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NOBACKOFF`"]
pub struct NOBACKOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> NOBACKOFF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `BP_NOBACKOFF`"]
pub type BP_NOBACKOFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BP_NOBACKOFF`"]
pub struct BP_NOBACKOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> BP_NOBACKOFF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `EXCESSDEF`"]
pub type EXCESSDEF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXCESSDEF`"]
pub struct EXCESSDEF_W<'a> {
    w: &'a mut W,
}
impl<'a> EXCESSDEF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Mode Select bit MAC-run operations"]
    #[inline(always)]
    pub fn fullduplex(&self) -> FULLDUPLEX_R {
        FULLDUPLEX_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable bit komparatsii length Frame"]
    #[inline(always)]
    pub fn lengthcheck(&self) -> LENGTHCHECK_R {
        LENGTHCHECK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable Bit Frame reception and transmission of arbitrary length"]
    #[inline(always)]
    pub fn hugeframe(&self) -> HUGEFRAME_R {
        HUGEFRAME_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable bit adding a 4-byte CRC"]
    #[inline(always)]
    pub fn dalaycrc(&self) -> DALAYCRC_R {
        DALAYCRC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable bit CRC inserter"]
    #[inline(always)]
    pub fn crcenable(&self) -> CRCENABLE_R {
        CRCENABLE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Bit resolution and functioning bits AUTOPAD VLANPAD"]
    #[inline(always)]
    pub fn padenable(&self) -> PADENABLE_R {
        PADENABLE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enable bit short additions Frame"]
    #[inline(always)]
    pub fn vlanpad(&self) -> VLANPAD_R {
        VLANPAD_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Bit enable automatic determination of type Frame"]
    #[inline(always)]
    pub fn autopad(&self) -> AUTOPAD_R {
        AUTOPAD_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Bit enable validation preamble"]
    #[inline(always)]
    pub fn purepre(&self) -> PUREPRE_R {
        PUREPRE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Select bit preamble length packets received"]
    #[inline(always)]
    pub fn longpre(&self) -> LONGPRE_R {
        LONGPRE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Bit parameter setting retransmission in conflict"]
    #[inline(always)]
    pub fn nobackoff(&self) -> NOBACKOFF_R {
        NOBACKOFF_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Enable bit retransmission in conflict"]
    #[inline(always)]
    pub fn bp_nobackoff(&self) -> BP_NOBACKOFF_R {
        BP_NOBACKOFF_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Mode Select bit packet processing"]
    #[inline(always)]
    pub fn excessdef(&self) -> EXCESSDEF_R {
        EXCESSDEF_R::new(((self.bits >> 14) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mode Select bit MAC-run operations"]
    #[inline(always)]
    pub fn fullduplex(&mut self) -> FULLDUPLEX_W {
        FULLDUPLEX_W { w: self }
    }
    #[doc = "Bit 1 - Enable bit komparatsii length Frame"]
    #[inline(always)]
    pub fn lengthcheck(&mut self) -> LENGTHCHECK_W {
        LENGTHCHECK_W { w: self }
    }
    #[doc = "Bit 2 - Enable Bit Frame reception and transmission of arbitrary length"]
    #[inline(always)]
    pub fn hugeframe(&mut self) -> HUGEFRAME_W {
        HUGEFRAME_W { w: self }
    }
    #[doc = "Bit 3 - Enable bit adding a 4-byte CRC"]
    #[inline(always)]
    pub fn dalaycrc(&mut self) -> DALAYCRC_W {
        DALAYCRC_W { w: self }
    }
    #[doc = "Bit 4 - Enable bit CRC inserter"]
    #[inline(always)]
    pub fn crcenable(&mut self) -> CRCENABLE_W {
        CRCENABLE_W { w: self }
    }
    #[doc = "Bit 5 - Bit resolution and functioning bits AUTOPAD VLANPAD"]
    #[inline(always)]
    pub fn padenable(&mut self) -> PADENABLE_W {
        PADENABLE_W { w: self }
    }
    #[doc = "Bit 6 - Enable bit short additions Frame"]
    #[inline(always)]
    pub fn vlanpad(&mut self) -> VLANPAD_W {
        VLANPAD_W { w: self }
    }
    #[doc = "Bit 7 - Bit enable automatic determination of type Frame"]
    #[inline(always)]
    pub fn autopad(&mut self) -> AUTOPAD_W {
        AUTOPAD_W { w: self }
    }
    #[doc = "Bit 8 - Bit enable validation preamble"]
    #[inline(always)]
    pub fn purepre(&mut self) -> PUREPRE_W {
        PUREPRE_W { w: self }
    }
    #[doc = "Bit 9 - Select bit preamble length packets received"]
    #[inline(always)]
    pub fn longpre(&mut self) -> LONGPRE_W {
        LONGPRE_W { w: self }
    }
    #[doc = "Bit 12 - Bit parameter setting retransmission in conflict"]
    #[inline(always)]
    pub fn nobackoff(&mut self) -> NOBACKOFF_W {
        NOBACKOFF_W { w: self }
    }
    #[doc = "Bit 13 - Enable bit retransmission in conflict"]
    #[inline(always)]
    pub fn bp_nobackoff(&mut self) -> BP_NOBACKOFF_W {
        BP_NOBACKOFF_W { w: self }
    }
    #[doc = "Bit 14 - Mode Select bit packet processing"]
    #[inline(always)]
    pub fn excessdef(&mut self) -> EXCESSDEF_W {
        EXCESSDEF_W { w: self }
    }
}
