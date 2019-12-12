#[doc = "Reader of register HDCTL"]
pub type R = crate::R<u32, super::HDCTL>;
#[doc = "Writer for register HDCTL"]
pub type W = crate::W<u32, super::HDCTL>;
#[doc = "Register HDCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::HDCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HDA_A {
    #[doc = "1: PWMA/PWMB go to 1 on failture"]
    SET,
    #[doc = "2: PWMA/PWMB go to 0 on failture"]
    CLEAR,
    #[doc = "3: no action on failture"]
    NOACTION,
}
impl From<HDA_A> for u8 {
    #[inline(always)]
    fn from(variant: HDA_A) -> Self {
        match variant {
            HDA_A::SET => 1,
            HDA_A::CLEAR => 2,
            HDA_A::NOACTION => 3,
        }
    }
}
#[doc = "Reader of field `HDA`"]
pub type HDA_R = crate::R<u8, HDA_A>;
impl HDA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, HDA_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(HDA_A::SET),
            2 => Val(HDA_A::CLEAR),
            3 => Val(HDA_A::NOACTION),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == HDA_A::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == HDA_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `NOACTION`"]
    #[inline(always)]
    pub fn is_no_action(&self) -> bool {
        *self == HDA_A::NOACTION
    }
}
#[doc = "Write proxy for field `HDA`"]
pub struct HDA_W<'a> {
    w: &'a mut W,
}
impl<'a> HDA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HDA_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PWMA/PWMB go to 1 on failture"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(HDA_A::SET)
    }
    #[doc = "PWMA/PWMB go to 0 on failture"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(HDA_A::CLEAR)
    }
    #[doc = "no action on failture"]
    #[inline(always)]
    pub fn no_action(self) -> &'a mut W {
        self.variant(HDA_A::NOACTION)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HDB_A {
    #[doc = "1: PWMA/PWMB go to 1 on failture"]
    SET,
    #[doc = "2: PWMA/PWMB go to 0 on failture"]
    CLEAR,
    #[doc = "3: no action on failture"]
    NOACTION,
}
impl From<HDB_A> for u8 {
    #[inline(always)]
    fn from(variant: HDB_A) -> Self {
        match variant {
            HDB_A::SET => 1,
            HDB_A::CLEAR => 2,
            HDB_A::NOACTION => 3,
        }
    }
}
#[doc = "Reader of field `HDB`"]
pub type HDB_R = crate::R<u8, HDB_A>;
impl HDB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, HDB_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(HDB_A::SET),
            2 => Val(HDB_A::CLEAR),
            3 => Val(HDB_A::NOACTION),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == HDB_A::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == HDB_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `NOACTION`"]
    #[inline(always)]
    pub fn is_no_action(&self) -> bool {
        *self == HDB_A::NOACTION
    }
}
#[doc = "Write proxy for field `HDB`"]
pub struct HDB_W<'a> {
    w: &'a mut W,
}
impl<'a> HDB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HDB_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PWMA/PWMB go to 1 on failture"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(HDB_A::SET)
    }
    #[doc = "PWMA/PWMB go to 0 on failture"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(HDB_A::CLEAR)
    }
    #[doc = "no action on failture"]
    #[inline(always)]
    pub fn no_action(self) -> &'a mut W {
        self.variant(HDB_A::NOACTION)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn hda(&self) -> HDA_R {
        HDA_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn hdb(&self) -> HDB_R {
        HDB_R::new(((self.bits >> 2) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn hda(&mut self) -> HDA_W {
        HDA_W { w: self }
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn hdb(&mut self) -> HDB_W {
        HDB_W { w: self }
    }
}
