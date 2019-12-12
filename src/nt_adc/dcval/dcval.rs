#[doc = "Reader of register DCVAL"]
pub type R = crate::R<u32, super::DCVAL>;
#[doc = "Reader of field `VAL`"]
pub type VAL_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:11 - Value of last compared AD conversion result"]
    #[inline(always)]
    pub fn val(&self) -> VAL_R {
        VAL_R::new((self.bits & 0x0fff) as u16)
    }
}
