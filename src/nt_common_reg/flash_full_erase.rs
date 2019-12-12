#[doc = "Reader of register FLASH_FULL_ERASE"]
pub type R = crate::R<u32, super::FLASH_FULL_ERASE>;
#[doc = "Writer for register FLASH_FULL_ERASE"]
pub type W = crate::W<u32, super::FLASH_FULL_ERASE>;
#[doc = "Register FLASH_FULL_ERASE `reset()`'s with value 0"]
impl crate::ResetValue for super::FLASH_FULL_ERASE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ERASE_FLASH`"]
pub type ERASE_FLASH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERASE_FLASH`"]
pub struct ERASE_FLASH_W<'a> {
    w: &'a mut W,
}
impl<'a> ERASE_FLASH_W<'a> {
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
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn erase_flash(&self) -> ERASE_FLASH_R {
        ERASE_FLASH_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn erase_flash(&mut self) -> ERASE_FLASH_W {
        ERASE_FLASH_W { w: self }
    }
}
