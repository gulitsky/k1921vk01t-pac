#[doc = "Reader of register CEP_DATA_BUF"]
pub type R = crate::R<u32, super::CEP_DATA_BUF>;
#[doc = "Writer for register CEP_DATA_BUF"]
pub type W = crate::W<u32, super::CEP_DATA_BUF>;
#[doc = "Register CEP_DATA_BUF `reset()`'s with value 0"]
impl crate::ResetValue for super::CEP_DATA_BUF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DATA_BUF`"]
pub type DATA_BUF_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DATA_BUF`"]
pub struct DATA_BUF_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_BUF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Data buffer for transmission"]
    #[inline(always)]
    pub fn data_buf(&self) -> DATA_BUF_R {
        DATA_BUF_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Data buffer for transmission"]
    #[inline(always)]
    pub fn data_buf(&mut self) -> DATA_BUF_W {
        DATA_BUF_W { w: self }
    }
}
