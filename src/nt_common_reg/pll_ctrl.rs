#[doc = "Reader of register PLL_CTRL"]
pub type R = crate::R<u32, super::PLL_CTRL>;
#[doc = "Writer for register PLL_CTRL"]
pub type W = crate::W<u32, super::PLL_CTRL>;
#[doc = "Register PLL_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::PLL_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "PLL reference clk selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REFSEL_A {
    #[doc = "0: tact signal from external oscillator"]
    EXTOSC,
    #[doc = "1: tact signal from USB clock(GPIO A0)"]
    USBCLK,
    #[doc = "2: tact signal from USB 60 MHz"]
    USB60MHZ,
    #[doc = "3: tact signal from Ethernet 25 MHz"]
    ETH25MHZ,
}
impl From<REFSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: REFSEL_A) -> Self {
        match variant {
            REFSEL_A::EXTOSC => 0,
            REFSEL_A::USBCLK => 1,
            REFSEL_A::USB60MHZ => 2,
            REFSEL_A::ETH25MHZ => 3,
        }
    }
}
#[doc = "Reader of field `REFSEL`"]
pub type REFSEL_R = crate::R<u8, REFSEL_A>;
impl REFSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REFSEL_A {
        match self.bits {
            0 => REFSEL_A::EXTOSC,
            1 => REFSEL_A::USBCLK,
            2 => REFSEL_A::USB60MHZ,
            3 => REFSEL_A::ETH25MHZ,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EXTOSC`"]
    #[inline(always)]
    pub fn is_ext_osc(&self) -> bool {
        *self == REFSEL_A::EXTOSC
    }
    #[doc = "Checks if the value of the field is `USBCLK`"]
    #[inline(always)]
    pub fn is_usbclk(&self) -> bool {
        *self == REFSEL_A::USBCLK
    }
    #[doc = "Checks if the value of the field is `USB60MHZ`"]
    #[inline(always)]
    pub fn is_usb60mhz(&self) -> bool {
        *self == REFSEL_A::USB60MHZ
    }
    #[doc = "Checks if the value of the field is `ETH25MHZ`"]
    #[inline(always)]
    pub fn is_eth25mhz(&self) -> bool {
        *self == REFSEL_A::ETH25MHZ
    }
}
#[doc = "Write proxy for field `REFSEL`"]
pub struct REFSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> REFSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REFSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "tact signal from external oscillator"]
    #[inline(always)]
    pub fn ext_osc(self) -> &'a mut W {
        self.variant(REFSEL_A::EXTOSC)
    }
    #[doc = "tact signal from USB clock(GPIO A0)"]
    #[inline(always)]
    pub fn usbclk(self) -> &'a mut W {
        self.variant(REFSEL_A::USBCLK)
    }
    #[doc = "tact signal from USB 60 MHz"]
    #[inline(always)]
    pub fn usb60mhz(self) -> &'a mut W {
        self.variant(REFSEL_A::USB60MHZ)
    }
    #[doc = "tact signal from Ethernet 25 MHz"]
    #[inline(always)]
    pub fn eth25mhz(self) -> &'a mut W {
        self.variant(REFSEL_A::ETH25MHZ)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `PLL_DIV`"]
pub type PLL_DIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PLL_DIV`"]
pub struct PLL_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `PD`"]
pub type PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PD`"]
pub struct PD_W<'a> {
    w: &'a mut W,
}
impl<'a> PD_W<'a> {
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
    #[doc = "Bits 0:1 - PLL reference clk selection"]
    #[inline(always)]
    pub fn refsel(&self) -> REFSEL_R {
        REFSEL_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 8:15 - External divider for PLL output"]
    #[inline(always)]
    pub fn pll_div(&self) -> PLL_DIV_R {
        PLL_DIV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 31 - PLL PowerDown mode enable"]
    #[inline(always)]
    pub fn pd(&self) -> PD_R {
        PD_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - PLL reference clk selection"]
    #[inline(always)]
    pub fn refsel(&mut self) -> REFSEL_W {
        REFSEL_W { w: self }
    }
    #[doc = "Bits 8:15 - External divider for PLL output"]
    #[inline(always)]
    pub fn pll_div(&mut self) -> PLL_DIV_W {
        PLL_DIV_W { w: self }
    }
    #[doc = "Bit 31 - PLL PowerDown mode enable"]
    #[inline(always)]
    pub fn pd(&mut self) -> PD_W {
        PD_W { w: self }
    }
}
