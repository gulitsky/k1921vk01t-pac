#[doc = "Reader of register VALUE"]
pub type R = crate::R<u32, super::VALUE>;
#[doc = "Reader of field `WDTVAL`"]
pub type WDTVAL_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn wdtval(&self) -> WDTVAL_R {
        WDTVAL_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
