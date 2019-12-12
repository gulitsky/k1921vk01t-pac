#[doc = "Reader of register QPOSILAT"]
pub type R = crate::R<u32, super::QPOSILAT>;
#[doc = "Reader of field `QPOSILAT`"]
pub type QPOSILAT_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn qposilat(&self) -> QPOSILAT_R {
        QPOSILAT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
