#[doc = "Reader of register UART_CLK"]
pub type R = crate::R<u32, super::UART_CLK>;
#[doc = "Writer for register UART_CLK"]
pub type W = crate::W<u32, super::UART_CLK>;
#[doc = "Register UART_CLK `reset()`'s with value 0"]
impl crate::ResetValue for super::UART_CLK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CLKEN_UART0`"]
pub type CLKEN_UART0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLKEN_UART0`"]
pub struct CLKEN_UART0_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKEN_UART0_W<'a> {
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
#[doc = "Reader of field `DIVEN_UART0`"]
pub type DIVEN_UART0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIVEN_UART0`"]
pub struct DIVEN_UART0_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVEN_UART0_W<'a> {
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
#[doc = "Reader of field `DIV_UART0`"]
pub type DIV_UART0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DIV_UART0`"]
pub struct DIV_UART0_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_UART0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 2)) | (((value as u32) & 0x3f) << 2);
        self.w
    }
}
#[doc = "Reader of field `CLKEN_UART1`"]
pub type CLKEN_UART1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLKEN_UART1`"]
pub struct CLKEN_UART1_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKEN_UART1_W<'a> {
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
#[doc = "Reader of field `DIVEN_UART1`"]
pub type DIVEN_UART1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIVEN_UART1`"]
pub struct DIVEN_UART1_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVEN_UART1_W<'a> {
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
#[doc = "Reader of field `DIV_UART1`"]
pub type DIV_UART1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DIV_UART1`"]
pub struct DIV_UART1_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_UART1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 10)) | (((value as u32) & 0x3f) << 10);
        self.w
    }
}
#[doc = "Reader of field `CLKEN_UART2`"]
pub type CLKEN_UART2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLKEN_UART2`"]
pub struct CLKEN_UART2_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKEN_UART2_W<'a> {
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
#[doc = "Reader of field `DIVEN_UART2`"]
pub type DIVEN_UART2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIVEN_UART2`"]
pub struct DIVEN_UART2_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVEN_UART2_W<'a> {
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
#[doc = "Reader of field `DIV_UART2`"]
pub type DIV_UART2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DIV_UART2`"]
pub struct DIV_UART2_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_UART2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 18)) | (((value as u32) & 0x3f) << 18);
        self.w
    }
}
#[doc = "Reader of field `CLKEN_UART3`"]
pub type CLKEN_UART3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLKEN_UART3`"]
pub struct CLKEN_UART3_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKEN_UART3_W<'a> {
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
#[doc = "Reader of field `DIVEN_UART3`"]
pub type DIVEN_UART3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIVEN_UART3`"]
pub struct DIVEN_UART3_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVEN_UART3_W<'a> {
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
#[doc = "Reader of field `DIV_UART3`"]
pub type DIV_UART3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DIV_UART3`"]
pub struct DIV_UART3_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_UART3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 26)) | (((value as u32) & 0x3f) << 26);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - UART0 clk enable"]
    #[inline(always)]
    pub fn clken_uart0(&self) -> CLKEN_UART0_R {
        CLKEN_UART0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - UART0 clk divider enable"]
    #[inline(always)]
    pub fn diven_uart0(&self) -> DIVEN_UART0_R {
        DIVEN_UART0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:7 - UART0 clk divider value"]
    #[inline(always)]
    pub fn div_uart0(&self) -> DIV_UART0_R {
        DIV_UART0_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bit 8 - UART1 clk enable"]
    #[inline(always)]
    pub fn clken_uart1(&self) -> CLKEN_UART1_R {
        CLKEN_UART1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - UART1 clk divider enable"]
    #[inline(always)]
    pub fn diven_uart1(&self) -> DIVEN_UART1_R {
        DIVEN_UART1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 10:15 - UART1 clk divider value"]
    #[inline(always)]
    pub fn div_uart1(&self) -> DIV_UART1_R {
        DIV_UART1_R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - UART2 clk enable"]
    #[inline(always)]
    pub fn clken_uart2(&self) -> CLKEN_UART2_R {
        CLKEN_UART2_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - UART2 clk divider enable"]
    #[inline(always)]
    pub fn diven_uart2(&self) -> DIVEN_UART2_R {
        DIVEN_UART2_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bits 18:23 - UART2 clk divider value"]
    #[inline(always)]
    pub fn div_uart2(&self) -> DIV_UART2_R {
        DIV_UART2_R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bit 24 - UART3 clk enable"]
    #[inline(always)]
    pub fn clken_uart3(&self) -> CLKEN_UART3_R {
        CLKEN_UART3_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - UART3 clk divider enable"]
    #[inline(always)]
    pub fn diven_uart3(&self) -> DIVEN_UART3_R {
        DIVEN_UART3_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bits 26:31 - UART3 clk divider value"]
    #[inline(always)]
    pub fn div_uart3(&self) -> DIV_UART3_R {
        DIV_UART3_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - UART0 clk enable"]
    #[inline(always)]
    pub fn clken_uart0(&mut self) -> CLKEN_UART0_W {
        CLKEN_UART0_W { w: self }
    }
    #[doc = "Bit 1 - UART0 clk divider enable"]
    #[inline(always)]
    pub fn diven_uart0(&mut self) -> DIVEN_UART0_W {
        DIVEN_UART0_W { w: self }
    }
    #[doc = "Bits 2:7 - UART0 clk divider value"]
    #[inline(always)]
    pub fn div_uart0(&mut self) -> DIV_UART0_W {
        DIV_UART0_W { w: self }
    }
    #[doc = "Bit 8 - UART1 clk enable"]
    #[inline(always)]
    pub fn clken_uart1(&mut self) -> CLKEN_UART1_W {
        CLKEN_UART1_W { w: self }
    }
    #[doc = "Bit 9 - UART1 clk divider enable"]
    #[inline(always)]
    pub fn diven_uart1(&mut self) -> DIVEN_UART1_W {
        DIVEN_UART1_W { w: self }
    }
    #[doc = "Bits 10:15 - UART1 clk divider value"]
    #[inline(always)]
    pub fn div_uart1(&mut self) -> DIV_UART1_W {
        DIV_UART1_W { w: self }
    }
    #[doc = "Bit 16 - UART2 clk enable"]
    #[inline(always)]
    pub fn clken_uart2(&mut self) -> CLKEN_UART2_W {
        CLKEN_UART2_W { w: self }
    }
    #[doc = "Bit 17 - UART2 clk divider enable"]
    #[inline(always)]
    pub fn diven_uart2(&mut self) -> DIVEN_UART2_W {
        DIVEN_UART2_W { w: self }
    }
    #[doc = "Bits 18:23 - UART2 clk divider value"]
    #[inline(always)]
    pub fn div_uart2(&mut self) -> DIV_UART2_W {
        DIV_UART2_W { w: self }
    }
    #[doc = "Bit 24 - UART3 clk enable"]
    #[inline(always)]
    pub fn clken_uart3(&mut self) -> CLKEN_UART3_W {
        CLKEN_UART3_W { w: self }
    }
    #[doc = "Bit 25 - UART3 clk divider enable"]
    #[inline(always)]
    pub fn diven_uart3(&mut self) -> DIVEN_UART3_W {
        DIVEN_UART3_W { w: self }
    }
    #[doc = "Bits 26:31 - UART3 clk divider value"]
    #[inline(always)]
    pub fn div_uart3(&mut self) -> DIV_UART3_W {
        DIV_UART3_W { w: self }
    }
}
