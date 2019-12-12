#[doc = "Reader of register AVAIL_CNT"]
pub type R = crate::R<u32, super::AVAIL_CNT>;
#[doc = "Reader of field `BUFFBYTENUM`"]
pub type BUFFBYTENUM_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:11 - Number of bytes in the buffer"]
    #[inline(always)]
    pub fn buffbytenum(&self) -> BUFFBYTENUM_R {
        BUFFBYTENUM_R::new((self.bits & 0x0fff) as u16)
    }
}
