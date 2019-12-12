#[doc = "Reader of register ACTSS"]
pub type R = crate::R<u32, super::ACTSS>;
#[doc = "Writer for register ACTSS"]
pub type W = crate::W<u32, super::ACTSS>;
#[doc = "Register ACTSS `reset()`'s with value 0"]
impl crate::ResetValue for super::ACTSS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ASEN0`"]
pub type ASEN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ASEN0`"]
pub struct ASEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> ASEN0_W<'a> {
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
#[doc = "Reader of field `ASEN1`"]
pub type ASEN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ASEN1`"]
pub struct ASEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> ASEN1_W<'a> {
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
#[doc = "Reader of field `ASEN2`"]
pub type ASEN2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ASEN2`"]
pub struct ASEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> ASEN2_W<'a> {
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
#[doc = "Reader of field `ASEN3`"]
pub type ASEN3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ASEN3`"]
pub struct ASEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> ASEN3_W<'a> {
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
#[doc = "Reader of field `ASEN4`"]
pub type ASEN4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ASEN4`"]
pub struct ASEN4_W<'a> {
    w: &'a mut W,
}
impl<'a> ASEN4_W<'a> {
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
#[doc = "Reader of field `ASEN5`"]
pub type ASEN5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ASEN5`"]
pub struct ASEN5_W<'a> {
    w: &'a mut W,
}
impl<'a> ASEN5_W<'a> {
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
#[doc = "Reader of field `ASEN6`"]
pub type ASEN6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ASEN6`"]
pub struct ASEN6_W<'a> {
    w: &'a mut W,
}
impl<'a> ASEN6_W<'a> {
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
#[doc = "Reader of field `ASEN7`"]
pub type ASEN7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ASEN7`"]
pub struct ASEN7_W<'a> {
    w: &'a mut W,
}
impl<'a> ASEN7_W<'a> {
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
    #[doc = "Bit 0 - Enable sequencer 0"]
    #[inline(always)]
    pub fn asen0(&self) -> ASEN0_R {
        ASEN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable sequencer 1"]
    #[inline(always)]
    pub fn asen1(&self) -> ASEN1_R {
        ASEN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable sequencer 2"]
    #[inline(always)]
    pub fn asen2(&self) -> ASEN2_R {
        ASEN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable sequencer 3"]
    #[inline(always)]
    pub fn asen3(&self) -> ASEN3_R {
        ASEN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable sequencer 4"]
    #[inline(always)]
    pub fn asen4(&self) -> ASEN4_R {
        ASEN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enable sequencer 5"]
    #[inline(always)]
    pub fn asen5(&self) -> ASEN5_R {
        ASEN5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enable sequencer 6"]
    #[inline(always)]
    pub fn asen6(&self) -> ASEN6_R {
        ASEN6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enable sequencer 7"]
    #[inline(always)]
    pub fn asen7(&self) -> ASEN7_R {
        ASEN7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable sequencer 0"]
    #[inline(always)]
    pub fn asen0(&mut self) -> ASEN0_W {
        ASEN0_W { w: self }
    }
    #[doc = "Bit 1 - Enable sequencer 1"]
    #[inline(always)]
    pub fn asen1(&mut self) -> ASEN1_W {
        ASEN1_W { w: self }
    }
    #[doc = "Bit 2 - Enable sequencer 2"]
    #[inline(always)]
    pub fn asen2(&mut self) -> ASEN2_W {
        ASEN2_W { w: self }
    }
    #[doc = "Bit 3 - Enable sequencer 3"]
    #[inline(always)]
    pub fn asen3(&mut self) -> ASEN3_W {
        ASEN3_W { w: self }
    }
    #[doc = "Bit 4 - Enable sequencer 4"]
    #[inline(always)]
    pub fn asen4(&mut self) -> ASEN4_W {
        ASEN4_W { w: self }
    }
    #[doc = "Bit 5 - Enable sequencer 5"]
    #[inline(always)]
    pub fn asen5(&mut self) -> ASEN5_W {
        ASEN5_W { w: self }
    }
    #[doc = "Bit 6 - Enable sequencer 6"]
    #[inline(always)]
    pub fn asen6(&mut self) -> ASEN6_W {
        ASEN6_W { w: self }
    }
    #[doc = "Bit 7 - Enable sequencer 7"]
    #[inline(always)]
    pub fn asen7(&mut self) -> ASEN7_W {
        ASEN7_W { w: self }
    }
}
