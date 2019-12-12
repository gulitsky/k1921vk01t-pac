#[doc = "Reader of register UART_SPI_CLK_SEL"]
pub type R = crate::R<u32, super::UART_SPI_CLK_SEL>;
#[doc = "Writer for register UART_SPI_CLK_SEL"]
pub type W = crate::W<u32, super::UART_SPI_CLK_SEL>;
#[doc = "Register UART_SPI_CLK_SEL `reset()`'s with value 0"]
impl crate::ResetValue for super::UART_SPI_CLK_SEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Select source clk UART0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEL_UART0_A {
    #[doc = "0: system clock"]
    SYSCLK,
    #[doc = "1: external oscillator clock (XI_OSC)"]
    EXTOSC,
    #[doc = "2: USB clock from GPIO A0"]
    USBCLK,
    #[doc = "3: USB clock 60 MHz"]
    USB60MHZ,
}
impl From<SEL_UART0_A> for u8 {
    #[inline(always)]
    fn from(variant: SEL_UART0_A) -> Self {
        match variant {
            SEL_UART0_A::SYSCLK => 0,
            SEL_UART0_A::EXTOSC => 1,
            SEL_UART0_A::USBCLK => 2,
            SEL_UART0_A::USB60MHZ => 3,
        }
    }
}
#[doc = "Reader of field `SEL_UART0`"]
pub type SEL_UART0_R = crate::R<u8, SEL_UART0_A>;
impl SEL_UART0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEL_UART0_A {
        match self.bits {
            0 => SEL_UART0_A::SYSCLK,
            1 => SEL_UART0_A::EXTOSC,
            2 => SEL_UART0_A::USBCLK,
            3 => SEL_UART0_A::USB60MHZ,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SYSCLK`"]
    #[inline(always)]
    pub fn is_sys_clk(&self) -> bool {
        *self == SEL_UART0_A::SYSCLK
    }
    #[doc = "Checks if the value of the field is `EXTOSC`"]
    #[inline(always)]
    pub fn is_ext_osc(&self) -> bool {
        *self == SEL_UART0_A::EXTOSC
    }
    #[doc = "Checks if the value of the field is `USBCLK`"]
    #[inline(always)]
    pub fn is_usbclk(&self) -> bool {
        *self == SEL_UART0_A::USBCLK
    }
    #[doc = "Checks if the value of the field is `USB60MHZ`"]
    #[inline(always)]
    pub fn is_usb60mhz(&self) -> bool {
        *self == SEL_UART0_A::USB60MHZ
    }
}
#[doc = "Write proxy for field `SEL_UART0`"]
pub struct SEL_UART0_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL_UART0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEL_UART0_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "system clock"]
    #[inline(always)]
    pub fn sys_clk(self) -> &'a mut W {
        self.variant(SEL_UART0_A::SYSCLK)
    }
    #[doc = "external oscillator clock (XI_OSC)"]
    #[inline(always)]
    pub fn ext_osc(self) -> &'a mut W {
        self.variant(SEL_UART0_A::EXTOSC)
    }
    #[doc = "USB clock from GPIO A0"]
    #[inline(always)]
    pub fn usbclk(self) -> &'a mut W {
        self.variant(SEL_UART0_A::USBCLK)
    }
    #[doc = "USB clock 60 MHz"]
    #[inline(always)]
    pub fn usb60mhz(self) -> &'a mut W {
        self.variant(SEL_UART0_A::USB60MHZ)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Select source clk UART1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEL_UART1_A {
    #[doc = "0: system clock"]
    SYSCLK,
    #[doc = "1: external oscillator clock (XI_OSC)"]
    EXTOSC,
    #[doc = "2: USB clock from GPIO A0"]
    USBCLK,
    #[doc = "3: USB clock 60 MHz"]
    USB60MHZ,
}
impl From<SEL_UART1_A> for u8 {
    #[inline(always)]
    fn from(variant: SEL_UART1_A) -> Self {
        match variant {
            SEL_UART1_A::SYSCLK => 0,
            SEL_UART1_A::EXTOSC => 1,
            SEL_UART1_A::USBCLK => 2,
            SEL_UART1_A::USB60MHZ => 3,
        }
    }
}
#[doc = "Reader of field `SEL_UART1`"]
pub type SEL_UART1_R = crate::R<u8, SEL_UART1_A>;
impl SEL_UART1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEL_UART1_A {
        match self.bits {
            0 => SEL_UART1_A::SYSCLK,
            1 => SEL_UART1_A::EXTOSC,
            2 => SEL_UART1_A::USBCLK,
            3 => SEL_UART1_A::USB60MHZ,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SYSCLK`"]
    #[inline(always)]
    pub fn is_sys_clk(&self) -> bool {
        *self == SEL_UART1_A::SYSCLK
    }
    #[doc = "Checks if the value of the field is `EXTOSC`"]
    #[inline(always)]
    pub fn is_ext_osc(&self) -> bool {
        *self == SEL_UART1_A::EXTOSC
    }
    #[doc = "Checks if the value of the field is `USBCLK`"]
    #[inline(always)]
    pub fn is_usbclk(&self) -> bool {
        *self == SEL_UART1_A::USBCLK
    }
    #[doc = "Checks if the value of the field is `USB60MHZ`"]
    #[inline(always)]
    pub fn is_usb60mhz(&self) -> bool {
        *self == SEL_UART1_A::USB60MHZ
    }
}
#[doc = "Write proxy for field `SEL_UART1`"]
pub struct SEL_UART1_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL_UART1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEL_UART1_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "system clock"]
    #[inline(always)]
    pub fn sys_clk(self) -> &'a mut W {
        self.variant(SEL_UART1_A::SYSCLK)
    }
    #[doc = "external oscillator clock (XI_OSC)"]
    #[inline(always)]
    pub fn ext_osc(self) -> &'a mut W {
        self.variant(SEL_UART1_A::EXTOSC)
    }
    #[doc = "USB clock from GPIO A0"]
    #[inline(always)]
    pub fn usbclk(self) -> &'a mut W {
        self.variant(SEL_UART1_A::USBCLK)
    }
    #[doc = "USB clock 60 MHz"]
    #[inline(always)]
    pub fn usb60mhz(self) -> &'a mut W {
        self.variant(SEL_UART1_A::USB60MHZ)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Select source clk UART2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEL_UART2_A {
    #[doc = "0: system clock"]
    SYSCLK,
    #[doc = "1: external oscillator clock (XI_OSC)"]
    EXTOSC,
    #[doc = "2: USB clock from GPIO A0"]
    USBCLK,
    #[doc = "3: USB clock 60 MHz"]
    USB60MHZ,
}
impl From<SEL_UART2_A> for u8 {
    #[inline(always)]
    fn from(variant: SEL_UART2_A) -> Self {
        match variant {
            SEL_UART2_A::SYSCLK => 0,
            SEL_UART2_A::EXTOSC => 1,
            SEL_UART2_A::USBCLK => 2,
            SEL_UART2_A::USB60MHZ => 3,
        }
    }
}
#[doc = "Reader of field `SEL_UART2`"]
pub type SEL_UART2_R = crate::R<u8, SEL_UART2_A>;
impl SEL_UART2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEL_UART2_A {
        match self.bits {
            0 => SEL_UART2_A::SYSCLK,
            1 => SEL_UART2_A::EXTOSC,
            2 => SEL_UART2_A::USBCLK,
            3 => SEL_UART2_A::USB60MHZ,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SYSCLK`"]
    #[inline(always)]
    pub fn is_sys_clk(&self) -> bool {
        *self == SEL_UART2_A::SYSCLK
    }
    #[doc = "Checks if the value of the field is `EXTOSC`"]
    #[inline(always)]
    pub fn is_ext_osc(&self) -> bool {
        *self == SEL_UART2_A::EXTOSC
    }
    #[doc = "Checks if the value of the field is `USBCLK`"]
    #[inline(always)]
    pub fn is_usbclk(&self) -> bool {
        *self == SEL_UART2_A::USBCLK
    }
    #[doc = "Checks if the value of the field is `USB60MHZ`"]
    #[inline(always)]
    pub fn is_usb60mhz(&self) -> bool {
        *self == SEL_UART2_A::USB60MHZ
    }
}
#[doc = "Write proxy for field `SEL_UART2`"]
pub struct SEL_UART2_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL_UART2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEL_UART2_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "system clock"]
    #[inline(always)]
    pub fn sys_clk(self) -> &'a mut W {
        self.variant(SEL_UART2_A::SYSCLK)
    }
    #[doc = "external oscillator clock (XI_OSC)"]
    #[inline(always)]
    pub fn ext_osc(self) -> &'a mut W {
        self.variant(SEL_UART2_A::EXTOSC)
    }
    #[doc = "USB clock from GPIO A0"]
    #[inline(always)]
    pub fn usbclk(self) -> &'a mut W {
        self.variant(SEL_UART2_A::USBCLK)
    }
    #[doc = "USB clock 60 MHz"]
    #[inline(always)]
    pub fn usb60mhz(self) -> &'a mut W {
        self.variant(SEL_UART2_A::USB60MHZ)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Select source clk UART3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEL_UART3_A {
    #[doc = "0: system clock"]
    SYSCLK,
    #[doc = "1: external oscillator clock (XI_OSC)"]
    EXTOSC,
    #[doc = "2: USB clock from GPIO A0"]
    USBCLK,
    #[doc = "3: USB clock 60 MHz"]
    USB60MHZ,
}
impl From<SEL_UART3_A> for u8 {
    #[inline(always)]
    fn from(variant: SEL_UART3_A) -> Self {
        match variant {
            SEL_UART3_A::SYSCLK => 0,
            SEL_UART3_A::EXTOSC => 1,
            SEL_UART3_A::USBCLK => 2,
            SEL_UART3_A::USB60MHZ => 3,
        }
    }
}
#[doc = "Reader of field `SEL_UART3`"]
pub type SEL_UART3_R = crate::R<u8, SEL_UART3_A>;
impl SEL_UART3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEL_UART3_A {
        match self.bits {
            0 => SEL_UART3_A::SYSCLK,
            1 => SEL_UART3_A::EXTOSC,
            2 => SEL_UART3_A::USBCLK,
            3 => SEL_UART3_A::USB60MHZ,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SYSCLK`"]
    #[inline(always)]
    pub fn is_sys_clk(&self) -> bool {
        *self == SEL_UART3_A::SYSCLK
    }
    #[doc = "Checks if the value of the field is `EXTOSC`"]
    #[inline(always)]
    pub fn is_ext_osc(&self) -> bool {
        *self == SEL_UART3_A::EXTOSC
    }
    #[doc = "Checks if the value of the field is `USBCLK`"]
    #[inline(always)]
    pub fn is_usbclk(&self) -> bool {
        *self == SEL_UART3_A::USBCLK
    }
    #[doc = "Checks if the value of the field is `USB60MHZ`"]
    #[inline(always)]
    pub fn is_usb60mhz(&self) -> bool {
        *self == SEL_UART3_A::USB60MHZ
    }
}
#[doc = "Write proxy for field `SEL_UART3`"]
pub struct SEL_UART3_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL_UART3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEL_UART3_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "system clock"]
    #[inline(always)]
    pub fn sys_clk(self) -> &'a mut W {
        self.variant(SEL_UART3_A::SYSCLK)
    }
    #[doc = "external oscillator clock (XI_OSC)"]
    #[inline(always)]
    pub fn ext_osc(self) -> &'a mut W {
        self.variant(SEL_UART3_A::EXTOSC)
    }
    #[doc = "USB clock from GPIO A0"]
    #[inline(always)]
    pub fn usbclk(self) -> &'a mut W {
        self.variant(SEL_UART3_A::USBCLK)
    }
    #[doc = "USB clock 60 MHz"]
    #[inline(always)]
    pub fn usb60mhz(self) -> &'a mut W {
        self.variant(SEL_UART3_A::USB60MHZ)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Select source clk SPI0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEL_SPI0_A {
    #[doc = "0: system clock"]
    SYSCLK,
    #[doc = "1: external oscillator clock (XI_OSC)"]
    EXTOSC,
    #[doc = "2: USB clock from GPIO A0"]
    USBCLK,
    #[doc = "3: USB clock 60 MHz"]
    USB60MHZ,
}
impl From<SEL_SPI0_A> for u8 {
    #[inline(always)]
    fn from(variant: SEL_SPI0_A) -> Self {
        match variant {
            SEL_SPI0_A::SYSCLK => 0,
            SEL_SPI0_A::EXTOSC => 1,
            SEL_SPI0_A::USBCLK => 2,
            SEL_SPI0_A::USB60MHZ => 3,
        }
    }
}
#[doc = "Reader of field `SEL_SPI0`"]
pub type SEL_SPI0_R = crate::R<u8, SEL_SPI0_A>;
impl SEL_SPI0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEL_SPI0_A {
        match self.bits {
            0 => SEL_SPI0_A::SYSCLK,
            1 => SEL_SPI0_A::EXTOSC,
            2 => SEL_SPI0_A::USBCLK,
            3 => SEL_SPI0_A::USB60MHZ,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SYSCLK`"]
    #[inline(always)]
    pub fn is_sys_clk(&self) -> bool {
        *self == SEL_SPI0_A::SYSCLK
    }
    #[doc = "Checks if the value of the field is `EXTOSC`"]
    #[inline(always)]
    pub fn is_ext_osc(&self) -> bool {
        *self == SEL_SPI0_A::EXTOSC
    }
    #[doc = "Checks if the value of the field is `USBCLK`"]
    #[inline(always)]
    pub fn is_usbclk(&self) -> bool {
        *self == SEL_SPI0_A::USBCLK
    }
    #[doc = "Checks if the value of the field is `USB60MHZ`"]
    #[inline(always)]
    pub fn is_usb60mhz(&self) -> bool {
        *self == SEL_SPI0_A::USB60MHZ
    }
}
#[doc = "Write proxy for field `SEL_SPI0`"]
pub struct SEL_SPI0_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL_SPI0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEL_SPI0_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "system clock"]
    #[inline(always)]
    pub fn sys_clk(self) -> &'a mut W {
        self.variant(SEL_SPI0_A::SYSCLK)
    }
    #[doc = "external oscillator clock (XI_OSC)"]
    #[inline(always)]
    pub fn ext_osc(self) -> &'a mut W {
        self.variant(SEL_SPI0_A::EXTOSC)
    }
    #[doc = "USB clock from GPIO A0"]
    #[inline(always)]
    pub fn usbclk(self) -> &'a mut W {
        self.variant(SEL_SPI0_A::USBCLK)
    }
    #[doc = "USB clock 60 MHz"]
    #[inline(always)]
    pub fn usb60mhz(self) -> &'a mut W {
        self.variant(SEL_SPI0_A::USB60MHZ)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Select source clk SPI1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEL_SPI1_A {
    #[doc = "0: system clock"]
    SYSCLK,
    #[doc = "1: external oscillator clock (XI_OSC)"]
    EXTOSC,
    #[doc = "2: USB clock from GPIO A0"]
    USBCLK,
    #[doc = "3: USB clock 60 MHz"]
    USB60MHZ,
}
impl From<SEL_SPI1_A> for u8 {
    #[inline(always)]
    fn from(variant: SEL_SPI1_A) -> Self {
        match variant {
            SEL_SPI1_A::SYSCLK => 0,
            SEL_SPI1_A::EXTOSC => 1,
            SEL_SPI1_A::USBCLK => 2,
            SEL_SPI1_A::USB60MHZ => 3,
        }
    }
}
#[doc = "Reader of field `SEL_SPI1`"]
pub type SEL_SPI1_R = crate::R<u8, SEL_SPI1_A>;
impl SEL_SPI1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEL_SPI1_A {
        match self.bits {
            0 => SEL_SPI1_A::SYSCLK,
            1 => SEL_SPI1_A::EXTOSC,
            2 => SEL_SPI1_A::USBCLK,
            3 => SEL_SPI1_A::USB60MHZ,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SYSCLK`"]
    #[inline(always)]
    pub fn is_sys_clk(&self) -> bool {
        *self == SEL_SPI1_A::SYSCLK
    }
    #[doc = "Checks if the value of the field is `EXTOSC`"]
    #[inline(always)]
    pub fn is_ext_osc(&self) -> bool {
        *self == SEL_SPI1_A::EXTOSC
    }
    #[doc = "Checks if the value of the field is `USBCLK`"]
    #[inline(always)]
    pub fn is_usbclk(&self) -> bool {
        *self == SEL_SPI1_A::USBCLK
    }
    #[doc = "Checks if the value of the field is `USB60MHZ`"]
    #[inline(always)]
    pub fn is_usb60mhz(&self) -> bool {
        *self == SEL_SPI1_A::USB60MHZ
    }
}
#[doc = "Write proxy for field `SEL_SPI1`"]
pub struct SEL_SPI1_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL_SPI1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEL_SPI1_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "system clock"]
    #[inline(always)]
    pub fn sys_clk(self) -> &'a mut W {
        self.variant(SEL_SPI1_A::SYSCLK)
    }
    #[doc = "external oscillator clock (XI_OSC)"]
    #[inline(always)]
    pub fn ext_osc(self) -> &'a mut W {
        self.variant(SEL_SPI1_A::EXTOSC)
    }
    #[doc = "USB clock from GPIO A0"]
    #[inline(always)]
    pub fn usbclk(self) -> &'a mut W {
        self.variant(SEL_SPI1_A::USBCLK)
    }
    #[doc = "USB clock 60 MHz"]
    #[inline(always)]
    pub fn usb60mhz(self) -> &'a mut W {
        self.variant(SEL_SPI1_A::USB60MHZ)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Select source clk SPI2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEL_SPI2_A {
    #[doc = "0: system clock"]
    SYSCLK,
    #[doc = "1: external oscillator clock (XI_OSC)"]
    EXTOSC,
    #[doc = "2: USB clock from GPIO A0"]
    USBCLK,
    #[doc = "3: USB clock 60 MHz"]
    USB60MHZ,
}
impl From<SEL_SPI2_A> for u8 {
    #[inline(always)]
    fn from(variant: SEL_SPI2_A) -> Self {
        match variant {
            SEL_SPI2_A::SYSCLK => 0,
            SEL_SPI2_A::EXTOSC => 1,
            SEL_SPI2_A::USBCLK => 2,
            SEL_SPI2_A::USB60MHZ => 3,
        }
    }
}
#[doc = "Reader of field `SEL_SPI2`"]
pub type SEL_SPI2_R = crate::R<u8, SEL_SPI2_A>;
impl SEL_SPI2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEL_SPI2_A {
        match self.bits {
            0 => SEL_SPI2_A::SYSCLK,
            1 => SEL_SPI2_A::EXTOSC,
            2 => SEL_SPI2_A::USBCLK,
            3 => SEL_SPI2_A::USB60MHZ,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SYSCLK`"]
    #[inline(always)]
    pub fn is_sys_clk(&self) -> bool {
        *self == SEL_SPI2_A::SYSCLK
    }
    #[doc = "Checks if the value of the field is `EXTOSC`"]
    #[inline(always)]
    pub fn is_ext_osc(&self) -> bool {
        *self == SEL_SPI2_A::EXTOSC
    }
    #[doc = "Checks if the value of the field is `USBCLK`"]
    #[inline(always)]
    pub fn is_usbclk(&self) -> bool {
        *self == SEL_SPI2_A::USBCLK
    }
    #[doc = "Checks if the value of the field is `USB60MHZ`"]
    #[inline(always)]
    pub fn is_usb60mhz(&self) -> bool {
        *self == SEL_SPI2_A::USB60MHZ
    }
}
#[doc = "Write proxy for field `SEL_SPI2`"]
pub struct SEL_SPI2_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL_SPI2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEL_SPI2_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "system clock"]
    #[inline(always)]
    pub fn sys_clk(self) -> &'a mut W {
        self.variant(SEL_SPI2_A::SYSCLK)
    }
    #[doc = "external oscillator clock (XI_OSC)"]
    #[inline(always)]
    pub fn ext_osc(self) -> &'a mut W {
        self.variant(SEL_SPI2_A::EXTOSC)
    }
    #[doc = "USB clock from GPIO A0"]
    #[inline(always)]
    pub fn usbclk(self) -> &'a mut W {
        self.variant(SEL_SPI2_A::USBCLK)
    }
    #[doc = "USB clock 60 MHz"]
    #[inline(always)]
    pub fn usb60mhz(self) -> &'a mut W {
        self.variant(SEL_SPI2_A::USB60MHZ)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Select source clk SPI3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEL_SPI3_A {
    #[doc = "0: system clock"]
    SYSCLK,
    #[doc = "1: external oscillator clock (XI_OSC)"]
    EXTOSC,
    #[doc = "2: USB clock from GPIO A0"]
    USBCLK,
    #[doc = "3: USB clock 60 MHz"]
    USB60MHZ,
}
impl From<SEL_SPI3_A> for u8 {
    #[inline(always)]
    fn from(variant: SEL_SPI3_A) -> Self {
        match variant {
            SEL_SPI3_A::SYSCLK => 0,
            SEL_SPI3_A::EXTOSC => 1,
            SEL_SPI3_A::USBCLK => 2,
            SEL_SPI3_A::USB60MHZ => 3,
        }
    }
}
#[doc = "Reader of field `SEL_SPI3`"]
pub type SEL_SPI3_R = crate::R<u8, SEL_SPI3_A>;
impl SEL_SPI3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEL_SPI3_A {
        match self.bits {
            0 => SEL_SPI3_A::SYSCLK,
            1 => SEL_SPI3_A::EXTOSC,
            2 => SEL_SPI3_A::USBCLK,
            3 => SEL_SPI3_A::USB60MHZ,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SYSCLK`"]
    #[inline(always)]
    pub fn is_sys_clk(&self) -> bool {
        *self == SEL_SPI3_A::SYSCLK
    }
    #[doc = "Checks if the value of the field is `EXTOSC`"]
    #[inline(always)]
    pub fn is_ext_osc(&self) -> bool {
        *self == SEL_SPI3_A::EXTOSC
    }
    #[doc = "Checks if the value of the field is `USBCLK`"]
    #[inline(always)]
    pub fn is_usbclk(&self) -> bool {
        *self == SEL_SPI3_A::USBCLK
    }
    #[doc = "Checks if the value of the field is `USB60MHZ`"]
    #[inline(always)]
    pub fn is_usb60mhz(&self) -> bool {
        *self == SEL_SPI3_A::USB60MHZ
    }
}
#[doc = "Write proxy for field `SEL_SPI3`"]
pub struct SEL_SPI3_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL_SPI3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEL_SPI3_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "system clock"]
    #[inline(always)]
    pub fn sys_clk(self) -> &'a mut W {
        self.variant(SEL_SPI3_A::SYSCLK)
    }
    #[doc = "external oscillator clock (XI_OSC)"]
    #[inline(always)]
    pub fn ext_osc(self) -> &'a mut W {
        self.variant(SEL_SPI3_A::EXTOSC)
    }
    #[doc = "USB clock from GPIO A0"]
    #[inline(always)]
    pub fn usbclk(self) -> &'a mut W {
        self.variant(SEL_SPI3_A::USBCLK)
    }
    #[doc = "USB clock 60 MHz"]
    #[inline(always)]
    pub fn usb60mhz(self) -> &'a mut W {
        self.variant(SEL_SPI3_A::USB60MHZ)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Select source clk UART0"]
    #[inline(always)]
    pub fn sel_uart0(&self) -> SEL_UART0_R {
        SEL_UART0_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Select source clk UART1"]
    #[inline(always)]
    pub fn sel_uart1(&self) -> SEL_UART1_R {
        SEL_UART1_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Select source clk UART2"]
    #[inline(always)]
    pub fn sel_uart2(&self) -> SEL_UART2_R {
        SEL_UART2_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Select source clk UART3"]
    #[inline(always)]
    pub fn sel_uart3(&self) -> SEL_UART3_R {
        SEL_UART3_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Select source clk SPI0"]
    #[inline(always)]
    pub fn sel_spi0(&self) -> SEL_SPI0_R {
        SEL_SPI0_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Select source clk SPI1"]
    #[inline(always)]
    pub fn sel_spi1(&self) -> SEL_SPI1_R {
        SEL_SPI1_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Select source clk SPI2"]
    #[inline(always)]
    pub fn sel_spi2(&self) -> SEL_SPI2_R {
        SEL_SPI2_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Select source clk SPI3"]
    #[inline(always)]
    pub fn sel_spi3(&self) -> SEL_SPI3_R {
        SEL_SPI3_R::new(((self.bits >> 14) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Select source clk UART0"]
    #[inline(always)]
    pub fn sel_uart0(&mut self) -> SEL_UART0_W {
        SEL_UART0_W { w: self }
    }
    #[doc = "Bits 2:3 - Select source clk UART1"]
    #[inline(always)]
    pub fn sel_uart1(&mut self) -> SEL_UART1_W {
        SEL_UART1_W { w: self }
    }
    #[doc = "Bits 4:5 - Select source clk UART2"]
    #[inline(always)]
    pub fn sel_uart2(&mut self) -> SEL_UART2_W {
        SEL_UART2_W { w: self }
    }
    #[doc = "Bits 6:7 - Select source clk UART3"]
    #[inline(always)]
    pub fn sel_uart3(&mut self) -> SEL_UART3_W {
        SEL_UART3_W { w: self }
    }
    #[doc = "Bits 8:9 - Select source clk SPI0"]
    #[inline(always)]
    pub fn sel_spi0(&mut self) -> SEL_SPI0_W {
        SEL_SPI0_W { w: self }
    }
    #[doc = "Bits 10:11 - Select source clk SPI1"]
    #[inline(always)]
    pub fn sel_spi1(&mut self) -> SEL_SPI1_W {
        SEL_SPI1_W { w: self }
    }
    #[doc = "Bits 12:13 - Select source clk SPI2"]
    #[inline(always)]
    pub fn sel_spi2(&mut self) -> SEL_SPI2_W {
        SEL_SPI2_W { w: self }
    }
    #[doc = "Bits 14:15 - Select source clk SPI3"]
    #[inline(always)]
    pub fn sel_spi3(&mut self) -> SEL_SPI3_W {
        SEL_SPI3_W { w: self }
    }
}
