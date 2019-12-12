#[doc = "Reader of register SA1"]
pub type R = crate::R<u32, super::SA1>;
#[doc = "Writer for register SA1"]
pub type W = crate::W<u32, super::SA1>;
#[doc = "Register SA1 `reset()`'s with value 0"]
impl crate::ResetValue for super::SA1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `STATION_ADDRESS_4B`"]
pub type STATION_ADDRESS_4B_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `STATION_ADDRESS_4B`"]
pub struct STATION_ADDRESS_4B_W<'a> {
    w: &'a mut W,
}
impl<'a> STATION_ADDRESS_4B_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `STATION_ADDRESS_3B`"]
pub type STATION_ADDRESS_3B_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `STATION_ADDRESS_3B`"]
pub struct STATION_ADDRESS_3B_W<'a> {
    w: &'a mut W,
}
impl<'a> STATION_ADDRESS_3B_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn station_address_4b(&self) -> STATION_ADDRESS_4B_R {
        STATION_ADDRESS_4B_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn station_address_3b(&self) -> STATION_ADDRESS_3B_R {
        STATION_ADDRESS_3B_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn station_address_4b(&mut self) -> STATION_ADDRESS_4B_W {
        STATION_ADDRESS_4B_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn station_address_3b(&mut self) -> STATION_ADDRESS_3B_W {
        STATION_ADDRESS_3B_W { w: self }
    }
}
