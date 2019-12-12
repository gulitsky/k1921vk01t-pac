#[doc = "Reader of register PLL_NF"]
pub type R = crate::R<u32, super::PLL_NF>;
#[doc = "Writer for register PLL_NF"]
pub type W = crate::W<u32, super::PLL_NF>;
#[doc = "Register PLL_NF `reset()`'s with value 0"]
impl crate::ResetValue for super::PLL_NF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `F_PLL`"]
pub type F_PLL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `F_PLL`"]
pub struct F_PLL_W<'a> {
    w: &'a mut W,
}
impl<'a> F_PLL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - PLL feedback divider \"NF = F_PLL+2\""]
    #[inline(always)]
    pub fn f_pll(&self) -> F_PLL_R {
        F_PLL_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - PLL feedback divider \"NF = F_PLL+2\""]
    #[inline(always)]
    pub fn f_pll(&mut self) -> F_PLL_W {
        F_PLL_W { w: self }
    }
}
