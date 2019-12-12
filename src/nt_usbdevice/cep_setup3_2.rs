#[doc = "Reader of register CEP_SETUP3_2"]
pub type R = crate::R<u32, super::CEP_SETUP3_2>;
#[doc = "Reader of field `SETUPPKT_2B`"]
pub type SETUPPKT_2B_R = crate::R<u8, u8>;
#[doc = "Reader of field `SETUPPKT_3B`"]
pub type SETUPPKT_3B_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Low byte field wValue"]
    #[inline(always)]
    pub fn setuppkt_2b(&self) -> SETUPPKT_2B_R {
        SETUPPKT_2B_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - High byte field wValue"]
    #[inline(always)]
    pub fn setuppkt_3b(&self) -> SETUPPKT_3B_R {
        SETUPPKT_3B_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
