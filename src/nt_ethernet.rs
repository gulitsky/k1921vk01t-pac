#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MAC configuration register 1"]
    pub mac1: MAC1,
    #[doc = "0x04 - MAC configuration register 2"]
    pub mac2: MAC2,
    #[doc = "0x08 - Register Back-to-Back Inter-Packet-Gap"]
    pub ipgt: IPGT,
    #[doc = "0x0c - Register Non-Back-to-Back Inter-Packet-Gap"]
    pub ipgr: IPGR,
    #[doc = "0x10 - Register collision window"]
    pub clrt: CLRT,
    #[doc = "0x14 - Register the upper limit size Frame"]
    pub maxf: MAXF,
    #[doc = "0x18 - Register PHY-support interface"]
    pub supp: SUPP,
    _reserved7: [u8; 4usize],
    #[doc = "0x20 - Configuration control register MII"]
    pub mcfg: MCFG,
    #[doc = "0x24 - Command register MII"]
    pub mcmd: MCMD,
    #[doc = "0x28 - MII address register"]
    pub madr: MADR,
    #[doc = "0x2c - Register data written in MII"]
    pub mwtd: MWTD,
    #[doc = "0x30 - Register read data from MII"]
    pub mrdd: MRDD,
    #[doc = "0x34 - MII status register flags"]
    pub mind: MIND,
    #[doc = "0x38 - MII controller status register"]
    pub smii: SMII,
    #[doc = "0x3c - MIIFIFO configurate register"]
    pub fifocfg: FIFOCFG,
    #[doc = "0x40 - Station address register 0"]
    pub sa0: SA0,
    #[doc = "0x44 - Station address register 1"]
    pub sa1: SA1,
    #[doc = "0x48 - Station address register 2"]
    pub sa2: SA2,
    _reserved18: [u8; 308usize],
    #[doc = "0x180 - Reception control register"]
    pub dmatx_ctrl: DMATXCTRL,
    #[doc = "0x184 - Reception descriptor pointer register"]
    pub dmatx_descriptor: DMATXDESCRIPTOR,
    #[doc = "0x188 - Status transmission register"]
    pub dmatx_status: DMATXSTATUS,
    #[doc = "0x18c - Control receive register"]
    pub dmarx_ctrl: DMARXCTRL,
    #[doc = "0x190 - Pointer receive descriptor register"]
    pub dmarx_descriptor: DMARXDESCRIPTOR,
    #[doc = "0x194 - Status receiving register"]
    pub dmarx_status: DMARXSTATUS,
    #[doc = "0x198 - Mask interrupt register"]
    pub dmaintmask: DMAINTMASK,
    #[doc = "0x19c - Interrupt register"]
    pub dmaint: DMAINT,
}
#[doc = "MAC configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mac1](mac1) module"]
pub type MAC1 = crate::Reg<u32, _MAC1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAC1;
#[doc = "`read()` method returns [mac1::R](mac1::R) reader structure"]
impl crate::Readable for MAC1 {}
#[doc = "`write(|w| ..)` method takes [mac1::W](mac1::W) writer structure"]
impl crate::Writable for MAC1 {}
#[doc = "MAC configuration register 1"]
pub mod mac1;
#[doc = "MAC configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mac2](mac2) module"]
pub type MAC2 = crate::Reg<u32, _MAC2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAC2;
#[doc = "`read()` method returns [mac2::R](mac2::R) reader structure"]
impl crate::Readable for MAC2 {}
#[doc = "`write(|w| ..)` method takes [mac2::W](mac2::W) writer structure"]
impl crate::Writable for MAC2 {}
#[doc = "MAC configuration register 2"]
pub mod mac2;
#[doc = "Register Back-to-Back Inter-Packet-Gap\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ipgt](ipgt) module"]
pub type IPGT = crate::Reg<u32, _IPGT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPGT;
#[doc = "`read()` method returns [ipgt::R](ipgt::R) reader structure"]
impl crate::Readable for IPGT {}
#[doc = "`write(|w| ..)` method takes [ipgt::W](ipgt::W) writer structure"]
impl crate::Writable for IPGT {}
#[doc = "Register Back-to-Back Inter-Packet-Gap"]
pub mod ipgt;
#[doc = "Register Non-Back-to-Back Inter-Packet-Gap\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ipgr](ipgr) module"]
pub type IPGR = crate::Reg<u32, _IPGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPGR;
#[doc = "`read()` method returns [ipgr::R](ipgr::R) reader structure"]
impl crate::Readable for IPGR {}
#[doc = "`write(|w| ..)` method takes [ipgr::W](ipgr::W) writer structure"]
impl crate::Writable for IPGR {}
#[doc = "Register Non-Back-to-Back Inter-Packet-Gap"]
pub mod ipgr;
#[doc = "Register collision window\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [clrt](clrt) module"]
pub type CLRT = crate::Reg<u32, _CLRT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLRT;
#[doc = "`read()` method returns [clrt::R](clrt::R) reader structure"]
impl crate::Readable for CLRT {}
#[doc = "`write(|w| ..)` method takes [clrt::W](clrt::W) writer structure"]
impl crate::Writable for CLRT {}
#[doc = "Register collision window"]
pub mod clrt;
#[doc = "Register the upper limit size Frame\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [maxf](maxf) module"]
pub type MAXF = crate::Reg<u32, _MAXF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAXF;
#[doc = "`read()` method returns [maxf::R](maxf::R) reader structure"]
impl crate::Readable for MAXF {}
#[doc = "`write(|w| ..)` method takes [maxf::W](maxf::W) writer structure"]
impl crate::Writable for MAXF {}
#[doc = "Register the upper limit size Frame"]
pub mod maxf;
#[doc = "Register PHY-support interface\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [supp](supp) module"]
pub type SUPP = crate::Reg<u32, _SUPP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SUPP;
#[doc = "`read()` method returns [supp::R](supp::R) reader structure"]
impl crate::Readable for SUPP {}
#[doc = "`write(|w| ..)` method takes [supp::W](supp::W) writer structure"]
impl crate::Writable for SUPP {}
#[doc = "Register PHY-support interface"]
pub mod supp;
#[doc = "Configuration control register MII\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcfg](mcfg) module"]
pub type MCFG = crate::Reg<u32, _MCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCFG;
#[doc = "`read()` method returns [mcfg::R](mcfg::R) reader structure"]
impl crate::Readable for MCFG {}
#[doc = "`write(|w| ..)` method takes [mcfg::W](mcfg::W) writer structure"]
impl crate::Writable for MCFG {}
#[doc = "Configuration control register MII"]
pub mod mcfg;
#[doc = "Command register MII\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcmd](mcmd) module"]
pub type MCMD = crate::Reg<u32, _MCMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCMD;
#[doc = "`read()` method returns [mcmd::R](mcmd::R) reader structure"]
impl crate::Readable for MCMD {}
#[doc = "`write(|w| ..)` method takes [mcmd::W](mcmd::W) writer structure"]
impl crate::Writable for MCMD {}
#[doc = "Command register MII"]
pub mod mcmd;
#[doc = "MII address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [madr](madr) module"]
pub type MADR = crate::Reg<u32, _MADR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MADR;
#[doc = "`read()` method returns [madr::R](madr::R) reader structure"]
impl crate::Readable for MADR {}
#[doc = "`write(|w| ..)` method takes [madr::W](madr::W) writer structure"]
impl crate::Writable for MADR {}
#[doc = "MII address register"]
pub mod madr;
#[doc = "Register data written in MII\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mwtd](mwtd) module"]
pub type MWTD = crate::Reg<u32, _MWTD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MWTD;
#[doc = "`read()` method returns [mwtd::R](mwtd::R) reader structure"]
impl crate::Readable for MWTD {}
#[doc = "`write(|w| ..)` method takes [mwtd::W](mwtd::W) writer structure"]
impl crate::Writable for MWTD {}
#[doc = "Register data written in MII"]
pub mod mwtd;
#[doc = "Register read data from MII\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mrdd](mrdd) module"]
pub type MRDD = crate::Reg<u32, _MRDD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MRDD;
#[doc = "`read()` method returns [mrdd::R](mrdd::R) reader structure"]
impl crate::Readable for MRDD {}
#[doc = "`write(|w| ..)` method takes [mrdd::W](mrdd::W) writer structure"]
impl crate::Writable for MRDD {}
#[doc = "Register read data from MII"]
pub mod mrdd;
#[doc = "MII status register flags\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mind](mind) module"]
pub type MIND = crate::Reg<u32, _MIND>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MIND;
#[doc = "`write(|w| ..)` method takes [mind::W](mind::W) writer structure"]
impl crate::Writable for MIND {}
#[doc = "MII status register flags"]
pub mod mind;
#[doc = "MII controller status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [smii](smii) module"]
pub type SMII = crate::Reg<u32, _SMII>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMII;
#[doc = "`read()` method returns [smii::R](smii::R) reader structure"]
impl crate::Readable for SMII {}
#[doc = "MII controller status register"]
pub mod smii;
#[doc = "MIIFIFO configurate register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fifocfg](fifocfg) module"]
pub type FIFOCFG = crate::Reg<u32, _FIFOCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFOCFG;
#[doc = "`read()` method returns [fifocfg::R](fifocfg::R) reader structure"]
impl crate::Readable for FIFOCFG {}
#[doc = "`write(|w| ..)` method takes [fifocfg::W](fifocfg::W) writer structure"]
impl crate::Writable for FIFOCFG {}
#[doc = "MIIFIFO configurate register"]
pub mod fifocfg;
#[doc = "Station address register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sa0](sa0) module"]
pub type SA0 = crate::Reg<u32, _SA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SA0;
#[doc = "`read()` method returns [sa0::R](sa0::R) reader structure"]
impl crate::Readable for SA0 {}
#[doc = "`write(|w| ..)` method takes [sa0::W](sa0::W) writer structure"]
impl crate::Writable for SA0 {}
#[doc = "Station address register 0"]
pub mod sa0;
#[doc = "Station address register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sa1](sa1) module"]
pub type SA1 = crate::Reg<u32, _SA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SA1;
#[doc = "`read()` method returns [sa1::R](sa1::R) reader structure"]
impl crate::Readable for SA1 {}
#[doc = "`write(|w| ..)` method takes [sa1::W](sa1::W) writer structure"]
impl crate::Writable for SA1 {}
#[doc = "Station address register 1"]
pub mod sa1;
#[doc = "Station address register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sa2](sa2) module"]
pub type SA2 = crate::Reg<u32, _SA2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SA2;
#[doc = "`read()` method returns [sa2::R](sa2::R) reader structure"]
impl crate::Readable for SA2 {}
#[doc = "`write(|w| ..)` method takes [sa2::W](sa2::W) writer structure"]
impl crate::Writable for SA2 {}
#[doc = "Station address register 2"]
pub mod sa2;
#[doc = "Reception control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dmatx_ctrl](dmatx_ctrl) module"]
pub type DMATXCTRL = crate::Reg<u32, _DMATXCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMATXCTRL;
#[doc = "`read()` method returns [dmatx_ctrl::R](dmatx_ctrl::R) reader structure"]
impl crate::Readable for DMATXCTRL {}
#[doc = "`write(|w| ..)` method takes [dmatx_ctrl::W](dmatx_ctrl::W) writer structure"]
impl crate::Writable for DMATXCTRL {}
#[doc = "Reception control register"]
pub mod dmatx_ctrl;
#[doc = "Reception descriptor pointer register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dmatx_descriptor](dmatx_descriptor) module"]
pub type DMATXDESCRIPTOR = crate::Reg<u32, _DMATXDESCRIPTOR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMATXDESCRIPTOR;
#[doc = "`read()` method returns [dmatx_descriptor::R](dmatx_descriptor::R) reader structure"]
impl crate::Readable for DMATXDESCRIPTOR {}
#[doc = "`write(|w| ..)` method takes [dmatx_descriptor::W](dmatx_descriptor::W) writer structure"]
impl crate::Writable for DMATXDESCRIPTOR {}
#[doc = "Reception descriptor pointer register"]
pub mod dmatx_descriptor;
#[doc = "Status transmission register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dmatx_status](dmatx_status) module"]
pub type DMATXSTATUS = crate::Reg<u32, _DMATXSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMATXSTATUS;
#[doc = "`read()` method returns [dmatx_status::R](dmatx_status::R) reader structure"]
impl crate::Readable for DMATXSTATUS {}
#[doc = "`write(|w| ..)` method takes [dmatx_status::W](dmatx_status::W) writer structure"]
impl crate::Writable for DMATXSTATUS {}
#[doc = "Status transmission register"]
pub mod dmatx_status;
#[doc = "Control receive register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dmarx_ctrl](dmarx_ctrl) module"]
pub type DMARXCTRL = crate::Reg<u32, _DMARXCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMARXCTRL;
#[doc = "`read()` method returns [dmarx_ctrl::R](dmarx_ctrl::R) reader structure"]
impl crate::Readable for DMARXCTRL {}
#[doc = "`write(|w| ..)` method takes [dmarx_ctrl::W](dmarx_ctrl::W) writer structure"]
impl crate::Writable for DMARXCTRL {}
#[doc = "Control receive register"]
pub mod dmarx_ctrl;
#[doc = "Pointer receive descriptor register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dmarx_descriptor](dmarx_descriptor) module"]
pub type DMARXDESCRIPTOR = crate::Reg<u32, _DMARXDESCRIPTOR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMARXDESCRIPTOR;
#[doc = "`read()` method returns [dmarx_descriptor::R](dmarx_descriptor::R) reader structure"]
impl crate::Readable for DMARXDESCRIPTOR {}
#[doc = "`write(|w| ..)` method takes [dmarx_descriptor::W](dmarx_descriptor::W) writer structure"]
impl crate::Writable for DMARXDESCRIPTOR {}
#[doc = "Pointer receive descriptor register"]
pub mod dmarx_descriptor;
#[doc = "Status receiving register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dmarx_status](dmarx_status) module"]
pub type DMARXSTATUS = crate::Reg<u32, _DMARXSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMARXSTATUS;
#[doc = "`read()` method returns [dmarx_status::R](dmarx_status::R) reader structure"]
impl crate::Readable for DMARXSTATUS {}
#[doc = "`write(|w| ..)` method takes [dmarx_status::W](dmarx_status::W) writer structure"]
impl crate::Writable for DMARXSTATUS {}
#[doc = "Status receiving register"]
pub mod dmarx_status;
#[doc = "Mask interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dmaintmask](dmaintmask) module"]
pub type DMAINTMASK = crate::Reg<u32, _DMAINTMASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAINTMASK;
#[doc = "`read()` method returns [dmaintmask::R](dmaintmask::R) reader structure"]
impl crate::Readable for DMAINTMASK {}
#[doc = "`write(|w| ..)` method takes [dmaintmask::W](dmaintmask::W) writer structure"]
impl crate::Writable for DMAINTMASK {}
#[doc = "Mask interrupt register"]
pub mod dmaintmask;
#[doc = "Interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dmaint](dmaint) module"]
pub type DMAINT = crate::Reg<u32, _DMAINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAINT;
#[doc = "`read()` method returns [dmaint::R](dmaint::R) reader structure"]
impl crate::Readable for DMAINT {}
#[doc = "`write(|w| ..)` method takes [dmaint::W](dmaint::W) writer structure"]
impl crate::Writable for DMAINT {}
#[doc = "Interrupt register"]
pub mod dmaint;
