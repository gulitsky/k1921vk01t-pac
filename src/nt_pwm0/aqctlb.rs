#[doc = "Reader of register AQCTLB"]
pub type R = crate::R<u32, super::AQCTLB>;
#[doc = "Writer for register AQCTLB"]
pub type W = crate::W<u32, super::AQCTLB>;
#[doc = "Register AQCTLB `reset()`'s with value 0"]
impl crate::ResetValue for super::AQCTLB {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ZRO_A {
    #[doc = "0: no action"]
    NOACTION,
    #[doc = "1: clear PWMA/PWMB"]
    CLEAR,
    #[doc = "2: set PWMA/PWMB"]
    SET,
    #[doc = "3: inverse PWMA/PWMB"]
    TOOGLE,
}
impl From<ZRO_A> for u8 {
    #[inline(always)]
    fn from(variant: ZRO_A) -> Self {
        match variant {
            ZRO_A::NOACTION => 0,
            ZRO_A::CLEAR => 1,
            ZRO_A::SET => 2,
            ZRO_A::TOOGLE => 3,
        }
    }
}
#[doc = "Reader of field `ZRO`"]
pub type ZRO_R = crate::R<u8, ZRO_A>;
impl ZRO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ZRO_A {
        match self.bits {
            0 => ZRO_A::NOACTION,
            1 => ZRO_A::CLEAR,
            2 => ZRO_A::SET,
            3 => ZRO_A::TOOGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NOACTION`"]
    #[inline(always)]
    pub fn is_no_action(&self) -> bool {
        *self == ZRO_A::NOACTION
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == ZRO_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == ZRO_A::SET
    }
    #[doc = "Checks if the value of the field is `TOOGLE`"]
    #[inline(always)]
    pub fn is_toogle(&self) -> bool {
        *self == ZRO_A::TOOGLE
    }
}
#[doc = "Write proxy for field `ZRO`"]
pub struct ZRO_W<'a> {
    w: &'a mut W,
}
impl<'a> ZRO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ZRO_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "no action"]
    #[inline(always)]
    pub fn no_action(self) -> &'a mut W {
        self.variant(ZRO_A::NOACTION)
    }
    #[doc = "clear PWMA/PWMB"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ZRO_A::CLEAR)
    }
    #[doc = "set PWMA/PWMB"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(ZRO_A::SET)
    }
    #[doc = "inverse PWMA/PWMB"]
    #[inline(always)]
    pub fn toogle(self) -> &'a mut W {
        self.variant(ZRO_A::TOOGLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRD_A {
    #[doc = "0: no action"]
    NOACTION,
    #[doc = "1: clear PWMA/PWMB"]
    CLEAR,
    #[doc = "2: set PWMA/PWMB"]
    SET,
    #[doc = "3: inverse PWMA/PWMB"]
    TOOGLE,
}
impl From<PRD_A> for u8 {
    #[inline(always)]
    fn from(variant: PRD_A) -> Self {
        match variant {
            PRD_A::NOACTION => 0,
            PRD_A::CLEAR => 1,
            PRD_A::SET => 2,
            PRD_A::TOOGLE => 3,
        }
    }
}
#[doc = "Reader of field `PRD`"]
pub type PRD_R = crate::R<u8, PRD_A>;
impl PRD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRD_A {
        match self.bits {
            0 => PRD_A::NOACTION,
            1 => PRD_A::CLEAR,
            2 => PRD_A::SET,
            3 => PRD_A::TOOGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NOACTION`"]
    #[inline(always)]
    pub fn is_no_action(&self) -> bool {
        *self == PRD_A::NOACTION
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == PRD_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == PRD_A::SET
    }
    #[doc = "Checks if the value of the field is `TOOGLE`"]
    #[inline(always)]
    pub fn is_toogle(&self) -> bool {
        *self == PRD_A::TOOGLE
    }
}
#[doc = "Write proxy for field `PRD`"]
pub struct PRD_W<'a> {
    w: &'a mut W,
}
impl<'a> PRD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRD_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "no action"]
    #[inline(always)]
    pub fn no_action(self) -> &'a mut W {
        self.variant(PRD_A::NOACTION)
    }
    #[doc = "clear PWMA/PWMB"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PRD_A::CLEAR)
    }
    #[doc = "set PWMA/PWMB"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(PRD_A::SET)
    }
    #[doc = "inverse PWMA/PWMB"]
    #[inline(always)]
    pub fn toogle(self) -> &'a mut W {
        self.variant(PRD_A::TOOGLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAU_A {
    #[doc = "0: no action"]
    NOACTION,
    #[doc = "1: clear PWMA/PWMB"]
    CLEAR,
    #[doc = "2: set PWMA/PWMB"]
    SET,
    #[doc = "3: inverse PWMA/PWMB"]
    TOOGLE,
}
impl From<CAU_A> for u8 {
    #[inline(always)]
    fn from(variant: CAU_A) -> Self {
        match variant {
            CAU_A::NOACTION => 0,
            CAU_A::CLEAR => 1,
            CAU_A::SET => 2,
            CAU_A::TOOGLE => 3,
        }
    }
}
#[doc = "Reader of field `CAU`"]
pub type CAU_R = crate::R<u8, CAU_A>;
impl CAU_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAU_A {
        match self.bits {
            0 => CAU_A::NOACTION,
            1 => CAU_A::CLEAR,
            2 => CAU_A::SET,
            3 => CAU_A::TOOGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NOACTION`"]
    #[inline(always)]
    pub fn is_no_action(&self) -> bool {
        *self == CAU_A::NOACTION
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CAU_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == CAU_A::SET
    }
    #[doc = "Checks if the value of the field is `TOOGLE`"]
    #[inline(always)]
    pub fn is_toogle(&self) -> bool {
        *self == CAU_A::TOOGLE
    }
}
#[doc = "Write proxy for field `CAU`"]
pub struct CAU_W<'a> {
    w: &'a mut W,
}
impl<'a> CAU_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAU_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "no action"]
    #[inline(always)]
    pub fn no_action(self) -> &'a mut W {
        self.variant(CAU_A::NOACTION)
    }
    #[doc = "clear PWMA/PWMB"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CAU_A::CLEAR)
    }
    #[doc = "set PWMA/PWMB"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CAU_A::SET)
    }
    #[doc = "inverse PWMA/PWMB"]
    #[inline(always)]
    pub fn toogle(self) -> &'a mut W {
        self.variant(CAU_A::TOOGLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAD_A {
    #[doc = "0: no action"]
    NOACTION,
    #[doc = "1: clear PWMA/PWMB"]
    CLEAR,
    #[doc = "2: set PWMA/PWMB"]
    SET,
    #[doc = "3: inverse PWMA/PWMB"]
    TOOGLE,
}
impl From<CAD_A> for u8 {
    #[inline(always)]
    fn from(variant: CAD_A) -> Self {
        match variant {
            CAD_A::NOACTION => 0,
            CAD_A::CLEAR => 1,
            CAD_A::SET => 2,
            CAD_A::TOOGLE => 3,
        }
    }
}
#[doc = "Reader of field `CAD`"]
pub type CAD_R = crate::R<u8, CAD_A>;
impl CAD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAD_A {
        match self.bits {
            0 => CAD_A::NOACTION,
            1 => CAD_A::CLEAR,
            2 => CAD_A::SET,
            3 => CAD_A::TOOGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NOACTION`"]
    #[inline(always)]
    pub fn is_no_action(&self) -> bool {
        *self == CAD_A::NOACTION
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CAD_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == CAD_A::SET
    }
    #[doc = "Checks if the value of the field is `TOOGLE`"]
    #[inline(always)]
    pub fn is_toogle(&self) -> bool {
        *self == CAD_A::TOOGLE
    }
}
#[doc = "Write proxy for field `CAD`"]
pub struct CAD_W<'a> {
    w: &'a mut W,
}
impl<'a> CAD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAD_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "no action"]
    #[inline(always)]
    pub fn no_action(self) -> &'a mut W {
        self.variant(CAD_A::NOACTION)
    }
    #[doc = "clear PWMA/PWMB"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CAD_A::CLEAR)
    }
    #[doc = "set PWMA/PWMB"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CAD_A::SET)
    }
    #[doc = "inverse PWMA/PWMB"]
    #[inline(always)]
    pub fn toogle(self) -> &'a mut W {
        self.variant(CAD_A::TOOGLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CBU_A {
    #[doc = "0: no action"]
    NOACTION,
    #[doc = "1: clear PWMA/PWMB"]
    CLEAR,
    #[doc = "2: set PWMA/PWMB"]
    SET,
    #[doc = "3: inverse PWMA/PWMB"]
    TOOGLE,
}
impl From<CBU_A> for u8 {
    #[inline(always)]
    fn from(variant: CBU_A) -> Self {
        match variant {
            CBU_A::NOACTION => 0,
            CBU_A::CLEAR => 1,
            CBU_A::SET => 2,
            CBU_A::TOOGLE => 3,
        }
    }
}
#[doc = "Reader of field `CBU`"]
pub type CBU_R = crate::R<u8, CBU_A>;
impl CBU_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CBU_A {
        match self.bits {
            0 => CBU_A::NOACTION,
            1 => CBU_A::CLEAR,
            2 => CBU_A::SET,
            3 => CBU_A::TOOGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NOACTION`"]
    #[inline(always)]
    pub fn is_no_action(&self) -> bool {
        *self == CBU_A::NOACTION
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CBU_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == CBU_A::SET
    }
    #[doc = "Checks if the value of the field is `TOOGLE`"]
    #[inline(always)]
    pub fn is_toogle(&self) -> bool {
        *self == CBU_A::TOOGLE
    }
}
#[doc = "Write proxy for field `CBU`"]
pub struct CBU_W<'a> {
    w: &'a mut W,
}
impl<'a> CBU_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CBU_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "no action"]
    #[inline(always)]
    pub fn no_action(self) -> &'a mut W {
        self.variant(CBU_A::NOACTION)
    }
    #[doc = "clear PWMA/PWMB"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CBU_A::CLEAR)
    }
    #[doc = "set PWMA/PWMB"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CBU_A::SET)
    }
    #[doc = "inverse PWMA/PWMB"]
    #[inline(always)]
    pub fn toogle(self) -> &'a mut W {
        self.variant(CBU_A::TOOGLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CBD_A {
    #[doc = "0: no action"]
    NOACTION,
    #[doc = "1: clear PWMA/PWMB"]
    CLEAR,
    #[doc = "2: set PWMA/PWMB"]
    SET,
    #[doc = "3: inverse PWMA/PWMB"]
    TOOGLE,
}
impl From<CBD_A> for u8 {
    #[inline(always)]
    fn from(variant: CBD_A) -> Self {
        match variant {
            CBD_A::NOACTION => 0,
            CBD_A::CLEAR => 1,
            CBD_A::SET => 2,
            CBD_A::TOOGLE => 3,
        }
    }
}
#[doc = "Reader of field `CBD`"]
pub type CBD_R = crate::R<u8, CBD_A>;
impl CBD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CBD_A {
        match self.bits {
            0 => CBD_A::NOACTION,
            1 => CBD_A::CLEAR,
            2 => CBD_A::SET,
            3 => CBD_A::TOOGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NOACTION`"]
    #[inline(always)]
    pub fn is_no_action(&self) -> bool {
        *self == CBD_A::NOACTION
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CBD_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == CBD_A::SET
    }
    #[doc = "Checks if the value of the field is `TOOGLE`"]
    #[inline(always)]
    pub fn is_toogle(&self) -> bool {
        *self == CBD_A::TOOGLE
    }
}
#[doc = "Write proxy for field `CBD`"]
pub struct CBD_W<'a> {
    w: &'a mut W,
}
impl<'a> CBD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CBD_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "no action"]
    #[inline(always)]
    pub fn no_action(self) -> &'a mut W {
        self.variant(CBD_A::NOACTION)
    }
    #[doc = "clear PWMA/PWMB"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CBD_A::CLEAR)
    }
    #[doc = "set PWMA/PWMB"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CBD_A::SET)
    }
    #[doc = "inverse PWMA/PWMB"]
    #[inline(always)]
    pub fn toogle(self) -> &'a mut W {
        self.variant(CBD_A::TOOGLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn zro(&self) -> ZRO_R {
        ZRO_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn prd(&self) -> PRD_R {
        PRD_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn cau(&self) -> CAU_R {
        CAU_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn cad(&self) -> CAD_R {
        CAD_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn cbu(&self) -> CBU_R {
        CBU_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn cbd(&self) -> CBD_R {
        CBD_R::new(((self.bits >> 10) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn zro(&mut self) -> ZRO_W {
        ZRO_W { w: self }
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn prd(&mut self) -> PRD_W {
        PRD_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn cau(&mut self) -> CAU_W {
        CAU_W { w: self }
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn cad(&mut self) -> CAD_W {
        CAD_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn cbu(&mut self) -> CBU_W {
        CBU_W { w: self }
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn cbd(&mut self) -> CBD_W {
        CBD_W { w: self }
    }
}
