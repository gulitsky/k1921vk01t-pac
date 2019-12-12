#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register 0"]
    pub spi_cr0: SPI_CR0,
    #[doc = "0x04 - Control register 1"]
    pub spi_cr1: SPI_CR1,
    #[doc = "0x08 - Data register"]
    pub spi_dr: SPI_DR,
    #[doc = "0x0c - State register"]
    pub spi_sr: SPI_SR,
    #[doc = "0x10 - Clock division factor register"]
    pub spi_cpsr: SPI_CPSR,
    #[doc = "0x14 - Mask interrupt register"]
    pub spi_imsc: SPI_IMSC,
    #[doc = "0x18 - Status register interrupt without mask"]
    pub spi_ris: SPI_RIS,
    #[doc = "0x1c - Status register interrupt masking account"]
    pub spi_mis: SPI_MIS,
    #[doc = "0x20 - Register reset interrupt"]
    pub spi_icr: SPI_ICR,
    #[doc = "0x24 - Control register DMA"]
    pub spi_dmacr: SPI_DMACR,
}
#[doc = "Control register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spi_cr0](spi_cr0) module"]
pub type SPI_CR0 = crate::Reg<u32, _SPI_CR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_CR0;
#[doc = "`read()` method returns [spi_cr0::R](spi_cr0::R) reader structure"]
impl crate::Readable for SPI_CR0 {}
#[doc = "`write(|w| ..)` method takes [spi_cr0::W](spi_cr0::W) writer structure"]
impl crate::Writable for SPI_CR0 {}
#[doc = "Control register 0"]
pub mod spi_cr0;
#[doc = "Control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spi_cr1](spi_cr1) module"]
pub type SPI_CR1 = crate::Reg<u32, _SPI_CR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_CR1;
#[doc = "`read()` method returns [spi_cr1::R](spi_cr1::R) reader structure"]
impl crate::Readable for SPI_CR1 {}
#[doc = "`write(|w| ..)` method takes [spi_cr1::W](spi_cr1::W) writer structure"]
impl crate::Writable for SPI_CR1 {}
#[doc = "Control register 1"]
pub mod spi_cr1;
#[doc = "Data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spi_dr](spi_dr) module"]
pub type SPI_DR = crate::Reg<u32, _SPI_DR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_DR;
#[doc = "`read()` method returns [spi_dr::R](spi_dr::R) reader structure"]
impl crate::Readable for SPI_DR {}
#[doc = "`write(|w| ..)` method takes [spi_dr::W](spi_dr::W) writer structure"]
impl crate::Writable for SPI_DR {}
#[doc = "Data register"]
pub mod spi_dr;
#[doc = "State register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spi_sr](spi_sr) module"]
pub type SPI_SR = crate::Reg<u32, _SPI_SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_SR;
#[doc = "`read()` method returns [spi_sr::R](spi_sr::R) reader structure"]
impl crate::Readable for SPI_SR {}
#[doc = "State register"]
pub mod spi_sr;
#[doc = "Clock division factor register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spi_cpsr](spi_cpsr) module"]
pub type SPI_CPSR = crate::Reg<u32, _SPI_CPSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_CPSR;
#[doc = "`read()` method returns [spi_cpsr::R](spi_cpsr::R) reader structure"]
impl crate::Readable for SPI_CPSR {}
#[doc = "`write(|w| ..)` method takes [spi_cpsr::W](spi_cpsr::W) writer structure"]
impl crate::Writable for SPI_CPSR {}
#[doc = "Clock division factor register"]
pub mod spi_cpsr;
#[doc = "Mask interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spi_imsc](spi_imsc) module"]
pub type SPI_IMSC = crate::Reg<u32, _SPI_IMSC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_IMSC;
#[doc = "`read()` method returns [spi_imsc::R](spi_imsc::R) reader structure"]
impl crate::Readable for SPI_IMSC {}
#[doc = "`write(|w| ..)` method takes [spi_imsc::W](spi_imsc::W) writer structure"]
impl crate::Writable for SPI_IMSC {}
#[doc = "Mask interrupt register"]
pub mod spi_imsc;
#[doc = "Status register interrupt without mask\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spi_ris](spi_ris) module"]
pub type SPI_RIS = crate::Reg<u32, _SPI_RIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_RIS;
#[doc = "`read()` method returns [spi_ris::R](spi_ris::R) reader structure"]
impl crate::Readable for SPI_RIS {}
#[doc = "Status register interrupt without mask"]
pub mod spi_ris;
#[doc = "Status register interrupt masking account\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spi_mis](spi_mis) module"]
pub type SPI_MIS = crate::Reg<u32, _SPI_MIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_MIS;
#[doc = "`read()` method returns [spi_mis::R](spi_mis::R) reader structure"]
impl crate::Readable for SPI_MIS {}
#[doc = "Status register interrupt masking account"]
pub mod spi_mis;
#[doc = "Register reset interrupt\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spi_icr](spi_icr) module"]
pub type SPI_ICR = crate::Reg<u32, _SPI_ICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_ICR;
#[doc = "`write(|w| ..)` method takes [spi_icr::W](spi_icr::W) writer structure"]
impl crate::Writable for SPI_ICR {}
#[doc = "Register reset interrupt"]
pub mod spi_icr;
#[doc = "Control register DMA\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spi_dmacr](spi_dmacr) module"]
pub type SPI_DMACR = crate::Reg<u32, _SPI_DMACR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_DMACR;
#[doc = "`read()` method returns [spi_dmacr::R](spi_dmacr::R) reader structure"]
impl crate::Readable for SPI_DMACR {}
#[doc = "`write(|w| ..)` method takes [spi_dmacr::W](spi_dmacr::W) writer structure"]
impl crate::Writable for SPI_DMACR {}
#[doc = "Control register DMA"]
pub mod spi_dmacr;
