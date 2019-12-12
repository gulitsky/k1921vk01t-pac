#[doc = "Reader of register FSTAT"]
pub type R = crate::R<u32, super::FSTAT>;
#[doc = "Reader of field `FLOAD`"]
pub type FLOAD_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:5 - Sequencer FIFO current load value"]
    #[inline(always)]
    pub fn fload(&self) -> FLOAD_R {
        FLOAD_R::new((self.bits & 0x3f) as u8)
    }
}
