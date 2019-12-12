#[doc = "Reader of register MADR"]
pub type R = crate::R<u32, super::MADR>;
#[doc = "Writer for register MADR"]
pub type W = crate::W<u32, super::MADR>;
#[doc = "Register MADR `reset()`'s with value 0"]
impl crate::ResetValue for super::MADR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `REG_ADDRESS`"]
pub type REG_ADDRESS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `REG_ADDRESS`"]
pub struct REG_ADDRESS_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_ADDRESS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Reader of field `PHY_ADDRESS`"]
pub type PHY_ADDRESS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PHY_ADDRESS`"]
pub struct PHY_ADDRESS_W<'a> {
    w: &'a mut W,
}
impl<'a> PHY_ADDRESS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn reg_address(&self) -> REG_ADDRESS_R {
        REG_ADDRESS_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12"]
    #[inline(always)]
    pub fn phy_address(&self) -> PHY_ADDRESS_R {
        PHY_ADDRESS_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn reg_address(&mut self) -> REG_ADDRESS_W {
        REG_ADDRESS_W { w: self }
    }
    #[doc = "Bits 8:12"]
    #[inline(always)]
    pub fn phy_address(&mut self) -> PHY_ADDRESS_W {
        PHY_ADDRESS_W { w: self }
    }
}
