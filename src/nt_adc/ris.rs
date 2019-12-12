#[doc = "Reader of register RIS"]
pub type R = crate::R<u32, super::RIS>;
#[doc = "Reader of field `INR0`"]
pub type INR0_R = crate::R<bool, bool>;
#[doc = "Reader of field `INR1`"]
pub type INR1_R = crate::R<bool, bool>;
#[doc = "Reader of field `INR2`"]
pub type INR2_R = crate::R<bool, bool>;
#[doc = "Reader of field `INR3`"]
pub type INR3_R = crate::R<bool, bool>;
#[doc = "Reader of field `INR4`"]
pub type INR4_R = crate::R<bool, bool>;
#[doc = "Reader of field `INR5`"]
pub type INR5_R = crate::R<bool, bool>;
#[doc = "Reader of field `INR6`"]
pub type INR6_R = crate::R<bool, bool>;
#[doc = "Reader of field `INR7`"]
pub type INR7_R = crate::R<bool, bool>;
#[doc = "Reader of field `INRDC0`"]
pub type INRDC0_R = crate::R<bool, bool>;
#[doc = "Reader of field `INRDC1`"]
pub type INRDC1_R = crate::R<bool, bool>;
#[doc = "Reader of field `INRDC2`"]
pub type INRDC2_R = crate::R<bool, bool>;
#[doc = "Reader of field `INRDC3`"]
pub type INRDC3_R = crate::R<bool, bool>;
#[doc = "Reader of field `INRDC4`"]
pub type INRDC4_R = crate::R<bool, bool>;
#[doc = "Reader of field `INRDC5`"]
pub type INRDC5_R = crate::R<bool, bool>;
#[doc = "Reader of field `INRDC6`"]
pub type INRDC6_R = crate::R<bool, bool>;
#[doc = "Reader of field `INRDC7`"]
pub type INRDC7_R = crate::R<bool, bool>;
#[doc = "Reader of field `INRDC8`"]
pub type INRDC8_R = crate::R<bool, bool>;
#[doc = "Reader of field `INRDC9`"]
pub type INRDC9_R = crate::R<bool, bool>;
#[doc = "Reader of field `INRDC10`"]
pub type INRDC10_R = crate::R<bool, bool>;
#[doc = "Reader of field `INRDC11`"]
pub type INRDC11_R = crate::R<bool, bool>;
#[doc = "Reader of field `INRDC12`"]
pub type INRDC12_R = crate::R<bool, bool>;
#[doc = "Reader of field `INRDC13`"]
pub type INRDC13_R = crate::R<bool, bool>;
#[doc = "Reader of field `INRDC14`"]
pub type INRDC14_R = crate::R<bool, bool>;
#[doc = "Reader of field `INRDC15`"]
pub type INRDC15_R = crate::R<bool, bool>;
#[doc = "Reader of field `INRDC16`"]
pub type INRDC16_R = crate::R<bool, bool>;
#[doc = "Reader of field `INRDC17`"]
pub type INRDC17_R = crate::R<bool, bool>;
#[doc = "Reader of field `INRDC18`"]
pub type INRDC18_R = crate::R<bool, bool>;
#[doc = "Reader of field `INRDC19`"]
pub type INRDC19_R = crate::R<bool, bool>;
#[doc = "Reader of field `INRDC20`"]
pub type INRDC20_R = crate::R<bool, bool>;
#[doc = "Reader of field `INRDC21`"]
pub type INRDC21_R = crate::R<bool, bool>;
#[doc = "Reader of field `INRDC22`"]
pub type INRDC22_R = crate::R<bool, bool>;
#[doc = "Reader of field `INRDC23`"]
pub type INRDC23_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Sequencer 0 raw interrupt status"]
    #[inline(always)]
    pub fn inr0(&self) -> INR0_R {
        INR0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Sequencer 1 raw interrupt status"]
    #[inline(always)]
    pub fn inr1(&self) -> INR1_R {
        INR1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Sequencer 2 raw interrupt status"]
    #[inline(always)]
    pub fn inr2(&self) -> INR2_R {
        INR2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Sequencer 3 raw interrupt status"]
    #[inline(always)]
    pub fn inr3(&self) -> INR3_R {
        INR3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Sequencer 4 raw interrupt status"]
    #[inline(always)]
    pub fn inr4(&self) -> INR4_R {
        INR4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Sequencer 5 raw interrupt status"]
    #[inline(always)]
    pub fn inr5(&self) -> INR5_R {
        INR5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Sequencer 6 raw interrupt status"]
    #[inline(always)]
    pub fn inr6(&self) -> INR6_R {
        INR6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Sequencer 7 raw interrupt status"]
    #[inline(always)]
    pub fn inr7(&self) -> INR7_R {
        INR7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - DCMP 0 raw interrupt status"]
    #[inline(always)]
    pub fn inrdc0(&self) -> INRDC0_R {
        INRDC0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - DCMP 1 raw interrupt status"]
    #[inline(always)]
    pub fn inrdc1(&self) -> INRDC1_R {
        INRDC1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - DCMP 2 raw interrupt status"]
    #[inline(always)]
    pub fn inrdc2(&self) -> INRDC2_R {
        INRDC2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - DCMP 3 raw interrupt status"]
    #[inline(always)]
    pub fn inrdc3(&self) -> INRDC3_R {
        INRDC3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - DCMP 4 raw interrupt status"]
    #[inline(always)]
    pub fn inrdc4(&self) -> INRDC4_R {
        INRDC4_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - DCMP 5 raw interrupt status"]
    #[inline(always)]
    pub fn inrdc5(&self) -> INRDC5_R {
        INRDC5_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - DCMP 6 raw interrupt status"]
    #[inline(always)]
    pub fn inrdc6(&self) -> INRDC6_R {
        INRDC6_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - DCMP 7 raw interrupt status"]
    #[inline(always)]
    pub fn inrdc7(&self) -> INRDC7_R {
        INRDC7_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - DCMP 8 raw interrupt status"]
    #[inline(always)]
    pub fn inrdc8(&self) -> INRDC8_R {
        INRDC8_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - DCMP 9 raw interrupt status"]
    #[inline(always)]
    pub fn inrdc9(&self) -> INRDC9_R {
        INRDC9_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - DCMP 10 raw interrupt status"]
    #[inline(always)]
    pub fn inrdc10(&self) -> INRDC10_R {
        INRDC10_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - DCMP 11 raw interrupt status"]
    #[inline(always)]
    pub fn inrdc11(&self) -> INRDC11_R {
        INRDC11_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - DCMP 12 raw interrupt status"]
    #[inline(always)]
    pub fn inrdc12(&self) -> INRDC12_R {
        INRDC12_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - DCMP 13 raw interrupt status"]
    #[inline(always)]
    pub fn inrdc13(&self) -> INRDC13_R {
        INRDC13_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - DCMP 14 raw interrupt status"]
    #[inline(always)]
    pub fn inrdc14(&self) -> INRDC14_R {
        INRDC14_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - DCMP 15 raw interrupt status"]
    #[inline(always)]
    pub fn inrdc15(&self) -> INRDC15_R {
        INRDC15_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - DCMP 16 raw interrupt status"]
    #[inline(always)]
    pub fn inrdc16(&self) -> INRDC16_R {
        INRDC16_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - DCMP 17 raw interrupt status"]
    #[inline(always)]
    pub fn inrdc17(&self) -> INRDC17_R {
        INRDC17_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - DCMP 18 raw interrupt status"]
    #[inline(always)]
    pub fn inrdc18(&self) -> INRDC18_R {
        INRDC18_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - DCMP 19 raw interrupt status"]
    #[inline(always)]
    pub fn inrdc19(&self) -> INRDC19_R {
        INRDC19_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - DCMP 20 raw interrupt status"]
    #[inline(always)]
    pub fn inrdc20(&self) -> INRDC20_R {
        INRDC20_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - DCMP 21 raw interrupt status"]
    #[inline(always)]
    pub fn inrdc21(&self) -> INRDC21_R {
        INRDC21_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - DCMP 22 raw interrupt status"]
    #[inline(always)]
    pub fn inrdc22(&self) -> INRDC22_R {
        INRDC22_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - DCMP 23 raw interrupt status"]
    #[inline(always)]
    pub fn inrdc23(&self) -> INRDC23_R {
        INRDC23_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
