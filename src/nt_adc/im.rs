#[doc = "Reader of register IM"]
pub type R = crate::R<u32, super::IM>;
#[doc = "Writer for register IM"]
pub type W = crate::W<u32, super::IM>;
#[doc = "Register IM `reset()`'s with value 0"]
impl crate::ResetValue for super::IM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MASK0`"]
pub type MASK0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MASK0`"]
pub struct MASK0_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK0_W<'a> {
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
#[doc = "Reader of field `MASK1`"]
pub type MASK1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MASK1`"]
pub struct MASK1_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK1_W<'a> {
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
#[doc = "Reader of field `MASK2`"]
pub type MASK2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MASK2`"]
pub struct MASK2_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK2_W<'a> {
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
#[doc = "Reader of field `MASK3`"]
pub type MASK3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MASK3`"]
pub struct MASK3_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK3_W<'a> {
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
#[doc = "Reader of field `MASK4`"]
pub type MASK4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MASK4`"]
pub struct MASK4_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK4_W<'a> {
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
#[doc = "Reader of field `MASK5`"]
pub type MASK5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MASK5`"]
pub struct MASK5_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK5_W<'a> {
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
#[doc = "Reader of field `MASK6`"]
pub type MASK6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MASK6`"]
pub struct MASK6_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK6_W<'a> {
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
#[doc = "Reader of field `MASK7`"]
pub type MASK7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MASK7`"]
pub struct MASK7_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK7_W<'a> {
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
#[doc = "Reader of field `MIDC0`"]
pub type MIDC0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MIDC0`"]
pub struct MIDC0_W<'a> {
    w: &'a mut W,
}
impl<'a> MIDC0_W<'a> {
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
#[doc = "Reader of field `MIDC1`"]
pub type MIDC1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MIDC1`"]
pub struct MIDC1_W<'a> {
    w: &'a mut W,
}
impl<'a> MIDC1_W<'a> {
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
#[doc = "Reader of field `MIDC2`"]
pub type MIDC2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MIDC2`"]
pub struct MIDC2_W<'a> {
    w: &'a mut W,
}
impl<'a> MIDC2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `MIDC3`"]
pub type MIDC3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MIDC3`"]
pub struct MIDC3_W<'a> {
    w: &'a mut W,
}
impl<'a> MIDC3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `MIDC4`"]
pub type MIDC4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MIDC4`"]
pub struct MIDC4_W<'a> {
    w: &'a mut W,
}
impl<'a> MIDC4_W<'a> {
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
#[doc = "Reader of field `MIDC5`"]
pub type MIDC5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MIDC5`"]
pub struct MIDC5_W<'a> {
    w: &'a mut W,
}
impl<'a> MIDC5_W<'a> {
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
#[doc = "Reader of field `MIDC6`"]
pub type MIDC6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MIDC6`"]
pub struct MIDC6_W<'a> {
    w: &'a mut W,
}
impl<'a> MIDC6_W<'a> {
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
#[doc = "Reader of field `MIDC7`"]
pub type MIDC7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MIDC7`"]
pub struct MIDC7_W<'a> {
    w: &'a mut W,
}
impl<'a> MIDC7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `MIDC8`"]
pub type MIDC8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MIDC8`"]
pub struct MIDC8_W<'a> {
    w: &'a mut W,
}
impl<'a> MIDC8_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `MIDC9`"]
pub type MIDC9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MIDC9`"]
pub struct MIDC9_W<'a> {
    w: &'a mut W,
}
impl<'a> MIDC9_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `MIDC10`"]
pub type MIDC10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MIDC10`"]
pub struct MIDC10_W<'a> {
    w: &'a mut W,
}
impl<'a> MIDC10_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `MIDC11`"]
pub type MIDC11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MIDC11`"]
pub struct MIDC11_W<'a> {
    w: &'a mut W,
}
impl<'a> MIDC11_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `MIDC12`"]
pub type MIDC12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MIDC12`"]
pub struct MIDC12_W<'a> {
    w: &'a mut W,
}
impl<'a> MIDC12_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `MIDC13`"]
pub type MIDC13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MIDC13`"]
pub struct MIDC13_W<'a> {
    w: &'a mut W,
}
impl<'a> MIDC13_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `MIDC14`"]
pub type MIDC14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MIDC14`"]
pub struct MIDC14_W<'a> {
    w: &'a mut W,
}
impl<'a> MIDC14_W<'a> {
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
#[doc = "Reader of field `MIDC15`"]
pub type MIDC15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MIDC15`"]
pub struct MIDC15_W<'a> {
    w: &'a mut W,
}
impl<'a> MIDC15_W<'a> {
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
#[doc = "Reader of field `MIDC16`"]
pub type MIDC16_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MIDC16`"]
pub struct MIDC16_W<'a> {
    w: &'a mut W,
}
impl<'a> MIDC16_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `MIDC17`"]
pub type MIDC17_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MIDC17`"]
pub struct MIDC17_W<'a> {
    w: &'a mut W,
}
impl<'a> MIDC17_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `MIDC18`"]
pub type MIDC18_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MIDC18`"]
pub struct MIDC18_W<'a> {
    w: &'a mut W,
}
impl<'a> MIDC18_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `MIDC19`"]
pub type MIDC19_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MIDC19`"]
pub struct MIDC19_W<'a> {
    w: &'a mut W,
}
impl<'a> MIDC19_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `MIDC20`"]
pub type MIDC20_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MIDC20`"]
pub struct MIDC20_W<'a> {
    w: &'a mut W,
}
impl<'a> MIDC20_W<'a> {
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
#[doc = "Reader of field `MIDC21`"]
pub type MIDC21_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MIDC21`"]
pub struct MIDC21_W<'a> {
    w: &'a mut W,
}
impl<'a> MIDC21_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `MIDC22`"]
pub type MIDC22_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MIDC22`"]
pub struct MIDC22_W<'a> {
    w: &'a mut W,
}
impl<'a> MIDC22_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `MIDC23`"]
pub type MIDC23_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MIDC23`"]
pub struct MIDC23_W<'a> {
    w: &'a mut W,
}
impl<'a> MIDC23_W<'a> {
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
    #[doc = "Bit 0 - Sequencer 0 interrupt mask"]
    #[inline(always)]
    pub fn mask0(&self) -> MASK0_R {
        MASK0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Sequencer 1 interrupt mask"]
    #[inline(always)]
    pub fn mask1(&self) -> MASK1_R {
        MASK1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Sequencer 2 interrupt mask"]
    #[inline(always)]
    pub fn mask2(&self) -> MASK2_R {
        MASK2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Sequencer 3 interrupt mask"]
    #[inline(always)]
    pub fn mask3(&self) -> MASK3_R {
        MASK3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Sequencer 4 interrupt mask"]
    #[inline(always)]
    pub fn mask4(&self) -> MASK4_R {
        MASK4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Sequencer 5 interrupt mask"]
    #[inline(always)]
    pub fn mask5(&self) -> MASK5_R {
        MASK5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Sequencer 6 interrupt mask"]
    #[inline(always)]
    pub fn mask6(&self) -> MASK6_R {
        MASK6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Sequencer 7 interrupt mask"]
    #[inline(always)]
    pub fn mask7(&self) -> MASK7_R {
        MASK7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - DCMP 0 interrupt mask"]
    #[inline(always)]
    pub fn midc0(&self) -> MIDC0_R {
        MIDC0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - DCMP 1 interrupt mask"]
    #[inline(always)]
    pub fn midc1(&self) -> MIDC1_R {
        MIDC1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - DCMP 2 interrupt mask"]
    #[inline(always)]
    pub fn midc2(&self) -> MIDC2_R {
        MIDC2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - DCMP 3 interrupt mask"]
    #[inline(always)]
    pub fn midc3(&self) -> MIDC3_R {
        MIDC3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - DCMP 4 interrupt mask"]
    #[inline(always)]
    pub fn midc4(&self) -> MIDC4_R {
        MIDC4_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - DCMP 5 interrupt mask"]
    #[inline(always)]
    pub fn midc5(&self) -> MIDC5_R {
        MIDC5_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - DCMP 6 interrupt mask"]
    #[inline(always)]
    pub fn midc6(&self) -> MIDC6_R {
        MIDC6_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - DCMP 7 interrupt mask"]
    #[inline(always)]
    pub fn midc7(&self) -> MIDC7_R {
        MIDC7_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - DCMP 8 interrupt mask"]
    #[inline(always)]
    pub fn midc8(&self) -> MIDC8_R {
        MIDC8_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - DCMP 9 interrupt mask"]
    #[inline(always)]
    pub fn midc9(&self) -> MIDC9_R {
        MIDC9_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - DCMP 10 interrupt mask"]
    #[inline(always)]
    pub fn midc10(&self) -> MIDC10_R {
        MIDC10_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - DCMP 11 interrupt mask"]
    #[inline(always)]
    pub fn midc11(&self) -> MIDC11_R {
        MIDC11_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - DCMP 12 interrupt mask"]
    #[inline(always)]
    pub fn midc12(&self) -> MIDC12_R {
        MIDC12_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - DCMP 13 interrupt mask"]
    #[inline(always)]
    pub fn midc13(&self) -> MIDC13_R {
        MIDC13_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - DCMP 14 interrupt mask"]
    #[inline(always)]
    pub fn midc14(&self) -> MIDC14_R {
        MIDC14_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - DCMP 15 interrupt mask"]
    #[inline(always)]
    pub fn midc15(&self) -> MIDC15_R {
        MIDC15_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - DCMP 16 interrupt mask"]
    #[inline(always)]
    pub fn midc16(&self) -> MIDC16_R {
        MIDC16_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - DCMP 17 interrupt mask"]
    #[inline(always)]
    pub fn midc17(&self) -> MIDC17_R {
        MIDC17_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - DCMP 18 interrupt mask"]
    #[inline(always)]
    pub fn midc18(&self) -> MIDC18_R {
        MIDC18_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - DCMP 19 interrupt mask"]
    #[inline(always)]
    pub fn midc19(&self) -> MIDC19_R {
        MIDC19_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - DCMP 20 interrupt mask"]
    #[inline(always)]
    pub fn midc20(&self) -> MIDC20_R {
        MIDC20_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - DCMP 21 interrupt mask"]
    #[inline(always)]
    pub fn midc21(&self) -> MIDC21_R {
        MIDC21_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - DCMP 22 interrupt mask"]
    #[inline(always)]
    pub fn midc22(&self) -> MIDC22_R {
        MIDC22_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - DCMP 23 interrupt mask"]
    #[inline(always)]
    pub fn midc23(&self) -> MIDC23_R {
        MIDC23_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Sequencer 0 interrupt mask"]
    #[inline(always)]
    pub fn mask0(&mut self) -> MASK0_W {
        MASK0_W { w: self }
    }
    #[doc = "Bit 1 - Sequencer 1 interrupt mask"]
    #[inline(always)]
    pub fn mask1(&mut self) -> MASK1_W {
        MASK1_W { w: self }
    }
    #[doc = "Bit 2 - Sequencer 2 interrupt mask"]
    #[inline(always)]
    pub fn mask2(&mut self) -> MASK2_W {
        MASK2_W { w: self }
    }
    #[doc = "Bit 3 - Sequencer 3 interrupt mask"]
    #[inline(always)]
    pub fn mask3(&mut self) -> MASK3_W {
        MASK3_W { w: self }
    }
    #[doc = "Bit 4 - Sequencer 4 interrupt mask"]
    #[inline(always)]
    pub fn mask4(&mut self) -> MASK4_W {
        MASK4_W { w: self }
    }
    #[doc = "Bit 5 - Sequencer 5 interrupt mask"]
    #[inline(always)]
    pub fn mask5(&mut self) -> MASK5_W {
        MASK5_W { w: self }
    }
    #[doc = "Bit 6 - Sequencer 6 interrupt mask"]
    #[inline(always)]
    pub fn mask6(&mut self) -> MASK6_W {
        MASK6_W { w: self }
    }
    #[doc = "Bit 7 - Sequencer 7 interrupt mask"]
    #[inline(always)]
    pub fn mask7(&mut self) -> MASK7_W {
        MASK7_W { w: self }
    }
    #[doc = "Bit 8 - DCMP 0 interrupt mask"]
    #[inline(always)]
    pub fn midc0(&mut self) -> MIDC0_W {
        MIDC0_W { w: self }
    }
    #[doc = "Bit 9 - DCMP 1 interrupt mask"]
    #[inline(always)]
    pub fn midc1(&mut self) -> MIDC1_W {
        MIDC1_W { w: self }
    }
    #[doc = "Bit 10 - DCMP 2 interrupt mask"]
    #[inline(always)]
    pub fn midc2(&mut self) -> MIDC2_W {
        MIDC2_W { w: self }
    }
    #[doc = "Bit 11 - DCMP 3 interrupt mask"]
    #[inline(always)]
    pub fn midc3(&mut self) -> MIDC3_W {
        MIDC3_W { w: self }
    }
    #[doc = "Bit 12 - DCMP 4 interrupt mask"]
    #[inline(always)]
    pub fn midc4(&mut self) -> MIDC4_W {
        MIDC4_W { w: self }
    }
    #[doc = "Bit 13 - DCMP 5 interrupt mask"]
    #[inline(always)]
    pub fn midc5(&mut self) -> MIDC5_W {
        MIDC5_W { w: self }
    }
    #[doc = "Bit 14 - DCMP 6 interrupt mask"]
    #[inline(always)]
    pub fn midc6(&mut self) -> MIDC6_W {
        MIDC6_W { w: self }
    }
    #[doc = "Bit 15 - DCMP 7 interrupt mask"]
    #[inline(always)]
    pub fn midc7(&mut self) -> MIDC7_W {
        MIDC7_W { w: self }
    }
    #[doc = "Bit 16 - DCMP 8 interrupt mask"]
    #[inline(always)]
    pub fn midc8(&mut self) -> MIDC8_W {
        MIDC8_W { w: self }
    }
    #[doc = "Bit 17 - DCMP 9 interrupt mask"]
    #[inline(always)]
    pub fn midc9(&mut self) -> MIDC9_W {
        MIDC9_W { w: self }
    }
    #[doc = "Bit 18 - DCMP 10 interrupt mask"]
    #[inline(always)]
    pub fn midc10(&mut self) -> MIDC10_W {
        MIDC10_W { w: self }
    }
    #[doc = "Bit 19 - DCMP 11 interrupt mask"]
    #[inline(always)]
    pub fn midc11(&mut self) -> MIDC11_W {
        MIDC11_W { w: self }
    }
    #[doc = "Bit 20 - DCMP 12 interrupt mask"]
    #[inline(always)]
    pub fn midc12(&mut self) -> MIDC12_W {
        MIDC12_W { w: self }
    }
    #[doc = "Bit 21 - DCMP 13 interrupt mask"]
    #[inline(always)]
    pub fn midc13(&mut self) -> MIDC13_W {
        MIDC13_W { w: self }
    }
    #[doc = "Bit 22 - DCMP 14 interrupt mask"]
    #[inline(always)]
    pub fn midc14(&mut self) -> MIDC14_W {
        MIDC14_W { w: self }
    }
    #[doc = "Bit 23 - DCMP 15 interrupt mask"]
    #[inline(always)]
    pub fn midc15(&mut self) -> MIDC15_W {
        MIDC15_W { w: self }
    }
    #[doc = "Bit 24 - DCMP 16 interrupt mask"]
    #[inline(always)]
    pub fn midc16(&mut self) -> MIDC16_W {
        MIDC16_W { w: self }
    }
    #[doc = "Bit 25 - DCMP 17 interrupt mask"]
    #[inline(always)]
    pub fn midc17(&mut self) -> MIDC17_W {
        MIDC17_W { w: self }
    }
    #[doc = "Bit 26 - DCMP 18 interrupt mask"]
    #[inline(always)]
    pub fn midc18(&mut self) -> MIDC18_W {
        MIDC18_W { w: self }
    }
    #[doc = "Bit 27 - DCMP 19 interrupt mask"]
    #[inline(always)]
    pub fn midc19(&mut self) -> MIDC19_W {
        MIDC19_W { w: self }
    }
    #[doc = "Bit 28 - DCMP 20 interrupt mask"]
    #[inline(always)]
    pub fn midc20(&mut self) -> MIDC20_W {
        MIDC20_W { w: self }
    }
    #[doc = "Bit 29 - DCMP 21 interrupt mask"]
    #[inline(always)]
    pub fn midc21(&mut self) -> MIDC21_W {
        MIDC21_W { w: self }
    }
    #[doc = "Bit 30 - DCMP 22 interrupt mask"]
    #[inline(always)]
    pub fn midc22(&mut self) -> MIDC22_W {
        MIDC22_W { w: self }
    }
    #[doc = "Bit 31 - DCMP 23 interrupt mask"]
    #[inline(always)]
    pub fn midc23(&mut self) -> MIDC23_W {
        MIDC23_W { w: self }
    }
}
