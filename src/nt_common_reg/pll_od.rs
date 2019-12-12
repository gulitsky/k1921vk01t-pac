#[doc = "Reader of register PLL_OD"]
pub type R = crate::R<u32, super::PLL_OD>;
#[doc = "Writer for register PLL_OD"]
pub type W = crate::W<u32, super::PLL_OD>;
#[doc = "Register PLL_OD `reset()`'s with value 0"]
impl crate::ResetValue for super::PLL_OD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Output divider \"NO\" control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLL_OD_A {
    #[doc = "0: output PLL divider disabled"]
    DISABLE,
    #[doc = "1: output PLL divider by 2 enabled"]
    DIV2,
    #[doc = "3: output PLL divider by 4 enabled"]
    DIV4,
}
impl From<PLL_OD_A> for u8 {
    #[inline(always)]
    fn from(variant: PLL_OD_A) -> Self {
        match variant {
            PLL_OD_A::DISABLE => 0,
            PLL_OD_A::DIV2 => 1,
            PLL_OD_A::DIV4 => 3,
        }
    }
}
#[doc = "Reader of field `PLL_OD`"]
pub type PLL_OD_R = crate::R<u8, PLL_OD_A>;
impl PLL_OD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PLL_OD_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PLL_OD_A::DISABLE),
            1 => Val(PLL_OD_A::DIV2),
            3 => Val(PLL_OD_A::DIV4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PLL_OD_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PLL_OD_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PLL_OD_A::DIV4
    }
}
#[doc = "Write proxy for field `PLL_OD`"]
pub struct PLL_OD_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_OD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLL_OD_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "output PLL divider disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PLL_OD_A::DISABLE)
    }
    #[doc = "output PLL divider by 2 enabled"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PLL_OD_A::DIV2)
    }
    #[doc = "output PLL divider by 4 enabled"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PLL_OD_A::DIV4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Output divider \"NO\" control"]
    #[inline(always)]
    pub fn pll_od(&self) -> PLL_OD_R {
        PLL_OD_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Output divider \"NO\" control"]
    #[inline(always)]
    pub fn pll_od(&mut self) -> PLL_OD_W {
        PLL_OD_W { w: self }
    }
}
