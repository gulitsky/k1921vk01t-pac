#[doc = "Reader of register FCIS"]
pub type R = crate::R<u32, super::FCIS>;
#[doc = "Reader of field `OP_CMLT`"]
pub type OP_CMLT_R = crate::R<bool, bool>;
#[doc = "Reader of field `OP_ERROR`"]
pub type OP_ERROR_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Completion flag operation"]
    #[inline(always)]
    pub fn op_cmlt(&self) -> OP_CMLT_R {
        OP_CMLT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Flag write error, erase block write-protected or non-existent, or read from a non-existent block"]
    #[inline(always)]
    pub fn op_error(&self) -> OP_ERROR_R {
        OP_ERROR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
