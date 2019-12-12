#[doc = "Reader of register APB_CLK"]
pub type R = crate::R<u32, super::APB_CLK>;
#[doc = "Writer for register APB_CLK"]
pub type W = crate::W<u32, super::APB_CLK>;
#[doc = "Register APB_CLK `reset()`'s with value 0"]
impl crate::ResetValue for super::APB_CLK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `QEPEN0`"]
pub type QEPEN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `QEPEN0`"]
pub struct QEPEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> QEPEN0_W<'a> {
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
#[doc = "Reader of field `QEPEN1`"]
pub type QEPEN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `QEPEN1`"]
pub struct QEPEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> QEPEN1_W<'a> {
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
#[doc = "Reader of field `CMPEN`"]
pub type CMPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMPEN`"]
pub struct CMPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPEN_W<'a> {
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
#[doc = "Reader of field `PWMEN0`"]
pub type PWMEN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWMEN0`"]
pub struct PWMEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMEN0_W<'a> {
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
#[doc = "Reader of field `PWMEN1`"]
pub type PWMEN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWMEN1`"]
pub struct PWMEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMEN1_W<'a> {
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
#[doc = "Reader of field `PWMEN2`"]
pub type PWMEN2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWMEN2`"]
pub struct PWMEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMEN2_W<'a> {
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
#[doc = "Reader of field `PWMEN3`"]
pub type PWMEN3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWMEN3`"]
pub struct PWMEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMEN3_W<'a> {
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
#[doc = "Reader of field `PWMEN4`"]
pub type PWMEN4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWMEN4`"]
pub struct PWMEN4_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMEN4_W<'a> {
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
#[doc = "Reader of field `PWMEN5`"]
pub type PWMEN5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWMEN5`"]
pub struct PWMEN5_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMEN5_W<'a> {
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
#[doc = "Reader of field `PWMEN6`"]
pub type PWMEN6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWMEN6`"]
pub struct PWMEN6_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMEN6_W<'a> {
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
#[doc = "Reader of field `PWMEN7`"]
pub type PWMEN7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWMEN7`"]
pub struct PWMEN7_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMEN7_W<'a> {
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
#[doc = "Reader of field `PWMEN8`"]
pub type PWMEN8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWMEN8`"]
pub struct PWMEN8_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMEN8_W<'a> {
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
#[doc = "Reader of field `WDEN`"]
pub type WDEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WDEN`"]
pub struct WDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WDEN_W<'a> {
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
#[doc = "Reader of field `I2CEN0`"]
pub type I2CEN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2CEN0`"]
pub struct I2CEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> I2CEN0_W<'a> {
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
#[doc = "Reader of field `I2CEN1`"]
pub type I2CEN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2CEN1`"]
pub struct I2CEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> I2CEN1_W<'a> {
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
#[doc = "Reader of field `ADCEN`"]
pub type ADCEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADCEN`"]
pub struct ADCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCEN_W<'a> {
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
impl R {
    #[doc = "Bit 1 - QEP0 clk enable"]
    #[inline(always)]
    pub fn qepen0(&self) -> QEPEN0_R {
        QEPEN0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - QEP1 clk enable"]
    #[inline(always)]
    pub fn qepen1(&self) -> QEPEN1_R {
        QEPEN1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Analog comparator clk enable"]
    #[inline(always)]
    pub fn cmpen(&self) -> CMPEN_R {
        CMPEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - PWM0 clk enable"]
    #[inline(always)]
    pub fn pwmen0(&self) -> PWMEN0_R {
        PWMEN0_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - PWM1 clk enable"]
    #[inline(always)]
    pub fn pwmen1(&self) -> PWMEN1_R {
        PWMEN1_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - PWM2 clk enable"]
    #[inline(always)]
    pub fn pwmen2(&self) -> PWMEN2_R {
        PWMEN2_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - PWM3 clk enable"]
    #[inline(always)]
    pub fn pwmen3(&self) -> PWMEN3_R {
        PWMEN3_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - PWM4 clk enable"]
    #[inline(always)]
    pub fn pwmen4(&self) -> PWMEN4_R {
        PWMEN4_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - PWM5 clk enable"]
    #[inline(always)]
    pub fn pwmen5(&self) -> PWMEN5_R {
        PWMEN5_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - PWM6 clk enable"]
    #[inline(always)]
    pub fn pwmen6(&self) -> PWMEN6_R {
        PWMEN6_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - PWM7 clk enable"]
    #[inline(always)]
    pub fn pwmen7(&self) -> PWMEN7_R {
        PWMEN7_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - PWM8 clk enable"]
    #[inline(always)]
    pub fn pwmen8(&self) -> PWMEN8_R {
        PWMEN8_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - WDT clk enable"]
    #[inline(always)]
    pub fn wden(&self) -> WDEN_R {
        WDEN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - I2C0 clk enable"]
    #[inline(always)]
    pub fn i2cen0(&self) -> I2CEN0_R {
        I2CEN0_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - I2C1 clk enable"]
    #[inline(always)]
    pub fn i2cen1(&self) -> I2CEN1_R {
        I2CEN1_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 24 - ADC clk enable"]
    #[inline(always)]
    pub fn adcen(&self) -> ADCEN_R {
        ADCEN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - QEP0 clk enable"]
    #[inline(always)]
    pub fn qepen0(&mut self) -> QEPEN0_W {
        QEPEN0_W { w: self }
    }
    #[doc = "Bit 2 - QEP1 clk enable"]
    #[inline(always)]
    pub fn qepen1(&mut self) -> QEPEN1_W {
        QEPEN1_W { w: self }
    }
    #[doc = "Bit 9 - Analog comparator clk enable"]
    #[inline(always)]
    pub fn cmpen(&mut self) -> CMPEN_W {
        CMPEN_W { w: self }
    }
    #[doc = "Bit 10 - PWM0 clk enable"]
    #[inline(always)]
    pub fn pwmen0(&mut self) -> PWMEN0_W {
        PWMEN0_W { w: self }
    }
    #[doc = "Bit 11 - PWM1 clk enable"]
    #[inline(always)]
    pub fn pwmen1(&mut self) -> PWMEN1_W {
        PWMEN1_W { w: self }
    }
    #[doc = "Bit 12 - PWM2 clk enable"]
    #[inline(always)]
    pub fn pwmen2(&mut self) -> PWMEN2_W {
        PWMEN2_W { w: self }
    }
    #[doc = "Bit 13 - PWM3 clk enable"]
    #[inline(always)]
    pub fn pwmen3(&mut self) -> PWMEN3_W {
        PWMEN3_W { w: self }
    }
    #[doc = "Bit 14 - PWM4 clk enable"]
    #[inline(always)]
    pub fn pwmen4(&mut self) -> PWMEN4_W {
        PWMEN4_W { w: self }
    }
    #[doc = "Bit 15 - PWM5 clk enable"]
    #[inline(always)]
    pub fn pwmen5(&mut self) -> PWMEN5_W {
        PWMEN5_W { w: self }
    }
    #[doc = "Bit 16 - PWM6 clk enable"]
    #[inline(always)]
    pub fn pwmen6(&mut self) -> PWMEN6_W {
        PWMEN6_W { w: self }
    }
    #[doc = "Bit 17 - PWM7 clk enable"]
    #[inline(always)]
    pub fn pwmen7(&mut self) -> PWMEN7_W {
        PWMEN7_W { w: self }
    }
    #[doc = "Bit 18 - PWM8 clk enable"]
    #[inline(always)]
    pub fn pwmen8(&mut self) -> PWMEN8_W {
        PWMEN8_W { w: self }
    }
    #[doc = "Bit 19 - WDT clk enable"]
    #[inline(always)]
    pub fn wden(&mut self) -> WDEN_W {
        WDEN_W { w: self }
    }
    #[doc = "Bit 20 - I2C0 clk enable"]
    #[inline(always)]
    pub fn i2cen0(&mut self) -> I2CEN0_W {
        I2CEN0_W { w: self }
    }
    #[doc = "Bit 21 - I2C1 clk enable"]
    #[inline(always)]
    pub fn i2cen1(&mut self) -> I2CEN1_W {
        I2CEN1_W { w: self }
    }
    #[doc = "Bit 24 - ADC clk enable"]
    #[inline(always)]
    pub fn adcen(&mut self) -> ADCEN_W {
        ADCEN_W { w: self }
    }
}
