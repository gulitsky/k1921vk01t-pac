#[doc = "Reader of register ALT_CTRL_BASE_PTR"]
pub type R = crate::R<u32, super::ALT_CTRL_BASE_PTR>;
#[doc = "Reader of field `ALT_CTRL_BASE_PTR`"]
pub type ALT_CTRL_BASE_PTR_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Base address of alternative control data channels"]
    #[inline(always)]
    pub fn alt_ctrl_base_ptr(&self) -> ALT_CTRL_BASE_PTR_R {
        ALT_CTRL_BASE_PTR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
