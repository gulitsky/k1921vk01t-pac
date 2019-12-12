#[doc = "Reader of register CHNL_PRI_ALT_SET"]
pub type R = crate::R<u32, super::CHNL_PRI_ALT_SET>;
#[doc = "Writer for register CHNL_PRI_ALT_SET"]
pub type W = crate::W<u32, super::CHNL_PRI_ALT_SET>;
#[doc = "Register CHNL_PRI_ALT_SET `reset()`'s with value 0"]
impl crate::ResetValue for super::CHNL_PRI_ALT_SET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DMA_ALTSET0`"]
pub type DMA_ALTSET0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_ALTSET0`"]
pub struct DMA_ALTSET0_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ALTSET0_W<'a> {
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
#[doc = "Reader of field `DMA_ALTSET1`"]
pub type DMA_ALTSET1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_ALTSET1`"]
pub struct DMA_ALTSET1_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ALTSET1_W<'a> {
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
#[doc = "Reader of field `DMA_ALTSET2`"]
pub type DMA_ALTSET2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_ALTSET2`"]
pub struct DMA_ALTSET2_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ALTSET2_W<'a> {
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
#[doc = "Reader of field `DMA_ALTSET3`"]
pub type DMA_ALTSET3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_ALTSET3`"]
pub struct DMA_ALTSET3_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ALTSET3_W<'a> {
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
#[doc = "Reader of field `DMA_ALTSET4`"]
pub type DMA_ALTSET4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_ALTSET4`"]
pub struct DMA_ALTSET4_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ALTSET4_W<'a> {
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
#[doc = "Reader of field `DMA_ALTSET5`"]
pub type DMA_ALTSET5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_ALTSET5`"]
pub struct DMA_ALTSET5_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ALTSET5_W<'a> {
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
#[doc = "Reader of field `DMA_ALTSET6`"]
pub type DMA_ALTSET6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_ALTSET6`"]
pub struct DMA_ALTSET6_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ALTSET6_W<'a> {
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
#[doc = "Reader of field `DMA_ALTSET7`"]
pub type DMA_ALTSET7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_ALTSET7`"]
pub struct DMA_ALTSET7_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ALTSET7_W<'a> {
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
#[doc = "Reader of field `DMA_ALTSET8`"]
pub type DMA_ALTSET8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_ALTSET8`"]
pub struct DMA_ALTSET8_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ALTSET8_W<'a> {
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
#[doc = "Reader of field `DMA_ALTSET9`"]
pub type DMA_ALTSET9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_ALTSET9`"]
pub struct DMA_ALTSET9_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ALTSET9_W<'a> {
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
#[doc = "Reader of field `DMA_ALTSET10`"]
pub type DMA_ALTSET10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_ALTSET10`"]
pub struct DMA_ALTSET10_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ALTSET10_W<'a> {
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
#[doc = "Reader of field `DMA_ALTSET11`"]
pub type DMA_ALTSET11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_ALTSET11`"]
pub struct DMA_ALTSET11_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ALTSET11_W<'a> {
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
#[doc = "Reader of field `DMA_ALTSET12`"]
pub type DMA_ALTSET12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_ALTSET12`"]
pub struct DMA_ALTSET12_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ALTSET12_W<'a> {
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
#[doc = "Reader of field `DMA_ALTSET13`"]
pub type DMA_ALTSET13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_ALTSET13`"]
pub struct DMA_ALTSET13_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ALTSET13_W<'a> {
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
#[doc = "Reader of field `DMA_ALTSET14`"]
pub type DMA_ALTSET14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_ALTSET14`"]
pub struct DMA_ALTSET14_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ALTSET14_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `DMA_ALTSET15`"]
pub type DMA_ALTSET15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_ALTSET15`"]
pub struct DMA_ALTSET15_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ALTSET15_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `DMA_ALTSET16`"]
pub type DMA_ALTSET16_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_ALTSET16`"]
pub struct DMA_ALTSET16_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ALTSET16_W<'a> {
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
#[doc = "Reader of field `DMA_ALTSET17`"]
pub type DMA_ALTSET17_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_ALTSET17`"]
pub struct DMA_ALTSET17_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ALTSET17_W<'a> {
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
#[doc = "Reader of field `DMA_ALTSET18`"]
pub type DMA_ALTSET18_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_ALTSET18`"]
pub struct DMA_ALTSET18_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ALTSET18_W<'a> {
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
#[doc = "Reader of field `DMA_ALTSET19`"]
pub type DMA_ALTSET19_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_ALTSET19`"]
pub struct DMA_ALTSET19_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ALTSET19_W<'a> {
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
#[doc = "Reader of field `DMA_ALTSET20`"]
pub type DMA_ALTSET20_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_ALTSET20`"]
pub struct DMA_ALTSET20_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ALTSET20_W<'a> {
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
#[doc = "Reader of field `DMA_ALTSET21`"]
pub type DMA_ALTSET21_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_ALTSET21`"]
pub struct DMA_ALTSET21_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ALTSET21_W<'a> {
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
#[doc = "Reader of field `DMA_ALTSET22`"]
pub type DMA_ALTSET22_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_ALTSET22`"]
pub struct DMA_ALTSET22_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ALTSET22_W<'a> {
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
#[doc = "Reader of field `DMA_ALTSET23`"]
pub type DMA_ALTSET23_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_ALTSET23`"]
pub struct DMA_ALTSET23_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ALTSET23_W<'a> {
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
    #[doc = "Bit 0 - Set primary / alternate channel control data structure"]
    #[inline(always)]
    pub fn dma_altset0(&self) -> DMA_ALTSET0_R {
        DMA_ALTSET0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Set primary / alternate channel control data structure"]
    #[inline(always)]
    pub fn dma_altset1(&self) -> DMA_ALTSET1_R {
        DMA_ALTSET1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Set primary / alternate channel control data structure"]
    #[inline(always)]
    pub fn dma_altset2(&self) -> DMA_ALTSET2_R {
        DMA_ALTSET2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Set primary / alternate channel control data structure"]
    #[inline(always)]
    pub fn dma_altset3(&self) -> DMA_ALTSET3_R {
        DMA_ALTSET3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Set primary / alternate channel control data structure"]
    #[inline(always)]
    pub fn dma_altset4(&self) -> DMA_ALTSET4_R {
        DMA_ALTSET4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Set primary / alternate channel control data structure"]
    #[inline(always)]
    pub fn dma_altset5(&self) -> DMA_ALTSET5_R {
        DMA_ALTSET5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Set primary / alternate channel control data structure"]
    #[inline(always)]
    pub fn dma_altset6(&self) -> DMA_ALTSET6_R {
        DMA_ALTSET6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Set primary / alternate channel control data structure"]
    #[inline(always)]
    pub fn dma_altset7(&self) -> DMA_ALTSET7_R {
        DMA_ALTSET7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Set primary / alternate channel control data structure"]
    #[inline(always)]
    pub fn dma_altset8(&self) -> DMA_ALTSET8_R {
        DMA_ALTSET8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Set primary / alternate channel control data structure"]
    #[inline(always)]
    pub fn dma_altset9(&self) -> DMA_ALTSET9_R {
        DMA_ALTSET9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Set primary / alternate channel control data structure"]
    #[inline(always)]
    pub fn dma_altset10(&self) -> DMA_ALTSET10_R {
        DMA_ALTSET10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Set primary / alternate channel control data structure"]
    #[inline(always)]
    pub fn dma_altset11(&self) -> DMA_ALTSET11_R {
        DMA_ALTSET11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Set primary / alternate channel control data structure"]
    #[inline(always)]
    pub fn dma_altset12(&self) -> DMA_ALTSET12_R {
        DMA_ALTSET12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Set primary / alternate channel control data structure"]
    #[inline(always)]
    pub fn dma_altset13(&self) -> DMA_ALTSET13_R {
        DMA_ALTSET13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Set primary / alternate channel control data structure"]
    #[inline(always)]
    pub fn dma_altset14(&self) -> DMA_ALTSET14_R {
        DMA_ALTSET14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Set primary / alternate channel control data structure"]
    #[inline(always)]
    pub fn dma_altset15(&self) -> DMA_ALTSET15_R {
        DMA_ALTSET15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Set primary / alternate channel control data structure"]
    #[inline(always)]
    pub fn dma_altset16(&self) -> DMA_ALTSET16_R {
        DMA_ALTSET16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Set primary / alternate channel control data structure"]
    #[inline(always)]
    pub fn dma_altset17(&self) -> DMA_ALTSET17_R {
        DMA_ALTSET17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Set primary / alternate channel control data structure"]
    #[inline(always)]
    pub fn dma_altset18(&self) -> DMA_ALTSET18_R {
        DMA_ALTSET18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Set primary / alternate channel control data structure"]
    #[inline(always)]
    pub fn dma_altset19(&self) -> DMA_ALTSET19_R {
        DMA_ALTSET19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Set primary / alternate channel control data structure"]
    #[inline(always)]
    pub fn dma_altset20(&self) -> DMA_ALTSET20_R {
        DMA_ALTSET20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Set primary / alternate channel control data structure"]
    #[inline(always)]
    pub fn dma_altset21(&self) -> DMA_ALTSET21_R {
        DMA_ALTSET21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Set primary / alternate channel control data structure"]
    #[inline(always)]
    pub fn dma_altset22(&self) -> DMA_ALTSET22_R {
        DMA_ALTSET22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Set primary / alternate channel control data structure"]
    #[inline(always)]
    pub fn dma_altset23(&self) -> DMA_ALTSET23_R {
        DMA_ALTSET23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set primary / alternate channel control data structure"]
    #[inline(always)]
    pub fn dma_altset0(&mut self) -> DMA_ALTSET0_W {
        DMA_ALTSET0_W { w: self }
    }
    #[doc = "Bit 1 - Set primary / alternate channel control data structure"]
    #[inline(always)]
    pub fn dma_altset1(&mut self) -> DMA_ALTSET1_W {
        DMA_ALTSET1_W { w: self }
    }
    #[doc = "Bit 2 - Set primary / alternate channel control data structure"]
    #[inline(always)]
    pub fn dma_altset2(&mut self) -> DMA_ALTSET2_W {
        DMA_ALTSET2_W { w: self }
    }
    #[doc = "Bit 3 - Set primary / alternate channel control data structure"]
    #[inline(always)]
    pub fn dma_altset3(&mut self) -> DMA_ALTSET3_W {
        DMA_ALTSET3_W { w: self }
    }
    #[doc = "Bit 4 - Set primary / alternate channel control data structure"]
    #[inline(always)]
    pub fn dma_altset4(&mut self) -> DMA_ALTSET4_W {
        DMA_ALTSET4_W { w: self }
    }
    #[doc = "Bit 5 - Set primary / alternate channel control data structure"]
    #[inline(always)]
    pub fn dma_altset5(&mut self) -> DMA_ALTSET5_W {
        DMA_ALTSET5_W { w: self }
    }
    #[doc = "Bit 6 - Set primary / alternate channel control data structure"]
    #[inline(always)]
    pub fn dma_altset6(&mut self) -> DMA_ALTSET6_W {
        DMA_ALTSET6_W { w: self }
    }
    #[doc = "Bit 7 - Set primary / alternate channel control data structure"]
    #[inline(always)]
    pub fn dma_altset7(&mut self) -> DMA_ALTSET7_W {
        DMA_ALTSET7_W { w: self }
    }
    #[doc = "Bit 8 - Set primary / alternate channel control data structure"]
    #[inline(always)]
    pub fn dma_altset8(&mut self) -> DMA_ALTSET8_W {
        DMA_ALTSET8_W { w: self }
    }
    #[doc = "Bit 9 - Set primary / alternate channel control data structure"]
    #[inline(always)]
    pub fn dma_altset9(&mut self) -> DMA_ALTSET9_W {
        DMA_ALTSET9_W { w: self }
    }
    #[doc = "Bit 10 - Set primary / alternate channel control data structure"]
    #[inline(always)]
    pub fn dma_altset10(&mut self) -> DMA_ALTSET10_W {
        DMA_ALTSET10_W { w: self }
    }
    #[doc = "Bit 11 - Set primary / alternate channel control data structure"]
    #[inline(always)]
    pub fn dma_altset11(&mut self) -> DMA_ALTSET11_W {
        DMA_ALTSET11_W { w: self }
    }
    #[doc = "Bit 12 - Set primary / alternate channel control data structure"]
    #[inline(always)]
    pub fn dma_altset12(&mut self) -> DMA_ALTSET12_W {
        DMA_ALTSET12_W { w: self }
    }
    #[doc = "Bit 13 - Set primary / alternate channel control data structure"]
    #[inline(always)]
    pub fn dma_altset13(&mut self) -> DMA_ALTSET13_W {
        DMA_ALTSET13_W { w: self }
    }
    #[doc = "Bit 14 - Set primary / alternate channel control data structure"]
    #[inline(always)]
    pub fn dma_altset14(&mut self) -> DMA_ALTSET14_W {
        DMA_ALTSET14_W { w: self }
    }
    #[doc = "Bit 15 - Set primary / alternate channel control data structure"]
    #[inline(always)]
    pub fn dma_altset15(&mut self) -> DMA_ALTSET15_W {
        DMA_ALTSET15_W { w: self }
    }
    #[doc = "Bit 16 - Set primary / alternate channel control data structure"]
    #[inline(always)]
    pub fn dma_altset16(&mut self) -> DMA_ALTSET16_W {
        DMA_ALTSET16_W { w: self }
    }
    #[doc = "Bit 17 - Set primary / alternate channel control data structure"]
    #[inline(always)]
    pub fn dma_altset17(&mut self) -> DMA_ALTSET17_W {
        DMA_ALTSET17_W { w: self }
    }
    #[doc = "Bit 18 - Set primary / alternate channel control data structure"]
    #[inline(always)]
    pub fn dma_altset18(&mut self) -> DMA_ALTSET18_W {
        DMA_ALTSET18_W { w: self }
    }
    #[doc = "Bit 19 - Set primary / alternate channel control data structure"]
    #[inline(always)]
    pub fn dma_altset19(&mut self) -> DMA_ALTSET19_W {
        DMA_ALTSET19_W { w: self }
    }
    #[doc = "Bit 20 - Set primary / alternate channel control data structure"]
    #[inline(always)]
    pub fn dma_altset20(&mut self) -> DMA_ALTSET20_W {
        DMA_ALTSET20_W { w: self }
    }
    #[doc = "Bit 21 - Set primary / alternate channel control data structure"]
    #[inline(always)]
    pub fn dma_altset21(&mut self) -> DMA_ALTSET21_W {
        DMA_ALTSET21_W { w: self }
    }
    #[doc = "Bit 22 - Set primary / alternate channel control data structure"]
    #[inline(always)]
    pub fn dma_altset22(&mut self) -> DMA_ALTSET22_W {
        DMA_ALTSET22_W { w: self }
    }
    #[doc = "Bit 23 - Set primary / alternate channel control data structure"]
    #[inline(always)]
    pub fn dma_altset23(&mut self) -> DMA_ALTSET23_W {
        DMA_ALTSET23_W { w: self }
    }
}
