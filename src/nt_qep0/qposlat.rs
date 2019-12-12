#[doc = "Reader of register QPOSLAT"]
pub type R = crate::R<u32, super::QPOSLAT>;
#[doc = "Reader of field `QPOSLAT`"]
pub type QPOSLAT_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn qposlat(&self) -> QPOSLAT_R {
        QPOSLAT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
