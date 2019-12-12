#[doc = "Reader of register GPIOPCTLG"]
pub type R = crate::R<u32, super::GPIOPCTLG>;
#[doc = "Writer for register GPIOPCTLG"]
pub type W = crate::W<u32, super::GPIOPCTLG>;
#[doc = "Register GPIOPCTLG `reset()`'s with value 0"]
impl crate::ResetValue for super::GPIOPCTLG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Pin G0 alternative function selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN0_A {
    #[doc = "0: first alternative function for pin"]
    ALTFUNC1,
    #[doc = "1: second alternative function for pin"]
    ALTFUNC2,
    #[doc = "2: third alternative function for pin"]
    ALTFUNC3,
}
impl From<PIN0_A> for u8 {
    #[inline(always)]
    fn from(variant: PIN0_A) -> Self {
        match variant {
            PIN0_A::ALTFUNC1 => 0,
            PIN0_A::ALTFUNC2 => 1,
            PIN0_A::ALTFUNC3 => 2,
        }
    }
}
#[doc = "Reader of field `PIN0`"]
pub type PIN0_R = crate::R<u8, PIN0_A>;
impl PIN0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PIN0_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PIN0_A::ALTFUNC1),
            1 => Val(PIN0_A::ALTFUNC2),
            2 => Val(PIN0_A::ALTFUNC3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ALTFUNC1`"]
    #[inline(always)]
    pub fn is_alt_func1(&self) -> bool {
        *self == PIN0_A::ALTFUNC1
    }
    #[doc = "Checks if the value of the field is `ALTFUNC2`"]
    #[inline(always)]
    pub fn is_alt_func2(&self) -> bool {
        *self == PIN0_A::ALTFUNC2
    }
    #[doc = "Checks if the value of the field is `ALTFUNC3`"]
    #[inline(always)]
    pub fn is_alt_func3(&self) -> bool {
        *self == PIN0_A::ALTFUNC3
    }
}
#[doc = "Write proxy for field `PIN0`"]
pub struct PIN0_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIN0_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "first alternative function for pin"]
    #[inline(always)]
    pub fn alt_func1(self) -> &'a mut W {
        self.variant(PIN0_A::ALTFUNC1)
    }
    #[doc = "second alternative function for pin"]
    #[inline(always)]
    pub fn alt_func2(self) -> &'a mut W {
        self.variant(PIN0_A::ALTFUNC2)
    }
    #[doc = "third alternative function for pin"]
    #[inline(always)]
    pub fn alt_func3(self) -> &'a mut W {
        self.variant(PIN0_A::ALTFUNC3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Pin G1 alternative function selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN1_A {
    #[doc = "0: first alternative function for pin"]
    ALTFUNC1,
    #[doc = "1: second alternative function for pin"]
    ALTFUNC2,
    #[doc = "2: third alternative function for pin"]
    ALTFUNC3,
}
impl From<PIN1_A> for u8 {
    #[inline(always)]
    fn from(variant: PIN1_A) -> Self {
        match variant {
            PIN1_A::ALTFUNC1 => 0,
            PIN1_A::ALTFUNC2 => 1,
            PIN1_A::ALTFUNC3 => 2,
        }
    }
}
#[doc = "Reader of field `PIN1`"]
pub type PIN1_R = crate::R<u8, PIN1_A>;
impl PIN1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PIN1_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PIN1_A::ALTFUNC1),
            1 => Val(PIN1_A::ALTFUNC2),
            2 => Val(PIN1_A::ALTFUNC3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ALTFUNC1`"]
    #[inline(always)]
    pub fn is_alt_func1(&self) -> bool {
        *self == PIN1_A::ALTFUNC1
    }
    #[doc = "Checks if the value of the field is `ALTFUNC2`"]
    #[inline(always)]
    pub fn is_alt_func2(&self) -> bool {
        *self == PIN1_A::ALTFUNC2
    }
    #[doc = "Checks if the value of the field is `ALTFUNC3`"]
    #[inline(always)]
    pub fn is_alt_func3(&self) -> bool {
        *self == PIN1_A::ALTFUNC3
    }
}
#[doc = "Write proxy for field `PIN1`"]
pub struct PIN1_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIN1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "first alternative function for pin"]
    #[inline(always)]
    pub fn alt_func1(self) -> &'a mut W {
        self.variant(PIN1_A::ALTFUNC1)
    }
    #[doc = "second alternative function for pin"]
    #[inline(always)]
    pub fn alt_func2(self) -> &'a mut W {
        self.variant(PIN1_A::ALTFUNC2)
    }
    #[doc = "third alternative function for pin"]
    #[inline(always)]
    pub fn alt_func3(self) -> &'a mut W {
        self.variant(PIN1_A::ALTFUNC3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Pin G2 alternative function selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN2_A {
    #[doc = "0: first alternative function for pin"]
    ALTFUNC1,
    #[doc = "1: second alternative function for pin"]
    ALTFUNC2,
    #[doc = "2: third alternative function for pin"]
    ALTFUNC3,
}
impl From<PIN2_A> for u8 {
    #[inline(always)]
    fn from(variant: PIN2_A) -> Self {
        match variant {
            PIN2_A::ALTFUNC1 => 0,
            PIN2_A::ALTFUNC2 => 1,
            PIN2_A::ALTFUNC3 => 2,
        }
    }
}
#[doc = "Reader of field `PIN2`"]
pub type PIN2_R = crate::R<u8, PIN2_A>;
impl PIN2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PIN2_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PIN2_A::ALTFUNC1),
            1 => Val(PIN2_A::ALTFUNC2),
            2 => Val(PIN2_A::ALTFUNC3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ALTFUNC1`"]
    #[inline(always)]
    pub fn is_alt_func1(&self) -> bool {
        *self == PIN2_A::ALTFUNC1
    }
    #[doc = "Checks if the value of the field is `ALTFUNC2`"]
    #[inline(always)]
    pub fn is_alt_func2(&self) -> bool {
        *self == PIN2_A::ALTFUNC2
    }
    #[doc = "Checks if the value of the field is `ALTFUNC3`"]
    #[inline(always)]
    pub fn is_alt_func3(&self) -> bool {
        *self == PIN2_A::ALTFUNC3
    }
}
#[doc = "Write proxy for field `PIN2`"]
pub struct PIN2_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIN2_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "first alternative function for pin"]
    #[inline(always)]
    pub fn alt_func1(self) -> &'a mut W {
        self.variant(PIN2_A::ALTFUNC1)
    }
    #[doc = "second alternative function for pin"]
    #[inline(always)]
    pub fn alt_func2(self) -> &'a mut W {
        self.variant(PIN2_A::ALTFUNC2)
    }
    #[doc = "third alternative function for pin"]
    #[inline(always)]
    pub fn alt_func3(self) -> &'a mut W {
        self.variant(PIN2_A::ALTFUNC3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Pin G3 alternative function selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN3_A {
    #[doc = "0: first alternative function for pin"]
    ALTFUNC1,
    #[doc = "1: second alternative function for pin"]
    ALTFUNC2,
    #[doc = "2: third alternative function for pin"]
    ALTFUNC3,
}
impl From<PIN3_A> for u8 {
    #[inline(always)]
    fn from(variant: PIN3_A) -> Self {
        match variant {
            PIN3_A::ALTFUNC1 => 0,
            PIN3_A::ALTFUNC2 => 1,
            PIN3_A::ALTFUNC3 => 2,
        }
    }
}
#[doc = "Reader of field `PIN3`"]
pub type PIN3_R = crate::R<u8, PIN3_A>;
impl PIN3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PIN3_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PIN3_A::ALTFUNC1),
            1 => Val(PIN3_A::ALTFUNC2),
            2 => Val(PIN3_A::ALTFUNC3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ALTFUNC1`"]
    #[inline(always)]
    pub fn is_alt_func1(&self) -> bool {
        *self == PIN3_A::ALTFUNC1
    }
    #[doc = "Checks if the value of the field is `ALTFUNC2`"]
    #[inline(always)]
    pub fn is_alt_func2(&self) -> bool {
        *self == PIN3_A::ALTFUNC2
    }
    #[doc = "Checks if the value of the field is `ALTFUNC3`"]
    #[inline(always)]
    pub fn is_alt_func3(&self) -> bool {
        *self == PIN3_A::ALTFUNC3
    }
}
#[doc = "Write proxy for field `PIN3`"]
pub struct PIN3_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIN3_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "first alternative function for pin"]
    #[inline(always)]
    pub fn alt_func1(self) -> &'a mut W {
        self.variant(PIN3_A::ALTFUNC1)
    }
    #[doc = "second alternative function for pin"]
    #[inline(always)]
    pub fn alt_func2(self) -> &'a mut W {
        self.variant(PIN3_A::ALTFUNC2)
    }
    #[doc = "third alternative function for pin"]
    #[inline(always)]
    pub fn alt_func3(self) -> &'a mut W {
        self.variant(PIN3_A::ALTFUNC3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Pin G4 alternative function selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN4_A {
    #[doc = "0: first alternative function for pin"]
    ALTFUNC1,
    #[doc = "1: second alternative function for pin"]
    ALTFUNC2,
    #[doc = "2: third alternative function for pin"]
    ALTFUNC3,
}
impl From<PIN4_A> for u8 {
    #[inline(always)]
    fn from(variant: PIN4_A) -> Self {
        match variant {
            PIN4_A::ALTFUNC1 => 0,
            PIN4_A::ALTFUNC2 => 1,
            PIN4_A::ALTFUNC3 => 2,
        }
    }
}
#[doc = "Reader of field `PIN4`"]
pub type PIN4_R = crate::R<u8, PIN4_A>;
impl PIN4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PIN4_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PIN4_A::ALTFUNC1),
            1 => Val(PIN4_A::ALTFUNC2),
            2 => Val(PIN4_A::ALTFUNC3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ALTFUNC1`"]
    #[inline(always)]
    pub fn is_alt_func1(&self) -> bool {
        *self == PIN4_A::ALTFUNC1
    }
    #[doc = "Checks if the value of the field is `ALTFUNC2`"]
    #[inline(always)]
    pub fn is_alt_func2(&self) -> bool {
        *self == PIN4_A::ALTFUNC2
    }
    #[doc = "Checks if the value of the field is `ALTFUNC3`"]
    #[inline(always)]
    pub fn is_alt_func3(&self) -> bool {
        *self == PIN4_A::ALTFUNC3
    }
}
#[doc = "Write proxy for field `PIN4`"]
pub struct PIN4_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIN4_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "first alternative function for pin"]
    #[inline(always)]
    pub fn alt_func1(self) -> &'a mut W {
        self.variant(PIN4_A::ALTFUNC1)
    }
    #[doc = "second alternative function for pin"]
    #[inline(always)]
    pub fn alt_func2(self) -> &'a mut W {
        self.variant(PIN4_A::ALTFUNC2)
    }
    #[doc = "third alternative function for pin"]
    #[inline(always)]
    pub fn alt_func3(self) -> &'a mut W {
        self.variant(PIN4_A::ALTFUNC3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Pin G5 alternative function selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN5_A {
    #[doc = "0: first alternative function for pin"]
    ALTFUNC1,
    #[doc = "1: second alternative function for pin"]
    ALTFUNC2,
    #[doc = "2: third alternative function for pin"]
    ALTFUNC3,
}
impl From<PIN5_A> for u8 {
    #[inline(always)]
    fn from(variant: PIN5_A) -> Self {
        match variant {
            PIN5_A::ALTFUNC1 => 0,
            PIN5_A::ALTFUNC2 => 1,
            PIN5_A::ALTFUNC3 => 2,
        }
    }
}
#[doc = "Reader of field `PIN5`"]
pub type PIN5_R = crate::R<u8, PIN5_A>;
impl PIN5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PIN5_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PIN5_A::ALTFUNC1),
            1 => Val(PIN5_A::ALTFUNC2),
            2 => Val(PIN5_A::ALTFUNC3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ALTFUNC1`"]
    #[inline(always)]
    pub fn is_alt_func1(&self) -> bool {
        *self == PIN5_A::ALTFUNC1
    }
    #[doc = "Checks if the value of the field is `ALTFUNC2`"]
    #[inline(always)]
    pub fn is_alt_func2(&self) -> bool {
        *self == PIN5_A::ALTFUNC2
    }
    #[doc = "Checks if the value of the field is `ALTFUNC3`"]
    #[inline(always)]
    pub fn is_alt_func3(&self) -> bool {
        *self == PIN5_A::ALTFUNC3
    }
}
#[doc = "Write proxy for field `PIN5`"]
pub struct PIN5_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIN5_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "first alternative function for pin"]
    #[inline(always)]
    pub fn alt_func1(self) -> &'a mut W {
        self.variant(PIN5_A::ALTFUNC1)
    }
    #[doc = "second alternative function for pin"]
    #[inline(always)]
    pub fn alt_func2(self) -> &'a mut W {
        self.variant(PIN5_A::ALTFUNC2)
    }
    #[doc = "third alternative function for pin"]
    #[inline(always)]
    pub fn alt_func3(self) -> &'a mut W {
        self.variant(PIN5_A::ALTFUNC3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Pin G6 alternative function selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN6_A {
    #[doc = "0: first alternative function for pin"]
    ALTFUNC1,
    #[doc = "1: second alternative function for pin"]
    ALTFUNC2,
    #[doc = "2: third alternative function for pin"]
    ALTFUNC3,
}
impl From<PIN6_A> for u8 {
    #[inline(always)]
    fn from(variant: PIN6_A) -> Self {
        match variant {
            PIN6_A::ALTFUNC1 => 0,
            PIN6_A::ALTFUNC2 => 1,
            PIN6_A::ALTFUNC3 => 2,
        }
    }
}
#[doc = "Reader of field `PIN6`"]
pub type PIN6_R = crate::R<u8, PIN6_A>;
impl PIN6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PIN6_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PIN6_A::ALTFUNC1),
            1 => Val(PIN6_A::ALTFUNC2),
            2 => Val(PIN6_A::ALTFUNC3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ALTFUNC1`"]
    #[inline(always)]
    pub fn is_alt_func1(&self) -> bool {
        *self == PIN6_A::ALTFUNC1
    }
    #[doc = "Checks if the value of the field is `ALTFUNC2`"]
    #[inline(always)]
    pub fn is_alt_func2(&self) -> bool {
        *self == PIN6_A::ALTFUNC2
    }
    #[doc = "Checks if the value of the field is `ALTFUNC3`"]
    #[inline(always)]
    pub fn is_alt_func3(&self) -> bool {
        *self == PIN6_A::ALTFUNC3
    }
}
#[doc = "Write proxy for field `PIN6`"]
pub struct PIN6_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIN6_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "first alternative function for pin"]
    #[inline(always)]
    pub fn alt_func1(self) -> &'a mut W {
        self.variant(PIN6_A::ALTFUNC1)
    }
    #[doc = "second alternative function for pin"]
    #[inline(always)]
    pub fn alt_func2(self) -> &'a mut W {
        self.variant(PIN6_A::ALTFUNC2)
    }
    #[doc = "third alternative function for pin"]
    #[inline(always)]
    pub fn alt_func3(self) -> &'a mut W {
        self.variant(PIN6_A::ALTFUNC3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Pin G7 alternative function selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN7_A {
    #[doc = "0: first alternative function for pin"]
    ALTFUNC1,
    #[doc = "1: second alternative function for pin"]
    ALTFUNC2,
    #[doc = "2: third alternative function for pin"]
    ALTFUNC3,
}
impl From<PIN7_A> for u8 {
    #[inline(always)]
    fn from(variant: PIN7_A) -> Self {
        match variant {
            PIN7_A::ALTFUNC1 => 0,
            PIN7_A::ALTFUNC2 => 1,
            PIN7_A::ALTFUNC3 => 2,
        }
    }
}
#[doc = "Reader of field `PIN7`"]
pub type PIN7_R = crate::R<u8, PIN7_A>;
impl PIN7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PIN7_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PIN7_A::ALTFUNC1),
            1 => Val(PIN7_A::ALTFUNC2),
            2 => Val(PIN7_A::ALTFUNC3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ALTFUNC1`"]
    #[inline(always)]
    pub fn is_alt_func1(&self) -> bool {
        *self == PIN7_A::ALTFUNC1
    }
    #[doc = "Checks if the value of the field is `ALTFUNC2`"]
    #[inline(always)]
    pub fn is_alt_func2(&self) -> bool {
        *self == PIN7_A::ALTFUNC2
    }
    #[doc = "Checks if the value of the field is `ALTFUNC3`"]
    #[inline(always)]
    pub fn is_alt_func3(&self) -> bool {
        *self == PIN7_A::ALTFUNC3
    }
}
#[doc = "Write proxy for field `PIN7`"]
pub struct PIN7_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIN7_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "first alternative function for pin"]
    #[inline(always)]
    pub fn alt_func1(self) -> &'a mut W {
        self.variant(PIN7_A::ALTFUNC1)
    }
    #[doc = "second alternative function for pin"]
    #[inline(always)]
    pub fn alt_func2(self) -> &'a mut W {
        self.variant(PIN7_A::ALTFUNC2)
    }
    #[doc = "third alternative function for pin"]
    #[inline(always)]
    pub fn alt_func3(self) -> &'a mut W {
        self.variant(PIN7_A::ALTFUNC3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Pin G8 alternative function selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN8_A {
    #[doc = "0: first alternative function for pin"]
    ALTFUNC1,
    #[doc = "1: second alternative function for pin"]
    ALTFUNC2,
    #[doc = "2: third alternative function for pin"]
    ALTFUNC3,
}
impl From<PIN8_A> for u8 {
    #[inline(always)]
    fn from(variant: PIN8_A) -> Self {
        match variant {
            PIN8_A::ALTFUNC1 => 0,
            PIN8_A::ALTFUNC2 => 1,
            PIN8_A::ALTFUNC3 => 2,
        }
    }
}
#[doc = "Reader of field `PIN8`"]
pub type PIN8_R = crate::R<u8, PIN8_A>;
impl PIN8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PIN8_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PIN8_A::ALTFUNC1),
            1 => Val(PIN8_A::ALTFUNC2),
            2 => Val(PIN8_A::ALTFUNC3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ALTFUNC1`"]
    #[inline(always)]
    pub fn is_alt_func1(&self) -> bool {
        *self == PIN8_A::ALTFUNC1
    }
    #[doc = "Checks if the value of the field is `ALTFUNC2`"]
    #[inline(always)]
    pub fn is_alt_func2(&self) -> bool {
        *self == PIN8_A::ALTFUNC2
    }
    #[doc = "Checks if the value of the field is `ALTFUNC3`"]
    #[inline(always)]
    pub fn is_alt_func3(&self) -> bool {
        *self == PIN8_A::ALTFUNC3
    }
}
#[doc = "Write proxy for field `PIN8`"]
pub struct PIN8_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIN8_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "first alternative function for pin"]
    #[inline(always)]
    pub fn alt_func1(self) -> &'a mut W {
        self.variant(PIN8_A::ALTFUNC1)
    }
    #[doc = "second alternative function for pin"]
    #[inline(always)]
    pub fn alt_func2(self) -> &'a mut W {
        self.variant(PIN8_A::ALTFUNC2)
    }
    #[doc = "third alternative function for pin"]
    #[inline(always)]
    pub fn alt_func3(self) -> &'a mut W {
        self.variant(PIN8_A::ALTFUNC3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Pin G9 alternative function selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN9_A {
    #[doc = "0: first alternative function for pin"]
    ALTFUNC1,
    #[doc = "1: second alternative function for pin"]
    ALTFUNC2,
    #[doc = "2: third alternative function for pin"]
    ALTFUNC3,
}
impl From<PIN9_A> for u8 {
    #[inline(always)]
    fn from(variant: PIN9_A) -> Self {
        match variant {
            PIN9_A::ALTFUNC1 => 0,
            PIN9_A::ALTFUNC2 => 1,
            PIN9_A::ALTFUNC3 => 2,
        }
    }
}
#[doc = "Reader of field `PIN9`"]
pub type PIN9_R = crate::R<u8, PIN9_A>;
impl PIN9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PIN9_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PIN9_A::ALTFUNC1),
            1 => Val(PIN9_A::ALTFUNC2),
            2 => Val(PIN9_A::ALTFUNC3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ALTFUNC1`"]
    #[inline(always)]
    pub fn is_alt_func1(&self) -> bool {
        *self == PIN9_A::ALTFUNC1
    }
    #[doc = "Checks if the value of the field is `ALTFUNC2`"]
    #[inline(always)]
    pub fn is_alt_func2(&self) -> bool {
        *self == PIN9_A::ALTFUNC2
    }
    #[doc = "Checks if the value of the field is `ALTFUNC3`"]
    #[inline(always)]
    pub fn is_alt_func3(&self) -> bool {
        *self == PIN9_A::ALTFUNC3
    }
}
#[doc = "Write proxy for field `PIN9`"]
pub struct PIN9_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIN9_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "first alternative function for pin"]
    #[inline(always)]
    pub fn alt_func1(self) -> &'a mut W {
        self.variant(PIN9_A::ALTFUNC1)
    }
    #[doc = "second alternative function for pin"]
    #[inline(always)]
    pub fn alt_func2(self) -> &'a mut W {
        self.variant(PIN9_A::ALTFUNC2)
    }
    #[doc = "third alternative function for pin"]
    #[inline(always)]
    pub fn alt_func3(self) -> &'a mut W {
        self.variant(PIN9_A::ALTFUNC3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Pin G10 alternative function selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN10_A {
    #[doc = "0: first alternative function for pin"]
    ALTFUNC1,
    #[doc = "1: second alternative function for pin"]
    ALTFUNC2,
    #[doc = "2: third alternative function for pin"]
    ALTFUNC3,
}
impl From<PIN10_A> for u8 {
    #[inline(always)]
    fn from(variant: PIN10_A) -> Self {
        match variant {
            PIN10_A::ALTFUNC1 => 0,
            PIN10_A::ALTFUNC2 => 1,
            PIN10_A::ALTFUNC3 => 2,
        }
    }
}
#[doc = "Reader of field `PIN10`"]
pub type PIN10_R = crate::R<u8, PIN10_A>;
impl PIN10_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PIN10_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PIN10_A::ALTFUNC1),
            1 => Val(PIN10_A::ALTFUNC2),
            2 => Val(PIN10_A::ALTFUNC3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ALTFUNC1`"]
    #[inline(always)]
    pub fn is_alt_func1(&self) -> bool {
        *self == PIN10_A::ALTFUNC1
    }
    #[doc = "Checks if the value of the field is `ALTFUNC2`"]
    #[inline(always)]
    pub fn is_alt_func2(&self) -> bool {
        *self == PIN10_A::ALTFUNC2
    }
    #[doc = "Checks if the value of the field is `ALTFUNC3`"]
    #[inline(always)]
    pub fn is_alt_func3(&self) -> bool {
        *self == PIN10_A::ALTFUNC3
    }
}
#[doc = "Write proxy for field `PIN10`"]
pub struct PIN10_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIN10_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "first alternative function for pin"]
    #[inline(always)]
    pub fn alt_func1(self) -> &'a mut W {
        self.variant(PIN10_A::ALTFUNC1)
    }
    #[doc = "second alternative function for pin"]
    #[inline(always)]
    pub fn alt_func2(self) -> &'a mut W {
        self.variant(PIN10_A::ALTFUNC2)
    }
    #[doc = "third alternative function for pin"]
    #[inline(always)]
    pub fn alt_func3(self) -> &'a mut W {
        self.variant(PIN10_A::ALTFUNC3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Pin G11 alternative function selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN11_A {
    #[doc = "0: first alternative function for pin"]
    ALTFUNC1,
    #[doc = "1: second alternative function for pin"]
    ALTFUNC2,
    #[doc = "2: third alternative function for pin"]
    ALTFUNC3,
}
impl From<PIN11_A> for u8 {
    #[inline(always)]
    fn from(variant: PIN11_A) -> Self {
        match variant {
            PIN11_A::ALTFUNC1 => 0,
            PIN11_A::ALTFUNC2 => 1,
            PIN11_A::ALTFUNC3 => 2,
        }
    }
}
#[doc = "Reader of field `PIN11`"]
pub type PIN11_R = crate::R<u8, PIN11_A>;
impl PIN11_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PIN11_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PIN11_A::ALTFUNC1),
            1 => Val(PIN11_A::ALTFUNC2),
            2 => Val(PIN11_A::ALTFUNC3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ALTFUNC1`"]
    #[inline(always)]
    pub fn is_alt_func1(&self) -> bool {
        *self == PIN11_A::ALTFUNC1
    }
    #[doc = "Checks if the value of the field is `ALTFUNC2`"]
    #[inline(always)]
    pub fn is_alt_func2(&self) -> bool {
        *self == PIN11_A::ALTFUNC2
    }
    #[doc = "Checks if the value of the field is `ALTFUNC3`"]
    #[inline(always)]
    pub fn is_alt_func3(&self) -> bool {
        *self == PIN11_A::ALTFUNC3
    }
}
#[doc = "Write proxy for field `PIN11`"]
pub struct PIN11_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIN11_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "first alternative function for pin"]
    #[inline(always)]
    pub fn alt_func1(self) -> &'a mut W {
        self.variant(PIN11_A::ALTFUNC1)
    }
    #[doc = "second alternative function for pin"]
    #[inline(always)]
    pub fn alt_func2(self) -> &'a mut W {
        self.variant(PIN11_A::ALTFUNC2)
    }
    #[doc = "third alternative function for pin"]
    #[inline(always)]
    pub fn alt_func3(self) -> &'a mut W {
        self.variant(PIN11_A::ALTFUNC3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Pin G12 alternative function selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN12_A {
    #[doc = "0: first alternative function for pin"]
    ALTFUNC1,
    #[doc = "1: second alternative function for pin"]
    ALTFUNC2,
    #[doc = "2: third alternative function for pin"]
    ALTFUNC3,
}
impl From<PIN12_A> for u8 {
    #[inline(always)]
    fn from(variant: PIN12_A) -> Self {
        match variant {
            PIN12_A::ALTFUNC1 => 0,
            PIN12_A::ALTFUNC2 => 1,
            PIN12_A::ALTFUNC3 => 2,
        }
    }
}
#[doc = "Reader of field `PIN12`"]
pub type PIN12_R = crate::R<u8, PIN12_A>;
impl PIN12_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PIN12_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PIN12_A::ALTFUNC1),
            1 => Val(PIN12_A::ALTFUNC2),
            2 => Val(PIN12_A::ALTFUNC3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ALTFUNC1`"]
    #[inline(always)]
    pub fn is_alt_func1(&self) -> bool {
        *self == PIN12_A::ALTFUNC1
    }
    #[doc = "Checks if the value of the field is `ALTFUNC2`"]
    #[inline(always)]
    pub fn is_alt_func2(&self) -> bool {
        *self == PIN12_A::ALTFUNC2
    }
    #[doc = "Checks if the value of the field is `ALTFUNC3`"]
    #[inline(always)]
    pub fn is_alt_func3(&self) -> bool {
        *self == PIN12_A::ALTFUNC3
    }
}
#[doc = "Write proxy for field `PIN12`"]
pub struct PIN12_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIN12_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "first alternative function for pin"]
    #[inline(always)]
    pub fn alt_func1(self) -> &'a mut W {
        self.variant(PIN12_A::ALTFUNC1)
    }
    #[doc = "second alternative function for pin"]
    #[inline(always)]
    pub fn alt_func2(self) -> &'a mut W {
        self.variant(PIN12_A::ALTFUNC2)
    }
    #[doc = "third alternative function for pin"]
    #[inline(always)]
    pub fn alt_func3(self) -> &'a mut W {
        self.variant(PIN12_A::ALTFUNC3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Pin G13 alternative function selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN13_A {
    #[doc = "0: first alternative function for pin"]
    ALTFUNC1,
    #[doc = "1: second alternative function for pin"]
    ALTFUNC2,
    #[doc = "2: third alternative function for pin"]
    ALTFUNC3,
}
impl From<PIN13_A> for u8 {
    #[inline(always)]
    fn from(variant: PIN13_A) -> Self {
        match variant {
            PIN13_A::ALTFUNC1 => 0,
            PIN13_A::ALTFUNC2 => 1,
            PIN13_A::ALTFUNC3 => 2,
        }
    }
}
#[doc = "Reader of field `PIN13`"]
pub type PIN13_R = crate::R<u8, PIN13_A>;
impl PIN13_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PIN13_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PIN13_A::ALTFUNC1),
            1 => Val(PIN13_A::ALTFUNC2),
            2 => Val(PIN13_A::ALTFUNC3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ALTFUNC1`"]
    #[inline(always)]
    pub fn is_alt_func1(&self) -> bool {
        *self == PIN13_A::ALTFUNC1
    }
    #[doc = "Checks if the value of the field is `ALTFUNC2`"]
    #[inline(always)]
    pub fn is_alt_func2(&self) -> bool {
        *self == PIN13_A::ALTFUNC2
    }
    #[doc = "Checks if the value of the field is `ALTFUNC3`"]
    #[inline(always)]
    pub fn is_alt_func3(&self) -> bool {
        *self == PIN13_A::ALTFUNC3
    }
}
#[doc = "Write proxy for field `PIN13`"]
pub struct PIN13_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIN13_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "first alternative function for pin"]
    #[inline(always)]
    pub fn alt_func1(self) -> &'a mut W {
        self.variant(PIN13_A::ALTFUNC1)
    }
    #[doc = "second alternative function for pin"]
    #[inline(always)]
    pub fn alt_func2(self) -> &'a mut W {
        self.variant(PIN13_A::ALTFUNC2)
    }
    #[doc = "third alternative function for pin"]
    #[inline(always)]
    pub fn alt_func3(self) -> &'a mut W {
        self.variant(PIN13_A::ALTFUNC3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
#[doc = "Pin G14 alternative function selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN14_A {
    #[doc = "0: first alternative function for pin"]
    ALTFUNC1,
    #[doc = "1: second alternative function for pin"]
    ALTFUNC2,
    #[doc = "2: third alternative function for pin"]
    ALTFUNC3,
}
impl From<PIN14_A> for u8 {
    #[inline(always)]
    fn from(variant: PIN14_A) -> Self {
        match variant {
            PIN14_A::ALTFUNC1 => 0,
            PIN14_A::ALTFUNC2 => 1,
            PIN14_A::ALTFUNC3 => 2,
        }
    }
}
#[doc = "Reader of field `PIN14`"]
pub type PIN14_R = crate::R<u8, PIN14_A>;
impl PIN14_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PIN14_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PIN14_A::ALTFUNC1),
            1 => Val(PIN14_A::ALTFUNC2),
            2 => Val(PIN14_A::ALTFUNC3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ALTFUNC1`"]
    #[inline(always)]
    pub fn is_alt_func1(&self) -> bool {
        *self == PIN14_A::ALTFUNC1
    }
    #[doc = "Checks if the value of the field is `ALTFUNC2`"]
    #[inline(always)]
    pub fn is_alt_func2(&self) -> bool {
        *self == PIN14_A::ALTFUNC2
    }
    #[doc = "Checks if the value of the field is `ALTFUNC3`"]
    #[inline(always)]
    pub fn is_alt_func3(&self) -> bool {
        *self == PIN14_A::ALTFUNC3
    }
}
#[doc = "Write proxy for field `PIN14`"]
pub struct PIN14_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIN14_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "first alternative function for pin"]
    #[inline(always)]
    pub fn alt_func1(self) -> &'a mut W {
        self.variant(PIN14_A::ALTFUNC1)
    }
    #[doc = "second alternative function for pin"]
    #[inline(always)]
    pub fn alt_func2(self) -> &'a mut W {
        self.variant(PIN14_A::ALTFUNC2)
    }
    #[doc = "third alternative function for pin"]
    #[inline(always)]
    pub fn alt_func3(self) -> &'a mut W {
        self.variant(PIN14_A::ALTFUNC3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "Pin G15 alternative function selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN15_A {
    #[doc = "0: first alternative function for pin"]
    ALTFUNC1,
    #[doc = "1: second alternative function for pin"]
    ALTFUNC2,
    #[doc = "2: third alternative function for pin"]
    ALTFUNC3,
}
impl From<PIN15_A> for u8 {
    #[inline(always)]
    fn from(variant: PIN15_A) -> Self {
        match variant {
            PIN15_A::ALTFUNC1 => 0,
            PIN15_A::ALTFUNC2 => 1,
            PIN15_A::ALTFUNC3 => 2,
        }
    }
}
#[doc = "Reader of field `PIN15`"]
pub type PIN15_R = crate::R<u8, PIN15_A>;
impl PIN15_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PIN15_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PIN15_A::ALTFUNC1),
            1 => Val(PIN15_A::ALTFUNC2),
            2 => Val(PIN15_A::ALTFUNC3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ALTFUNC1`"]
    #[inline(always)]
    pub fn is_alt_func1(&self) -> bool {
        *self == PIN15_A::ALTFUNC1
    }
    #[doc = "Checks if the value of the field is `ALTFUNC2`"]
    #[inline(always)]
    pub fn is_alt_func2(&self) -> bool {
        *self == PIN15_A::ALTFUNC2
    }
    #[doc = "Checks if the value of the field is `ALTFUNC3`"]
    #[inline(always)]
    pub fn is_alt_func3(&self) -> bool {
        *self == PIN15_A::ALTFUNC3
    }
}
#[doc = "Write proxy for field `PIN15`"]
pub struct PIN15_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIN15_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "first alternative function for pin"]
    #[inline(always)]
    pub fn alt_func1(self) -> &'a mut W {
        self.variant(PIN15_A::ALTFUNC1)
    }
    #[doc = "second alternative function for pin"]
    #[inline(always)]
    pub fn alt_func2(self) -> &'a mut W {
        self.variant(PIN15_A::ALTFUNC2)
    }
    #[doc = "third alternative function for pin"]
    #[inline(always)]
    pub fn alt_func3(self) -> &'a mut W {
        self.variant(PIN15_A::ALTFUNC3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Pin G0 alternative function selection"]
    #[inline(always)]
    pub fn pin0(&self) -> PIN0_R {
        PIN0_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Pin G1 alternative function selection"]
    #[inline(always)]
    pub fn pin1(&self) -> PIN1_R {
        PIN1_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Pin G2 alternative function selection"]
    #[inline(always)]
    pub fn pin2(&self) -> PIN2_R {
        PIN2_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Pin G3 alternative function selection"]
    #[inline(always)]
    pub fn pin3(&self) -> PIN3_R {
        PIN3_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Pin G4 alternative function selection"]
    #[inline(always)]
    pub fn pin4(&self) -> PIN4_R {
        PIN4_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Pin G5 alternative function selection"]
    #[inline(always)]
    pub fn pin5(&self) -> PIN5_R {
        PIN5_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Pin G6 alternative function selection"]
    #[inline(always)]
    pub fn pin6(&self) -> PIN6_R {
        PIN6_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Pin G7 alternative function selection"]
    #[inline(always)]
    pub fn pin7(&self) -> PIN7_R {
        PIN7_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - Pin G8 alternative function selection"]
    #[inline(always)]
    pub fn pin8(&self) -> PIN8_R {
        PIN8_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - Pin G9 alternative function selection"]
    #[inline(always)]
    pub fn pin9(&self) -> PIN9_R {
        PIN9_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - Pin G10 alternative function selection"]
    #[inline(always)]
    pub fn pin10(&self) -> PIN10_R {
        PIN10_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - Pin G11 alternative function selection"]
    #[inline(always)]
    pub fn pin11(&self) -> PIN11_R {
        PIN11_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - Pin G12 alternative function selection"]
    #[inline(always)]
    pub fn pin12(&self) -> PIN12_R {
        PIN12_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - Pin G13 alternative function selection"]
    #[inline(always)]
    pub fn pin13(&self) -> PIN13_R {
        PIN13_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - Pin G14 alternative function selection"]
    #[inline(always)]
    pub fn pin14(&self) -> PIN14_R {
        PIN14_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - Pin G15 alternative function selection"]
    #[inline(always)]
    pub fn pin15(&self) -> PIN15_R {
        PIN15_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Pin G0 alternative function selection"]
    #[inline(always)]
    pub fn pin0(&mut self) -> PIN0_W {
        PIN0_W { w: self }
    }
    #[doc = "Bits 2:3 - Pin G1 alternative function selection"]
    #[inline(always)]
    pub fn pin1(&mut self) -> PIN1_W {
        PIN1_W { w: self }
    }
    #[doc = "Bits 4:5 - Pin G2 alternative function selection"]
    #[inline(always)]
    pub fn pin2(&mut self) -> PIN2_W {
        PIN2_W { w: self }
    }
    #[doc = "Bits 6:7 - Pin G3 alternative function selection"]
    #[inline(always)]
    pub fn pin3(&mut self) -> PIN3_W {
        PIN3_W { w: self }
    }
    #[doc = "Bits 8:9 - Pin G4 alternative function selection"]
    #[inline(always)]
    pub fn pin4(&mut self) -> PIN4_W {
        PIN4_W { w: self }
    }
    #[doc = "Bits 10:11 - Pin G5 alternative function selection"]
    #[inline(always)]
    pub fn pin5(&mut self) -> PIN5_W {
        PIN5_W { w: self }
    }
    #[doc = "Bits 12:13 - Pin G6 alternative function selection"]
    #[inline(always)]
    pub fn pin6(&mut self) -> PIN6_W {
        PIN6_W { w: self }
    }
    #[doc = "Bits 14:15 - Pin G7 alternative function selection"]
    #[inline(always)]
    pub fn pin7(&mut self) -> PIN7_W {
        PIN7_W { w: self }
    }
    #[doc = "Bits 16:17 - Pin G8 alternative function selection"]
    #[inline(always)]
    pub fn pin8(&mut self) -> PIN8_W {
        PIN8_W { w: self }
    }
    #[doc = "Bits 18:19 - Pin G9 alternative function selection"]
    #[inline(always)]
    pub fn pin9(&mut self) -> PIN9_W {
        PIN9_W { w: self }
    }
    #[doc = "Bits 20:21 - Pin G10 alternative function selection"]
    #[inline(always)]
    pub fn pin10(&mut self) -> PIN10_W {
        PIN10_W { w: self }
    }
    #[doc = "Bits 22:23 - Pin G11 alternative function selection"]
    #[inline(always)]
    pub fn pin11(&mut self) -> PIN11_W {
        PIN11_W { w: self }
    }
    #[doc = "Bits 24:25 - Pin G12 alternative function selection"]
    #[inline(always)]
    pub fn pin12(&mut self) -> PIN12_W {
        PIN12_W { w: self }
    }
    #[doc = "Bits 26:27 - Pin G13 alternative function selection"]
    #[inline(always)]
    pub fn pin13(&mut self) -> PIN13_W {
        PIN13_W { w: self }
    }
    #[doc = "Bits 28:29 - Pin G14 alternative function selection"]
    #[inline(always)]
    pub fn pin14(&mut self) -> PIN14_W {
        PIN14_W { w: self }
    }
    #[doc = "Bits 30:31 - Pin G15 alternative function selection"]
    #[inline(always)]
    pub fn pin15(&mut self) -> PIN15_W {
        PIN15_W { w: self }
    }
}
