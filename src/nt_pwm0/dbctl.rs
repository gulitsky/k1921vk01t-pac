#[doc = "Reader of register DBCTL"]
pub type R = crate::R<u32, super::DBCTL>;
#[doc = "Writer for register DBCTL"]
pub type W = crate::W<u32, super::DBCTL>;
#[doc = "Register DBCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::DBCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUT_MODE_A {
    #[doc = "0: edge for deadtime is no specified"]
    NOSPEC,
    #[doc = "1: deadtime on PWMB negedge"]
    BNEG,
    #[doc = "2: deadtime on PWMA posedge"]
    APOS,
    #[doc = "3: deadtime on PWMA posedge and PWMB negedge"]
    APOS_BNEG,
}
impl From<OUT_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: OUT_MODE_A) -> Self {
        match variant {
            OUT_MODE_A::NOSPEC => 0,
            OUT_MODE_A::BNEG => 1,
            OUT_MODE_A::APOS => 2,
            OUT_MODE_A::APOS_BNEG => 3,
        }
    }
}
#[doc = "Reader of field `OUT_MODE`"]
pub type OUT_MODE_R = crate::R<u8, OUT_MODE_A>;
impl OUT_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUT_MODE_A {
        match self.bits {
            0 => OUT_MODE_A::NOSPEC,
            1 => OUT_MODE_A::BNEG,
            2 => OUT_MODE_A::APOS,
            3 => OUT_MODE_A::APOS_BNEG,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NOSPEC`"]
    #[inline(always)]
    pub fn is_no_spec(&self) -> bool {
        *self == OUT_MODE_A::NOSPEC
    }
    #[doc = "Checks if the value of the field is `BNEG`"]
    #[inline(always)]
    pub fn is_bneg(&self) -> bool {
        *self == OUT_MODE_A::BNEG
    }
    #[doc = "Checks if the value of the field is `APOS`"]
    #[inline(always)]
    pub fn is_apos(&self) -> bool {
        *self == OUT_MODE_A::APOS
    }
    #[doc = "Checks if the value of the field is `APOS_BNEG`"]
    #[inline(always)]
    pub fn is_apos_bneg(&self) -> bool {
        *self == OUT_MODE_A::APOS_BNEG
    }
}
#[doc = "Write proxy for field `OUT_MODE`"]
pub struct OUT_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUT_MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "edge for deadtime is no specified"]
    #[inline(always)]
    pub fn no_spec(self) -> &'a mut W {
        self.variant(OUT_MODE_A::NOSPEC)
    }
    #[doc = "deadtime on PWMB negedge"]
    #[inline(always)]
    pub fn bneg(self) -> &'a mut W {
        self.variant(OUT_MODE_A::BNEG)
    }
    #[doc = "deadtime on PWMA posedge"]
    #[inline(always)]
    pub fn apos(self) -> &'a mut W {
        self.variant(OUT_MODE_A::APOS)
    }
    #[doc = "deadtime on PWMA posedge and PWMB negedge"]
    #[inline(always)]
    pub fn apos_bneg(self) -> &'a mut W {
        self.variant(OUT_MODE_A::APOS_BNEG)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POLSEL_A {
    #[doc = "0: inverse disabled"]
    INVDISABLE,
    #[doc = "1: inverse on PWMA"]
    INVA,
    #[doc = "2: inverse on PWMB"]
    INVB,
    #[doc = "3: inverse on PWMA and PWMB"]
    INVAB,
}
impl From<POLSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: POLSEL_A) -> Self {
        match variant {
            POLSEL_A::INVDISABLE => 0,
            POLSEL_A::INVA => 1,
            POLSEL_A::INVB => 2,
            POLSEL_A::INVAB => 3,
        }
    }
}
#[doc = "Reader of field `POLSEL`"]
pub type POLSEL_R = crate::R<u8, POLSEL_A>;
impl POLSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POLSEL_A {
        match self.bits {
            0 => POLSEL_A::INVDISABLE,
            1 => POLSEL_A::INVA,
            2 => POLSEL_A::INVB,
            3 => POLSEL_A::INVAB,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INVDISABLE`"]
    #[inline(always)]
    pub fn is_inv_disable(&self) -> bool {
        *self == POLSEL_A::INVDISABLE
    }
    #[doc = "Checks if the value of the field is `INVA`"]
    #[inline(always)]
    pub fn is_inv_a(&self) -> bool {
        *self == POLSEL_A::INVA
    }
    #[doc = "Checks if the value of the field is `INVB`"]
    #[inline(always)]
    pub fn is_inv_b(&self) -> bool {
        *self == POLSEL_A::INVB
    }
    #[doc = "Checks if the value of the field is `INVAB`"]
    #[inline(always)]
    pub fn is_inv_ab(&self) -> bool {
        *self == POLSEL_A::INVAB
    }
}
#[doc = "Write proxy for field `POLSEL`"]
pub struct POLSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> POLSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POLSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "inverse disabled"]
    #[inline(always)]
    pub fn inv_disable(self) -> &'a mut W {
        self.variant(POLSEL_A::INVDISABLE)
    }
    #[doc = "inverse on PWMA"]
    #[inline(always)]
    pub fn inv_a(self) -> &'a mut W {
        self.variant(POLSEL_A::INVA)
    }
    #[doc = "inverse on PWMB"]
    #[inline(always)]
    pub fn inv_b(self) -> &'a mut W {
        self.variant(POLSEL_A::INVB)
    }
    #[doc = "inverse on PWMA and PWMB"]
    #[inline(always)]
    pub fn inv_ab(self) -> &'a mut W {
        self.variant(POLSEL_A::INVAB)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN_MODE_A {
    #[doc = "0: PWMA is used for posedge and negedge control"]
    APOSNEG,
    #[doc = "1: PWMA is used for negedge and PWMB is used for posedge control"]
    ANEG_BPOS,
    #[doc = "2: PWMA is used for posedge and PWMB is used for negedge control"]
    APOS_BNEG,
    #[doc = "3: PWMB is used for posedge and negedge control"]
    BPOSNEG,
}
impl From<IN_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: IN_MODE_A) -> Self {
        match variant {
            IN_MODE_A::APOSNEG => 0,
            IN_MODE_A::ANEG_BPOS => 1,
            IN_MODE_A::APOS_BNEG => 2,
            IN_MODE_A::BPOSNEG => 3,
        }
    }
}
#[doc = "Reader of field `IN_MODE`"]
pub type IN_MODE_R = crate::R<u8, IN_MODE_A>;
impl IN_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IN_MODE_A {
        match self.bits {
            0 => IN_MODE_A::APOSNEG,
            1 => IN_MODE_A::ANEG_BPOS,
            2 => IN_MODE_A::APOS_BNEG,
            3 => IN_MODE_A::BPOSNEG,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `APOSNEG`"]
    #[inline(always)]
    pub fn is_apos_neg(&self) -> bool {
        *self == IN_MODE_A::APOSNEG
    }
    #[doc = "Checks if the value of the field is `ANEG_BPOS`"]
    #[inline(always)]
    pub fn is_aneg_bpos(&self) -> bool {
        *self == IN_MODE_A::ANEG_BPOS
    }
    #[doc = "Checks if the value of the field is `APOS_BNEG`"]
    #[inline(always)]
    pub fn is_apos_bneg(&self) -> bool {
        *self == IN_MODE_A::APOS_BNEG
    }
    #[doc = "Checks if the value of the field is `BPOSNEG`"]
    #[inline(always)]
    pub fn is_bpos_neg(&self) -> bool {
        *self == IN_MODE_A::BPOSNEG
    }
}
#[doc = "Write proxy for field `IN_MODE`"]
pub struct IN_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> IN_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IN_MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "PWMA is used for posedge and negedge control"]
    #[inline(always)]
    pub fn apos_neg(self) -> &'a mut W {
        self.variant(IN_MODE_A::APOSNEG)
    }
    #[doc = "PWMA is used for negedge and PWMB is used for posedge control"]
    #[inline(always)]
    pub fn aneg_bpos(self) -> &'a mut W {
        self.variant(IN_MODE_A::ANEG_BPOS)
    }
    #[doc = "PWMA is used for posedge and PWMB is used for negedge control"]
    #[inline(always)]
    pub fn apos_bneg(self) -> &'a mut W {
        self.variant(IN_MODE_A::APOS_BNEG)
    }
    #[doc = "PWMB is used for posedge and negedge control"]
    #[inline(always)]
    pub fn bpos_neg(self) -> &'a mut W {
        self.variant(IN_MODE_A::BPOSNEG)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn out_mode(&self) -> OUT_MODE_R {
        OUT_MODE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn polsel(&self) -> POLSEL_R {
        POLSEL_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn in_mode(&self) -> IN_MODE_R {
        IN_MODE_R::new(((self.bits >> 4) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn out_mode(&mut self) -> OUT_MODE_W {
        OUT_MODE_W { w: self }
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn polsel(&mut self) -> POLSEL_W {
        POLSEL_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn in_mode(&mut self) -> IN_MODE_W {
        IN_MODE_W { w: self }
    }
}
