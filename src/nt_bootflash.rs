#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Address bootflash register"]
    pub fma: FMA,
    #[doc = "0x04 - Data words bootable flash register 0"]
    pub fmd0: FMD0,
    #[doc = "0x08 - Command register"]
    pub fmc: FMC,
    #[doc = "0x0c - Status register"]
    pub fcis: FCIS,
    #[doc = "0x10 - Interrupt mask register"]
    pub fcim: FCIM,
    #[doc = "0x14 - Command register"]
    pub fcic: FCIC,
    _reserved6: [u8; 56usize],
    #[doc = "0x50 - Data words bootable flash register 1"]
    pub fmd1: FMD1,
    #[doc = "0x54 - Data words bootable flash register 2"]
    pub fmd2: FMD2,
    #[doc = "0x58 - Data words bootable flash register 3"]
    pub fmd3: FMD3,
    _reserved9: [u8; 36usize],
    #[doc = "0x80 - Register the delay of setting addresses to read data from the flash memory"]
    pub t_acc: T_ACC,
    #[doc = "0x84 - Register signal delay lifting NVSTR after lifting signal PROG"]
    pub t_nvs: T_NVS,
    #[doc = "0x88 - Register sets the release delay signal NVSTR after releasing signal PROG"]
    pub t_nvh: T_NVH,
    #[doc = "0x8c - Register delay from releasing signal NVSTR to raise it to the next transaction"]
    pub t_rcv: T_RCV,
    #[doc = "0x90 - Register delay lifting signal YE after lifting signal NVSTR"]
    pub t_pgs: T_PGS,
    #[doc = "0x94 - Register YE signal duration in recording transactions"]
    pub t_prog: T_PROG,
    #[doc = "0x98 - Register delay signal NVSTR after releasing signal YE"]
    pub t_pgh: T_PGH,
    #[doc = "0x9c - Register duration of the signal in transactions ERASE erasure"]
    pub t_erase: T_ERASE,
    #[doc = "0xa0 - Register specifies the duration of the signal in transactions ERASE erasure"]
    pub t_me: T_ME,
    #[doc = "0xa4 - Register specifies the duration of the signal in transactions ERASE erasure"]
    pub t_nvh1: T_NVH1,
}
#[doc = "Address bootflash register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fma](fma) module"]
pub type FMA = crate::Reg<u32, _FMA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMA;
#[doc = "`read()` method returns [fma::R](fma::R) reader structure"]
impl crate::Readable for FMA {}
#[doc = "`write(|w| ..)` method takes [fma::W](fma::W) writer structure"]
impl crate::Writable for FMA {}
#[doc = "Address bootflash register"]
pub mod fma;
#[doc = "Data words bootable flash register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fmd0](fmd0) module"]
pub type FMD0 = crate::Reg<u32, _FMD0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMD0;
#[doc = "`read()` method returns [fmd0::R](fmd0::R) reader structure"]
impl crate::Readable for FMD0 {}
#[doc = "`write(|w| ..)` method takes [fmd0::W](fmd0::W) writer structure"]
impl crate::Writable for FMD0 {}
#[doc = "Data words bootable flash register 0"]
pub mod fmd0;
#[doc = "Command register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fmc](fmc) module"]
pub type FMC = crate::Reg<u32, _FMC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMC;
#[doc = "`write(|w| ..)` method takes [fmc::W](fmc::W) writer structure"]
impl crate::Writable for FMC {}
#[doc = "Command register"]
pub mod fmc;
#[doc = "Status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fcis](fcis) module"]
pub type FCIS = crate::Reg<u32, _FCIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCIS;
#[doc = "`read()` method returns [fcis::R](fcis::R) reader structure"]
impl crate::Readable for FCIS {}
#[doc = "Status register"]
pub mod fcis;
#[doc = "Interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fcim](fcim) module"]
pub type FCIM = crate::Reg<u32, _FCIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCIM;
#[doc = "`read()` method returns [fcim::R](fcim::R) reader structure"]
impl crate::Readable for FCIM {}
#[doc = "`write(|w| ..)` method takes [fcim::W](fcim::W) writer structure"]
impl crate::Writable for FCIM {}
#[doc = "Interrupt mask register"]
pub mod fcim;
#[doc = "Command register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fcic](fcic) module"]
pub type FCIC = crate::Reg<u32, _FCIC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCIC;
#[doc = "`write(|w| ..)` method takes [fcic::W](fcic::W) writer structure"]
impl crate::Writable for FCIC {}
#[doc = "Command register"]
pub mod fcic;
#[doc = "Data words bootable flash register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fmd1](fmd1) module"]
pub type FMD1 = crate::Reg<u32, _FMD1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMD1;
#[doc = "`read()` method returns [fmd1::R](fmd1::R) reader structure"]
impl crate::Readable for FMD1 {}
#[doc = "`write(|w| ..)` method takes [fmd1::W](fmd1::W) writer structure"]
impl crate::Writable for FMD1 {}
#[doc = "Data words bootable flash register 1"]
pub mod fmd1;
#[doc = "Data words bootable flash register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fmd2](fmd2) module"]
pub type FMD2 = crate::Reg<u32, _FMD2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMD2;
#[doc = "`read()` method returns [fmd2::R](fmd2::R) reader structure"]
impl crate::Readable for FMD2 {}
#[doc = "`write(|w| ..)` method takes [fmd2::W](fmd2::W) writer structure"]
impl crate::Writable for FMD2 {}
#[doc = "Data words bootable flash register 2"]
pub mod fmd2;
#[doc = "Data words bootable flash register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fmd3](fmd3) module"]
pub type FMD3 = crate::Reg<u32, _FMD3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMD3;
#[doc = "`read()` method returns [fmd3::R](fmd3::R) reader structure"]
impl crate::Readable for FMD3 {}
#[doc = "`write(|w| ..)` method takes [fmd3::W](fmd3::W) writer structure"]
impl crate::Writable for FMD3 {}
#[doc = "Data words bootable flash register 3"]
pub mod fmd3;
#[doc = "Register the delay of setting addresses to read data from the flash memory\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [t_acc](t_acc) module"]
pub type T_ACC = crate::Reg<u32, _T_ACC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _T_ACC;
#[doc = "`read()` method returns [t_acc::R](t_acc::R) reader structure"]
impl crate::Readable for T_ACC {}
#[doc = "`write(|w| ..)` method takes [t_acc::W](t_acc::W) writer structure"]
impl crate::Writable for T_ACC {}
#[doc = "Register the delay of setting addresses to read data from the flash memory"]
pub mod t_acc;
#[doc = "Register signal delay lifting NVSTR after lifting signal PROG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [t_nvs](t_nvs) module"]
pub type T_NVS = crate::Reg<u32, _T_NVS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _T_NVS;
#[doc = "`read()` method returns [t_nvs::R](t_nvs::R) reader structure"]
impl crate::Readable for T_NVS {}
#[doc = "`write(|w| ..)` method takes [t_nvs::W](t_nvs::W) writer structure"]
impl crate::Writable for T_NVS {}
#[doc = "Register signal delay lifting NVSTR after lifting signal PROG"]
pub mod t_nvs;
#[doc = "Register sets the release delay signal NVSTR after releasing signal PROG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [t_nvh](t_nvh) module"]
pub type T_NVH = crate::Reg<u32, _T_NVH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _T_NVH;
#[doc = "`read()` method returns [t_nvh::R](t_nvh::R) reader structure"]
impl crate::Readable for T_NVH {}
#[doc = "`write(|w| ..)` method takes [t_nvh::W](t_nvh::W) writer structure"]
impl crate::Writable for T_NVH {}
#[doc = "Register sets the release delay signal NVSTR after releasing signal PROG"]
pub mod t_nvh;
#[doc = "Register delay from releasing signal NVSTR to raise it to the next transaction\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [t_rcv](t_rcv) module"]
pub type T_RCV = crate::Reg<u32, _T_RCV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _T_RCV;
#[doc = "`read()` method returns [t_rcv::R](t_rcv::R) reader structure"]
impl crate::Readable for T_RCV {}
#[doc = "`write(|w| ..)` method takes [t_rcv::W](t_rcv::W) writer structure"]
impl crate::Writable for T_RCV {}
#[doc = "Register delay from releasing signal NVSTR to raise it to the next transaction"]
pub mod t_rcv;
#[doc = "Register delay lifting signal YE after lifting signal NVSTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [t_pgs](t_pgs) module"]
pub type T_PGS = crate::Reg<u32, _T_PGS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _T_PGS;
#[doc = "`read()` method returns [t_pgs::R](t_pgs::R) reader structure"]
impl crate::Readable for T_PGS {}
#[doc = "`write(|w| ..)` method takes [t_pgs::W](t_pgs::W) writer structure"]
impl crate::Writable for T_PGS {}
#[doc = "Register delay lifting signal YE after lifting signal NVSTR"]
pub mod t_pgs;
#[doc = "Register YE signal duration in recording transactions\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [t_prog](t_prog) module"]
pub type T_PROG = crate::Reg<u32, _T_PROG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _T_PROG;
#[doc = "`read()` method returns [t_prog::R](t_prog::R) reader structure"]
impl crate::Readable for T_PROG {}
#[doc = "`write(|w| ..)` method takes [t_prog::W](t_prog::W) writer structure"]
impl crate::Writable for T_PROG {}
#[doc = "Register YE signal duration in recording transactions"]
pub mod t_prog;
#[doc = "Register delay signal NVSTR after releasing signal YE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [t_pgh](t_pgh) module"]
pub type T_PGH = crate::Reg<u32, _T_PGH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _T_PGH;
#[doc = "`read()` method returns [t_pgh::R](t_pgh::R) reader structure"]
impl crate::Readable for T_PGH {}
#[doc = "`write(|w| ..)` method takes [t_pgh::W](t_pgh::W) writer structure"]
impl crate::Writable for T_PGH {}
#[doc = "Register delay signal NVSTR after releasing signal YE"]
pub mod t_pgh;
#[doc = "Register duration of the signal in transactions ERASE erasure\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [t_erase](t_erase) module"]
pub type T_ERASE = crate::Reg<u32, _T_ERASE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _T_ERASE;
#[doc = "`read()` method returns [t_erase::R](t_erase::R) reader structure"]
impl crate::Readable for T_ERASE {}
#[doc = "`write(|w| ..)` method takes [t_erase::W](t_erase::W) writer structure"]
impl crate::Writable for T_ERASE {}
#[doc = "Register duration of the signal in transactions ERASE erasure"]
pub mod t_erase;
#[doc = "Register specifies the duration of the signal in transactions ERASE erasure\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [t_me](t_me) module"]
pub type T_ME = crate::Reg<u32, _T_ME>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _T_ME;
#[doc = "`read()` method returns [t_me::R](t_me::R) reader structure"]
impl crate::Readable for T_ME {}
#[doc = "`write(|w| ..)` method takes [t_me::W](t_me::W) writer structure"]
impl crate::Writable for T_ME {}
#[doc = "Register specifies the duration of the signal in transactions ERASE erasure"]
pub mod t_me;
#[doc = "Register specifies the duration of the signal in transactions ERASE erasure\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [t_nvh1](t_nvh1) module"]
pub type T_NVH1 = crate::Reg<u32, _T_NVH1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _T_NVH1;
#[doc = "`read()` method returns [t_nvh1::R](t_nvh1::R) reader structure"]
impl crate::Readable for T_NVH1 {}
#[doc = "`write(|w| ..)` method takes [t_nvh1::W](t_nvh1::W) writer structure"]
impl crate::Writable for T_NVH1 {}
#[doc = "Register specifies the duration of the signal in transactions ERASE erasure"]
pub mod t_nvh1;
