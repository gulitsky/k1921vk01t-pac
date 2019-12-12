#[doc = "Reader of register MRDD"]
pub type R = crate::R<u32, super::MRDD>;
#[doc = "Writer for register MRDD"]
pub type W = crate::W<u32, super::MRDD>;
#[doc = "Register MRDD `reset()`'s with value 0"]
impl crate::ResetValue for super::MRDD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `READ_DATA`"]
pub type READ_DATA_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn read_data(&self) -> READ_DATA_R {
        READ_DATA_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {}
