#[doc = "Reader of register QEPCTL"]
pub type R = crate::R<u32, super::QEPCTL>;
#[doc = "Writer for register QEPCTL"]
pub type W = crate::W<u32, super::QEPCTL>;
#[doc = "Register QEPCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::QEPCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WDE`"]
pub type WDE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WDE`"]
pub struct WDE_W<'a> {
    w: &'a mut W,
}
impl<'a> WDE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `UTE`"]
pub type UTE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UTE`"]
pub struct UTE_W<'a> {
    w: &'a mut W,
}
impl<'a> UTE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `QCLM`"]
pub type QCLM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `QCLM`"]
pub struct QCLM_W<'a> {
    w: &'a mut W,
}
impl<'a> QCLM_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `QPEN`"]
pub type QPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `QPEN`"]
pub struct QPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> QPEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IEL_A {
    #[doc = "0: no position counter latch"]
    NOLATCH,
    #[doc = "1: latch on index signal posedge"]
    INDPOS,
    #[doc = "2: latch on index signal negedge"]
    INDNEG,
    #[doc = "3: latch on index marker"]
    INDMARK,
}
impl From<IEL_A> for u8 {
    #[inline(always)]
    fn from(variant: IEL_A) -> Self {
        match variant {
            IEL_A::NOLATCH => 0,
            IEL_A::INDPOS => 1,
            IEL_A::INDNEG => 2,
            IEL_A::INDMARK => 3,
        }
    }
}
#[doc = "Reader of field `IEL`"]
pub type IEL_R = crate::R<u8, IEL_A>;
impl IEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IEL_A {
        match self.bits {
            0 => IEL_A::NOLATCH,
            1 => IEL_A::INDPOS,
            2 => IEL_A::INDNEG,
            3 => IEL_A::INDMARK,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NOLATCH`"]
    #[inline(always)]
    pub fn is_no_latch(&self) -> bool {
        *self == IEL_A::NOLATCH
    }
    #[doc = "Checks if the value of the field is `INDPOS`"]
    #[inline(always)]
    pub fn is_ind_pos(&self) -> bool {
        *self == IEL_A::INDPOS
    }
    #[doc = "Checks if the value of the field is `INDNEG`"]
    #[inline(always)]
    pub fn is_ind_neg(&self) -> bool {
        *self == IEL_A::INDNEG
    }
    #[doc = "Checks if the value of the field is `INDMARK`"]
    #[inline(always)]
    pub fn is_ind_mark(&self) -> bool {
        *self == IEL_A::INDMARK
    }
}
#[doc = "Write proxy for field `IEL`"]
pub struct IEL_W<'a> {
    w: &'a mut W,
}
impl<'a> IEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "no position counter latch"]
    #[inline(always)]
    pub fn no_latch(self) -> &'a mut W {
        self.variant(IEL_A::NOLATCH)
    }
    #[doc = "latch on index signal posedge"]
    #[inline(always)]
    pub fn ind_pos(self) -> &'a mut W {
        self.variant(IEL_A::INDPOS)
    }
    #[doc = "latch on index signal negedge"]
    #[inline(always)]
    pub fn ind_neg(self) -> &'a mut W {
        self.variant(IEL_A::INDNEG)
    }
    #[doc = "latch on index marker"]
    #[inline(always)]
    pub fn ind_mark(self) -> &'a mut W {
        self.variant(IEL_A::INDMARK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `SEL`"]
pub type SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SEL`"]
pub struct SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `SWI`"]
pub type SWI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWI`"]
pub struct SWI_W<'a> {
    w: &'a mut W,
}
impl<'a> SWI_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IEI_A {
    #[doc = "0: no initialization"]
    NOINIT,
    #[doc = "2: init on posedge QEPI"]
    QEPIPOS,
    #[doc = "3: init on negedge QEPI"]
    QEPINEG,
}
impl From<IEI_A> for u8 {
    #[inline(always)]
    fn from(variant: IEI_A) -> Self {
        match variant {
            IEI_A::NOINIT => 0,
            IEI_A::QEPIPOS => 2,
            IEI_A::QEPINEG => 3,
        }
    }
}
#[doc = "Reader of field `IEI`"]
pub type IEI_R = crate::R<u8, IEI_A>;
impl IEI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, IEI_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(IEI_A::NOINIT),
            2 => Val(IEI_A::QEPIPOS),
            3 => Val(IEI_A::QEPINEG),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NOINIT`"]
    #[inline(always)]
    pub fn is_no_init(&self) -> bool {
        *self == IEI_A::NOINIT
    }
    #[doc = "Checks if the value of the field is `QEPIPOS`"]
    #[inline(always)]
    pub fn is_qepipos(&self) -> bool {
        *self == IEI_A::QEPIPOS
    }
    #[doc = "Checks if the value of the field is `QEPINEG`"]
    #[inline(always)]
    pub fn is_qepineg(&self) -> bool {
        *self == IEI_A::QEPINEG
    }
}
#[doc = "Write proxy for field `IEI`"]
pub struct IEI_W<'a> {
    w: &'a mut W,
}
impl<'a> IEI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IEI_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "no initialization"]
    #[inline(always)]
    pub fn no_init(self) -> &'a mut W {
        self.variant(IEI_A::NOINIT)
    }
    #[doc = "init on posedge QEPI"]
    #[inline(always)]
    pub fn qepipos(self) -> &'a mut W {
        self.variant(IEI_A::QEPIPOS)
    }
    #[doc = "init on negedge QEPI"]
    #[inline(always)]
    pub fn qepineg(self) -> &'a mut W {
        self.variant(IEI_A::QEPINEG)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEI_A {
    #[doc = "0: no initialization"]
    NOINIT,
    #[doc = "2: init on posedge QEPI"]
    QEPSPOS,
    #[doc = "3: init depends on direction - on posedge if direction is up, on negedge if direction is down"]
    QEPSDIR,
}
impl From<SEI_A> for u8 {
    #[inline(always)]
    fn from(variant: SEI_A) -> Self {
        match variant {
            SEI_A::NOINIT => 0,
            SEI_A::QEPSPOS => 2,
            SEI_A::QEPSDIR => 3,
        }
    }
}
#[doc = "Reader of field `SEI`"]
pub type SEI_R = crate::R<u8, SEI_A>;
impl SEI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SEI_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SEI_A::NOINIT),
            2 => Val(SEI_A::QEPSPOS),
            3 => Val(SEI_A::QEPSDIR),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NOINIT`"]
    #[inline(always)]
    pub fn is_no_init(&self) -> bool {
        *self == SEI_A::NOINIT
    }
    #[doc = "Checks if the value of the field is `QEPSPOS`"]
    #[inline(always)]
    pub fn is_qepspos(&self) -> bool {
        *self == SEI_A::QEPSPOS
    }
    #[doc = "Checks if the value of the field is `QEPSDIR`"]
    #[inline(always)]
    pub fn is_qepsdir(&self) -> bool {
        *self == SEI_A::QEPSDIR
    }
}
#[doc = "Write proxy for field `SEI`"]
pub struct SEI_W<'a> {
    w: &'a mut W,
}
impl<'a> SEI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEI_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "no initialization"]
    #[inline(always)]
    pub fn no_init(self) -> &'a mut W {
        self.variant(SEI_A::NOINIT)
    }
    #[doc = "init on posedge QEPI"]
    #[inline(always)]
    pub fn qepspos(self) -> &'a mut W {
        self.variant(SEI_A::QEPSPOS)
    }
    #[doc = "init depends on direction - on posedge if direction is up, on negedge if direction is down"]
    #[inline(always)]
    pub fn qepsdir(self) -> &'a mut W {
        self.variant(SEI_A::QEPSDIR)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCRM_A {
    #[doc = "0: reset on index"]
    IND,
    #[doc = "1: reset on max position count"]
    POSMAX,
    #[doc = "2: reset on the first index"]
    FIRSTIND,
    #[doc = "3: reset on time counter"]
    TIME,
}
impl From<PCRM_A> for u8 {
    #[inline(always)]
    fn from(variant: PCRM_A) -> Self {
        match variant {
            PCRM_A::IND => 0,
            PCRM_A::POSMAX => 1,
            PCRM_A::FIRSTIND => 2,
            PCRM_A::TIME => 3,
        }
    }
}
#[doc = "Reader of field `PCRM`"]
pub type PCRM_R = crate::R<u8, PCRM_A>;
impl PCRM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCRM_A {
        match self.bits {
            0 => PCRM_A::IND,
            1 => PCRM_A::POSMAX,
            2 => PCRM_A::FIRSTIND,
            3 => PCRM_A::TIME,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `IND`"]
    #[inline(always)]
    pub fn is_ind(&self) -> bool {
        *self == PCRM_A::IND
    }
    #[doc = "Checks if the value of the field is `POSMAX`"]
    #[inline(always)]
    pub fn is_pos_max(&self) -> bool {
        *self == PCRM_A::POSMAX
    }
    #[doc = "Checks if the value of the field is `FIRSTIND`"]
    #[inline(always)]
    pub fn is_first_ind(&self) -> bool {
        *self == PCRM_A::FIRSTIND
    }
    #[doc = "Checks if the value of the field is `TIME`"]
    #[inline(always)]
    pub fn is_time(&self) -> bool {
        *self == PCRM_A::TIME
    }
}
#[doc = "Write proxy for field `PCRM`"]
pub struct PCRM_W<'a> {
    w: &'a mut W,
}
impl<'a> PCRM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCRM_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "reset on index"]
    #[inline(always)]
    pub fn ind(self) -> &'a mut W {
        self.variant(PCRM_A::IND)
    }
    #[doc = "reset on max position count"]
    #[inline(always)]
    pub fn pos_max(self) -> &'a mut W {
        self.variant(PCRM_A::POSMAX)
    }
    #[doc = "reset on the first index"]
    #[inline(always)]
    pub fn first_ind(self) -> &'a mut W {
        self.variant(PCRM_A::FIRSTIND)
    }
    #[doc = "reset on time counter"]
    #[inline(always)]
    pub fn time(self) -> &'a mut W {
        self.variant(PCRM_A::TIME)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FREE_SOFT_A {
    #[doc = "0: counters are blocked"]
    STOP,
    #[doc = "1: stop after overflow"]
    STOPATOVF,
    #[doc = "2: no count stop in debug mode"]
    FREE,
}
impl From<FREE_SOFT_A> for u8 {
    #[inline(always)]
    fn from(variant: FREE_SOFT_A) -> Self {
        match variant {
            FREE_SOFT_A::STOP => 0,
            FREE_SOFT_A::STOPATOVF => 1,
            FREE_SOFT_A::FREE => 2,
        }
    }
}
#[doc = "Reader of field `FREE_SOFT`"]
pub type FREE_SOFT_R = crate::R<u8, FREE_SOFT_A>;
impl FREE_SOFT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FREE_SOFT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FREE_SOFT_A::STOP),
            1 => Val(FREE_SOFT_A::STOPATOVF),
            2 => Val(FREE_SOFT_A::FREE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == FREE_SOFT_A::STOP
    }
    #[doc = "Checks if the value of the field is `STOPATOVF`"]
    #[inline(always)]
    pub fn is_stop_at_ovf(&self) -> bool {
        *self == FREE_SOFT_A::STOPATOVF
    }
    #[doc = "Checks if the value of the field is `FREE`"]
    #[inline(always)]
    pub fn is_free(&self) -> bool {
        *self == FREE_SOFT_A::FREE
    }
}
#[doc = "Write proxy for field `FREE_SOFT`"]
pub struct FREE_SOFT_W<'a> {
    w: &'a mut W,
}
impl<'a> FREE_SOFT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FREE_SOFT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "counters are blocked"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(FREE_SOFT_A::STOP)
    }
    #[doc = "stop after overflow"]
    #[inline(always)]
    pub fn stop_at_ovf(self) -> &'a mut W {
        self.variant(FREE_SOFT_A::STOPATOVF)
    }
    #[doc = "no count stop in debug mode"]
    #[inline(always)]
    pub fn free(self) -> &'a mut W {
        self.variant(FREE_SOFT_A::FREE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - enable watchdog timer"]
    #[inline(always)]
    pub fn wde(&self) -> WDE_R {
        WDE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - enable time reading timer"]
    #[inline(always)]
    pub fn ute(&self) -> UTE_R {
        UTE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - save countposition meaning"]
    #[inline(always)]
    pub fn qclm(&self) -> QCLM_R {
        QCLM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - reset meaning countposition"]
    #[inline(always)]
    pub fn qpen(&self) -> QPEN_R {
        QPEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn iel(&self) -> IEL_R {
        IEL_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 6 - sttobevent"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - enable positioncount"]
    #[inline(always)]
    pub fn swi(&self) -> SWI_R {
        SWI_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn iei(&self) -> IEI_R {
        IEI_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn sei(&self) -> SEI_R {
        SEI_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn pcrm(&self) -> PCRM_R {
        PCRM_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn free_soft(&self) -> FREE_SOFT_R {
        FREE_SOFT_R::new(((self.bits >> 14) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - enable watchdog timer"]
    #[inline(always)]
    pub fn wde(&mut self) -> WDE_W {
        WDE_W { w: self }
    }
    #[doc = "Bit 1 - enable time reading timer"]
    #[inline(always)]
    pub fn ute(&mut self) -> UTE_W {
        UTE_W { w: self }
    }
    #[doc = "Bit 2 - save countposition meaning"]
    #[inline(always)]
    pub fn qclm(&mut self) -> QCLM_W {
        QCLM_W { w: self }
    }
    #[doc = "Bit 3 - reset meaning countposition"]
    #[inline(always)]
    pub fn qpen(&mut self) -> QPEN_W {
        QPEN_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn iel(&mut self) -> IEL_W {
        IEL_W { w: self }
    }
    #[doc = "Bit 6 - sttobevent"]
    #[inline(always)]
    pub fn sel(&mut self) -> SEL_W {
        SEL_W { w: self }
    }
    #[doc = "Bit 7 - enable positioncount"]
    #[inline(always)]
    pub fn swi(&mut self) -> SWI_W {
        SWI_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn iei(&mut self) -> IEI_W {
        IEI_W { w: self }
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn sei(&mut self) -> SEI_W {
        SEI_W { w: self }
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn pcrm(&mut self) -> PCRM_W {
        PCRM_W { w: self }
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn free_soft(&mut self) -> FREE_SOFT_W {
        FREE_SOFT_W { w: self }
    }
}
