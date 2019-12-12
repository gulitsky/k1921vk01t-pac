#[doc = "Reader of register USB_EP_CFG"]
pub type R = crate::R<u32, super::USB_EP_CFG>;
#[doc = "Writer for register USB_EP_CFG"]
pub type W = crate::W<u32, super::USB_EP_CFG>;
#[doc = "Register USB_EP_CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::USB_EP_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EP_VALID`"]
pub type EP_VALID_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP_VALID`"]
pub struct EP_VALID_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_VALID_W<'a> {
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
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EP_TYPE_A {
    #[doc = "1: Bulk"]
    BULK,
    #[doc = "2: Interrupt"]
    INT,
    #[doc = "3: Isochronous"]
    ISOCHRON,
}
impl From<EP_TYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: EP_TYPE_A) -> Self {
        match variant {
            EP_TYPE_A::BULK => 1,
            EP_TYPE_A::INT => 2,
            EP_TYPE_A::ISOCHRON => 3,
        }
    }
}
#[doc = "Reader of field `EP_TYPE`"]
pub type EP_TYPE_R = crate::R<u8, EP_TYPE_A>;
impl EP_TYPE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, EP_TYPE_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(EP_TYPE_A::BULK),
            2 => Val(EP_TYPE_A::INT),
            3 => Val(EP_TYPE_A::ISOCHRON),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BULK`"]
    #[inline(always)]
    pub fn is_bulk(&self) -> bool {
        *self == EP_TYPE_A::BULK
    }
    #[doc = "Checks if the value of the field is `INT`"]
    #[inline(always)]
    pub fn is_int(&self) -> bool {
        *self == EP_TYPE_A::INT
    }
    #[doc = "Checks if the value of the field is `ISOCHRON`"]
    #[inline(always)]
    pub fn is_isochron(&self) -> bool {
        *self == EP_TYPE_A::ISOCHRON
    }
}
#[doc = "Write proxy for field `EP_TYPE`"]
pub struct EP_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_TYPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EP_TYPE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Bulk"]
    #[inline(always)]
    pub fn bulk(self) -> &'a mut W {
        self.variant(EP_TYPE_A::BULK)
    }
    #[doc = "Interrupt"]
    #[inline(always)]
    pub fn int(self) -> &'a mut W {
        self.variant(EP_TYPE_A::INT)
    }
    #[doc = "Isochronous"]
    #[inline(always)]
    pub fn isochron(self) -> &'a mut W {
        self.variant(EP_TYPE_A::ISOCHRON)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u32) & 0x03) << 1);
        self.w
    }
}
#[doc = "Field of type (direction) buffer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EP_DIR_A {
    #[doc = "0: Out"]
    OUT,
    #[doc = "1: In"]
    IN,
}
impl From<EP_DIR_A> for bool {
    #[inline(always)]
    fn from(variant: EP_DIR_A) -> Self {
        match variant {
            EP_DIR_A::OUT => false,
            EP_DIR_A::IN => true,
        }
    }
}
#[doc = "Reader of field `EP_DIR`"]
pub type EP_DIR_R = crate::R<bool, EP_DIR_A>;
impl EP_DIR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EP_DIR_A {
        match self.bits {
            false => EP_DIR_A::OUT,
            true => EP_DIR_A::IN,
        }
    }
    #[doc = "Checks if the value of the field is `OUT`"]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        *self == EP_DIR_A::OUT
    }
    #[doc = "Checks if the value of the field is `IN`"]
    #[inline(always)]
    pub fn is_in_(&self) -> bool {
        *self == EP_DIR_A::IN
    }
}
#[doc = "Write proxy for field `EP_DIR`"]
pub struct EP_DIR_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_DIR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EP_DIR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Out"]
    #[inline(always)]
    pub fn out(self) -> &'a mut W {
        self.variant(EP_DIR_A::OUT)
    }
    #[doc = "In"]
    #[inline(always)]
    pub fn in_(self) -> &'a mut W {
        self.variant(EP_DIR_A::IN)
    }
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
#[doc = "Reader of field `EP_NUM`"]
pub type EP_NUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EP_NUM`"]
pub struct EP_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MULT_A {
    #[doc = "0: 1 operation"]
    _1,
    #[doc = "1: 2 operation"]
    _2,
    #[doc = "2: 3 operation"]
    _3,
}
impl From<MULT_A> for u8 {
    #[inline(always)]
    fn from(variant: MULT_A) -> Self {
        match variant {
            MULT_A::_1 => 0,
            MULT_A::_2 => 1,
            MULT_A::_3 => 2,
        }
    }
}
#[doc = "Reader of field `MULT`"]
pub type MULT_R = crate::R<u8, MULT_A>;
impl MULT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MULT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MULT_A::_1),
            1 => Val(MULT_A::_2),
            2 => Val(MULT_A::_3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MULT_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == MULT_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        *self == MULT_A::_3
    }
}
#[doc = "Write proxy for field `MULT`"]
pub struct MULT_W<'a> {
    w: &'a mut W,
}
impl<'a> MULT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MULT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "1 operation"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MULT_A::_1)
    }
    #[doc = "2 operation"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(MULT_A::_2)
    }
    #[doc = "3 operation"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(MULT_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Resolution of buffer"]
    #[inline(always)]
    pub fn ep_valid(&self) -> EP_VALID_R {
        EP_VALID_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    pub fn ep_type(&self) -> EP_TYPE_R {
        EP_TYPE_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 3 - Field of type (direction) buffer"]
    #[inline(always)]
    pub fn ep_dir(&self) -> EP_DIR_R {
        EP_DIR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:7 - Number field buffer"]
    #[inline(always)]
    pub fn ep_num(&self) -> EP_NUM_R {
        EP_NUM_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn mult(&self) -> MULT_R {
        MULT_R::new(((self.bits >> 8) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Resolution of buffer"]
    #[inline(always)]
    pub fn ep_valid(&mut self) -> EP_VALID_W {
        EP_VALID_W { w: self }
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    pub fn ep_type(&mut self) -> EP_TYPE_W {
        EP_TYPE_W { w: self }
    }
    #[doc = "Bit 3 - Field of type (direction) buffer"]
    #[inline(always)]
    pub fn ep_dir(&mut self) -> EP_DIR_W {
        EP_DIR_W { w: self }
    }
    #[doc = "Bits 4:7 - Number field buffer"]
    #[inline(always)]
    pub fn ep_num(&mut self) -> EP_NUM_W {
        EP_NUM_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn mult(&mut self) -> MULT_W {
        MULT_W { w: self }
    }
}
