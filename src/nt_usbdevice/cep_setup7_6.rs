#[doc = "Reader of register CEP_SETUP7_6"]
pub type R = crate::R<u32, super::CEP_SETUP7_6>;
#[doc = "Reader of field `SETUPPKT_6B`"]
pub type SETUPPKT_6B_R = crate::R<u8, u8>;
#[doc = "Reader of field `SETUPPKT_7B`"]
pub type SETUPPKT_7B_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Low byte field wLength"]
    #[inline(always)]
    pub fn setuppkt_6b(&self) -> SETUPPKT_6B_R {
        SETUPPKT_6B_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - High byte field wLength"]
    #[inline(always)]
    pub fn setuppkt_7b(&self) -> SETUPPKT_7B_R {
        SETUPPKT_7B_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
