#[doc = "Reader of register IPGR"]
pub type R = crate::R<u32, super::IPGR>;
#[doc = "Writer for register IPGR"]
pub type W = crate::W<u32, super::IPGR>;
#[doc = "Register IPGR `reset()`'s with value 0"]
impl crate::ResetValue for super::IPGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `NBTBIPG_TRANS_PART2`"]
pub type NBTBIPG_TRANS_PART2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NBTBIPG_TRANS_PART2`"]
pub struct NBTBIPG_TRANS_PART2_W<'a> {
    w: &'a mut W,
}
impl<'a> NBTBIPG_TRANS_PART2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `NBTBIPG_TRANS_PART1`"]
pub type NBTBIPG_TRANS_PART1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NBTBIPG_TRANS_PART1`"]
pub struct NBTBIPG_TRANS_PART1_W<'a> {
    w: &'a mut W,
}
impl<'a> NBTBIPG_TRANS_PART1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u32) & 0x7f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn nbtbipg_trans_part2(&self) -> NBTBIPG_TRANS_PART2_R {
        NBTBIPG_TRANS_PART2_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14"]
    #[inline(always)]
    pub fn nbtbipg_trans_part1(&self) -> NBTBIPG_TRANS_PART1_R {
        NBTBIPG_TRANS_PART1_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn nbtbipg_trans_part2(&mut self) -> NBTBIPG_TRANS_PART2_W {
        NBTBIPG_TRANS_PART2_W { w: self }
    }
    #[doc = "Bits 8:14"]
    #[inline(always)]
    pub fn nbtbipg_trans_part1(&mut self) -> NBTBIPG_TRANS_PART1_W {
        NBTBIPG_TRANS_PART1_W { w: self }
    }
}
