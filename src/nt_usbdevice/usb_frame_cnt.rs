#[doc = "Reader of register USB_FRAME_CNT"]
pub type R = crate::R<u32, super::USB_FRAME_CNT>;
#[doc = "Reader of field `MICRO_FRAME_COUNTER`"]
pub type MICRO_FRAME_COUNTER_R = crate::R<u8, u8>;
#[doc = "Reader of field `FRAME_COUNTER`"]
pub type FRAME_COUNTER_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:2 - Number field current a chip"]
    #[inline(always)]
    pub fn micro_frame_counter(&self) -> MICRO_FRAME_COUNTER_R {
        MICRO_FRAME_COUNTER_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:13 - Field frame counter since last SOF packet"]
    #[inline(always)]
    pub fn frame_counter(&self) -> FRAME_COUNTER_R {
        FRAME_COUNTER_R::new(((self.bits >> 3) & 0x07ff) as u16)
    }
}
