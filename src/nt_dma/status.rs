#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Reader of field `MASTER_EN`"]
pub type MASTER_EN_R = crate::R<bool, bool>;
#[doc = "State of DMA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STATE_A {
    #[doc = "0: At rest"]
    FREE,
    #[doc = "1: Reading the config data structure"]
    READCONFIGDATA,
    #[doc = "2: Reading sourse data end pointer"]
    READSRCDATAENDPTR,
    #[doc = "3: Reading destination data end pointer"]
    READDSTDATAENDPTR,
    #[doc = "4: Reading source data"]
    READSRCDATA,
    #[doc = "5: Writing data to the destination"]
    WRIREDSTDATA,
    #[doc = "6: Waiting for a request"]
    WAITREQ,
    #[doc = "7: Write config structure of the channel"]
    WRITECONFIGDATA,
    #[doc = "8: Suspended"]
    PAUSE,
    #[doc = "9: Executed"]
    DONE,
    #[doc = "10: mode \"peripheral scather-gather\""]
    PERIPHSCATGATH,
}
impl From<STATE_A> for u8 {
    #[inline(always)]
    fn from(variant: STATE_A) -> Self {
        match variant {
            STATE_A::FREE => 0,
            STATE_A::READCONFIGDATA => 1,
            STATE_A::READSRCDATAENDPTR => 2,
            STATE_A::READDSTDATAENDPTR => 3,
            STATE_A::READSRCDATA => 4,
            STATE_A::WRIREDSTDATA => 5,
            STATE_A::WAITREQ => 6,
            STATE_A::WRITECONFIGDATA => 7,
            STATE_A::PAUSE => 8,
            STATE_A::DONE => 9,
            STATE_A::PERIPHSCATGATH => 10,
        }
    }
}
#[doc = "Reader of field `STATE`"]
pub type STATE_R = crate::R<u8, STATE_A>;
impl STATE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, STATE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(STATE_A::FREE),
            1 => Val(STATE_A::READCONFIGDATA),
            2 => Val(STATE_A::READSRCDATAENDPTR),
            3 => Val(STATE_A::READDSTDATAENDPTR),
            4 => Val(STATE_A::READSRCDATA),
            5 => Val(STATE_A::WRIREDSTDATA),
            6 => Val(STATE_A::WAITREQ),
            7 => Val(STATE_A::WRITECONFIGDATA),
            8 => Val(STATE_A::PAUSE),
            9 => Val(STATE_A::DONE),
            10 => Val(STATE_A::PERIPHSCATGATH),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `FREE`"]
    #[inline(always)]
    pub fn is_free(&self) -> bool {
        *self == STATE_A::FREE
    }
    #[doc = "Checks if the value of the field is `READCONFIGDATA`"]
    #[inline(always)]
    pub fn is_read_config_data(&self) -> bool {
        *self == STATE_A::READCONFIGDATA
    }
    #[doc = "Checks if the value of the field is `READSRCDATAENDPTR`"]
    #[inline(always)]
    pub fn is_read_src_data_end_ptr(&self) -> bool {
        *self == STATE_A::READSRCDATAENDPTR
    }
    #[doc = "Checks if the value of the field is `READDSTDATAENDPTR`"]
    #[inline(always)]
    pub fn is_read_dst_data_end_ptr(&self) -> bool {
        *self == STATE_A::READDSTDATAENDPTR
    }
    #[doc = "Checks if the value of the field is `READSRCDATA`"]
    #[inline(always)]
    pub fn is_read_src_data(&self) -> bool {
        *self == STATE_A::READSRCDATA
    }
    #[doc = "Checks if the value of the field is `WRIREDSTDATA`"]
    #[inline(always)]
    pub fn is_wrire_dst_data(&self) -> bool {
        *self == STATE_A::WRIREDSTDATA
    }
    #[doc = "Checks if the value of the field is `WAITREQ`"]
    #[inline(always)]
    pub fn is_wait_req(&self) -> bool {
        *self == STATE_A::WAITREQ
    }
    #[doc = "Checks if the value of the field is `WRITECONFIGDATA`"]
    #[inline(always)]
    pub fn is_write_config_data(&self) -> bool {
        *self == STATE_A::WRITECONFIGDATA
    }
    #[doc = "Checks if the value of the field is `PAUSE`"]
    #[inline(always)]
    pub fn is_pause(&self) -> bool {
        *self == STATE_A::PAUSE
    }
    #[doc = "Checks if the value of the field is `DONE`"]
    #[inline(always)]
    pub fn is_done(&self) -> bool {
        *self == STATE_A::DONE
    }
    #[doc = "Checks if the value of the field is `PERIPHSCATGATH`"]
    #[inline(always)]
    pub fn is_periph_scat_gath(&self) -> bool {
        *self == STATE_A::PERIPHSCATGATH
    }
}
#[doc = "Reader of field `CHNLS`"]
pub type CHNLS_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - Indicate enable DMA"]
    #[inline(always)]
    pub fn master_en(&self) -> MASTER_EN_R {
        MASTER_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 4:7 - State of DMA"]
    #[inline(always)]
    pub fn state(&self) -> STATE_R {
        STATE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 16:20 - Number channel DMA (write: N-1)"]
    #[inline(always)]
    pub fn chnls(&self) -> CHNLS_R {
        CHNLS_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
