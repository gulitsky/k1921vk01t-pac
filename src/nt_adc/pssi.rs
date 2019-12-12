#[doc = "Reader of register PSSI"]
pub type R = crate::R<u32, super::PSSI>;
#[doc = "Writer for register PSSI"]
pub type W = crate::W<u32, super::PSSI>;
#[doc = "Register PSSI `reset()`'s with value 0"]
impl crate::ResetValue for super::PSSI {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `SS0`"]
pub struct SS0_W<'a> {
    w: &'a mut W,
}
impl<'a> SS0_W<'a> {
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
#[doc = "Write proxy for field `SS1`"]
pub struct SS1_W<'a> {
    w: &'a mut W,
}
impl<'a> SS1_W<'a> {
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
#[doc = "Write proxy for field `SS2`"]
pub struct SS2_W<'a> {
    w: &'a mut W,
}
impl<'a> SS2_W<'a> {
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
#[doc = "Write proxy for field `SS3`"]
pub struct SS3_W<'a> {
    w: &'a mut W,
}
impl<'a> SS3_W<'a> {
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
#[doc = "Write proxy for field `SS4`"]
pub struct SS4_W<'a> {
    w: &'a mut W,
}
impl<'a> SS4_W<'a> {
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
#[doc = "Write proxy for field `SS5`"]
pub struct SS5_W<'a> {
    w: &'a mut W,
}
impl<'a> SS5_W<'a> {
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
#[doc = "Write proxy for field `SS6`"]
pub struct SS6_W<'a> {
    w: &'a mut W,
}
impl<'a> SS6_W<'a> {
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
#[doc = "Write proxy for field `SS7`"]
pub struct SS7_W<'a> {
    w: &'a mut W,
}
impl<'a> SS7_W<'a> {
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
#[doc = "Reader of field `GSYNC`"]
pub type GSYNC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GSYNC`"]
pub struct GSYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> GSYNC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - Sync all sequencers"]
    #[inline(always)]
    pub fn gsync(&self) -> GSYNC_R {
        GSYNC_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable sequencer 0 software sync"]
    #[inline(always)]
    pub fn ss0(&mut self) -> SS0_W {
        SS0_W { w: self }
    }
    #[doc = "Bit 1 - Enable sequencer 1 software sync"]
    #[inline(always)]
    pub fn ss1(&mut self) -> SS1_W {
        SS1_W { w: self }
    }
    #[doc = "Bit 2 - Enable sequencer 2 software sync"]
    #[inline(always)]
    pub fn ss2(&mut self) -> SS2_W {
        SS2_W { w: self }
    }
    #[doc = "Bit 3 - Enable sequencer 3 software sync"]
    #[inline(always)]
    pub fn ss3(&mut self) -> SS3_W {
        SS3_W { w: self }
    }
    #[doc = "Bit 4 - Enable sequencer 4 software sync"]
    #[inline(always)]
    pub fn ss4(&mut self) -> SS4_W {
        SS4_W { w: self }
    }
    #[doc = "Bit 5 - Enable sequencer 5 software sync"]
    #[inline(always)]
    pub fn ss5(&mut self) -> SS5_W {
        SS5_W { w: self }
    }
    #[doc = "Bit 6 - Enable sequencer 6 software sync"]
    #[inline(always)]
    pub fn ss6(&mut self) -> SS6_W {
        SS6_W { w: self }
    }
    #[doc = "Bit 7 - Enable sequencer 7 software sync"]
    #[inline(always)]
    pub fn ss7(&mut self) -> SS7_W {
        SS7_W { w: self }
    }
    #[doc = "Bit 31 - Sync all sequencers"]
    #[inline(always)]
    pub fn gsync(&mut self) -> GSYNC_W {
        GSYNC_W { w: self }
    }
}
