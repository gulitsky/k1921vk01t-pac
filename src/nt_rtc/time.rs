#[doc = "Reader of register TIME"]
pub type R = crate::R<u32, super::TIME>;
#[doc = "Reader of field `SPLIT_SEC`"]
pub type SPLIT_SEC_R = crate::R<u16, u16>;
#[doc = "Reader of field `SECOND`"]
pub type SECOND_R = crate::R<u8, u8>;
#[doc = "Reader of field `MINUTE`"]
pub type MINUTE_R = crate::R<u8, u8>;
#[doc = "Reader of field `HOUR`"]
pub type HOUR_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn split_sec(&self) -> SPLIT_SEC_R {
        SPLIT_SEC_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:16"]
    #[inline(always)]
    pub fn second(&self) -> SECOND_R {
        SECOND_R::new(((self.bits >> 10) & 0x7f) as u8)
    }
    #[doc = "Bits 17:23"]
    #[inline(always)]
    pub fn minute(&self) -> MINUTE_R {
        MINUTE_R::new(((self.bits >> 17) & 0x7f) as u8)
    }
    #[doc = "Bits 24:29"]
    #[inline(always)]
    pub fn hour(&self) -> HOUR_R {
        HOUR_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
