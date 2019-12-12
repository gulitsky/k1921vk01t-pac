#[doc = "Writer for register DCRTC"]
pub type W = crate::W<u32, super::DCRTC>;
#[doc = "Register DCRTC `reset()`'s with value 0"]
impl crate::ResetValue for super::DCRTC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `DCTRIG0`"]
pub struct DCTRIG0_W<'a> {
    w: &'a mut W,
}
impl<'a> DCTRIG0_W<'a> {
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
#[doc = "Write proxy for field `DCTRIG1`"]
pub struct DCTRIG1_W<'a> {
    w: &'a mut W,
}
impl<'a> DCTRIG1_W<'a> {
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
#[doc = "Write proxy for field `DCTRIG2`"]
pub struct DCTRIG2_W<'a> {
    w: &'a mut W,
}
impl<'a> DCTRIG2_W<'a> {
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
#[doc = "Write proxy for field `DCTRIG3`"]
pub struct DCTRIG3_W<'a> {
    w: &'a mut W,
}
impl<'a> DCTRIG3_W<'a> {
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
#[doc = "Write proxy for field `DCTRIG4`"]
pub struct DCTRIG4_W<'a> {
    w: &'a mut W,
}
impl<'a> DCTRIG4_W<'a> {
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
#[doc = "Write proxy for field `DCTRIG5`"]
pub struct DCTRIG5_W<'a> {
    w: &'a mut W,
}
impl<'a> DCTRIG5_W<'a> {
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
#[doc = "Write proxy for field `DCTRIG6`"]
pub struct DCTRIG6_W<'a> {
    w: &'a mut W,
}
impl<'a> DCTRIG6_W<'a> {
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
#[doc = "Write proxy for field `DCTRIG7`"]
pub struct DCTRIG7_W<'a> {
    w: &'a mut W,
}
impl<'a> DCTRIG7_W<'a> {
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
#[doc = "Write proxy for field `DCTRIG8`"]
pub struct DCTRIG8_W<'a> {
    w: &'a mut W,
}
impl<'a> DCTRIG8_W<'a> {
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
#[doc = "Write proxy for field `DCTRIG9`"]
pub struct DCTRIG9_W<'a> {
    w: &'a mut W,
}
impl<'a> DCTRIG9_W<'a> {
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
#[doc = "Write proxy for field `DCTRIG10`"]
pub struct DCTRIG10_W<'a> {
    w: &'a mut W,
}
impl<'a> DCTRIG10_W<'a> {
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
#[doc = "Write proxy for field `DCTRIG11`"]
pub struct DCTRIG11_W<'a> {
    w: &'a mut W,
}
impl<'a> DCTRIG11_W<'a> {
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
#[doc = "Write proxy for field `DCTRIG12`"]
pub struct DCTRIG12_W<'a> {
    w: &'a mut W,
}
impl<'a> DCTRIG12_W<'a> {
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
#[doc = "Write proxy for field `DCTRIG13`"]
pub struct DCTRIG13_W<'a> {
    w: &'a mut W,
}
impl<'a> DCTRIG13_W<'a> {
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
#[doc = "Write proxy for field `DCTRIG14`"]
pub struct DCTRIG14_W<'a> {
    w: &'a mut W,
}
impl<'a> DCTRIG14_W<'a> {
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
#[doc = "Write proxy for field `DCTRIG15`"]
pub struct DCTRIG15_W<'a> {
    w: &'a mut W,
}
impl<'a> DCTRIG15_W<'a> {
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
#[doc = "Write proxy for field `DCTRIG16`"]
pub struct DCTRIG16_W<'a> {
    w: &'a mut W,
}
impl<'a> DCTRIG16_W<'a> {
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
#[doc = "Write proxy for field `DCTRIG17`"]
pub struct DCTRIG17_W<'a> {
    w: &'a mut W,
}
impl<'a> DCTRIG17_W<'a> {
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
#[doc = "Write proxy for field `DCTRIG18`"]
pub struct DCTRIG18_W<'a> {
    w: &'a mut W,
}
impl<'a> DCTRIG18_W<'a> {
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
#[doc = "Write proxy for field `DCTRIG19`"]
pub struct DCTRIG19_W<'a> {
    w: &'a mut W,
}
impl<'a> DCTRIG19_W<'a> {
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
#[doc = "Write proxy for field `DCTRIG20`"]
pub struct DCTRIG20_W<'a> {
    w: &'a mut W,
}
impl<'a> DCTRIG20_W<'a> {
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
#[doc = "Write proxy for field `DCTRIG21`"]
pub struct DCTRIG21_W<'a> {
    w: &'a mut W,
}
impl<'a> DCTRIG21_W<'a> {
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
#[doc = "Write proxy for field `DCTRIG22`"]
pub struct DCTRIG22_W<'a> {
    w: &'a mut W,
}
impl<'a> DCTRIG22_W<'a> {
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
#[doc = "Write proxy for field `DCTRIG23`"]
pub struct DCTRIG23_W<'a> {
    w: &'a mut W,
}
impl<'a> DCTRIG23_W<'a> {
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
impl W {
    #[doc = "Bit 0 - DCMP 0 output trigger status"]
    #[inline(always)]
    pub fn dctrig0(&mut self) -> DCTRIG0_W {
        DCTRIG0_W { w: self }
    }
    #[doc = "Bit 1 - DCMP 1 output trigger status"]
    #[inline(always)]
    pub fn dctrig1(&mut self) -> DCTRIG1_W {
        DCTRIG1_W { w: self }
    }
    #[doc = "Bit 2 - DCMP 2 output trigger status"]
    #[inline(always)]
    pub fn dctrig2(&mut self) -> DCTRIG2_W {
        DCTRIG2_W { w: self }
    }
    #[doc = "Bit 3 - DCMP 3 output trigger status"]
    #[inline(always)]
    pub fn dctrig3(&mut self) -> DCTRIG3_W {
        DCTRIG3_W { w: self }
    }
    #[doc = "Bit 4 - DCMP 4 output trigger status"]
    #[inline(always)]
    pub fn dctrig4(&mut self) -> DCTRIG4_W {
        DCTRIG4_W { w: self }
    }
    #[doc = "Bit 5 - DCMP 5 output trigger status"]
    #[inline(always)]
    pub fn dctrig5(&mut self) -> DCTRIG5_W {
        DCTRIG5_W { w: self }
    }
    #[doc = "Bit 6 - DCMP 6 output trigger status"]
    #[inline(always)]
    pub fn dctrig6(&mut self) -> DCTRIG6_W {
        DCTRIG6_W { w: self }
    }
    #[doc = "Bit 7 - DCMP 7 output trigger status"]
    #[inline(always)]
    pub fn dctrig7(&mut self) -> DCTRIG7_W {
        DCTRIG7_W { w: self }
    }
    #[doc = "Bit 8 - DCMP 8 output trigger status"]
    #[inline(always)]
    pub fn dctrig8(&mut self) -> DCTRIG8_W {
        DCTRIG8_W { w: self }
    }
    #[doc = "Bit 9 - DCMP 9 output trigger status"]
    #[inline(always)]
    pub fn dctrig9(&mut self) -> DCTRIG9_W {
        DCTRIG9_W { w: self }
    }
    #[doc = "Bit 10 - DCMP 10 output trigger status"]
    #[inline(always)]
    pub fn dctrig10(&mut self) -> DCTRIG10_W {
        DCTRIG10_W { w: self }
    }
    #[doc = "Bit 11 - DCMP 11 output trigger status"]
    #[inline(always)]
    pub fn dctrig11(&mut self) -> DCTRIG11_W {
        DCTRIG11_W { w: self }
    }
    #[doc = "Bit 12 - DCMP 12 output trigger status"]
    #[inline(always)]
    pub fn dctrig12(&mut self) -> DCTRIG12_W {
        DCTRIG12_W { w: self }
    }
    #[doc = "Bit 13 - DCMP 13 output trigger status"]
    #[inline(always)]
    pub fn dctrig13(&mut self) -> DCTRIG13_W {
        DCTRIG13_W { w: self }
    }
    #[doc = "Bit 14 - DCMP 14 output trigger status"]
    #[inline(always)]
    pub fn dctrig14(&mut self) -> DCTRIG14_W {
        DCTRIG14_W { w: self }
    }
    #[doc = "Bit 15 - DCMP 15 output trigger status"]
    #[inline(always)]
    pub fn dctrig15(&mut self) -> DCTRIG15_W {
        DCTRIG15_W { w: self }
    }
    #[doc = "Bit 16 - DCMP 16 output trigger status"]
    #[inline(always)]
    pub fn dctrig16(&mut self) -> DCTRIG16_W {
        DCTRIG16_W { w: self }
    }
    #[doc = "Bit 17 - DCMP 17 output trigger status"]
    #[inline(always)]
    pub fn dctrig17(&mut self) -> DCTRIG17_W {
        DCTRIG17_W { w: self }
    }
    #[doc = "Bit 18 - DCMP 18 output trigger status"]
    #[inline(always)]
    pub fn dctrig18(&mut self) -> DCTRIG18_W {
        DCTRIG18_W { w: self }
    }
    #[doc = "Bit 19 - DCMP 19 output trigger status"]
    #[inline(always)]
    pub fn dctrig19(&mut self) -> DCTRIG19_W {
        DCTRIG19_W { w: self }
    }
    #[doc = "Bit 20 - DCMP 20 output trigger status"]
    #[inline(always)]
    pub fn dctrig20(&mut self) -> DCTRIG20_W {
        DCTRIG20_W { w: self }
    }
    #[doc = "Bit 21 - DCMP 21 output trigger status"]
    #[inline(always)]
    pub fn dctrig21(&mut self) -> DCTRIG21_W {
        DCTRIG21_W { w: self }
    }
    #[doc = "Bit 22 - DCMP 22 output trigger status"]
    #[inline(always)]
    pub fn dctrig22(&mut self) -> DCTRIG22_W {
        DCTRIG22_W { w: self }
    }
    #[doc = "Bit 23 - DCMP 23 output trigger status"]
    #[inline(always)]
    pub fn dctrig23(&mut self) -> DCTRIG23_W {
        DCTRIG23_W { w: self }
    }
}
