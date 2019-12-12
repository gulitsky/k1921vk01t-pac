#[doc = "Reader of register PWM_SYNC"]
pub type R = crate::R<u32, super::PWM_SYNC>;
#[doc = "Writer for register PWM_SYNC"]
pub type W = crate::W<u32, super::PWM_SYNC>;
#[doc = "Register PWM_SYNC `reset()`'s with value 0"]
impl crate::ResetValue for super::PWM_SYNC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TBCLKSYNC`"]
pub type TBCLKSYNC_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TBCLKSYNC`"]
pub struct TBCLKSYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> TBCLKSYNC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - PWM prescalers enable"]
    #[inline(always)]
    pub fn tbclksync(&self) -> TBCLKSYNC_R {
        TBCLKSYNC_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - PWM prescalers enable"]
    #[inline(always)]
    pub fn tbclksync(&mut self) -> TBCLKSYNC_W {
        TBCLKSYNC_W { w: self }
    }
}
