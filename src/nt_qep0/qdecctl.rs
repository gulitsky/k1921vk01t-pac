#[doc = "Reader of register QDECCTL"]
pub type R = crate::R<u32, super::QDECCTL>;
#[doc = "Writer for register QDECCTL"]
pub type W = crate::W<u32, super::QDECCTL>;
#[doc = "Register QDECCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::QDECCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `QSP`"]
pub type QSP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `QSP`"]
pub struct QSP_W<'a> {
    w: &'a mut W,
}
impl<'a> QSP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `QIP`"]
pub type QIP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `QIP`"]
pub struct QIP_W<'a> {
    w: &'a mut W,
}
impl<'a> QIP_W<'a> {
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
#[doc = "Reader of field `QBP`"]
pub type QBP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `QBP`"]
pub struct QBP_W<'a> {
    w: &'a mut W,
}
impl<'a> QBP_W<'a> {
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
#[doc = "Reader of field `QAP`"]
pub type QAP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `QAP`"]
pub struct QAP_W<'a> {
    w: &'a mut W,
}
impl<'a> QAP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `IGATE`"]
pub type IGATE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IGATE`"]
pub struct IGATE_W<'a> {
    w: &'a mut W,
}
impl<'a> IGATE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `SWAP`"]
pub type SWAP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWAP`"]
pub struct SWAP_W<'a> {
    w: &'a mut W,
}
impl<'a> SWAP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `XCR`"]
pub type XCR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XCR`"]
pub struct XCR_W<'a> {
    w: &'a mut W,
}
impl<'a> XCR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `SPSEL`"]
pub type SPSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPSEL`"]
pub struct SPSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SPSEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `SOEN`"]
pub type SOEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOEN`"]
pub struct SOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SOEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QSRC_A {
    #[doc = "0: quadrature mode"]
    QUAD,
    #[doc = "1: count/direction mode"]
    COUNTDIR,
    #[doc = "2: count up"]
    UP,
    #[doc = "3: count down"]
    DOWN,
}
impl From<QSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: QSRC_A) -> Self {
        match variant {
            QSRC_A::QUAD => 0,
            QSRC_A::COUNTDIR => 1,
            QSRC_A::UP => 2,
            QSRC_A::DOWN => 3,
        }
    }
}
#[doc = "Reader of field `QSRC`"]
pub type QSRC_R = crate::R<u8, QSRC_A>;
impl QSRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> QSRC_A {
        match self.bits {
            0 => QSRC_A::QUAD,
            1 => QSRC_A::COUNTDIR,
            2 => QSRC_A::UP,
            3 => QSRC_A::DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `QUAD`"]
    #[inline(always)]
    pub fn is_quad(&self) -> bool {
        *self == QSRC_A::QUAD
    }
    #[doc = "Checks if the value of the field is `COUNTDIR`"]
    #[inline(always)]
    pub fn is_count_dir(&self) -> bool {
        *self == QSRC_A::COUNTDIR
    }
    #[doc = "Checks if the value of the field is `UP`"]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        *self == QSRC_A::UP
    }
    #[doc = "Checks if the value of the field is `DOWN`"]
    #[inline(always)]
    pub fn is_down(&self) -> bool {
        *self == QSRC_A::DOWN
    }
}
#[doc = "Write proxy for field `QSRC`"]
pub struct QSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> QSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: QSRC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "quadrature mode"]
    #[inline(always)]
    pub fn quad(self) -> &'a mut W {
        self.variant(QSRC_A::QUAD)
    }
    #[doc = "count/direction mode"]
    #[inline(always)]
    pub fn count_dir(self) -> &'a mut W {
        self.variant(QSRC_A::COUNTDIR)
    }
    #[doc = "count up"]
    #[inline(always)]
    pub fn up(self) -> &'a mut W {
        self.variant(QSRC_A::UP)
    }
    #[doc = "count down"]
    #[inline(always)]
    pub fn down(self) -> &'a mut W {
        self.variant(QSRC_A::DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bit 5 - inversion QEPS"]
    #[inline(always)]
    pub fn qsp(&self) -> QSP_R {
        QSP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - inv. QEPI"]
    #[inline(always)]
    pub fn qip(&self) -> QIP_R {
        QIP_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - inv. QEPB"]
    #[inline(always)]
    pub fn qbp(&self) -> QBP_R {
        QBP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - inv. QEPA"]
    #[inline(always)]
    pub fn qap(&self) -> QAP_R {
        QAP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - strobing input signal"]
    #[inline(always)]
    pub fn igate(&self) -> IGATE_R {
        IGATE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - trade places QEPA and QEPB"]
    #[inline(always)]
    pub fn swap(&self) -> SWAP_R {
        SWAP_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - edge selector"]
    #[inline(always)]
    pub fn xcr(&self) -> XCR_R {
        XCR_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - index/strobe output"]
    #[inline(always)]
    pub fn spsel(&self) -> SPSEL_R {
        SPSEL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - enable PCSOUT"]
    #[inline(always)]
    pub fn soen(&self) -> SOEN_R {
        SOEN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn qsrc(&self) -> QSRC_R {
        QSRC_R::new(((self.bits >> 14) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 5 - inversion QEPS"]
    #[inline(always)]
    pub fn qsp(&mut self) -> QSP_W {
        QSP_W { w: self }
    }
    #[doc = "Bit 6 - inv. QEPI"]
    #[inline(always)]
    pub fn qip(&mut self) -> QIP_W {
        QIP_W { w: self }
    }
    #[doc = "Bit 7 - inv. QEPB"]
    #[inline(always)]
    pub fn qbp(&mut self) -> QBP_W {
        QBP_W { w: self }
    }
    #[doc = "Bit 8 - inv. QEPA"]
    #[inline(always)]
    pub fn qap(&mut self) -> QAP_W {
        QAP_W { w: self }
    }
    #[doc = "Bit 9 - strobing input signal"]
    #[inline(always)]
    pub fn igate(&mut self) -> IGATE_W {
        IGATE_W { w: self }
    }
    #[doc = "Bit 10 - trade places QEPA and QEPB"]
    #[inline(always)]
    pub fn swap(&mut self) -> SWAP_W {
        SWAP_W { w: self }
    }
    #[doc = "Bit 11 - edge selector"]
    #[inline(always)]
    pub fn xcr(&mut self) -> XCR_W {
        XCR_W { w: self }
    }
    #[doc = "Bit 12 - index/strobe output"]
    #[inline(always)]
    pub fn spsel(&mut self) -> SPSEL_W {
        SPSEL_W { w: self }
    }
    #[doc = "Bit 13 - enable PCSOUT"]
    #[inline(always)]
    pub fn soen(&mut self) -> SOEN_W {
        SOEN_W { w: self }
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn qsrc(&mut self) -> QSRC_W {
        QSRC_W { w: self }
    }
}
