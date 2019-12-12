#[doc = "Reader of register SYS_CLK"]
pub type R = crate::R<u32, super::SYS_CLK>;
#[doc = "Writer for register SYS_CLK"]
pub type W = crate::W<u32, super::SYS_CLK>;
#[doc = "Register SYS_CLK `reset()`'s with value 0"]
impl crate::ResetValue for super::SYS_CLK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "System clk source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEL_SRC_A {
    #[doc = "0: select POR clock or XI_OSC by input CPE"]
    CPE,
    #[doc = "1: POR clock"]
    POR,
    #[doc = "2:  external oscillator clock (XI_OSC)"]
    EXTOSC,
    #[doc = "3: PLL clock"]
    PLL,
    #[doc = "4: PLL clock with post-divider"]
    PLLDIV,
    #[doc = "5: USB clock 60 MHz"]
    USB60MHZ,
    #[doc = "6: clock from input CLK_USB GPIO A0 (12/24 MHz)"]
    USBCLK,
    #[doc = "7: clock from external Ethernet clock GPIO B3 or GPIO A8"]
    ETH25MHZ,
}
impl From<SEL_SRC_A> for u8 {
    #[inline(always)]
    fn from(variant: SEL_SRC_A) -> Self {
        match variant {
            SEL_SRC_A::CPE => 0,
            SEL_SRC_A::POR => 1,
            SEL_SRC_A::EXTOSC => 2,
            SEL_SRC_A::PLL => 3,
            SEL_SRC_A::PLLDIV => 4,
            SEL_SRC_A::USB60MHZ => 5,
            SEL_SRC_A::USBCLK => 6,
            SEL_SRC_A::ETH25MHZ => 7,
        }
    }
}
#[doc = "Reader of field `SEL_SRC`"]
pub type SEL_SRC_R = crate::R<u8, SEL_SRC_A>;
impl SEL_SRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEL_SRC_A {
        match self.bits {
            0 => SEL_SRC_A::CPE,
            1 => SEL_SRC_A::POR,
            2 => SEL_SRC_A::EXTOSC,
            3 => SEL_SRC_A::PLL,
            4 => SEL_SRC_A::PLLDIV,
            5 => SEL_SRC_A::USB60MHZ,
            6 => SEL_SRC_A::USBCLK,
            7 => SEL_SRC_A::ETH25MHZ,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CPE`"]
    #[inline(always)]
    pub fn is_cpe(&self) -> bool {
        *self == SEL_SRC_A::CPE
    }
    #[doc = "Checks if the value of the field is `POR`"]
    #[inline(always)]
    pub fn is_por(&self) -> bool {
        *self == SEL_SRC_A::POR
    }
    #[doc = "Checks if the value of the field is `EXTOSC`"]
    #[inline(always)]
    pub fn is_ext_osc(&self) -> bool {
        *self == SEL_SRC_A::EXTOSC
    }
    #[doc = "Checks if the value of the field is `PLL`"]
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        *self == SEL_SRC_A::PLL
    }
    #[doc = "Checks if the value of the field is `PLLDIV`"]
    #[inline(always)]
    pub fn is_plldiv(&self) -> bool {
        *self == SEL_SRC_A::PLLDIV
    }
    #[doc = "Checks if the value of the field is `USB60MHZ`"]
    #[inline(always)]
    pub fn is_usb60mhz(&self) -> bool {
        *self == SEL_SRC_A::USB60MHZ
    }
    #[doc = "Checks if the value of the field is `USBCLK`"]
    #[inline(always)]
    pub fn is_usbclk(&self) -> bool {
        *self == SEL_SRC_A::USBCLK
    }
    #[doc = "Checks if the value of the field is `ETH25MHZ`"]
    #[inline(always)]
    pub fn is_eth25mhz(&self) -> bool {
        *self == SEL_SRC_A::ETH25MHZ
    }
}
#[doc = "Write proxy for field `SEL_SRC`"]
pub struct SEL_SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL_SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEL_SRC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "select POR clock or XI_OSC by input CPE"]
    #[inline(always)]
    pub fn cpe(self) -> &'a mut W {
        self.variant(SEL_SRC_A::CPE)
    }
    #[doc = "POR clock"]
    #[inline(always)]
    pub fn por(self) -> &'a mut W {
        self.variant(SEL_SRC_A::POR)
    }
    #[doc = "external oscillator clock (XI_OSC)"]
    #[inline(always)]
    pub fn ext_osc(self) -> &'a mut W {
        self.variant(SEL_SRC_A::EXTOSC)
    }
    #[doc = "PLL clock"]
    #[inline(always)]
    pub fn pll(self) -> &'a mut W {
        self.variant(SEL_SRC_A::PLL)
    }
    #[doc = "PLL clock with post-divider"]
    #[inline(always)]
    pub fn plldiv(self) -> &'a mut W {
        self.variant(SEL_SRC_A::PLLDIV)
    }
    #[doc = "USB clock 60 MHz"]
    #[inline(always)]
    pub fn usb60mhz(self) -> &'a mut W {
        self.variant(SEL_SRC_A::USB60MHZ)
    }
    #[doc = "clock from input CLK_USB GPIO A0 (12/24 MHz)"]
    #[inline(always)]
    pub fn usbclk(self) -> &'a mut W {
        self.variant(SEL_SRC_A::USBCLK)
    }
    #[doc = "clock from external Ethernet clock GPIO B3 or GPIO A8"]
    #[inline(always)]
    pub fn eth25mhz(self) -> &'a mut W {
        self.variant(SEL_SRC_A::ETH25MHZ)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Current system clk source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CURR_SRC_A {
    #[doc = "0: select POR clock or XI_OSC by input CPE"]
    CPE,
    #[doc = "1: POR clock"]
    POR,
    #[doc = "2:  external oscillator clock (XI_OSC)"]
    EXTOSC,
    #[doc = "3: PLL clock"]
    PLL,
    #[doc = "4: PLL clock with post-divider"]
    PLLDIV,
    #[doc = "5: USB clock 60 MHz"]
    USB60MHZ,
    #[doc = "6: clock from input CLK_USB GPIO A0 (12/24 MHz)"]
    USBCLK,
    #[doc = "7: clock from external Ethernet clock GPIO B3 or GPIO A8"]
    ETH25MHZ,
}
impl From<CURR_SRC_A> for u8 {
    #[inline(always)]
    fn from(variant: CURR_SRC_A) -> Self {
        match variant {
            CURR_SRC_A::CPE => 0,
            CURR_SRC_A::POR => 1,
            CURR_SRC_A::EXTOSC => 2,
            CURR_SRC_A::PLL => 3,
            CURR_SRC_A::PLLDIV => 4,
            CURR_SRC_A::USB60MHZ => 5,
            CURR_SRC_A::USBCLK => 6,
            CURR_SRC_A::ETH25MHZ => 7,
        }
    }
}
#[doc = "Reader of field `CURR_SRC`"]
pub type CURR_SRC_R = crate::R<u8, CURR_SRC_A>;
impl CURR_SRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CURR_SRC_A {
        match self.bits {
            0 => CURR_SRC_A::CPE,
            1 => CURR_SRC_A::POR,
            2 => CURR_SRC_A::EXTOSC,
            3 => CURR_SRC_A::PLL,
            4 => CURR_SRC_A::PLLDIV,
            5 => CURR_SRC_A::USB60MHZ,
            6 => CURR_SRC_A::USBCLK,
            7 => CURR_SRC_A::ETH25MHZ,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CPE`"]
    #[inline(always)]
    pub fn is_cpe(&self) -> bool {
        *self == CURR_SRC_A::CPE
    }
    #[doc = "Checks if the value of the field is `POR`"]
    #[inline(always)]
    pub fn is_por(&self) -> bool {
        *self == CURR_SRC_A::POR
    }
    #[doc = "Checks if the value of the field is `EXTOSC`"]
    #[inline(always)]
    pub fn is_ext_osc(&self) -> bool {
        *self == CURR_SRC_A::EXTOSC
    }
    #[doc = "Checks if the value of the field is `PLL`"]
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        *self == CURR_SRC_A::PLL
    }
    #[doc = "Checks if the value of the field is `PLLDIV`"]
    #[inline(always)]
    pub fn is_plldiv(&self) -> bool {
        *self == CURR_SRC_A::PLLDIV
    }
    #[doc = "Checks if the value of the field is `USB60MHZ`"]
    #[inline(always)]
    pub fn is_usb60mhz(&self) -> bool {
        *self == CURR_SRC_A::USB60MHZ
    }
    #[doc = "Checks if the value of the field is `USBCLK`"]
    #[inline(always)]
    pub fn is_usbclk(&self) -> bool {
        *self == CURR_SRC_A::USBCLK
    }
    #[doc = "Checks if the value of the field is `ETH25MHZ`"]
    #[inline(always)]
    pub fn is_eth25mhz(&self) -> bool {
        *self == CURR_SRC_A::ETH25MHZ
    }
}
impl R {
    #[doc = "Bits 0:2 - System clk source selection"]
    #[inline(always)]
    pub fn sel_src(&self) -> SEL_SRC_R {
        SEL_SRC_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 8:10 - Current system clk source"]
    #[inline(always)]
    pub fn curr_src(&self) -> CURR_SRC_R {
        CURR_SRC_R::new(((self.bits >> 8) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - System clk source selection"]
    #[inline(always)]
    pub fn sel_src(&mut self) -> SEL_SRC_W {
        SEL_SRC_W { w: self }
    }
}
