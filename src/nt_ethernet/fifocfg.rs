#[doc = "Reader of register FIFOCFG"]
pub type R = crate::R<u32, super::FIFOCFG>;
#[doc = "Writer for register FIFOCFG"]
pub type W = crate::W<u32, super::FIFOCFG>;
#[doc = "Register FIFOCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::FIFOCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RST`"]
pub type RST_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RST`"]
pub struct RST_W<'a> {
    w: &'a mut W,
}
impl<'a> RST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Enable request MIIFIFO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENREQ_A {
    #[doc = "0: disable MIIFIFO requests"]
    DISABLE,
    #[doc = "31: enable MIIFIFO requests"]
    ENABLE,
}
impl From<ENREQ_A> for u8 {
    #[inline(always)]
    fn from(variant: ENREQ_A) -> Self {
        match variant {
            ENREQ_A::DISABLE => 0,
            ENREQ_A::ENABLE => 31,
        }
    }
}
#[doc = "Reader of field `ENREQ`"]
pub type ENREQ_R = crate::R<u8, ENREQ_A>;
impl ENREQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ENREQ_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ENREQ_A::DISABLE),
            31 => Val(ENREQ_A::ENABLE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ENREQ_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ENREQ_A::ENABLE
    }
}
#[doc = "Write proxy for field `ENREQ`"]
pub struct ENREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> ENREQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENREQ_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "disable MIIFIFO requests"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ENREQ_A::DISABLE)
    }
    #[doc = "enable MIIFIFO requests"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ENREQ_A::ENABLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Indicate enable MIIFIFO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENRPLY_A {
    #[doc = "0: disable MIIFIFO modules"]
    DISABLE,
    #[doc = "31: enable MIIFIFO modules"]
    ENABLE,
}
impl From<ENRPLY_A> for u8 {
    #[inline(always)]
    fn from(variant: ENRPLY_A) -> Self {
        match variant {
            ENRPLY_A::DISABLE => 0,
            ENRPLY_A::ENABLE => 31,
        }
    }
}
#[doc = "Reader of field `ENRPLY`"]
pub type ENRPLY_R = crate::R<u8, ENRPLY_A>;
impl ENRPLY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ENRPLY_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ENRPLY_A::DISABLE),
            31 => Val(ENRPLY_A::ENABLE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ENRPLY_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ENRPLY_A::ENABLE
    }
}
impl R {
    #[doc = "Bits 0:4 - Reset MIIFIFO"]
    #[inline(always)]
    pub fn rst(&self) -> RST_R {
        RST_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Enable request MIIFIFO"]
    #[inline(always)]
    pub fn enreq(&self) -> ENREQ_R {
        ENREQ_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Indicate enable MIIFIFO"]
    #[inline(always)]
    pub fn enrply(&self) -> ENRPLY_R {
        ENRPLY_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Reset MIIFIFO"]
    #[inline(always)]
    pub fn rst(&mut self) -> RST_W {
        RST_W { w: self }
    }
    #[doc = "Bits 8:12 - Enable request MIIFIFO"]
    #[inline(always)]
    pub fn enreq(&mut self) -> ENREQ_W {
        ENREQ_W { w: self }
    }
}
