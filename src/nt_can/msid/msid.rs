#[doc = "Reader of register MSID"]
pub type R = crate::R<u32, super::MSID>;
#[doc = "Reader of field `INDEX`"]
pub type INDEX_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn index(&self) -> INDEX_R {
        INDEX_R::new((self.bits & 0xff) as u8)
    }
}
