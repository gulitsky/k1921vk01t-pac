#[doc = "Reader of register STAT2"]
pub type R = crate::R<u32, super::STAT2>;
#[doc = "Reader of field `OVAL`"]
pub type OVAL_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 1 - output value of the comparator2"]
    #[inline(always)]
    pub fn oval(&self) -> OVAL_R {
        OVAL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
