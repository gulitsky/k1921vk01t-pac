#[doc = "Reader of register ECFLG"]
pub type R = crate::R<u32, super::ECFLG>;
#[doc = "Reader of field `INT`"]
pub type INT_R = crate::R<bool, bool>;
#[doc = "Reader of field `CEVT0`"]
pub type CEVT0_R = crate::R<bool, bool>;
#[doc = "Reader of field `CEVT1`"]
pub type CEVT1_R = crate::R<bool, bool>;
#[doc = "Reader of field `CEVT2`"]
pub type CEVT2_R = crate::R<bool, bool>;
#[doc = "Reader of field `CEVT3`"]
pub type CEVT3_R = crate::R<bool, bool>;
#[doc = "Reader of field `CTR_OVF`"]
pub type CTR_OVF_R = crate::R<bool, bool>;
#[doc = "Reader of field `CTR_PRD`"]
pub type CTR_PRD_R = crate::R<bool, bool>;
#[doc = "Reader of field `CTR_CMP`"]
pub type CTR_CMP_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - indicate global interrupt"]
    #[inline(always)]
    pub fn int(&self) -> INT_R {
        INT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Hap interrupt CEVT0"]
    #[inline(always)]
    pub fn cevt0(&self) -> CEVT0_R {
        CEVT0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Hap interrupt CEVT1"]
    #[inline(always)]
    pub fn cevt1(&self) -> CEVT1_R {
        CEVT1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Hap interrupt CEVT2"]
    #[inline(always)]
    pub fn cevt2(&self) -> CEVT2_R {
        CEVT2_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Hap interrupt CEVT3"]
    #[inline(always)]
    pub fn cevt3(&self) -> CEVT3_R {
        CEVT3_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Hap interrupt CTROVF"]
    #[inline(always)]
    pub fn ctr_ovf(&self) -> CTR_OVF_R {
        CTR_OVF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Hap interrupt CTR=PRD"]
    #[inline(always)]
    pub fn ctr_prd(&self) -> CTR_PRD_R {
        CTR_PRD_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Hap interrupt CTR=CMP"]
    #[inline(always)]
    pub fn ctr_cmp(&self) -> CTR_CMP_R {
        CTR_CMP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
