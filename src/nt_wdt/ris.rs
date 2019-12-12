#[doc = "Reader of register RIS"]
pub type R = crate::R<u32, super::RIS>;
#[doc = "Reader of field `RAWWDTINT`"]
pub type RAWWDTINT_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Raw interrupt status from the counter"]
    #[inline(always)]
    pub fn rawwdtint(&self) -> RAWWDTINT_R {
        RAWWDTINT_R::new((self.bits & 0x01) != 0)
    }
}
