#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USB OTG interrupt flag register"]
    pub otg_irq_stat: OTG_IRQ_STAT,
    #[doc = "0x04 - Interrupt Enable USB OTG register"]
    pub otg_irq_en: OTG_IRQ_EN,
}
#[doc = "USB OTG interrupt flag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [otg_irq_stat](otg_irq_stat) module"]
pub type OTG_IRQ_STAT = crate::Reg<u32, _OTG_IRQ_STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_IRQ_STAT;
#[doc = "`read()` method returns [otg_irq_stat::R](otg_irq_stat::R) reader structure"]
impl crate::Readable for OTG_IRQ_STAT {}
#[doc = "`write(|w| ..)` method takes [otg_irq_stat::W](otg_irq_stat::W) writer structure"]
impl crate::Writable for OTG_IRQ_STAT {}
#[doc = "USB OTG interrupt flag register"]
pub mod otg_irq_stat;
#[doc = "Interrupt Enable USB OTG register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [otg_irq_en](otg_irq_en) module"]
pub type OTG_IRQ_EN = crate::Reg<u32, _OTG_IRQ_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_IRQ_EN;
#[doc = "`read()` method returns [otg_irq_en::R](otg_irq_en::R) reader structure"]
impl crate::Readable for OTG_IRQ_EN {}
#[doc = "`write(|w| ..)` method takes [otg_irq_en::W](otg_irq_en::W) writer structure"]
impl crate::Writable for OTG_IRQ_EN {}
#[doc = "Interrupt Enable USB OTG register"]
pub mod otg_irq_en;
