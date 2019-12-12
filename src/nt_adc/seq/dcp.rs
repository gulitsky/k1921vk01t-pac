#[doc = "Reader of register DCP"]
pub type R = crate::R<u32, super::DCP>;
#[doc = "Writer for register DCP"]
pub type W = crate::W<u32, super::DCP>;
#[doc = "Register DCP `reset()`'s with value 0"]
impl crate::ResetValue for super::DCP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CMP0`"]
pub type CMP0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMP0`"]
pub struct CMP0_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP0_W<'a> {
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
#[doc = "Reader of field `CMP1`"]
pub type CMP1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMP1`"]
pub struct CMP1_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP1_W<'a> {
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
#[doc = "Reader of field `CMP2`"]
pub type CMP2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMP2`"]
pub struct CMP2_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP2_W<'a> {
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
#[doc = "Reader of field `CMP3`"]
pub type CMP3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMP3`"]
pub struct CMP3_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP3_W<'a> {
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
#[doc = "Reader of field `CMP4`"]
pub type CMP4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMP4`"]
pub struct CMP4_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP4_W<'a> {
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
#[doc = "Reader of field `CMP5`"]
pub type CMP5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMP5`"]
pub struct CMP5_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP5_W<'a> {
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
#[doc = "Reader of field `CMP6`"]
pub type CMP6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMP6`"]
pub struct CMP6_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP6_W<'a> {
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
#[doc = "Reader of field `CMP7`"]
pub type CMP7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMP7`"]
pub struct CMP7_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP7_W<'a> {
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
#[doc = "Reader of field `CMP8`"]
pub type CMP8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMP8`"]
pub struct CMP8_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP8_W<'a> {
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
#[doc = "Reader of field `CMP9`"]
pub type CMP9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMP9`"]
pub struct CMP9_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP9_W<'a> {
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
#[doc = "Reader of field `CMP10`"]
pub type CMP10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMP10`"]
pub struct CMP10_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP10_W<'a> {
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
#[doc = "Reader of field `CMP11`"]
pub type CMP11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMP11`"]
pub struct CMP11_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP11_W<'a> {
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
#[doc = "Reader of field `CMP12`"]
pub type CMP12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMP12`"]
pub struct CMP12_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP12_W<'a> {
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
#[doc = "Reader of field `CMP13`"]
pub type CMP13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMP13`"]
pub struct CMP13_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP13_W<'a> {
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
#[doc = "Reader of field `CMP14`"]
pub type CMP14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMP14`"]
pub struct CMP14_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP14_W<'a> {
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
#[doc = "Reader of field `CMP15`"]
pub type CMP15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMP15`"]
pub struct CMP15_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP15_W<'a> {
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
#[doc = "Reader of field `CMP16`"]
pub type CMP16_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMP16`"]
pub struct CMP16_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP16_W<'a> {
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
#[doc = "Reader of field `CMP17`"]
pub type CMP17_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMP17`"]
pub struct CMP17_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP17_W<'a> {
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
#[doc = "Reader of field `CMP18`"]
pub type CMP18_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMP18`"]
pub struct CMP18_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP18_W<'a> {
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
#[doc = "Reader of field `CMP19`"]
pub type CMP19_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMP19`"]
pub struct CMP19_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP19_W<'a> {
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
#[doc = "Reader of field `CMP20`"]
pub type CMP20_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMP20`"]
pub struct CMP20_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP20_W<'a> {
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
#[doc = "Reader of field `CMP21`"]
pub type CMP21_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMP21`"]
pub struct CMP21_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP21_W<'a> {
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
#[doc = "Reader of field `CMP22`"]
pub type CMP22_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMP22`"]
pub struct CMP22_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP22_W<'a> {
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
#[doc = "Reader of field `CMP23`"]
pub type CMP23_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMP23`"]
pub struct CMP23_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP23_W<'a> {
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
    #[doc = "Bit 0 - Enable DCMP 0"]
    #[inline(always)]
    pub fn cmp0(&self) -> CMP0_R {
        CMP0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable DCMP 1"]
    #[inline(always)]
    pub fn cmp1(&self) -> CMP1_R {
        CMP1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable DCMP 2"]
    #[inline(always)]
    pub fn cmp2(&self) -> CMP2_R {
        CMP2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable DCMP 3"]
    #[inline(always)]
    pub fn cmp3(&self) -> CMP3_R {
        CMP3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable DCMP 4"]
    #[inline(always)]
    pub fn cmp4(&self) -> CMP4_R {
        CMP4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enable DCMP 5"]
    #[inline(always)]
    pub fn cmp5(&self) -> CMP5_R {
        CMP5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enable DCMP 6"]
    #[inline(always)]
    pub fn cmp6(&self) -> CMP6_R {
        CMP6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enable DCMP 7"]
    #[inline(always)]
    pub fn cmp7(&self) -> CMP7_R {
        CMP7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enable DCMP 8"]
    #[inline(always)]
    pub fn cmp8(&self) -> CMP8_R {
        CMP8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Enable DCMP 9"]
    #[inline(always)]
    pub fn cmp9(&self) -> CMP9_R {
        CMP9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Enable DCMP 10"]
    #[inline(always)]
    pub fn cmp10(&self) -> CMP10_R {
        CMP10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Enable DCMP 11"]
    #[inline(always)]
    pub fn cmp11(&self) -> CMP11_R {
        CMP11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Enable DCMP 12"]
    #[inline(always)]
    pub fn cmp12(&self) -> CMP12_R {
        CMP12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Enable DCMP 13"]
    #[inline(always)]
    pub fn cmp13(&self) -> CMP13_R {
        CMP13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Enable DCMP 14"]
    #[inline(always)]
    pub fn cmp14(&self) -> CMP14_R {
        CMP14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Enable DCMP 15"]
    #[inline(always)]
    pub fn cmp15(&self) -> CMP15_R {
        CMP15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Enable DCMP 16"]
    #[inline(always)]
    pub fn cmp16(&self) -> CMP16_R {
        CMP16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Enable DCMP 17"]
    #[inline(always)]
    pub fn cmp17(&self) -> CMP17_R {
        CMP17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Enable DCMP 18"]
    #[inline(always)]
    pub fn cmp18(&self) -> CMP18_R {
        CMP18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Enable DCMP 19"]
    #[inline(always)]
    pub fn cmp19(&self) -> CMP19_R {
        CMP19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Enable DCMP 20"]
    #[inline(always)]
    pub fn cmp20(&self) -> CMP20_R {
        CMP20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Enable DCMP 21"]
    #[inline(always)]
    pub fn cmp21(&self) -> CMP21_R {
        CMP21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Enable DCMP 22"]
    #[inline(always)]
    pub fn cmp22(&self) -> CMP22_R {
        CMP22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Enable DCMP 23"]
    #[inline(always)]
    pub fn cmp23(&self) -> CMP23_R {
        CMP23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable DCMP 0"]
    #[inline(always)]
    pub fn cmp0(&mut self) -> CMP0_W {
        CMP0_W { w: self }
    }
    #[doc = "Bit 1 - Enable DCMP 1"]
    #[inline(always)]
    pub fn cmp1(&mut self) -> CMP1_W {
        CMP1_W { w: self }
    }
    #[doc = "Bit 2 - Enable DCMP 2"]
    #[inline(always)]
    pub fn cmp2(&mut self) -> CMP2_W {
        CMP2_W { w: self }
    }
    #[doc = "Bit 3 - Enable DCMP 3"]
    #[inline(always)]
    pub fn cmp3(&mut self) -> CMP3_W {
        CMP3_W { w: self }
    }
    #[doc = "Bit 4 - Enable DCMP 4"]
    #[inline(always)]
    pub fn cmp4(&mut self) -> CMP4_W {
        CMP4_W { w: self }
    }
    #[doc = "Bit 5 - Enable DCMP 5"]
    #[inline(always)]
    pub fn cmp5(&mut self) -> CMP5_W {
        CMP5_W { w: self }
    }
    #[doc = "Bit 6 - Enable DCMP 6"]
    #[inline(always)]
    pub fn cmp6(&mut self) -> CMP6_W {
        CMP6_W { w: self }
    }
    #[doc = "Bit 7 - Enable DCMP 7"]
    #[inline(always)]
    pub fn cmp7(&mut self) -> CMP7_W {
        CMP7_W { w: self }
    }
    #[doc = "Bit 8 - Enable DCMP 8"]
    #[inline(always)]
    pub fn cmp8(&mut self) -> CMP8_W {
        CMP8_W { w: self }
    }
    #[doc = "Bit 9 - Enable DCMP 9"]
    #[inline(always)]
    pub fn cmp9(&mut self) -> CMP9_W {
        CMP9_W { w: self }
    }
    #[doc = "Bit 10 - Enable DCMP 10"]
    #[inline(always)]
    pub fn cmp10(&mut self) -> CMP10_W {
        CMP10_W { w: self }
    }
    #[doc = "Bit 11 - Enable DCMP 11"]
    #[inline(always)]
    pub fn cmp11(&mut self) -> CMP11_W {
        CMP11_W { w: self }
    }
    #[doc = "Bit 12 - Enable DCMP 12"]
    #[inline(always)]
    pub fn cmp12(&mut self) -> CMP12_W {
        CMP12_W { w: self }
    }
    #[doc = "Bit 13 - Enable DCMP 13"]
    #[inline(always)]
    pub fn cmp13(&mut self) -> CMP13_W {
        CMP13_W { w: self }
    }
    #[doc = "Bit 14 - Enable DCMP 14"]
    #[inline(always)]
    pub fn cmp14(&mut self) -> CMP14_W {
        CMP14_W { w: self }
    }
    #[doc = "Bit 15 - Enable DCMP 15"]
    #[inline(always)]
    pub fn cmp15(&mut self) -> CMP15_W {
        CMP15_W { w: self }
    }
    #[doc = "Bit 16 - Enable DCMP 16"]
    #[inline(always)]
    pub fn cmp16(&mut self) -> CMP16_W {
        CMP16_W { w: self }
    }
    #[doc = "Bit 17 - Enable DCMP 17"]
    #[inline(always)]
    pub fn cmp17(&mut self) -> CMP17_W {
        CMP17_W { w: self }
    }
    #[doc = "Bit 18 - Enable DCMP 18"]
    #[inline(always)]
    pub fn cmp18(&mut self) -> CMP18_W {
        CMP18_W { w: self }
    }
    #[doc = "Bit 19 - Enable DCMP 19"]
    #[inline(always)]
    pub fn cmp19(&mut self) -> CMP19_W {
        CMP19_W { w: self }
    }
    #[doc = "Bit 20 - Enable DCMP 20"]
    #[inline(always)]
    pub fn cmp20(&mut self) -> CMP20_W {
        CMP20_W { w: self }
    }
    #[doc = "Bit 21 - Enable DCMP 21"]
    #[inline(always)]
    pub fn cmp21(&mut self) -> CMP21_W {
        CMP21_W { w: self }
    }
    #[doc = "Bit 22 - Enable DCMP 22"]
    #[inline(always)]
    pub fn cmp22(&mut self) -> CMP22_W {
        CMP22_W { w: self }
    }
    #[doc = "Bit 23 - Enable DCMP 23"]
    #[inline(always)]
    pub fn cmp23(&mut self) -> CMP23_W {
        CMP23_W { w: self }
    }
}
