#[doc = "Reader of register SLAVE_IN_COUNT"]
pub type R = crate::R<u32, super::SLAVE_IN_COUNT>;
#[doc = "Reader of field `BYTECOUNT`"]
pub type BYTECOUNT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:10 - Indication of the number of bytes received from the device when prompted interruption 'Data ready'"]
    #[inline(always)]
    pub fn bytecount(&self) -> BYTECOUNT_R {
        BYTECOUNT_R::new((self.bits & 0x07ff) as u16)
    }
}
