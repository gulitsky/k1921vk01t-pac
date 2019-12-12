#[doc = "Reader of register CEP_OUT_XFRCNT"]
pub type R = crate::R<u32, super::CEP_OUT_XFRCNT>;
#[doc = "Reader of field `DATACOUNT`"]
pub type DATACOUNT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - The number of bytes received of data"]
    #[inline(always)]
    pub fn datacount(&self) -> DATACOUNT_R {
        DATACOUNT_R::new((self.bits & 0xffff) as u16)
    }
}
