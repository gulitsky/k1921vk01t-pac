#[doc = "Writer for register FMC"]
pub type W = crate::W<u32, super::FMC>;
#[doc = "Register FMC `reset()`'s with value 0"]
impl crate::ResetValue for super::FMC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `WRITE`"]
pub struct WRITE_W<'a> {
    w: &'a mut W,
}
impl<'a> WRITE_W<'a> {
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
#[doc = "Write proxy for field `PAGE_ERASE`"]
pub struct PAGE_ERASE_W<'a> {
    w: &'a mut W,
}
impl<'a> PAGE_ERASE_W<'a> {
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
#[doc = "Write proxy for field `FULL_ERASE`"]
pub struct FULL_ERASE_W<'a> {
    w: &'a mut W,
}
impl<'a> FULL_ERASE_W<'a> {
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
#[doc = "Write proxy for field `WRITE_IFB`"]
pub struct WRITE_IFB_W<'a> {
    w: &'a mut W,
}
impl<'a> WRITE_IFB_W<'a> {
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
#[doc = "Write proxy for field `PAGEERASE_IFB`"]
pub struct PAGEERASE_IFB_W<'a> {
    w: &'a mut W,
}
impl<'a> PAGEERASE_IFB_W<'a> {
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
#[doc = "Write proxy for field `MAGIC_KEY`"]
pub struct MAGIC_KEY_W<'a> {
    w: &'a mut W,
}
impl<'a> MAGIC_KEY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Bit writing in main block"]
    #[inline(always)]
    pub fn write(&mut self) -> WRITE_W {
        WRITE_W { w: self }
    }
    #[doc = "Bit 1 - Bit paged erase the main block"]
    #[inline(always)]
    pub fn page_erase(&mut self) -> PAGE_ERASE_W {
        PAGE_ERASE_W { w: self }
    }
    #[doc = "Bit 2 - Bit erase main block"]
    #[inline(always)]
    pub fn full_erase(&mut self) -> FULL_ERASE_W {
        FULL_ERASE_W { w: self }
    }
    #[doc = "Bit 4 - Bit writing on information block"]
    #[inline(always)]
    pub fn write_ifb(&mut self) -> WRITE_IFB_W {
        WRITE_IFB_W { w: self }
    }
    #[doc = "Bit 5 - Bit erase page of information block"]
    #[inline(always)]
    pub fn pageerase_ifb(&mut self) -> PAGEERASE_IFB_W {
        PAGEERASE_IFB_W { w: self }
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn magic_key(&mut self) -> MAGIC_KEY_W {
        MAGIC_KEY_W { w: self }
    }
}
