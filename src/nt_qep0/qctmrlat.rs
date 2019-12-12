#[doc = "Reader of register QCTMRLAT"]
pub type R = crate::R<u32, super::QCTMRLAT>;
#[doc = "Reader of field `QCTMRLAT`"]
pub type QCTMRLAT_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn qctmrlat(&self) -> QCTMRLAT_R {
        QCTMRLAT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
