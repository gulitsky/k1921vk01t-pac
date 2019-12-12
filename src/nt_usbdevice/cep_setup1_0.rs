#[doc = "Reader of register CEP_SETUP1_0"]
pub type R = crate::R<u32, super::CEP_SETUP1_0>;
#[doc = "Reader of field `SETUPPKT_0B`"]
pub type SETUPPKT_0B_R = crate::R<u8, u8>;
#[doc = "Reader of field `SETUPPKT_1B`"]
pub type SETUPPKT_1B_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - destination"]
    #[inline(always)]
    pub fn setuppkt_0b(&self) -> SETUPPKT_0B_R {
        SETUPPKT_0B_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - The first byte of the packet Setup"]
    #[inline(always)]
    pub fn setuppkt_1b(&self) -> SETUPPKT_1B_R {
        SETUPPKT_1B_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
