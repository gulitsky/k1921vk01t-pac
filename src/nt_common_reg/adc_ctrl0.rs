#[doc = "Reader of register ADC_CTRL0"]
pub type R = crate::R<u32, super::ADC_CTRL0>;
#[doc = "Writer for register ADC_CTRL0"]
pub type W = crate::W<u32, super::ADC_CTRL0>;
#[doc = "Register ADC_CTRL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC_CTRL0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CLKEN_ADC0`"]
pub type CLKEN_ADC0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLKEN_ADC0`"]
pub struct CLKEN_ADC0_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKEN_ADC0_W<'a> {
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
#[doc = "Reader of field `DIVEN_ADC0`"]
pub type DIVEN_ADC0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIVEN_ADC0`"]
pub struct DIVEN_ADC0_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVEN_ADC0_W<'a> {
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
#[doc = "Reader of field `DIV_ADC0`"]
pub type DIV_ADC0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DIV_ADC0`"]
pub struct DIV_ADC0_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_ADC0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 2)) | (((value as u32) & 0x3f) << 2);
        self.w
    }
}
#[doc = "Reader of field `CLKEN_ADC1`"]
pub type CLKEN_ADC1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLKEN_ADC1`"]
pub struct CLKEN_ADC1_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKEN_ADC1_W<'a> {
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
#[doc = "Reader of field `DIVEN_ADC1`"]
pub type DIVEN_ADC1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIVEN_ADC1`"]
pub struct DIVEN_ADC1_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVEN_ADC1_W<'a> {
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
#[doc = "Reader of field `DIV_ADC1`"]
pub type DIV_ADC1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DIV_ADC1`"]
pub struct DIV_ADC1_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_ADC1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 10)) | (((value as u32) & 0x3f) << 10);
        self.w
    }
}
#[doc = "Reader of field `CLKEN_ADC2`"]
pub type CLKEN_ADC2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLKEN_ADC2`"]
pub struct CLKEN_ADC2_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKEN_ADC2_W<'a> {
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
#[doc = "Reader of field `DIVEN_ADC2`"]
pub type DIVEN_ADC2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIVEN_ADC2`"]
pub struct DIVEN_ADC2_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVEN_ADC2_W<'a> {
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
#[doc = "Reader of field `DIV_ADC2`"]
pub type DIV_ADC2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DIV_ADC2`"]
pub struct DIV_ADC2_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_ADC2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 18)) | (((value as u32) & 0x3f) << 18);
        self.w
    }
}
#[doc = "Reader of field `CLKEN_ADC3`"]
pub type CLKEN_ADC3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLKEN_ADC3`"]
pub struct CLKEN_ADC3_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKEN_ADC3_W<'a> {
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
#[doc = "Reader of field `DIVEN_ADC3`"]
pub type DIVEN_ADC3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIVEN_ADC3`"]
pub struct DIVEN_ADC3_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVEN_ADC3_W<'a> {
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
#[doc = "Reader of field `DIV_ADC3`"]
pub type DIV_ADC3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DIV_ADC3`"]
pub struct DIV_ADC3_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_ADC3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 26)) | (((value as u32) & 0x3f) << 26);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - ADC0 clk enable"]
    #[inline(always)]
    pub fn clken_adc0(&self) -> CLKEN_ADC0_R {
        CLKEN_ADC0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ADC0 clk divider enable"]
    #[inline(always)]
    pub fn diven_adc0(&self) -> DIVEN_ADC0_R {
        DIVEN_ADC0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:7 - ADC0 clk divider value"]
    #[inline(always)]
    pub fn div_adc0(&self) -> DIV_ADC0_R {
        DIV_ADC0_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bit 8 - ADC1 clk enable"]
    #[inline(always)]
    pub fn clken_adc1(&self) -> CLKEN_ADC1_R {
        CLKEN_ADC1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - ADC1 clk divider enable"]
    #[inline(always)]
    pub fn diven_adc1(&self) -> DIVEN_ADC1_R {
        DIVEN_ADC1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 10:15 - ADC1 clk divider value"]
    #[inline(always)]
    pub fn div_adc1(&self) -> DIV_ADC1_R {
        DIV_ADC1_R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - ADC2 clk enable"]
    #[inline(always)]
    pub fn clken_adc2(&self) -> CLKEN_ADC2_R {
        CLKEN_ADC2_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - ADC2 clk divider enable"]
    #[inline(always)]
    pub fn diven_adc2(&self) -> DIVEN_ADC2_R {
        DIVEN_ADC2_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bits 18:23 - ADC2 clk divider value"]
    #[inline(always)]
    pub fn div_adc2(&self) -> DIV_ADC2_R {
        DIV_ADC2_R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bit 24 - ADC3 clk enable"]
    #[inline(always)]
    pub fn clken_adc3(&self) -> CLKEN_ADC3_R {
        CLKEN_ADC3_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - ADC3 clk divider enable"]
    #[inline(always)]
    pub fn diven_adc3(&self) -> DIVEN_ADC3_R {
        DIVEN_ADC3_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bits 26:31 - ADC3 clk divider value"]
    #[inline(always)]
    pub fn div_adc3(&self) -> DIV_ADC3_R {
        DIV_ADC3_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - ADC0 clk enable"]
    #[inline(always)]
    pub fn clken_adc0(&mut self) -> CLKEN_ADC0_W {
        CLKEN_ADC0_W { w: self }
    }
    #[doc = "Bit 1 - ADC0 clk divider enable"]
    #[inline(always)]
    pub fn diven_adc0(&mut self) -> DIVEN_ADC0_W {
        DIVEN_ADC0_W { w: self }
    }
    #[doc = "Bits 2:7 - ADC0 clk divider value"]
    #[inline(always)]
    pub fn div_adc0(&mut self) -> DIV_ADC0_W {
        DIV_ADC0_W { w: self }
    }
    #[doc = "Bit 8 - ADC1 clk enable"]
    #[inline(always)]
    pub fn clken_adc1(&mut self) -> CLKEN_ADC1_W {
        CLKEN_ADC1_W { w: self }
    }
    #[doc = "Bit 9 - ADC1 clk divider enable"]
    #[inline(always)]
    pub fn diven_adc1(&mut self) -> DIVEN_ADC1_W {
        DIVEN_ADC1_W { w: self }
    }
    #[doc = "Bits 10:15 - ADC1 clk divider value"]
    #[inline(always)]
    pub fn div_adc1(&mut self) -> DIV_ADC1_W {
        DIV_ADC1_W { w: self }
    }
    #[doc = "Bit 16 - ADC2 clk enable"]
    #[inline(always)]
    pub fn clken_adc2(&mut self) -> CLKEN_ADC2_W {
        CLKEN_ADC2_W { w: self }
    }
    #[doc = "Bit 17 - ADC2 clk divider enable"]
    #[inline(always)]
    pub fn diven_adc2(&mut self) -> DIVEN_ADC2_W {
        DIVEN_ADC2_W { w: self }
    }
    #[doc = "Bits 18:23 - ADC2 clk divider value"]
    #[inline(always)]
    pub fn div_adc2(&mut self) -> DIV_ADC2_W {
        DIV_ADC2_W { w: self }
    }
    #[doc = "Bit 24 - ADC3 clk enable"]
    #[inline(always)]
    pub fn clken_adc3(&mut self) -> CLKEN_ADC3_W {
        CLKEN_ADC3_W { w: self }
    }
    #[doc = "Bit 25 - ADC3 clk divider enable"]
    #[inline(always)]
    pub fn diven_adc3(&mut self) -> DIVEN_ADC3_W {
        DIVEN_ADC3_W { w: self }
    }
    #[doc = "Bits 26:31 - ADC3 clk divider value"]
    #[inline(always)]
    pub fn div_adc3(&mut self) -> DIV_ADC3_W {
        DIV_ADC3_W { w: self }
    }
}
