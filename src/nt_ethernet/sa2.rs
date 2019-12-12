#[doc = "Reader of register SA2"]
pub type R = crate::R<u32, super::SA2>;
#[doc = "Writer for register SA2"]
pub type W = crate::W<u32, super::SA2>;
#[doc = "Register SA2 `reset()`'s with value 0"]
impl crate::ResetValue for super::SA2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `STATION_ADDRESS_6B`"]
pub type STATION_ADDRESS_6B_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `STATION_ADDRESS_6B`"]
pub struct STATION_ADDRESS_6B_W<'a> {
    w: &'a mut W,
}
impl<'a> STATION_ADDRESS_6B_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `STATION_ADDRESS_5B`"]
pub type STATION_ADDRESS_5B_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `STATION_ADDRESS_5B`"]
pub struct STATION_ADDRESS_5B_W<'a> {
    w: &'a mut W,
}
impl<'a> STATION_ADDRESS_5B_W<'a> {
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
    pub fn station_address_6b(&self) -> STATION_ADDRESS_6B_R {
        STATION_ADDRESS_6B_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn station_address_5b(&self) -> STATION_ADDRESS_5B_R {
        STATION_ADDRESS_5B_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn station_address_6b(&mut self) -> STATION_ADDRESS_6B_W {
        STATION_ADDRESS_6B_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn station_address_5b(&mut self) -> STATION_ADDRESS_5B_W {
        STATION_ADDRESS_5B_W { w: self }
    }
}
