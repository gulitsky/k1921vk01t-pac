#[doc = "Reader of register IPGT"]
pub type R = crate::R<u32, super::IPGT>;
#[doc = "Writer for register IPGT"]
pub type W = crate::W<u32, super::IPGT>;
#[doc = "Register IPGT `reset()`'s with value 0"]
impl crate::ResetValue for super::IPGT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BTBIPG_TRANS`"]
pub type BTBIPG_TRANS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BTBIPG_TRANS`"]
pub struct BTBIPG_TRANS_W<'a> {
    w: &'a mut W,
}
impl<'a> BTBIPG_TRANS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn btbipg_trans(&self) -> BTBIPG_TRANS_R {
        BTBIPG_TRANS_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn btbipg_trans(&mut self) -> BTBIPG_TRANS_W {
        BTBIPG_TRANS_W { w: self }
    }
}
