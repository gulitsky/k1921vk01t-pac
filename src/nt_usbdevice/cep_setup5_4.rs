#[doc = "Reader of register CEP_SETUP5_4"]
pub type R = crate::R<u32, super::CEP_SETUP5_4>;
#[doc = "Reader of field `SETUPPKT_4B`"]
pub type SETUPPKT_4B_R = crate::R<u8, u8>;
#[doc = "Reader of field `SETUPPKT_5B`"]
pub type SETUPPKT_5B_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Low byte field windex"]
    #[inline(always)]
    pub fn setuppkt_4b(&self) -> SETUPPKT_4B_R {
        SETUPPKT_4B_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - High byte field windex"]
    #[inline(always)]
    pub fn setuppkt_5b(&self) -> SETUPPKT_5B_R {
        SETUPPKT_5B_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
