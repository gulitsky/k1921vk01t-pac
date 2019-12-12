#[doc = "Reader of register CTL2"]
pub type R = crate::R<u32, super::CTL2>;
#[doc = "Writer for register CTL2"]
pub type W = crate::W<u32, super::CTL2>;
#[doc = "Register CTL2 `reset()`'s with value 0"]
impl crate::ResetValue for super::CTL2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CEN`"]
pub type CEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CEN`"]
pub struct CEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CEN_W<'a> {
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
#[doc = "Reader of field `CINV`"]
pub type CINV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CINV`"]
pub struct CINV_W<'a> {
    w: &'a mut W,
}
impl<'a> CINV_W<'a> {
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
#[doc = "Reader of field `ISEN`"]
pub type ISEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ISEN`"]
pub struct ISEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ISEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `ISVAL`"]
pub type ISVAL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ISVAL`"]
pub struct ISVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> ISVAL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSEN_A {
    #[doc = "0: interrupt on level depended of IVAL state"]
    LVLISVAL,
    #[doc = "1: interrupt on posedge"]
    POSEDGE,
    #[doc = "2: interrupt on negedge"]
    NEGEDGE,
    #[doc = "3: interrupt on posedge and negedge"]
    EVERYEDGE,
}
impl From<TSEN_A> for u8 {
    #[inline(always)]
    fn from(variant: TSEN_A) -> Self {
        match variant {
            TSEN_A::LVLISVAL => 0,
            TSEN_A::POSEDGE => 1,
            TSEN_A::NEGEDGE => 2,
            TSEN_A::EVERYEDGE => 3,
        }
    }
}
#[doc = "Reader of field `TSEN`"]
pub type TSEN_R = crate::R<u8, TSEN_A>;
impl TSEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSEN_A {
        match self.bits {
            0 => TSEN_A::LVLISVAL,
            1 => TSEN_A::POSEDGE,
            2 => TSEN_A::NEGEDGE,
            3 => TSEN_A::EVERYEDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LVLISVAL`"]
    #[inline(always)]
    pub fn is_lvl_isval(&self) -> bool {
        *self == TSEN_A::LVLISVAL
    }
    #[doc = "Checks if the value of the field is `POSEDGE`"]
    #[inline(always)]
    pub fn is_pos_edge(&self) -> bool {
        *self == TSEN_A::POSEDGE
    }
    #[doc = "Checks if the value of the field is `NEGEDGE`"]
    #[inline(always)]
    pub fn is_neg_edge(&self) -> bool {
        *self == TSEN_A::NEGEDGE
    }
    #[doc = "Checks if the value of the field is `EVERYEDGE`"]
    #[inline(always)]
    pub fn is_every_edge(&self) -> bool {
        *self == TSEN_A::EVERYEDGE
    }
}
#[doc = "Write proxy for field `TSEN`"]
pub struct TSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TSEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSEN_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "interrupt on level depended of IVAL state"]
    #[inline(always)]
    pub fn lvl_isval(self) -> &'a mut W {
        self.variant(TSEN_A::LVLISVAL)
    }
    #[doc = "interrupt on posedge"]
    #[inline(always)]
    pub fn pos_edge(self) -> &'a mut W {
        self.variant(TSEN_A::POSEDGE)
    }
    #[doc = "interrupt on negedge"]
    #[inline(always)]
    pub fn neg_edge(self) -> &'a mut W {
        self.variant(TSEN_A::NEGEDGE)
    }
    #[doc = "interrupt on posedge and negedge"]
    #[inline(always)]
    pub fn every_edge(self) -> &'a mut W {
        self.variant(TSEN_A::EVERYEDGE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASRCP_A {
    #[doc = "0: pin C2+ connected as comparator 2 + in"]
    C2PLUS,
    #[doc = "1: pin C1+ connected as comparator 2 + in"]
    C1PLUS,
    #[doc = "2: DAC 2 out connected as comparator 2 + in"]
    DAC2,
    #[doc = "3: DAC 0 out connected as comparator 2 + in"]
    DAC0,
}
impl From<ASRCP_A> for u8 {
    #[inline(always)]
    fn from(variant: ASRCP_A) -> Self {
        match variant {
            ASRCP_A::C2PLUS => 0,
            ASRCP_A::C1PLUS => 1,
            ASRCP_A::DAC2 => 2,
            ASRCP_A::DAC0 => 3,
        }
    }
}
#[doc = "Reader of field `ASRCP`"]
pub type ASRCP_R = crate::R<u8, ASRCP_A>;
impl ASRCP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASRCP_A {
        match self.bits {
            0 => ASRCP_A::C2PLUS,
            1 => ASRCP_A::C1PLUS,
            2 => ASRCP_A::DAC2,
            3 => ASRCP_A::DAC0,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `C2PLUS`"]
    #[inline(always)]
    pub fn is_c2plus(&self) -> bool {
        *self == ASRCP_A::C2PLUS
    }
    #[doc = "Checks if the value of the field is `C1PLUS`"]
    #[inline(always)]
    pub fn is_c1plus(&self) -> bool {
        *self == ASRCP_A::C1PLUS
    }
    #[doc = "Checks if the value of the field is `DAC2`"]
    #[inline(always)]
    pub fn is_dac2(&self) -> bool {
        *self == ASRCP_A::DAC2
    }
    #[doc = "Checks if the value of the field is `DAC0`"]
    #[inline(always)]
    pub fn is_dac0(&self) -> bool {
        *self == ASRCP_A::DAC0
    }
}
#[doc = "Write proxy for field `ASRCP`"]
pub struct ASRCP_W<'a> {
    w: &'a mut W,
}
impl<'a> ASRCP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ASRCP_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "pin C2+ connected as comparator 2 + in"]
    #[inline(always)]
    pub fn c2plus(self) -> &'a mut W {
        self.variant(ASRCP_A::C2PLUS)
    }
    #[doc = "pin C1+ connected as comparator 2 + in"]
    #[inline(always)]
    pub fn c1plus(self) -> &'a mut W {
        self.variant(ASRCP_A::C1PLUS)
    }
    #[doc = "DAC 2 out connected as comparator 2 + in"]
    #[inline(always)]
    pub fn dac2(self) -> &'a mut W {
        self.variant(ASRCP_A::DAC2)
    }
    #[doc = "DAC 0 out connected as comparator 2 + in"]
    #[inline(always)]
    pub fn dac0(self) -> &'a mut W {
        self.variant(ASRCP_A::DAC0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `TOEN`"]
pub type TOEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TOEN`"]
pub struct TOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TOEN_W<'a> {
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
#[doc = "Reader of field `TSVAL`"]
pub type TSVAL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSVAL`"]
pub struct TSVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> TSVAL_W<'a> {
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
impl R {
    #[doc = "Bit 0 - enable compare 2"]
    #[inline(always)]
    pub fn cen(&self) -> CEN_R {
        CEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - polarity of output signal"]
    #[inline(always)]
    pub fn cinv(&self) -> CINV_R {
        CINV_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn isen(&self) -> ISEN_R {
        ISEN_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 4 - interrupt on a low level"]
    #[inline(always)]
    pub fn isval(&self) -> ISVAL_R {
        ISVAL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn tsen(&self) -> TSEN_R {
        TSEN_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn asrcp(&self) -> ASRCP_R {
        ASRCP_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 10 - ADC start generating events"]
    #[inline(always)]
    pub fn toen(&self) -> TOEN_R {
        TOEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 7 - ADC start at a low level"]
    #[inline(always)]
    pub fn tsval(&self) -> TSVAL_R {
        TSVAL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - enable compare 2"]
    #[inline(always)]
    pub fn cen(&mut self) -> CEN_W {
        CEN_W { w: self }
    }
    #[doc = "Bit 1 - polarity of output signal"]
    #[inline(always)]
    pub fn cinv(&mut self) -> CINV_W {
        CINV_W { w: self }
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn isen(&mut self) -> ISEN_W {
        ISEN_W { w: self }
    }
    #[doc = "Bit 4 - interrupt on a low level"]
    #[inline(always)]
    pub fn isval(&mut self) -> ISVAL_W {
        ISVAL_W { w: self }
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn tsen(&mut self) -> TSEN_W {
        TSEN_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn asrcp(&mut self) -> ASRCP_W {
        ASRCP_W { w: self }
    }
    #[doc = "Bit 10 - ADC start generating events"]
    #[inline(always)]
    pub fn toen(&mut self) -> TOEN_W {
        TOEN_W { w: self }
    }
    #[doc = "Bit 7 - ADC start at a low level"]
    #[inline(always)]
    pub fn tsval(&mut self) -> TSVAL_W {
        TSVAL_W { w: self }
    }
}
