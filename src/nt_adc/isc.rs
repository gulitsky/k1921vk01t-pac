#[doc = "Reader of register ISC"]
pub type R = crate::R<u32, super::ISC>;
#[doc = "Writer for register ISC"]
pub type W = crate::W<u32, super::ISC>;
#[doc = "Register ISC `reset()`'s with value 0"]
impl crate::ResetValue for super::ISC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IN0`"]
pub type IN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IN0`"]
pub struct IN0_W<'a> {
    w: &'a mut W,
}
impl<'a> IN0_W<'a> {
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
#[doc = "Reader of field `IN1`"]
pub type IN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IN1`"]
pub struct IN1_W<'a> {
    w: &'a mut W,
}
impl<'a> IN1_W<'a> {
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
#[doc = "Reader of field `IN2`"]
pub type IN2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IN2`"]
pub struct IN2_W<'a> {
    w: &'a mut W,
}
impl<'a> IN2_W<'a> {
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
#[doc = "Reader of field `IN3`"]
pub type IN3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IN3`"]
pub struct IN3_W<'a> {
    w: &'a mut W,
}
impl<'a> IN3_W<'a> {
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
#[doc = "Reader of field `IN4`"]
pub type IN4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IN4`"]
pub struct IN4_W<'a> {
    w: &'a mut W,
}
impl<'a> IN4_W<'a> {
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
#[doc = "Reader of field `IN5`"]
pub type IN5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IN5`"]
pub struct IN5_W<'a> {
    w: &'a mut W,
}
impl<'a> IN5_W<'a> {
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
#[doc = "Reader of field `IN6`"]
pub type IN6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IN6`"]
pub struct IN6_W<'a> {
    w: &'a mut W,
}
impl<'a> IN6_W<'a> {
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
#[doc = "Reader of field `IN7`"]
pub type IN7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IN7`"]
pub struct IN7_W<'a> {
    w: &'a mut W,
}
impl<'a> IN7_W<'a> {
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
#[doc = "Reader of field `DCIN0`"]
pub type DCIN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCIN0`"]
pub struct DCIN0_W<'a> {
    w: &'a mut W,
}
impl<'a> DCIN0_W<'a> {
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
#[doc = "Reader of field `DCIN1`"]
pub type DCIN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCIN1`"]
pub struct DCIN1_W<'a> {
    w: &'a mut W,
}
impl<'a> DCIN1_W<'a> {
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
#[doc = "Reader of field `DCIN2`"]
pub type DCIN2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCIN2`"]
pub struct DCIN2_W<'a> {
    w: &'a mut W,
}
impl<'a> DCIN2_W<'a> {
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
#[doc = "Reader of field `DCIN3`"]
pub type DCIN3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCIN3`"]
pub struct DCIN3_W<'a> {
    w: &'a mut W,
}
impl<'a> DCIN3_W<'a> {
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
#[doc = "Reader of field `DCIN4`"]
pub type DCIN4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCIN4`"]
pub struct DCIN4_W<'a> {
    w: &'a mut W,
}
impl<'a> DCIN4_W<'a> {
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
#[doc = "Reader of field `DCIN5`"]
pub type DCIN5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCIN5`"]
pub struct DCIN5_W<'a> {
    w: &'a mut W,
}
impl<'a> DCIN5_W<'a> {
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
#[doc = "Reader of field `DCIN6`"]
pub type DCIN6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCIN6`"]
pub struct DCIN6_W<'a> {
    w: &'a mut W,
}
impl<'a> DCIN6_W<'a> {
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
#[doc = "Reader of field `DCIN7`"]
pub type DCIN7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCIN7`"]
pub struct DCIN7_W<'a> {
    w: &'a mut W,
}
impl<'a> DCIN7_W<'a> {
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
#[doc = "Reader of field `DCIN8`"]
pub type DCIN8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCIN8`"]
pub struct DCIN8_W<'a> {
    w: &'a mut W,
}
impl<'a> DCIN8_W<'a> {
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
#[doc = "Reader of field `DCIN9`"]
pub type DCIN9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCIN9`"]
pub struct DCIN9_W<'a> {
    w: &'a mut W,
}
impl<'a> DCIN9_W<'a> {
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
#[doc = "Reader of field `DCIN10`"]
pub type DCIN10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCIN10`"]
pub struct DCIN10_W<'a> {
    w: &'a mut W,
}
impl<'a> DCIN10_W<'a> {
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
#[doc = "Reader of field `DCIN11`"]
pub type DCIN11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCIN11`"]
pub struct DCIN11_W<'a> {
    w: &'a mut W,
}
impl<'a> DCIN11_W<'a> {
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
#[doc = "Reader of field `DCIN12`"]
pub type DCIN12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCIN12`"]
pub struct DCIN12_W<'a> {
    w: &'a mut W,
}
impl<'a> DCIN12_W<'a> {
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
#[doc = "Reader of field `DCIN13`"]
pub type DCIN13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCIN13`"]
pub struct DCIN13_W<'a> {
    w: &'a mut W,
}
impl<'a> DCIN13_W<'a> {
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
#[doc = "Reader of field `DCIN14`"]
pub type DCIN14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCIN14`"]
pub struct DCIN14_W<'a> {
    w: &'a mut W,
}
impl<'a> DCIN14_W<'a> {
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
#[doc = "Reader of field `DCIN15`"]
pub type DCIN15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCIN15`"]
pub struct DCIN15_W<'a> {
    w: &'a mut W,
}
impl<'a> DCIN15_W<'a> {
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
#[doc = "Reader of field `DCIN16`"]
pub type DCIN16_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCIN16`"]
pub struct DCIN16_W<'a> {
    w: &'a mut W,
}
impl<'a> DCIN16_W<'a> {
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
#[doc = "Reader of field `DCIN17`"]
pub type DCIN17_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCIN17`"]
pub struct DCIN17_W<'a> {
    w: &'a mut W,
}
impl<'a> DCIN17_W<'a> {
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
#[doc = "Reader of field `DCIN18`"]
pub type DCIN18_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCIN18`"]
pub struct DCIN18_W<'a> {
    w: &'a mut W,
}
impl<'a> DCIN18_W<'a> {
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
#[doc = "Reader of field `DCIN19`"]
pub type DCIN19_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCIN19`"]
pub struct DCIN19_W<'a> {
    w: &'a mut W,
}
impl<'a> DCIN19_W<'a> {
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
#[doc = "Reader of field `DCIN20`"]
pub type DCIN20_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCIN20`"]
pub struct DCIN20_W<'a> {
    w: &'a mut W,
}
impl<'a> DCIN20_W<'a> {
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
#[doc = "Reader of field `DCIN21`"]
pub type DCIN21_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCIN21`"]
pub struct DCIN21_W<'a> {
    w: &'a mut W,
}
impl<'a> DCIN21_W<'a> {
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
#[doc = "Reader of field `DCIN22`"]
pub type DCIN22_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCIN22`"]
pub struct DCIN22_W<'a> {
    w: &'a mut W,
}
impl<'a> DCIN22_W<'a> {
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
#[doc = "Reader of field `DCIN23`"]
pub type DCIN23_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCIN23`"]
pub struct DCIN23_W<'a> {
    w: &'a mut W,
}
impl<'a> DCIN23_W<'a> {
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
    #[doc = "Bit 0 - Sequencer 0 masked interrupt status / Write 1 to clear (also clear raw status)"]
    #[inline(always)]
    pub fn in0(&self) -> IN0_R {
        IN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Sequencer 1 masked interrupt status / Write 1 to clear (also clear raw status)"]
    #[inline(always)]
    pub fn in1(&self) -> IN1_R {
        IN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Sequencer 2 masked interrupt status / Write 1 to clear (also clear raw status)"]
    #[inline(always)]
    pub fn in2(&self) -> IN2_R {
        IN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Sequencer 3 masked interrupt status / Write 1 to clear (also clear raw status)"]
    #[inline(always)]
    pub fn in3(&self) -> IN3_R {
        IN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Sequencer 4 masked interrupt status / Write 1 to clear (also clear raw status)"]
    #[inline(always)]
    pub fn in4(&self) -> IN4_R {
        IN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Sequencer 5 masked interrupt status / Write 1 to clear (also clear raw status)"]
    #[inline(always)]
    pub fn in5(&self) -> IN5_R {
        IN5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Sequencer 6 masked interrupt status / Write 1 to clear (also clear raw status)"]
    #[inline(always)]
    pub fn in6(&self) -> IN6_R {
        IN6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Sequencer 7 masked interrupt status / Write 1 to clear (also clear raw status)"]
    #[inline(always)]
    pub fn in7(&self) -> IN7_R {
        IN7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - DCMP 0 masked interrupt status / Write 1 to clear (also clear raw status)"]
    #[inline(always)]
    pub fn dcin0(&self) -> DCIN0_R {
        DCIN0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - DCMP 1 masked interrupt status / Write 1 to clear (also clear raw status)"]
    #[inline(always)]
    pub fn dcin1(&self) -> DCIN1_R {
        DCIN1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - DCMP 2 masked interrupt status / Write 1 to clear (also clear raw status)"]
    #[inline(always)]
    pub fn dcin2(&self) -> DCIN2_R {
        DCIN2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - DCMP 3 masked interrupt status / Write 1 to clear (also clear raw status)"]
    #[inline(always)]
    pub fn dcin3(&self) -> DCIN3_R {
        DCIN3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - DCMP 4 masked interrupt status / Write 1 to clear (also clear raw status)"]
    #[inline(always)]
    pub fn dcin4(&self) -> DCIN4_R {
        DCIN4_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - DCMP 5 masked interrupt status / Write 1 to clear (also clear raw status)"]
    #[inline(always)]
    pub fn dcin5(&self) -> DCIN5_R {
        DCIN5_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - DCMP 6 masked interrupt status / Write 1 to clear (also clear raw status)"]
    #[inline(always)]
    pub fn dcin6(&self) -> DCIN6_R {
        DCIN6_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - DCMP 7 masked interrupt status / Write 1 to clear (also clear raw status)"]
    #[inline(always)]
    pub fn dcin7(&self) -> DCIN7_R {
        DCIN7_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - DCMP 8 masked interrupt status / Write 1 to clear (also clear raw status)"]
    #[inline(always)]
    pub fn dcin8(&self) -> DCIN8_R {
        DCIN8_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - DCMP 9 masked interrupt status / Write 1 to clear (also clear raw status)"]
    #[inline(always)]
    pub fn dcin9(&self) -> DCIN9_R {
        DCIN9_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - DCMP 10 masked interrupt status / Write 1 to clear (also clear raw status)"]
    #[inline(always)]
    pub fn dcin10(&self) -> DCIN10_R {
        DCIN10_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - DCMP 11 masked interrupt status / Write 1 to clear (also clear raw status)"]
    #[inline(always)]
    pub fn dcin11(&self) -> DCIN11_R {
        DCIN11_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - DCMP 12 masked interrupt status / Write 1 to clear (also clear raw status)"]
    #[inline(always)]
    pub fn dcin12(&self) -> DCIN12_R {
        DCIN12_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - DCMP 13 masked interrupt status / Write 1 to clear (also clear raw status)"]
    #[inline(always)]
    pub fn dcin13(&self) -> DCIN13_R {
        DCIN13_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - DCMP 14 masked interrupt status / Write 1 to clear (also clear raw status)"]
    #[inline(always)]
    pub fn dcin14(&self) -> DCIN14_R {
        DCIN14_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - DCMP 15 masked interrupt status / Write 1 to clear (also clear raw status)"]
    #[inline(always)]
    pub fn dcin15(&self) -> DCIN15_R {
        DCIN15_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - DCMP 16 masked interrupt status / Write 1 to clear (also clear raw status)"]
    #[inline(always)]
    pub fn dcin16(&self) -> DCIN16_R {
        DCIN16_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - DCMP 17 masked interrupt status / Write 1 to clear (also clear raw status)"]
    #[inline(always)]
    pub fn dcin17(&self) -> DCIN17_R {
        DCIN17_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - DCMP 18 masked interrupt status / Write 1 to clear (also clear raw status)"]
    #[inline(always)]
    pub fn dcin18(&self) -> DCIN18_R {
        DCIN18_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - DCMP 19 masked interrupt status / Write 1 to clear (also clear raw status)"]
    #[inline(always)]
    pub fn dcin19(&self) -> DCIN19_R {
        DCIN19_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - DCMP 20 masked interrupt status / Write 1 to clear (also clear raw status)"]
    #[inline(always)]
    pub fn dcin20(&self) -> DCIN20_R {
        DCIN20_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - DCMP 21 masked interrupt status / Write 1 to clear (also clear raw status)"]
    #[inline(always)]
    pub fn dcin21(&self) -> DCIN21_R {
        DCIN21_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - DCMP 22 masked interrupt status / Write 1 to clear (also clear raw status)"]
    #[inline(always)]
    pub fn dcin22(&self) -> DCIN22_R {
        DCIN22_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - DCMP 23 masked interrupt status / Write 1 to clear (also clear raw status)"]
    #[inline(always)]
    pub fn dcin23(&self) -> DCIN23_R {
        DCIN23_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Sequencer 0 masked interrupt status / Write 1 to clear (also clear raw status)"]
    #[inline(always)]
    pub fn in0(&mut self) -> IN0_W {
        IN0_W { w: self }
    }
    #[doc = "Bit 1 - Sequencer 1 masked interrupt status / Write 1 to clear (also clear raw status)"]
    #[inline(always)]
    pub fn in1(&mut self) -> IN1_W {
        IN1_W { w: self }
    }
    #[doc = "Bit 2 - Sequencer 2 masked interrupt status / Write 1 to clear (also clear raw status)"]
    #[inline(always)]
    pub fn in2(&mut self) -> IN2_W {
        IN2_W { w: self }
    }
    #[doc = "Bit 3 - Sequencer 3 masked interrupt status / Write 1 to clear (also clear raw status)"]
    #[inline(always)]
    pub fn in3(&mut self) -> IN3_W {
        IN3_W { w: self }
    }
    #[doc = "Bit 4 - Sequencer 4 masked interrupt status / Write 1 to clear (also clear raw status)"]
    #[inline(always)]
    pub fn in4(&mut self) -> IN4_W {
        IN4_W { w: self }
    }
    #[doc = "Bit 5 - Sequencer 5 masked interrupt status / Write 1 to clear (also clear raw status)"]
    #[inline(always)]
    pub fn in5(&mut self) -> IN5_W {
        IN5_W { w: self }
    }
    #[doc = "Bit 6 - Sequencer 6 masked interrupt status / Write 1 to clear (also clear raw status)"]
    #[inline(always)]
    pub fn in6(&mut self) -> IN6_W {
        IN6_W { w: self }
    }
    #[doc = "Bit 7 - Sequencer 7 masked interrupt status / Write 1 to clear (also clear raw status)"]
    #[inline(always)]
    pub fn in7(&mut self) -> IN7_W {
        IN7_W { w: self }
    }
    #[doc = "Bit 8 - DCMP 0 masked interrupt status / Write 1 to clear (also clear raw status)"]
    #[inline(always)]
    pub fn dcin0(&mut self) -> DCIN0_W {
        DCIN0_W { w: self }
    }
    #[doc = "Bit 9 - DCMP 1 masked interrupt status / Write 1 to clear (also clear raw status)"]
    #[inline(always)]
    pub fn dcin1(&mut self) -> DCIN1_W {
        DCIN1_W { w: self }
    }
    #[doc = "Bit 10 - DCMP 2 masked interrupt status / Write 1 to clear (also clear raw status)"]
    #[inline(always)]
    pub fn dcin2(&mut self) -> DCIN2_W {
        DCIN2_W { w: self }
    }
    #[doc = "Bit 11 - DCMP 3 masked interrupt status / Write 1 to clear (also clear raw status)"]
    #[inline(always)]
    pub fn dcin3(&mut self) -> DCIN3_W {
        DCIN3_W { w: self }
    }
    #[doc = "Bit 12 - DCMP 4 masked interrupt status / Write 1 to clear (also clear raw status)"]
    #[inline(always)]
    pub fn dcin4(&mut self) -> DCIN4_W {
        DCIN4_W { w: self }
    }
    #[doc = "Bit 13 - DCMP 5 masked interrupt status / Write 1 to clear (also clear raw status)"]
    #[inline(always)]
    pub fn dcin5(&mut self) -> DCIN5_W {
        DCIN5_W { w: self }
    }
    #[doc = "Bit 14 - DCMP 6 masked interrupt status / Write 1 to clear (also clear raw status)"]
    #[inline(always)]
    pub fn dcin6(&mut self) -> DCIN6_W {
        DCIN6_W { w: self }
    }
    #[doc = "Bit 15 - DCMP 7 masked interrupt status / Write 1 to clear (also clear raw status)"]
    #[inline(always)]
    pub fn dcin7(&mut self) -> DCIN7_W {
        DCIN7_W { w: self }
    }
    #[doc = "Bit 16 - DCMP 8 masked interrupt status / Write 1 to clear (also clear raw status)"]
    #[inline(always)]
    pub fn dcin8(&mut self) -> DCIN8_W {
        DCIN8_W { w: self }
    }
    #[doc = "Bit 17 - DCMP 9 masked interrupt status / Write 1 to clear (also clear raw status)"]
    #[inline(always)]
    pub fn dcin9(&mut self) -> DCIN9_W {
        DCIN9_W { w: self }
    }
    #[doc = "Bit 18 - DCMP 10 masked interrupt status / Write 1 to clear (also clear raw status)"]
    #[inline(always)]
    pub fn dcin10(&mut self) -> DCIN10_W {
        DCIN10_W { w: self }
    }
    #[doc = "Bit 19 - DCMP 11 masked interrupt status / Write 1 to clear (also clear raw status)"]
    #[inline(always)]
    pub fn dcin11(&mut self) -> DCIN11_W {
        DCIN11_W { w: self }
    }
    #[doc = "Bit 20 - DCMP 12 masked interrupt status / Write 1 to clear (also clear raw status)"]
    #[inline(always)]
    pub fn dcin12(&mut self) -> DCIN12_W {
        DCIN12_W { w: self }
    }
    #[doc = "Bit 21 - DCMP 13 masked interrupt status / Write 1 to clear (also clear raw status)"]
    #[inline(always)]
    pub fn dcin13(&mut self) -> DCIN13_W {
        DCIN13_W { w: self }
    }
    #[doc = "Bit 22 - DCMP 14 masked interrupt status / Write 1 to clear (also clear raw status)"]
    #[inline(always)]
    pub fn dcin14(&mut self) -> DCIN14_W {
        DCIN14_W { w: self }
    }
    #[doc = "Bit 23 - DCMP 15 masked interrupt status / Write 1 to clear (also clear raw status)"]
    #[inline(always)]
    pub fn dcin15(&mut self) -> DCIN15_W {
        DCIN15_W { w: self }
    }
    #[doc = "Bit 24 - DCMP 16 masked interrupt status / Write 1 to clear (also clear raw status)"]
    #[inline(always)]
    pub fn dcin16(&mut self) -> DCIN16_W {
        DCIN16_W { w: self }
    }
    #[doc = "Bit 25 - DCMP 17 masked interrupt status / Write 1 to clear (also clear raw status)"]
    #[inline(always)]
    pub fn dcin17(&mut self) -> DCIN17_W {
        DCIN17_W { w: self }
    }
    #[doc = "Bit 26 - DCMP 18 masked interrupt status / Write 1 to clear (also clear raw status)"]
    #[inline(always)]
    pub fn dcin18(&mut self) -> DCIN18_W {
        DCIN18_W { w: self }
    }
    #[doc = "Bit 27 - DCMP 19 masked interrupt status / Write 1 to clear (also clear raw status)"]
    #[inline(always)]
    pub fn dcin19(&mut self) -> DCIN19_W {
        DCIN19_W { w: self }
    }
    #[doc = "Bit 28 - DCMP 20 masked interrupt status / Write 1 to clear (also clear raw status)"]
    #[inline(always)]
    pub fn dcin20(&mut self) -> DCIN20_W {
        DCIN20_W { w: self }
    }
    #[doc = "Bit 29 - DCMP 21 masked interrupt status / Write 1 to clear (also clear raw status)"]
    #[inline(always)]
    pub fn dcin21(&mut self) -> DCIN21_W {
        DCIN21_W { w: self }
    }
    #[doc = "Bit 30 - DCMP 22 masked interrupt status / Write 1 to clear (also clear raw status)"]
    #[inline(always)]
    pub fn dcin22(&mut self) -> DCIN22_W {
        DCIN22_W { w: self }
    }
    #[doc = "Bit 31 - DCMP 23 masked interrupt status / Write 1 to clear (also clear raw status)"]
    #[inline(always)]
    pub fn dcin23(&mut self) -> DCIN23_W {
        DCIN23_W { w: self }
    }
}
