#[doc = "Reader of register CNT"]
pub type R = crate::R<u32, super::CNT>;
#[doc = "Writer for register CNT"]
pub type W = crate::W<u32, super::CNT>;
#[doc = "Register CNT `reset()`'s with value 0"]
impl crate::ResetValue for super::CNT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EP_TFR_CNT`"]
pub type EP_TFR_CNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EP_TFR_CNT`"]
pub struct EP_TFR_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_TFR_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn ep_tfr_cnt(&self) -> EP_TFR_CNT_R {
        EP_TFR_CNT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn ep_tfr_cnt(&mut self) -> EP_TFR_CNT_W {
        EP_TFR_CNT_W { w: self }
    }
}
