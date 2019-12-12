#[doc = "Reader of register TZCTL"]
pub type R = crate::R<u32, super::TZCTL>;
#[doc = "Writer for register TZCTL"]
pub type W = crate::W<u32, super::TZCTL>;
#[doc = "Register TZCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::TZCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TZA_A {
    #[doc = "0: PWMA/PWMB go to Z on failture"]
    Z,
    #[doc = "1: PWMA/PWMB go to 1 on failture"]
    SET,
    #[doc = "2: PWMA/PWMB go to 0 on failture"]
    CLEAR,
    #[doc = "3: no action on failture"]
    NOACTION,
}
impl From<TZA_A> for u8 {
    #[inline(always)]
    fn from(variant: TZA_A) -> Self {
        match variant {
            TZA_A::Z => 0,
            TZA_A::SET => 1,
            TZA_A::CLEAR => 2,
            TZA_A::NOACTION => 3,
        }
    }
}
#[doc = "Reader of field `TZA`"]
pub type TZA_R = crate::R<u8, TZA_A>;
impl TZA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TZA_A {
        match self.bits {
            0 => TZA_A::Z,
            1 => TZA_A::SET,
            2 => TZA_A::CLEAR,
            3 => TZA_A::NOACTION,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Z`"]
    #[inline(always)]
    pub fn is_z(&self) -> bool {
        *self == TZA_A::Z
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == TZA_A::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == TZA_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `NOACTION`"]
    #[inline(always)]
    pub fn is_no_action(&self) -> bool {
        *self == TZA_A::NOACTION
    }
}
#[doc = "Write proxy for field `TZA`"]
pub struct TZA_W<'a> {
    w: &'a mut W,
}
impl<'a> TZA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TZA_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "PWMA/PWMB go to Z on failture"]
    #[inline(always)]
    pub fn z(self) -> &'a mut W {
        self.variant(TZA_A::Z)
    }
    #[doc = "PWMA/PWMB go to 1 on failture"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(TZA_A::SET)
    }
    #[doc = "PWMA/PWMB go to 0 on failture"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TZA_A::CLEAR)
    }
    #[doc = "no action on failture"]
    #[inline(always)]
    pub fn no_action(self) -> &'a mut W {
        self.variant(TZA_A::NOACTION)
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
pub enum TZB_A {
    #[doc = "0: PWMA/PWMB go to Z on failture"]
    Z,
    #[doc = "1: PWMA/PWMB go to 1 on failture"]
    SET,
    #[doc = "2: PWMA/PWMB go to 0 on failture"]
    CLEAR,
    #[doc = "3: no action on failture"]
    NOACTION,
}
impl From<TZB_A> for u8 {
    #[inline(always)]
    fn from(variant: TZB_A) -> Self {
        match variant {
            TZB_A::Z => 0,
            TZB_A::SET => 1,
            TZB_A::CLEAR => 2,
            TZB_A::NOACTION => 3,
        }
    }
}
#[doc = "Reader of field `TZB`"]
pub type TZB_R = crate::R<u8, TZB_A>;
impl TZB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TZB_A {
        match self.bits {
            0 => TZB_A::Z,
            1 => TZB_A::SET,
            2 => TZB_A::CLEAR,
            3 => TZB_A::NOACTION,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Z`"]
    #[inline(always)]
    pub fn is_z(&self) -> bool {
        *self == TZB_A::Z
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == TZB_A::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == TZB_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `NOACTION`"]
    #[inline(always)]
    pub fn is_no_action(&self) -> bool {
        *self == TZB_A::NOACTION
    }
}
#[doc = "Write proxy for field `TZB`"]
pub struct TZB_W<'a> {
    w: &'a mut W,
}
impl<'a> TZB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TZB_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "PWMA/PWMB go to Z on failture"]
    #[inline(always)]
    pub fn z(self) -> &'a mut W {
        self.variant(TZB_A::Z)
    }
    #[doc = "PWMA/PWMB go to 1 on failture"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(TZB_A::SET)
    }
    #[doc = "PWMA/PWMB go to 0 on failture"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TZB_A::CLEAR)
    }
    #[doc = "no action on failture"]
    #[inline(always)]
    pub fn no_action(self) -> &'a mut W {
        self.variant(TZB_A::NOACTION)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn tza(&self) -> TZA_R {
        TZA_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn tzb(&self) -> TZB_R {
        TZB_R::new(((self.bits >> 2) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn tza(&mut self) -> TZA_W {
        TZA_W { w: self }
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn tzb(&mut self) -> TZB_W {
        TZB_W { w: self }
    }
}
