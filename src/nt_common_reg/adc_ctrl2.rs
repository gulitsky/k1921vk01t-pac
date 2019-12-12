#[doc = "Reader of register ADC_CTRL2"]
pub type R = crate::R<u32, super::ADC_CTRL2>;
#[doc = "Writer for register ADC_CTRL2"]
pub type W = crate::W<u32, super::ADC_CTRL2>;
#[doc = "Register ADC_CTRL2 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC_CTRL2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CLKEN_ADC8`"]
pub type CLKEN_ADC8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLKEN_ADC8`"]
pub struct CLKEN_ADC8_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKEN_ADC8_W<'a> {
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
#[doc = "Reader of field `DIVEN_ADC8`"]
pub type DIVEN_ADC8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIVEN_ADC8`"]
pub struct DIVEN_ADC8_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVEN_ADC8_W<'a> {
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
#[doc = "Reader of field `DIV_ADC8`"]
pub type DIV_ADC8_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DIV_ADC8`"]
pub struct DIV_ADC8_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_ADC8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 2)) | (((value as u32) & 0x3f) << 2);
        self.w
    }
}
#[doc = "Reader of field `CLKEN_ADC9`"]
pub type CLKEN_ADC9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLKEN_ADC9`"]
pub struct CLKEN_ADC9_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKEN_ADC9_W<'a> {
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
#[doc = "Reader of field `DIVEN_ADC9`"]
pub type DIVEN_ADC9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIVEN_ADC9`"]
pub struct DIVEN_ADC9_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVEN_ADC9_W<'a> {
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
#[doc = "Reader of field `DIV_ADC9`"]
pub type DIV_ADC9_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DIV_ADC9`"]
pub struct DIV_ADC9_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_ADC9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 10)) | (((value as u32) & 0x3f) << 10);
        self.w
    }
}
#[doc = "Reader of field `CLKEN_ADC10`"]
pub type CLKEN_ADC10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLKEN_ADC10`"]
pub struct CLKEN_ADC10_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKEN_ADC10_W<'a> {
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
#[doc = "Reader of field `DIVEN_ADC10`"]
pub type DIVEN_ADC10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIVEN_ADC10`"]
pub struct DIVEN_ADC10_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVEN_ADC10_W<'a> {
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
#[doc = "Reader of field `DIV_ADC10`"]
pub type DIV_ADC10_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DIV_ADC10`"]
pub struct DIV_ADC10_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_ADC10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 18)) | (((value as u32) & 0x3f) << 18);
        self.w
    }
}
#[doc = "Reader of field `CLKEN_ADC11`"]
pub type CLKEN_ADC11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLKEN_ADC11`"]
pub struct CLKEN_ADC11_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKEN_ADC11_W<'a> {
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
#[doc = "Reader of field `DIVEN_ADC11`"]
pub type DIVEN_ADC11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIVEN_ADC11`"]
pub struct DIVEN_ADC11_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVEN_ADC11_W<'a> {
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
#[doc = "Reader of field `DIV_ADC11`"]
pub type DIV_ADC11_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DIV_ADC11`"]
pub struct DIV_ADC11_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_ADC11_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 26)) | (((value as u32) & 0x3f) << 26);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable clk ADC8"]
    #[inline(always)]
    pub fn clken_adc8(&self) -> CLKEN_ADC8_R {
        CLKEN_ADC8_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable divider clk ADC8"]
    #[inline(always)]
    pub fn diven_adc8(&self) -> DIVEN_ADC8_R {
        DIVEN_ADC8_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:7 - Divider clk ADC8"]
    #[inline(always)]
    pub fn div_adc8(&self) -> DIV_ADC8_R {
        DIV_ADC8_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bit 8 - Enable clk ADC9"]
    #[inline(always)]
    pub fn clken_adc9(&self) -> CLKEN_ADC9_R {
        CLKEN_ADC9_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Enable divider clk ADC9"]
    #[inline(always)]
    pub fn diven_adc9(&self) -> DIVEN_ADC9_R {
        DIVEN_ADC9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 10:15 - Divider clk ADC9"]
    #[inline(always)]
    pub fn div_adc9(&self) -> DIV_ADC9_R {
        DIV_ADC9_R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - Enable clk ADC10"]
    #[inline(always)]
    pub fn clken_adc10(&self) -> CLKEN_ADC10_R {
        CLKEN_ADC10_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Enable divider clk ADC10"]
    #[inline(always)]
    pub fn diven_adc10(&self) -> DIVEN_ADC10_R {
        DIVEN_ADC10_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bits 18:23 - Divider clk ADC10"]
    #[inline(always)]
    pub fn div_adc10(&self) -> DIV_ADC10_R {
        DIV_ADC10_R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bit 24 - Enable clk ADC11"]
    #[inline(always)]
    pub fn clken_adc11(&self) -> CLKEN_ADC11_R {
        CLKEN_ADC11_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Enable divider clk ADC11"]
    #[inline(always)]
    pub fn diven_adc11(&self) -> DIVEN_ADC11_R {
        DIVEN_ADC11_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bits 26:31 - Divider clk ADC11"]
    #[inline(always)]
    pub fn div_adc11(&self) -> DIV_ADC11_R {
        DIV_ADC11_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable clk ADC8"]
    #[inline(always)]
    pub fn clken_adc8(&mut self) -> CLKEN_ADC8_W {
        CLKEN_ADC8_W { w: self }
    }
    #[doc = "Bit 1 - Enable divider clk ADC8"]
    #[inline(always)]
    pub fn diven_adc8(&mut self) -> DIVEN_ADC8_W {
        DIVEN_ADC8_W { w: self }
    }
    #[doc = "Bits 2:7 - Divider clk ADC8"]
    #[inline(always)]
    pub fn div_adc8(&mut self) -> DIV_ADC8_W {
        DIV_ADC8_W { w: self }
    }
    #[doc = "Bit 8 - Enable clk ADC9"]
    #[inline(always)]
    pub fn clken_adc9(&mut self) -> CLKEN_ADC9_W {
        CLKEN_ADC9_W { w: self }
    }
    #[doc = "Bit 9 - Enable divider clk ADC9"]
    #[inline(always)]
    pub fn diven_adc9(&mut self) -> DIVEN_ADC9_W {
        DIVEN_ADC9_W { w: self }
    }
    #[doc = "Bits 10:15 - Divider clk ADC9"]
    #[inline(always)]
    pub fn div_adc9(&mut self) -> DIV_ADC9_W {
        DIV_ADC9_W { w: self }
    }
    #[doc = "Bit 16 - Enable clk ADC10"]
    #[inline(always)]
    pub fn clken_adc10(&mut self) -> CLKEN_ADC10_W {
        CLKEN_ADC10_W { w: self }
    }
    #[doc = "Bit 17 - Enable divider clk ADC10"]
    #[inline(always)]
    pub fn diven_adc10(&mut self) -> DIVEN_ADC10_W {
        DIVEN_ADC10_W { w: self }
    }
    #[doc = "Bits 18:23 - Divider clk ADC10"]
    #[inline(always)]
    pub fn div_adc10(&mut self) -> DIV_ADC10_W {
        DIV_ADC10_W { w: self }
    }
    #[doc = "Bit 24 - Enable clk ADC11"]
    #[inline(always)]
    pub fn clken_adc11(&mut self) -> CLKEN_ADC11_W {
        CLKEN_ADC11_W { w: self }
    }
    #[doc = "Bit 25 - Enable divider clk ADC11"]
    #[inline(always)]
    pub fn diven_adc11(&mut self) -> DIVEN_ADC11_W {
        DIVEN_ADC11_W { w: self }
    }
    #[doc = "Bits 26:31 - Divider clk ADC11"]
    #[inline(always)]
    pub fn div_adc11(&mut self) -> DIV_ADC11_W {
        DIV_ADC11_W { w: self }
    }
}
