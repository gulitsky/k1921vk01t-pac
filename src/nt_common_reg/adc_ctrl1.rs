#[doc = "Reader of register ADC_CTRL1"]
pub type R = crate::R<u32, super::ADC_CTRL1>;
#[doc = "Writer for register ADC_CTRL1"]
pub type W = crate::W<u32, super::ADC_CTRL1>;
#[doc = "Register ADC_CTRL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC_CTRL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CLKEN_ADC4`"]
pub type CLKEN_ADC4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLKEN_ADC4`"]
pub struct CLKEN_ADC4_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKEN_ADC4_W<'a> {
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
#[doc = "Reader of field `DIVEN_ADC4`"]
pub type DIVEN_ADC4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIVEN_ADC4`"]
pub struct DIVEN_ADC4_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVEN_ADC4_W<'a> {
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
#[doc = "Reader of field `DIV_ADC4`"]
pub type DIV_ADC4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DIV_ADC4`"]
pub struct DIV_ADC4_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_ADC4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 2)) | (((value as u32) & 0x3f) << 2);
        self.w
    }
}
#[doc = "Reader of field `CLKEN_ADC5`"]
pub type CLKEN_ADC5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLKEN_ADC5`"]
pub struct CLKEN_ADC5_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKEN_ADC5_W<'a> {
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
#[doc = "Reader of field `DIVEN_ADC5`"]
pub type DIVEN_ADC5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIVEN_ADC5`"]
pub struct DIVEN_ADC5_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVEN_ADC5_W<'a> {
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
#[doc = "Reader of field `DIV_ADC5`"]
pub type DIV_ADC5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DIV_ADC5`"]
pub struct DIV_ADC5_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_ADC5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 10)) | (((value as u32) & 0x3f) << 10);
        self.w
    }
}
#[doc = "Reader of field `CLKEN_ADC6`"]
pub type CLKEN_ADC6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLKEN_ADC6`"]
pub struct CLKEN_ADC6_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKEN_ADC6_W<'a> {
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
#[doc = "Reader of field `DIVEN_ADC6`"]
pub type DIVEN_ADC6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIVEN_ADC6`"]
pub struct DIVEN_ADC6_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVEN_ADC6_W<'a> {
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
#[doc = "Reader of field `DIV_ADC6`"]
pub type DIV_ADC6_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DIV_ADC6`"]
pub struct DIV_ADC6_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_ADC6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 18)) | (((value as u32) & 0x3f) << 18);
        self.w
    }
}
#[doc = "Reader of field `CLKEN_ADC7`"]
pub type CLKEN_ADC7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLKEN_ADC7`"]
pub struct CLKEN_ADC7_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKEN_ADC7_W<'a> {
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
#[doc = "Reader of field `DIVEN_ADC7`"]
pub type DIVEN_ADC7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIVEN_ADC7`"]
pub struct DIVEN_ADC7_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVEN_ADC7_W<'a> {
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
#[doc = "Reader of field `DIV_ADC7`"]
pub type DIV_ADC7_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DIV_ADC7`"]
pub struct DIV_ADC7_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_ADC7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 26)) | (((value as u32) & 0x3f) << 26);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - ADC4 clk enable"]
    #[inline(always)]
    pub fn clken_adc4(&self) -> CLKEN_ADC4_R {
        CLKEN_ADC4_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ADC4 clk divider enable"]
    #[inline(always)]
    pub fn diven_adc4(&self) -> DIVEN_ADC4_R {
        DIVEN_ADC4_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:7 - ADC4 clk divider value"]
    #[inline(always)]
    pub fn div_adc4(&self) -> DIV_ADC4_R {
        DIV_ADC4_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bit 8 - ADC5 clk enable"]
    #[inline(always)]
    pub fn clken_adc5(&self) -> CLKEN_ADC5_R {
        CLKEN_ADC5_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - ADC5 clk divider enable"]
    #[inline(always)]
    pub fn diven_adc5(&self) -> DIVEN_ADC5_R {
        DIVEN_ADC5_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 10:15 - ADC5 clk divider value"]
    #[inline(always)]
    pub fn div_adc5(&self) -> DIV_ADC5_R {
        DIV_ADC5_R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - ADC6 clk enable"]
    #[inline(always)]
    pub fn clken_adc6(&self) -> CLKEN_ADC6_R {
        CLKEN_ADC6_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - ADC6 clk divider enable"]
    #[inline(always)]
    pub fn diven_adc6(&self) -> DIVEN_ADC6_R {
        DIVEN_ADC6_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bits 18:23 - ADC6 clk divider value"]
    #[inline(always)]
    pub fn div_adc6(&self) -> DIV_ADC6_R {
        DIV_ADC6_R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bit 24 - ADC7 clk enable"]
    #[inline(always)]
    pub fn clken_adc7(&self) -> CLKEN_ADC7_R {
        CLKEN_ADC7_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - ADC7 clk divider enable"]
    #[inline(always)]
    pub fn diven_adc7(&self) -> DIVEN_ADC7_R {
        DIVEN_ADC7_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bits 26:31 - ADC7 clk divider value"]
    #[inline(always)]
    pub fn div_adc7(&self) -> DIV_ADC7_R {
        DIV_ADC7_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - ADC4 clk enable"]
    #[inline(always)]
    pub fn clken_adc4(&mut self) -> CLKEN_ADC4_W {
        CLKEN_ADC4_W { w: self }
    }
    #[doc = "Bit 1 - ADC4 clk divider enable"]
    #[inline(always)]
    pub fn diven_adc4(&mut self) -> DIVEN_ADC4_W {
        DIVEN_ADC4_W { w: self }
    }
    #[doc = "Bits 2:7 - ADC4 clk divider value"]
    #[inline(always)]
    pub fn div_adc4(&mut self) -> DIV_ADC4_W {
        DIV_ADC4_W { w: self }
    }
    #[doc = "Bit 8 - ADC5 clk enable"]
    #[inline(always)]
    pub fn clken_adc5(&mut self) -> CLKEN_ADC5_W {
        CLKEN_ADC5_W { w: self }
    }
    #[doc = "Bit 9 - ADC5 clk divider enable"]
    #[inline(always)]
    pub fn diven_adc5(&mut self) -> DIVEN_ADC5_W {
        DIVEN_ADC5_W { w: self }
    }
    #[doc = "Bits 10:15 - ADC5 clk divider value"]
    #[inline(always)]
    pub fn div_adc5(&mut self) -> DIV_ADC5_W {
        DIV_ADC5_W { w: self }
    }
    #[doc = "Bit 16 - ADC6 clk enable"]
    #[inline(always)]
    pub fn clken_adc6(&mut self) -> CLKEN_ADC6_W {
        CLKEN_ADC6_W { w: self }
    }
    #[doc = "Bit 17 - ADC6 clk divider enable"]
    #[inline(always)]
    pub fn diven_adc6(&mut self) -> DIVEN_ADC6_W {
        DIVEN_ADC6_W { w: self }
    }
    #[doc = "Bits 18:23 - ADC6 clk divider value"]
    #[inline(always)]
    pub fn div_adc6(&mut self) -> DIV_ADC6_W {
        DIV_ADC6_W { w: self }
    }
    #[doc = "Bit 24 - ADC7 clk enable"]
    #[inline(always)]
    pub fn clken_adc7(&mut self) -> CLKEN_ADC7_W {
        CLKEN_ADC7_W { w: self }
    }
    #[doc = "Bit 25 - ADC7 clk divider enable"]
    #[inline(always)]
    pub fn diven_adc7(&mut self) -> DIVEN_ADC7_W {
        DIVEN_ADC7_W { w: self }
    }
    #[doc = "Bits 26:31 - ADC7 clk divider value"]
    #[inline(always)]
    pub fn div_adc7(&mut self) -> DIV_ADC7_W {
        DIV_ADC7_W { w: self }
    }
}
