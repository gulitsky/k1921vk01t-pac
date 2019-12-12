#[doc = "Reader of register PLL_NR"]
pub type R = crate::R<u32, super::PLL_NR>;
#[doc = "Writer for register PLL_NR"]
pub type W = crate::W<u32, super::PLL_NR>;
#[doc = "Register PLL_NR `reset()`'s with value 0"]
impl crate::ResetValue for super::PLL_NR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `R_PLL`"]
pub type R_PLL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `R_PLL`"]
pub struct R_PLL_W<'a> {
    w: &'a mut W,
}
impl<'a> R_PLL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - PLL reference divider \"NR = R_PLL+2\""]
    #[inline(always)]
    pub fn r_pll(&self) -> R_PLL_R {
        R_PLL_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - PLL reference divider \"NR = R_PLL+2\""]
    #[inline(always)]
    pub fn r_pll(&mut self) -> R_PLL_W {
        R_PLL_W { w: self }
    }
}
