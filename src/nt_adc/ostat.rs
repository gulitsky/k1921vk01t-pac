#[doc = "Reader of register OSTAT"]
pub type R = crate::R<u32, super::OSTAT>;
#[doc = "Writer for register OSTAT"]
pub type W = crate::W<u32, super::OSTAT>;
#[doc = "Register OSTAT `reset()`'s with value 0"]
impl crate::ResetValue for super::OSTAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OV0`"]
pub type OV0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OV0`"]
pub struct OV0_W<'a> {
    w: &'a mut W,
}
impl<'a> OV0_W<'a> {
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
#[doc = "Reader of field `OV1`"]
pub type OV1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OV1`"]
pub struct OV1_W<'a> {
    w: &'a mut W,
}
impl<'a> OV1_W<'a> {
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
#[doc = "Reader of field `OV2`"]
pub type OV2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OV2`"]
pub struct OV2_W<'a> {
    w: &'a mut W,
}
impl<'a> OV2_W<'a> {
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
#[doc = "Reader of field `OV3`"]
pub type OV3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OV3`"]
pub struct OV3_W<'a> {
    w: &'a mut W,
}
impl<'a> OV3_W<'a> {
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
#[doc = "Reader of field `OV4`"]
pub type OV4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OV4`"]
pub struct OV4_W<'a> {
    w: &'a mut W,
}
impl<'a> OV4_W<'a> {
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
#[doc = "Reader of field `OV5`"]
pub type OV5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OV5`"]
pub struct OV5_W<'a> {
    w: &'a mut W,
}
impl<'a> OV5_W<'a> {
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
#[doc = "Reader of field `OV6`"]
pub type OV6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OV6`"]
pub struct OV6_W<'a> {
    w: &'a mut W,
}
impl<'a> OV6_W<'a> {
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
#[doc = "Reader of field `OV7`"]
pub type OV7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OV7`"]
pub struct OV7_W<'a> {
    w: &'a mut W,
}
impl<'a> OV7_W<'a> {
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
#[doc = "Reader of field `DOV0`"]
pub type DOV0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DOV0`"]
pub struct DOV0_W<'a> {
    w: &'a mut W,
}
impl<'a> DOV0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `DOV1`"]
pub type DOV1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DOV1`"]
pub struct DOV1_W<'a> {
    w: &'a mut W,
}
impl<'a> DOV1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `DOV2`"]
pub type DOV2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DOV2`"]
pub struct DOV2_W<'a> {
    w: &'a mut W,
}
impl<'a> DOV2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `DOV3`"]
pub type DOV3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DOV3`"]
pub struct DOV3_W<'a> {
    w: &'a mut W,
}
impl<'a> DOV3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `DOV4`"]
pub type DOV4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DOV4`"]
pub struct DOV4_W<'a> {
    w: &'a mut W,
}
impl<'a> DOV4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `DOV5`"]
pub type DOV5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DOV5`"]
pub struct DOV5_W<'a> {
    w: &'a mut W,
}
impl<'a> DOV5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `DOV6`"]
pub type DOV6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DOV6`"]
pub struct DOV6_W<'a> {
    w: &'a mut W,
}
impl<'a> DOV6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `DOV7`"]
pub type DOV7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DOV7`"]
pub struct DOV7_W<'a> {
    w: &'a mut W,
}
impl<'a> DOV7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Sequencer 0 FIFO overflow"]
    #[inline(always)]
    pub fn ov0(&self) -> OV0_R {
        OV0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Sequencer 1 FIFO overflow"]
    #[inline(always)]
    pub fn ov1(&self) -> OV1_R {
        OV1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Sequencer 2 FIFO overflow"]
    #[inline(always)]
    pub fn ov2(&self) -> OV2_R {
        OV2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Sequencer 3 FIFO overflow"]
    #[inline(always)]
    pub fn ov3(&self) -> OV3_R {
        OV3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Sequencer 4 FIFO overflow"]
    #[inline(always)]
    pub fn ov4(&self) -> OV4_R {
        OV4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Sequencer 5 FIFO overflow"]
    #[inline(always)]
    pub fn ov5(&self) -> OV5_R {
        OV5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Sequencer 6 FIFO overflow"]
    #[inline(always)]
    pub fn ov6(&self) -> OV6_R {
        OV6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Sequencer 7 FIFO overflow"]
    #[inline(always)]
    pub fn ov7(&self) -> OV7_R {
        OV7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Sequencer 0 FIFO DMA request overflow"]
    #[inline(always)]
    pub fn dov0(&self) -> DOV0_R {
        DOV0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Sequencer 1 FIFO DMA request overflow"]
    #[inline(always)]
    pub fn dov1(&self) -> DOV1_R {
        DOV1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Sequencer 2 FIFO DMA request overflow"]
    #[inline(always)]
    pub fn dov2(&self) -> DOV2_R {
        DOV2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Sequencer 3 FIFO DMA request overflow"]
    #[inline(always)]
    pub fn dov3(&self) -> DOV3_R {
        DOV3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Sequencer 4 FIFO DMA request overflow"]
    #[inline(always)]
    pub fn dov4(&self) -> DOV4_R {
        DOV4_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Sequencer 5 FIFO DMA request overflow"]
    #[inline(always)]
    pub fn dov5(&self) -> DOV5_R {
        DOV5_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Sequencer 6 FIFO DMA request overflow"]
    #[inline(always)]
    pub fn dov6(&self) -> DOV6_R {
        DOV6_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Sequencer 7 FIFO DMA request overflow"]
    #[inline(always)]
    pub fn dov7(&self) -> DOV7_R {
        DOV7_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Sequencer 0 FIFO overflow"]
    #[inline(always)]
    pub fn ov0(&mut self) -> OV0_W {
        OV0_W { w: self }
    }
    #[doc = "Bit 1 - Sequencer 1 FIFO overflow"]
    #[inline(always)]
    pub fn ov1(&mut self) -> OV1_W {
        OV1_W { w: self }
    }
    #[doc = "Bit 2 - Sequencer 2 FIFO overflow"]
    #[inline(always)]
    pub fn ov2(&mut self) -> OV2_W {
        OV2_W { w: self }
    }
    #[doc = "Bit 3 - Sequencer 3 FIFO overflow"]
    #[inline(always)]
    pub fn ov3(&mut self) -> OV3_W {
        OV3_W { w: self }
    }
    #[doc = "Bit 4 - Sequencer 4 FIFO overflow"]
    #[inline(always)]
    pub fn ov4(&mut self) -> OV4_W {
        OV4_W { w: self }
    }
    #[doc = "Bit 5 - Sequencer 5 FIFO overflow"]
    #[inline(always)]
    pub fn ov5(&mut self) -> OV5_W {
        OV5_W { w: self }
    }
    #[doc = "Bit 6 - Sequencer 6 FIFO overflow"]
    #[inline(always)]
    pub fn ov6(&mut self) -> OV6_W {
        OV6_W { w: self }
    }
    #[doc = "Bit 7 - Sequencer 7 FIFO overflow"]
    #[inline(always)]
    pub fn ov7(&mut self) -> OV7_W {
        OV7_W { w: self }
    }
    #[doc = "Bit 16 - Sequencer 0 FIFO DMA request overflow"]
    #[inline(always)]
    pub fn dov0(&mut self) -> DOV0_W {
        DOV0_W { w: self }
    }
    #[doc = "Bit 17 - Sequencer 1 FIFO DMA request overflow"]
    #[inline(always)]
    pub fn dov1(&mut self) -> DOV1_W {
        DOV1_W { w: self }
    }
    #[doc = "Bit 18 - Sequencer 2 FIFO DMA request overflow"]
    #[inline(always)]
    pub fn dov2(&mut self) -> DOV2_W {
        DOV2_W { w: self }
    }
    #[doc = "Bit 19 - Sequencer 3 FIFO DMA request overflow"]
    #[inline(always)]
    pub fn dov3(&mut self) -> DOV3_W {
        DOV3_W { w: self }
    }
    #[doc = "Bit 20 - Sequencer 4 FIFO DMA request overflow"]
    #[inline(always)]
    pub fn dov4(&mut self) -> DOV4_W {
        DOV4_W { w: self }
    }
    #[doc = "Bit 21 - Sequencer 5 FIFO DMA request overflow"]
    #[inline(always)]
    pub fn dov5(&mut self) -> DOV5_W {
        DOV5_W { w: self }
    }
    #[doc = "Bit 22 - Sequencer 6 FIFO DMA request overflow"]
    #[inline(always)]
    pub fn dov6(&mut self) -> DOV6_W {
        DOV6_W { w: self }
    }
    #[doc = "Bit 23 - Sequencer 7 FIFO DMA request overflow"]
    #[inline(always)]
    pub fn dov7(&mut self) -> DOV7_W {
        DOV7_W { w: self }
    }
}
