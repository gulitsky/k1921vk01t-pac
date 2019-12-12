#[doc = "Reader of register SPI_CLK"]
pub type R = crate::R<u32, super::SPI_CLK>;
#[doc = "Writer for register SPI_CLK"]
pub type W = crate::W<u32, super::SPI_CLK>;
#[doc = "Register SPI_CLK `reset()`'s with value 0"]
impl crate::ResetValue for super::SPI_CLK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CLKEN_SPI0`"]
pub type CLKEN_SPI0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLKEN_SPI0`"]
pub struct CLKEN_SPI0_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKEN_SPI0_W<'a> {
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
#[doc = "Reader of field `DIVEN_SPI0`"]
pub type DIVEN_SPI0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIVEN_SPI0`"]
pub struct DIVEN_SPI0_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVEN_SPI0_W<'a> {
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
#[doc = "Reader of field `DIV_SPI0`"]
pub type DIV_SPI0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DIV_SPI0`"]
pub struct DIV_SPI0_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_SPI0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 2)) | (((value as u32) & 0x3f) << 2);
        self.w
    }
}
#[doc = "Reader of field `CLKEN_SPI1`"]
pub type CLKEN_SPI1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLKEN_SPI1`"]
pub struct CLKEN_SPI1_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKEN_SPI1_W<'a> {
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
#[doc = "Reader of field `DIVEN_SPI1`"]
pub type DIVEN_SPI1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIVEN_SPI1`"]
pub struct DIVEN_SPI1_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVEN_SPI1_W<'a> {
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
#[doc = "Reader of field `DIV_SPI1`"]
pub type DIV_SPI1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DIV_SPI1`"]
pub struct DIV_SPI1_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_SPI1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 10)) | (((value as u32) & 0x3f) << 10);
        self.w
    }
}
#[doc = "Reader of field `CLKEN_SPI2`"]
pub type CLKEN_SPI2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLKEN_SPI2`"]
pub struct CLKEN_SPI2_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKEN_SPI2_W<'a> {
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
#[doc = "Reader of field `DIVEN_SPI2`"]
pub type DIVEN_SPI2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIVEN_SPI2`"]
pub struct DIVEN_SPI2_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVEN_SPI2_W<'a> {
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
#[doc = "Reader of field `DIV_SPI2`"]
pub type DIV_SPI2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DIV_SPI2`"]
pub struct DIV_SPI2_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_SPI2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 18)) | (((value as u32) & 0x3f) << 18);
        self.w
    }
}
#[doc = "Reader of field `CLKEN_SPI3`"]
pub type CLKEN_SPI3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLKEN_SPI3`"]
pub struct CLKEN_SPI3_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKEN_SPI3_W<'a> {
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
#[doc = "Reader of field `DIVEN_SPI3`"]
pub type DIVEN_SPI3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIVEN_SPI3`"]
pub struct DIVEN_SPI3_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVEN_SPI3_W<'a> {
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
#[doc = "Reader of field `DIV_SPI3`"]
pub type DIV_SPI3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DIV_SPI3`"]
pub struct DIV_SPI3_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_SPI3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 26)) | (((value as u32) & 0x3f) << 26);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - SPI0 clk enable"]
    #[inline(always)]
    pub fn clken_spi0(&self) -> CLKEN_SPI0_R {
        CLKEN_SPI0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SPI0 clk divider enable"]
    #[inline(always)]
    pub fn diven_spi0(&self) -> DIVEN_SPI0_R {
        DIVEN_SPI0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:7 - SPI0 clk divider value"]
    #[inline(always)]
    pub fn div_spi0(&self) -> DIV_SPI0_R {
        DIV_SPI0_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bit 8 - SPI1 clk enable"]
    #[inline(always)]
    pub fn clken_spi1(&self) -> CLKEN_SPI1_R {
        CLKEN_SPI1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - SPI1 clk divider enable"]
    #[inline(always)]
    pub fn diven_spi1(&self) -> DIVEN_SPI1_R {
        DIVEN_SPI1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 10:15 - SPI1 clk divider value"]
    #[inline(always)]
    pub fn div_spi1(&self) -> DIV_SPI1_R {
        DIV_SPI1_R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SPI2 clk enable"]
    #[inline(always)]
    pub fn clken_spi2(&self) -> CLKEN_SPI2_R {
        CLKEN_SPI2_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - SPI2 clk divider enable"]
    #[inline(always)]
    pub fn diven_spi2(&self) -> DIVEN_SPI2_R {
        DIVEN_SPI2_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bits 18:23 - SPI2 clk divider value"]
    #[inline(always)]
    pub fn div_spi2(&self) -> DIV_SPI2_R {
        DIV_SPI2_R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bit 24 - SPI3 clk enable"]
    #[inline(always)]
    pub fn clken_spi3(&self) -> CLKEN_SPI3_R {
        CLKEN_SPI3_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - SPI3 clk divider enable"]
    #[inline(always)]
    pub fn diven_spi3(&self) -> DIVEN_SPI3_R {
        DIVEN_SPI3_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bits 26:31 - SPI3 clk divider value"]
    #[inline(always)]
    pub fn div_spi3(&self) -> DIV_SPI3_R {
        DIV_SPI3_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - SPI0 clk enable"]
    #[inline(always)]
    pub fn clken_spi0(&mut self) -> CLKEN_SPI0_W {
        CLKEN_SPI0_W { w: self }
    }
    #[doc = "Bit 1 - SPI0 clk divider enable"]
    #[inline(always)]
    pub fn diven_spi0(&mut self) -> DIVEN_SPI0_W {
        DIVEN_SPI0_W { w: self }
    }
    #[doc = "Bits 2:7 - SPI0 clk divider value"]
    #[inline(always)]
    pub fn div_spi0(&mut self) -> DIV_SPI0_W {
        DIV_SPI0_W { w: self }
    }
    #[doc = "Bit 8 - SPI1 clk enable"]
    #[inline(always)]
    pub fn clken_spi1(&mut self) -> CLKEN_SPI1_W {
        CLKEN_SPI1_W { w: self }
    }
    #[doc = "Bit 9 - SPI1 clk divider enable"]
    #[inline(always)]
    pub fn diven_spi1(&mut self) -> DIVEN_SPI1_W {
        DIVEN_SPI1_W { w: self }
    }
    #[doc = "Bits 10:15 - SPI1 clk divider value"]
    #[inline(always)]
    pub fn div_spi1(&mut self) -> DIV_SPI1_W {
        DIV_SPI1_W { w: self }
    }
    #[doc = "Bit 16 - SPI2 clk enable"]
    #[inline(always)]
    pub fn clken_spi2(&mut self) -> CLKEN_SPI2_W {
        CLKEN_SPI2_W { w: self }
    }
    #[doc = "Bit 17 - SPI2 clk divider enable"]
    #[inline(always)]
    pub fn diven_spi2(&mut self) -> DIVEN_SPI2_W {
        DIVEN_SPI2_W { w: self }
    }
    #[doc = "Bits 18:23 - SPI2 clk divider value"]
    #[inline(always)]
    pub fn div_spi2(&mut self) -> DIV_SPI2_W {
        DIV_SPI2_W { w: self }
    }
    #[doc = "Bit 24 - SPI3 clk enable"]
    #[inline(always)]
    pub fn clken_spi3(&mut self) -> CLKEN_SPI3_W {
        CLKEN_SPI3_W { w: self }
    }
    #[doc = "Bit 25 - SPI3 clk divider enable"]
    #[inline(always)]
    pub fn diven_spi3(&mut self) -> DIVEN_SPI3_W {
        DIVEN_SPI3_W { w: self }
    }
    #[doc = "Bits 26:31 - SPI3 clk divider value"]
    #[inline(always)]
    pub fn div_spi3(&mut self) -> DIV_SPI3_W {
        DIV_SPI3_W { w: self }
    }
}
