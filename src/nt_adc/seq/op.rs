#[doc = "Reader of register OP"]
pub type R = crate::R<u32, super::OP>;
#[doc = "Writer for register OP"]
pub type W = crate::W<u32, super::OP>;
#[doc = "Register OP `reset()`'s with value 0"]
impl crate::ResetValue for super::OP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RCNT`"]
pub type RCNT_R = crate::R<u8, u8>;
#[doc = "Reader of field `ICNT`"]
pub type ICNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ICNT`"]
pub struct ICNT_W<'a> {
    w: &'a mut W,
}
impl<'a> ICNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Current number of ADC restarts by seqensor"]
    #[inline(always)]
    pub fn rcnt(&self) -> RCNT_R {
        RCNT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Current number of ADC restarts for interrupt generation"]
    #[inline(always)]
    pub fn icnt(&self) -> ICNT_R {
        ICNT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:23 - Current number of ADC restarts for interrupt generation"]
    #[inline(always)]
    pub fn icnt(&mut self) -> ICNT_W {
        ICNT_W { w: self }
    }
}
