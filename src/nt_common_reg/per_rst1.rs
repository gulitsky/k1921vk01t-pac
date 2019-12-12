#[doc = "Reader of register PER_RST1"]
pub type R = crate::R<u32, super::PER_RST1>;
#[doc = "Writer for register PER_RST1"]
pub type W = crate::W<u32, super::PER_RST1>;
#[doc = "Register PER_RST1 `reset()`'s with value 0"]
impl crate::ResetValue for super::PER_RST1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `QEPRST0`"]
pub type QEPRST0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `QEPRST0`"]
pub struct QEPRST0_W<'a> {
    w: &'a mut W,
}
impl<'a> QEPRST0_W<'a> {
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
#[doc = "Reader of field `QEPRST1`"]
pub type QEPRST1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `QEPRST1`"]
pub struct QEPRST1_W<'a> {
    w: &'a mut W,
}
impl<'a> QEPRST1_W<'a> {
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
#[doc = "Reader of field `PWMRST0`"]
pub type PWMRST0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWMRST0`"]
pub struct PWMRST0_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMRST0_W<'a> {
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
#[doc = "Reader of field `PWMRST1`"]
pub type PWMRST1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWMRST1`"]
pub struct PWMRST1_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMRST1_W<'a> {
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
#[doc = "Reader of field `PWMRST2`"]
pub type PWMRST2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWMRST2`"]
pub struct PWMRST2_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMRST2_W<'a> {
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
#[doc = "Reader of field `PWMRST3`"]
pub type PWMRST3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWMRST3`"]
pub struct PWMRST3_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMRST3_W<'a> {
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
#[doc = "Reader of field `PWMRST4`"]
pub type PWMRST4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWMRST4`"]
pub struct PWMRST4_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMRST4_W<'a> {
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
#[doc = "Reader of field `PWMRST5`"]
pub type PWMRST5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWMRST5`"]
pub struct PWMRST5_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMRST5_W<'a> {
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
#[doc = "Reader of field `PWMRST6`"]
pub type PWMRST6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWMRST6`"]
pub struct PWMRST6_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMRST6_W<'a> {
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
#[doc = "Reader of field `PWMRST7`"]
pub type PWMRST7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWMRST7`"]
pub struct PWMRST7_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMRST7_W<'a> {
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
#[doc = "Reader of field `PWMRST8`"]
pub type PWMRST8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWMRST8`"]
pub struct PWMRST8_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMRST8_W<'a> {
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
#[doc = "Reader of field `CAPRST0`"]
pub type CAPRST0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAPRST0`"]
pub struct CAPRST0_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPRST0_W<'a> {
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
#[doc = "Reader of field `CAPRST1`"]
pub type CAPRST1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAPRST1`"]
pub struct CAPRST1_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPRST1_W<'a> {
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
#[doc = "Reader of field `CAPRST2`"]
pub type CAPRST2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAPRST2`"]
pub struct CAPRST2_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPRST2_W<'a> {
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
#[doc = "Reader of field `CAPRST3`"]
pub type CAPRST3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAPRST3`"]
pub struct CAPRST3_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPRST3_W<'a> {
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
#[doc = "Reader of field `CAPRST4`"]
pub type CAPRST4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAPRST4`"]
pub struct CAPRST4_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPRST4_W<'a> {
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
#[doc = "Reader of field `CAPRST5`"]
pub type CAPRST5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAPRST5`"]
pub struct CAPRST5_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPRST5_W<'a> {
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
#[doc = "Reader of field `ECMPRST`"]
pub type ECMPRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ECMPRST`"]
pub struct ECMPRST_W<'a> {
    w: &'a mut W,
}
impl<'a> ECMPRST_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Reset QEPR0"]
    #[inline(always)]
    pub fn qeprst0(&self) -> QEPRST0_R {
        QEPRST0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Reset QEPR0"]
    #[inline(always)]
    pub fn qeprst1(&self) -> QEPRST1_R {
        QEPRST1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Reset module PWM0"]
    #[inline(always)]
    pub fn pwmrst0(&self) -> PWMRST0_R {
        PWMRST0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Reset module PWM1"]
    #[inline(always)]
    pub fn pwmrst1(&self) -> PWMRST1_R {
        PWMRST1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Reset module PWM2"]
    #[inline(always)]
    pub fn pwmrst2(&self) -> PWMRST2_R {
        PWMRST2_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Reset module PWM3"]
    #[inline(always)]
    pub fn pwmrst3(&self) -> PWMRST3_R {
        PWMRST3_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Reset module PWM4"]
    #[inline(always)]
    pub fn pwmrst4(&self) -> PWMRST4_R {
        PWMRST4_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Reset module PWM5"]
    #[inline(always)]
    pub fn pwmrst5(&self) -> PWMRST5_R {
        PWMRST5_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Reset module PWM6"]
    #[inline(always)]
    pub fn pwmrst6(&self) -> PWMRST6_R {
        PWMRST6_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Reset module PWM7"]
    #[inline(always)]
    pub fn pwmrst7(&self) -> PWMRST7_R {
        PWMRST7_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Reset module PWM8"]
    #[inline(always)]
    pub fn pwmrst8(&self) -> PWMRST8_R {
        PWMRST8_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Reset module CAP0"]
    #[inline(always)]
    pub fn caprst0(&self) -> CAPRST0_R {
        CAPRST0_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Reset module CAP1"]
    #[inline(always)]
    pub fn caprst1(&self) -> CAPRST1_R {
        CAPRST1_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Reset module CAP2"]
    #[inline(always)]
    pub fn caprst2(&self) -> CAPRST2_R {
        CAPRST2_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Reset module CAP3"]
    #[inline(always)]
    pub fn caprst3(&self) -> CAPRST3_R {
        CAPRST3_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Reset module CAP4"]
    #[inline(always)]
    pub fn caprst4(&self) -> CAPRST4_R {
        CAPRST4_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Reset module CAP5"]
    #[inline(always)]
    pub fn caprst5(&self) -> CAPRST5_R {
        CAPRST5_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Reset of analog comparator"]
    #[inline(always)]
    pub fn ecmprst(&self) -> ECMPRST_R {
        ECMPRST_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reset QEPR0"]
    #[inline(always)]
    pub fn qeprst0(&mut self) -> QEPRST0_W {
        QEPRST0_W { w: self }
    }
    #[doc = "Bit 1 - Reset QEPR0"]
    #[inline(always)]
    pub fn qeprst1(&mut self) -> QEPRST1_W {
        QEPRST1_W { w: self }
    }
    #[doc = "Bit 2 - Reset module PWM0"]
    #[inline(always)]
    pub fn pwmrst0(&mut self) -> PWMRST0_W {
        PWMRST0_W { w: self }
    }
    #[doc = "Bit 3 - Reset module PWM1"]
    #[inline(always)]
    pub fn pwmrst1(&mut self) -> PWMRST1_W {
        PWMRST1_W { w: self }
    }
    #[doc = "Bit 4 - Reset module PWM2"]
    #[inline(always)]
    pub fn pwmrst2(&mut self) -> PWMRST2_W {
        PWMRST2_W { w: self }
    }
    #[doc = "Bit 5 - Reset module PWM3"]
    #[inline(always)]
    pub fn pwmrst3(&mut self) -> PWMRST3_W {
        PWMRST3_W { w: self }
    }
    #[doc = "Bit 6 - Reset module PWM4"]
    #[inline(always)]
    pub fn pwmrst4(&mut self) -> PWMRST4_W {
        PWMRST4_W { w: self }
    }
    #[doc = "Bit 7 - Reset module PWM5"]
    #[inline(always)]
    pub fn pwmrst5(&mut self) -> PWMRST5_W {
        PWMRST5_W { w: self }
    }
    #[doc = "Bit 8 - Reset module PWM6"]
    #[inline(always)]
    pub fn pwmrst6(&mut self) -> PWMRST6_W {
        PWMRST6_W { w: self }
    }
    #[doc = "Bit 9 - Reset module PWM7"]
    #[inline(always)]
    pub fn pwmrst7(&mut self) -> PWMRST7_W {
        PWMRST7_W { w: self }
    }
    #[doc = "Bit 10 - Reset module PWM8"]
    #[inline(always)]
    pub fn pwmrst8(&mut self) -> PWMRST8_W {
        PWMRST8_W { w: self }
    }
    #[doc = "Bit 11 - Reset module CAP0"]
    #[inline(always)]
    pub fn caprst0(&mut self) -> CAPRST0_W {
        CAPRST0_W { w: self }
    }
    #[doc = "Bit 12 - Reset module CAP1"]
    #[inline(always)]
    pub fn caprst1(&mut self) -> CAPRST1_W {
        CAPRST1_W { w: self }
    }
    #[doc = "Bit 13 - Reset module CAP2"]
    #[inline(always)]
    pub fn caprst2(&mut self) -> CAPRST2_W {
        CAPRST2_W { w: self }
    }
    #[doc = "Bit 14 - Reset module CAP3"]
    #[inline(always)]
    pub fn caprst3(&mut self) -> CAPRST3_W {
        CAPRST3_W { w: self }
    }
    #[doc = "Bit 15 - Reset module CAP4"]
    #[inline(always)]
    pub fn caprst4(&mut self) -> CAPRST4_W {
        CAPRST4_W { w: self }
    }
    #[doc = "Bit 16 - Reset module CAP5"]
    #[inline(always)]
    pub fn caprst5(&mut self) -> CAPRST5_W {
        CAPRST5_W { w: self }
    }
    #[doc = "Bit 17 - Reset of analog comparator"]
    #[inline(always)]
    pub fn ecmprst(&mut self) -> ECMPRST_W {
        ECMPRST_W { w: self }
    }
}
