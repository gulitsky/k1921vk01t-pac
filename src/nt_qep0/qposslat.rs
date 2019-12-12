#[doc = "Reader of register QPOSSLAT"]
pub type R = crate::R<u32, super::QPOSSLAT>;
#[doc = "Reader of field `QPOSSLAT`"]
pub type QPOSSLAT_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn qposslat(&self) -> QPOSSLAT_R {
        QPOSSLAT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
