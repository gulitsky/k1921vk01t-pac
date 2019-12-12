#[doc = "Reader of register HDFLG"]
pub type R = crate::R<u32, super::HDFLG>;
#[doc = "Reader of field `HDINT`"]
pub type HDINT_R = crate::R<bool, bool>;
#[doc = "Reader of field `CBC`"]
pub type CBC_R = crate::R<bool, bool>;
#[doc = "Reader of field `OST`"]
pub type OST_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn hdint(&self) -> HDINT_R {
        HDINT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Activation threshold trigger in cycle mode"]
    #[inline(always)]
    pub fn cbc(&self) -> CBC_R {
        CBC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Activation threshold trigger in single mode"]
    #[inline(always)]
    pub fn ost(&self) -> OST_R {
        OST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
