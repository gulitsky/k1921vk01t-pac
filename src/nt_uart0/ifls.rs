#[doc = "Reader of register IFLS"]
pub type R = crate::R<u32, super::IFLS>;
#[doc = "Writer for register IFLS"]
pub type W = crate::W<u32, super::IFLS>;
#[doc = "Register IFLS `reset()`'s with value 0"]
impl crate::ResetValue for super::IFLS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXIFLSEL_A {
    #[doc = "0: interrupt on 1/8"]
    LVL_1_8,
    #[doc = "1: interrupt on 1/4"]
    LVL_1_4,
    #[doc = "2: interrupt on 1/2"]
    LVL_1_2,
    #[doc = "3: interrupt on 3/4"]
    LVL_3_4,
    #[doc = "4: interrupt on 7/8"]
    LVL_7_8,
}
impl From<TXIFLSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TXIFLSEL_A) -> Self {
        match variant {
            TXIFLSEL_A::LVL_1_8 => 0,
            TXIFLSEL_A::LVL_1_4 => 1,
            TXIFLSEL_A::LVL_1_2 => 2,
            TXIFLSEL_A::LVL_3_4 => 3,
            TXIFLSEL_A::LVL_7_8 => 4,
        }
    }
}
#[doc = "Reader of field `TXIFLSEL`"]
pub type TXIFLSEL_R = crate::R<u8, TXIFLSEL_A>;
impl TXIFLSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TXIFLSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TXIFLSEL_A::LVL_1_8),
            1 => Val(TXIFLSEL_A::LVL_1_4),
            2 => Val(TXIFLSEL_A::LVL_1_2),
            3 => Val(TXIFLSEL_A::LVL_3_4),
            4 => Val(TXIFLSEL_A::LVL_7_8),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LVL_1_8`"]
    #[inline(always)]
    pub fn is_lvl_1_8(&self) -> bool {
        *self == TXIFLSEL_A::LVL_1_8
    }
    #[doc = "Checks if the value of the field is `LVL_1_4`"]
    #[inline(always)]
    pub fn is_lvl_1_4(&self) -> bool {
        *self == TXIFLSEL_A::LVL_1_4
    }
    #[doc = "Checks if the value of the field is `LVL_1_2`"]
    #[inline(always)]
    pub fn is_lvl_1_2(&self) -> bool {
        *self == TXIFLSEL_A::LVL_1_2
    }
    #[doc = "Checks if the value of the field is `LVL_3_4`"]
    #[inline(always)]
    pub fn is_lvl_3_4(&self) -> bool {
        *self == TXIFLSEL_A::LVL_3_4
    }
    #[doc = "Checks if the value of the field is `LVL_7_8`"]
    #[inline(always)]
    pub fn is_lvl_7_8(&self) -> bool {
        *self == TXIFLSEL_A::LVL_7_8
    }
}
#[doc = "Write proxy for field `TXIFLSEL`"]
pub struct TXIFLSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TXIFLSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXIFLSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "interrupt on 1/8"]
    #[inline(always)]
    pub fn lvl_1_8(self) -> &'a mut W {
        self.variant(TXIFLSEL_A::LVL_1_8)
    }
    #[doc = "interrupt on 1/4"]
    #[inline(always)]
    pub fn lvl_1_4(self) -> &'a mut W {
        self.variant(TXIFLSEL_A::LVL_1_4)
    }
    #[doc = "interrupt on 1/2"]
    #[inline(always)]
    pub fn lvl_1_2(self) -> &'a mut W {
        self.variant(TXIFLSEL_A::LVL_1_2)
    }
    #[doc = "interrupt on 3/4"]
    #[inline(always)]
    pub fn lvl_3_4(self) -> &'a mut W {
        self.variant(TXIFLSEL_A::LVL_3_4)
    }
    #[doc = "interrupt on 7/8"]
    #[inline(always)]
    pub fn lvl_7_8(self) -> &'a mut W {
        self.variant(TXIFLSEL_A::LVL_7_8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXIFLSEL_A {
    #[doc = "0: interrupt on 1/8"]
    LVL_1_8,
    #[doc = "1: interrupt on 1/4"]
    LVL_1_4,
    #[doc = "2: interrupt on 1/2"]
    LVL_1_2,
    #[doc = "3: interrupt on 3/4"]
    LVL_3_4,
    #[doc = "4: interrupt on 7/8"]
    LVL_7_8,
}
impl From<RXIFLSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: RXIFLSEL_A) -> Self {
        match variant {
            RXIFLSEL_A::LVL_1_8 => 0,
            RXIFLSEL_A::LVL_1_4 => 1,
            RXIFLSEL_A::LVL_1_2 => 2,
            RXIFLSEL_A::LVL_3_4 => 3,
            RXIFLSEL_A::LVL_7_8 => 4,
        }
    }
}
#[doc = "Reader of field `RXIFLSEL`"]
pub type RXIFLSEL_R = crate::R<u8, RXIFLSEL_A>;
impl RXIFLSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RXIFLSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RXIFLSEL_A::LVL_1_8),
            1 => Val(RXIFLSEL_A::LVL_1_4),
            2 => Val(RXIFLSEL_A::LVL_1_2),
            3 => Val(RXIFLSEL_A::LVL_3_4),
            4 => Val(RXIFLSEL_A::LVL_7_8),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LVL_1_8`"]
    #[inline(always)]
    pub fn is_lvl_1_8(&self) -> bool {
        *self == RXIFLSEL_A::LVL_1_8
    }
    #[doc = "Checks if the value of the field is `LVL_1_4`"]
    #[inline(always)]
    pub fn is_lvl_1_4(&self) -> bool {
        *self == RXIFLSEL_A::LVL_1_4
    }
    #[doc = "Checks if the value of the field is `LVL_1_2`"]
    #[inline(always)]
    pub fn is_lvl_1_2(&self) -> bool {
        *self == RXIFLSEL_A::LVL_1_2
    }
    #[doc = "Checks if the value of the field is `LVL_3_4`"]
    #[inline(always)]
    pub fn is_lvl_3_4(&self) -> bool {
        *self == RXIFLSEL_A::LVL_3_4
    }
    #[doc = "Checks if the value of the field is `LVL_7_8`"]
    #[inline(always)]
    pub fn is_lvl_7_8(&self) -> bool {
        *self == RXIFLSEL_A::LVL_7_8
    }
}
#[doc = "Write proxy for field `RXIFLSEL`"]
pub struct RXIFLSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RXIFLSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXIFLSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "interrupt on 1/8"]
    #[inline(always)]
    pub fn lvl_1_8(self) -> &'a mut W {
        self.variant(RXIFLSEL_A::LVL_1_8)
    }
    #[doc = "interrupt on 1/4"]
    #[inline(always)]
    pub fn lvl_1_4(self) -> &'a mut W {
        self.variant(RXIFLSEL_A::LVL_1_4)
    }
    #[doc = "interrupt on 1/2"]
    #[inline(always)]
    pub fn lvl_1_2(self) -> &'a mut W {
        self.variant(RXIFLSEL_A::LVL_1_2)
    }
    #[doc = "interrupt on 3/4"]
    #[inline(always)]
    pub fn lvl_3_4(self) -> &'a mut W {
        self.variant(RXIFLSEL_A::LVL_3_4)
    }
    #[doc = "interrupt on 7/8"]
    #[inline(always)]
    pub fn lvl_7_8(self) -> &'a mut W {
        self.variant(RXIFLSEL_A::LVL_7_8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | (((value as u32) & 0x07) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn txiflsel(&self) -> TXIFLSEL_R {
        TXIFLSEL_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:5"]
    #[inline(always)]
    pub fn rxiflsel(&self) -> RXIFLSEL_R {
        RXIFLSEL_R::new(((self.bits >> 3) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn txiflsel(&mut self) -> TXIFLSEL_W {
        TXIFLSEL_W { w: self }
    }
    #[doc = "Bits 3:5"]
    #[inline(always)]
    pub fn rxiflsel(&mut self) -> RXIFLSEL_W {
        RXIFLSEL_W { w: self }
    }
}
