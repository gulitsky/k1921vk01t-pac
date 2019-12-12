#[doc = "Reader of register MIS"]
pub type R = crate::R<u32, super::MIS>;
#[doc = "Reader of field `WDTINT`"]
pub type WDTINT_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Enabled interrupt status from the counter"]
    #[inline(always)]
    pub fn wdtint(&self) -> WDTINT_R {
        WDTINT_R::new((self.bits & 0x01) != 0)
    }
}
