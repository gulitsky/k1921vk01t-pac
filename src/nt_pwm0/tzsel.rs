#[doc = "Reader of register TZSEL"]
pub type R = crate::R<u32, super::TZSEL>;
#[doc = "Writer for register TZSEL"]
pub type W = crate::W<u32, super::TZSEL>;
#[doc = "Register TZSEL `reset()`'s with value 0"]
impl crate::ResetValue for super::TZSEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CBC0`"]
pub type CBC0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CBC0`"]
pub struct CBC0_W<'a> {
    w: &'a mut W,
}
impl<'a> CBC0_W<'a> {
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
#[doc = "Reader of field `CBC1`"]
pub type CBC1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CBC1`"]
pub struct CBC1_W<'a> {
    w: &'a mut W,
}
impl<'a> CBC1_W<'a> {
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
#[doc = "Reader of field `CBC2`"]
pub type CBC2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CBC2`"]
pub struct CBC2_W<'a> {
    w: &'a mut W,
}
impl<'a> CBC2_W<'a> {
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
#[doc = "Reader of field `CBC3`"]
pub type CBC3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CBC3`"]
pub struct CBC3_W<'a> {
    w: &'a mut W,
}
impl<'a> CBC3_W<'a> {
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
#[doc = "Reader of field `CBC4`"]
pub type CBC4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CBC4`"]
pub struct CBC4_W<'a> {
    w: &'a mut W,
}
impl<'a> CBC4_W<'a> {
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
#[doc = "Reader of field `CBC5`"]
pub type CBC5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CBC5`"]
pub struct CBC5_W<'a> {
    w: &'a mut W,
}
impl<'a> CBC5_W<'a> {
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
#[doc = "Reader of field `OSHT0`"]
pub type OSHT0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OSHT0`"]
pub struct OSHT0_W<'a> {
    w: &'a mut W,
}
impl<'a> OSHT0_W<'a> {
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
#[doc = "Reader of field `OSHT1`"]
pub type OSHT1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OSHT1`"]
pub struct OSHT1_W<'a> {
    w: &'a mut W,
}
impl<'a> OSHT1_W<'a> {
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
#[doc = "Reader of field `OSHT2`"]
pub type OSHT2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OSHT2`"]
pub struct OSHT2_W<'a> {
    w: &'a mut W,
}
impl<'a> OSHT2_W<'a> {
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
#[doc = "Reader of field `OSHT3`"]
pub type OSHT3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OSHT3`"]
pub struct OSHT3_W<'a> {
    w: &'a mut W,
}
impl<'a> OSHT3_W<'a> {
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
#[doc = "Reader of field `OSHT4`"]
pub type OSHT4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OSHT4`"]
pub struct OSHT4_W<'a> {
    w: &'a mut W,
}
impl<'a> OSHT4_W<'a> {
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
#[doc = "Reader of field `OSHT5`"]
pub type OSHT5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OSHT5`"]
pub struct OSHT5_W<'a> {
    w: &'a mut W,
}
impl<'a> OSHT5_W<'a> {
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
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cbc0(&self) -> CBC0_R {
        CBC0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cbc1(&self) -> CBC1_R {
        CBC1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cbc2(&self) -> CBC2_R {
        CBC2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cbc3(&self) -> CBC3_R {
        CBC3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cbc4(&self) -> CBC4_R {
        CBC4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - source signal with the output of the accident Tzx(cyclic mode)"]
    #[inline(always)]
    pub fn cbc5(&self) -> CBC5_R {
        CBC5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn osht0(&self) -> OSHT0_R {
        OSHT0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn osht1(&self) -> OSHT1_R {
        OSHT1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn osht2(&self) -> OSHT2_R {
        OSHT2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn osht3(&self) -> OSHT3_R {
        OSHT3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn osht4(&self) -> OSHT4_R {
        OSHT4_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - source signal with the output of the accident Tzx(Single mode)"]
    #[inline(always)]
    pub fn osht5(&self) -> OSHT5_R {
        OSHT5_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cbc0(&mut self) -> CBC0_W {
        CBC0_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cbc1(&mut self) -> CBC1_W {
        CBC1_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cbc2(&mut self) -> CBC2_W {
        CBC2_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cbc3(&mut self) -> CBC3_W {
        CBC3_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cbc4(&mut self) -> CBC4_W {
        CBC4_W { w: self }
    }
    #[doc = "Bit 5 - source signal with the output of the accident Tzx(cyclic mode)"]
    #[inline(always)]
    pub fn cbc5(&mut self) -> CBC5_W {
        CBC5_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn osht0(&mut self) -> OSHT0_W {
        OSHT0_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn osht1(&mut self) -> OSHT1_W {
        OSHT1_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn osht2(&mut self) -> OSHT2_W {
        OSHT2_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn osht3(&mut self) -> OSHT3_W {
        OSHT3_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn osht4(&mut self) -> OSHT4_W {
        OSHT4_W { w: self }
    }
    #[doc = "Bit 13 - source signal with the output of the accident Tzx(Single mode)"]
    #[inline(always)]
    pub fn osht5(&mut self) -> OSHT5_W {
        OSHT5_W { w: self }
    }
}
