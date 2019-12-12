#[doc = "Reader of register LIST"]
pub type R = crate::R<u32, super::LIST>;
#[doc = "Reader of field `BEGIN`"]
pub type BEGIN_R = crate::R<u8, u8>;
#[doc = "Reader of field `END`"]
pub type END_R = crate::R<u8, u8>;
#[doc = "Reader of field `SIZE`"]
pub type SIZE_R = crate::R<u8, u8>;
#[doc = "Reader of field `EMPTY`"]
pub type EMPTY_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn begin(&self) -> BEGIN_R {
        BEGIN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn end(&self) -> END_R {
        END_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - List size"]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Indicate empty list"]
    #[inline(always)]
    pub fn empty(&self) -> EMPTY_R {
        EMPTY_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
