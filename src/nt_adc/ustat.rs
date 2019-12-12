#[doc = "Reader of register USTAT"]
pub type R = crate::R<u32, super::USTAT>;
#[doc = "Writer for register USTAT"]
pub type W = crate::W<u32, super::USTAT>;
#[doc = "Register USTAT `reset()`'s with value 0"]
impl crate::ResetValue for super::USTAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UV0`"]
pub type UV0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UV0`"]
pub struct UV0_W<'a> {
    w: &'a mut W,
}
impl<'a> UV0_W<'a> {
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
#[doc = "Reader of field `UV1`"]
pub type UV1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UV1`"]
pub struct UV1_W<'a> {
    w: &'a mut W,
}
impl<'a> UV1_W<'a> {
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
#[doc = "Reader of field `UV2`"]
pub type UV2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UV2`"]
pub struct UV2_W<'a> {
    w: &'a mut W,
}
impl<'a> UV2_W<'a> {
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
#[doc = "Reader of field `UV3`"]
pub type UV3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UV3`"]
pub struct UV3_W<'a> {
    w: &'a mut W,
}
impl<'a> UV3_W<'a> {
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
#[doc = "Reader of field `UV4`"]
pub type UV4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UV4`"]
pub struct UV4_W<'a> {
    w: &'a mut W,
}
impl<'a> UV4_W<'a> {
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
#[doc = "Reader of field `UV5`"]
pub type UV5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UV5`"]
pub struct UV5_W<'a> {
    w: &'a mut W,
}
impl<'a> UV5_W<'a> {
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
#[doc = "Reader of field `UV6`"]
pub type UV6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UV6`"]
pub struct UV6_W<'a> {
    w: &'a mut W,
}
impl<'a> UV6_W<'a> {
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
#[doc = "Reader of field `UV7`"]
pub type UV7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UV7`"]
pub struct UV7_W<'a> {
    w: &'a mut W,
}
impl<'a> UV7_W<'a> {
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
    #[doc = "Bit 0 - Sequencer 0 FIFO underflow"]
    #[inline(always)]
    pub fn uv0(&self) -> UV0_R {
        UV0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Sequencer 1 FIFO underflow"]
    #[inline(always)]
    pub fn uv1(&self) -> UV1_R {
        UV1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Sequencer 2 FIFO underflow"]
    #[inline(always)]
    pub fn uv2(&self) -> UV2_R {
        UV2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Sequencer 3 FIFO underflow"]
    #[inline(always)]
    pub fn uv3(&self) -> UV3_R {
        UV3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Sequencer 4 FIFO underflow"]
    #[inline(always)]
    pub fn uv4(&self) -> UV4_R {
        UV4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Sequencer 5 FIFO underflow"]
    #[inline(always)]
    pub fn uv5(&self) -> UV5_R {
        UV5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Sequencer 6 FIFO underflow"]
    #[inline(always)]
    pub fn uv6(&self) -> UV6_R {
        UV6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Sequencer 7 FIFO underflow"]
    #[inline(always)]
    pub fn uv7(&self) -> UV7_R {
        UV7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Sequencer 0 FIFO underflow"]
    #[inline(always)]
    pub fn uv0(&mut self) -> UV0_W {
        UV0_W { w: self }
    }
    #[doc = "Bit 1 - Sequencer 1 FIFO underflow"]
    #[inline(always)]
    pub fn uv1(&mut self) -> UV1_W {
        UV1_W { w: self }
    }
    #[doc = "Bit 2 - Sequencer 2 FIFO underflow"]
    #[inline(always)]
    pub fn uv2(&mut self) -> UV2_W {
        UV2_W { w: self }
    }
    #[doc = "Bit 3 - Sequencer 3 FIFO underflow"]
    #[inline(always)]
    pub fn uv3(&mut self) -> UV3_W {
        UV3_W { w: self }
    }
    #[doc = "Bit 4 - Sequencer 4 FIFO underflow"]
    #[inline(always)]
    pub fn uv4(&mut self) -> UV4_W {
        UV4_W { w: self }
    }
    #[doc = "Bit 5 - Sequencer 5 FIFO underflow"]
    #[inline(always)]
    pub fn uv5(&mut self) -> UV5_W {
        UV5_W { w: self }
    }
    #[doc = "Bit 6 - Sequencer 6 FIFO underflow"]
    #[inline(always)]
    pub fn uv6(&mut self) -> UV6_W {
        UV6_W { w: self }
    }
    #[doc = "Bit 7 - Sequencer 7 FIFO underflow"]
    #[inline(always)]
    pub fn uv7(&mut self) -> UV7_W {
        UV7_W { w: self }
    }
}
