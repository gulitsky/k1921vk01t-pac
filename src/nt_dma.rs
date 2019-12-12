#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Status DMA register"]
    pub status: STATUS,
    #[doc = "0x04 - Configurate DMA register"]
    pub cfg: CFG,
    #[doc = "0x08 - Base address register control data channels"]
    pub ctrl_base_ptr: CTRL_BASE_PTR,
    #[doc = "0x0c - Alternate base address register control data channels"]
    pub alt_ctrl_base_ptr: ALT_CTRL_BASE_PTR,
    #[doc = "0x10 - DMA wait request status"]
    pub waitonreq_status: WAITONREQ_STATUS,
    #[doc = "0x14 - Register software process request DMA channels"]
    pub chnl_sw_request: CHNL_SW_REQUEST,
    #[doc = "0x18 - Register Set Packet Exchange DMA channels"]
    pub chnl_useburst_set: CHNL_USEBURST_SET,
    #[doc = "0x1c - Register Clear Packet Exchange DMA channels"]
    pub chnl_useburst_clr: CHNL_USEBURST_CLR,
    #[doc = "0x20 - DMA request mask set"]
    pub chnl_req_mask_set: CHNL_REQ_MASK_SET,
    #[doc = "0x24 - DMA request mask clear"]
    pub chnl_req_mask_clr: CHNL_REQ_MASK_CLR,
    #[doc = "0x28 - DMA channel enabled set"]
    pub chnl_enable_set: CHNL_ENABLE_SET,
    #[doc = "0x2c - DMA channel enabled clear"]
    pub chnl_enable_clr: CHNL_ENABLE_CLR,
    #[doc = "0x30 - Register setting primary / alternate channel control data structure"]
    pub chnl_pri_alt_set: CHNL_PRI_ALT_SET,
    #[doc = "0x34 - Register clearing primary / alternate channel control data structure"]
    pub chnl_pri_alt_clr: CHNL_PRI_ALT_CLR,
    #[doc = "0x38 - Register set the priority channel"]
    pub chnl_priority_set: CHNL_PRIORITY_SET,
    #[doc = "0x3c - Register clear the priority channel"]
    pub chnl_priority_clr: CHNL_PRIORITY_CLR,
    _reserved16: [u8; 12usize],
    #[doc = "0x4c - Flag error clear register"]
    pub err_clr: ERR_CLR,
}
#[doc = "Status DMA register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "Status DMA register"]
pub mod status;
#[doc = "Configurate DMA register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub type CFG = crate::Reg<u32, _CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFG;
#[doc = "`write(|w| ..)` method takes [cfg::W](cfg::W) writer structure"]
impl crate::Writable for CFG {}
#[doc = "Configurate DMA register"]
pub mod cfg;
#[doc = "Base address register control data channels\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctrl_base_ptr](ctrl_base_ptr) module"]
pub type CTRL_BASE_PTR = crate::Reg<u32, _CTRL_BASE_PTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL_BASE_PTR;
#[doc = "`read()` method returns [ctrl_base_ptr::R](ctrl_base_ptr::R) reader structure"]
impl crate::Readable for CTRL_BASE_PTR {}
#[doc = "`write(|w| ..)` method takes [ctrl_base_ptr::W](ctrl_base_ptr::W) writer structure"]
impl crate::Writable for CTRL_BASE_PTR {}
#[doc = "Base address register control data channels"]
pub mod ctrl_base_ptr;
#[doc = "Alternate base address register control data channels\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [alt_ctrl_base_ptr](alt_ctrl_base_ptr) module"]
pub type ALT_CTRL_BASE_PTR = crate::Reg<u32, _ALT_CTRL_BASE_PTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALT_CTRL_BASE_PTR;
#[doc = "`read()` method returns [alt_ctrl_base_ptr::R](alt_ctrl_base_ptr::R) reader structure"]
impl crate::Readable for ALT_CTRL_BASE_PTR {}
#[doc = "Alternate base address register control data channels"]
pub mod alt_ctrl_base_ptr;
#[doc = "DMA wait request status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [waitonreq_status](waitonreq_status) module"]
pub type WAITONREQ_STATUS = crate::Reg<u32, _WAITONREQ_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WAITONREQ_STATUS;
#[doc = "`read()` method returns [waitonreq_status::R](waitonreq_status::R) reader structure"]
impl crate::Readable for WAITONREQ_STATUS {}
#[doc = "DMA wait request status"]
pub mod waitonreq_status;
#[doc = "Register software process request DMA channels\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [chnl_sw_request](chnl_sw_request) module"]
pub type CHNL_SW_REQUEST = crate::Reg<u32, _CHNL_SW_REQUEST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHNL_SW_REQUEST;
#[doc = "`write(|w| ..)` method takes [chnl_sw_request::W](chnl_sw_request::W) writer structure"]
impl crate::Writable for CHNL_SW_REQUEST {}
#[doc = "Register software process request DMA channels"]
pub mod chnl_sw_request;
#[doc = "Register Set Packet Exchange DMA channels\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [chnl_useburst_set](chnl_useburst_set) module"]
pub type CHNL_USEBURST_SET = crate::Reg<u32, _CHNL_USEBURST_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHNL_USEBURST_SET;
#[doc = "`read()` method returns [chnl_useburst_set::R](chnl_useburst_set::R) reader structure"]
impl crate::Readable for CHNL_USEBURST_SET {}
#[doc = "`write(|w| ..)` method takes [chnl_useburst_set::W](chnl_useburst_set::W) writer structure"]
impl crate::Writable for CHNL_USEBURST_SET {}
#[doc = "Register Set Packet Exchange DMA channels"]
pub mod chnl_useburst_set;
#[doc = "Register Clear Packet Exchange DMA channels\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [chnl_useburst_clr](chnl_useburst_clr) module"]
pub type CHNL_USEBURST_CLR = crate::Reg<u32, _CHNL_USEBURST_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHNL_USEBURST_CLR;
#[doc = "`write(|w| ..)` method takes [chnl_useburst_clr::W](chnl_useburst_clr::W) writer structure"]
impl crate::Writable for CHNL_USEBURST_CLR {}
#[doc = "Register Clear Packet Exchange DMA channels"]
pub mod chnl_useburst_clr;
#[doc = "DMA request mask set\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [chnl_req_mask_set](chnl_req_mask_set) module"]
pub type CHNL_REQ_MASK_SET = crate::Reg<u32, _CHNL_REQ_MASK_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHNL_REQ_MASK_SET;
#[doc = "`read()` method returns [chnl_req_mask_set::R](chnl_req_mask_set::R) reader structure"]
impl crate::Readable for CHNL_REQ_MASK_SET {}
#[doc = "`write(|w| ..)` method takes [chnl_req_mask_set::W](chnl_req_mask_set::W) writer structure"]
impl crate::Writable for CHNL_REQ_MASK_SET {}
#[doc = "DMA request mask set"]
pub mod chnl_req_mask_set;
#[doc = "DMA request mask clear\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [chnl_req_mask_clr](chnl_req_mask_clr) module"]
pub type CHNL_REQ_MASK_CLR = crate::Reg<u32, _CHNL_REQ_MASK_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHNL_REQ_MASK_CLR;
#[doc = "`write(|w| ..)` method takes [chnl_req_mask_clr::W](chnl_req_mask_clr::W) writer structure"]
impl crate::Writable for CHNL_REQ_MASK_CLR {}
#[doc = "DMA request mask clear"]
pub mod chnl_req_mask_clr;
#[doc = "DMA channel enabled set\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [chnl_enable_set](chnl_enable_set) module"]
pub type CHNL_ENABLE_SET = crate::Reg<u32, _CHNL_ENABLE_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHNL_ENABLE_SET;
#[doc = "`read()` method returns [chnl_enable_set::R](chnl_enable_set::R) reader structure"]
impl crate::Readable for CHNL_ENABLE_SET {}
#[doc = "`write(|w| ..)` method takes [chnl_enable_set::W](chnl_enable_set::W) writer structure"]
impl crate::Writable for CHNL_ENABLE_SET {}
#[doc = "DMA channel enabled set"]
pub mod chnl_enable_set;
#[doc = "DMA channel enabled clear\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [chnl_enable_clr](chnl_enable_clr) module"]
pub type CHNL_ENABLE_CLR = crate::Reg<u32, _CHNL_ENABLE_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHNL_ENABLE_CLR;
#[doc = "`write(|w| ..)` method takes [chnl_enable_clr::W](chnl_enable_clr::W) writer structure"]
impl crate::Writable for CHNL_ENABLE_CLR {}
#[doc = "DMA channel enabled clear"]
pub mod chnl_enable_clr;
#[doc = "Register setting primary / alternate channel control data structure\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [chnl_pri_alt_set](chnl_pri_alt_set) module"]
pub type CHNL_PRI_ALT_SET = crate::Reg<u32, _CHNL_PRI_ALT_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHNL_PRI_ALT_SET;
#[doc = "`read()` method returns [chnl_pri_alt_set::R](chnl_pri_alt_set::R) reader structure"]
impl crate::Readable for CHNL_PRI_ALT_SET {}
#[doc = "`write(|w| ..)` method takes [chnl_pri_alt_set::W](chnl_pri_alt_set::W) writer structure"]
impl crate::Writable for CHNL_PRI_ALT_SET {}
#[doc = "Register setting primary / alternate channel control data structure"]
pub mod chnl_pri_alt_set;
#[doc = "Register clearing primary / alternate channel control data structure\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [chnl_pri_alt_clr](chnl_pri_alt_clr) module"]
pub type CHNL_PRI_ALT_CLR = crate::Reg<u32, _CHNL_PRI_ALT_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHNL_PRI_ALT_CLR;
#[doc = "`write(|w| ..)` method takes [chnl_pri_alt_clr::W](chnl_pri_alt_clr::W) writer structure"]
impl crate::Writable for CHNL_PRI_ALT_CLR {}
#[doc = "Register clearing primary / alternate channel control data structure"]
pub mod chnl_pri_alt_clr;
#[doc = "Register set the priority channel\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [chnl_priority_set](chnl_priority_set) module"]
pub type CHNL_PRIORITY_SET = crate::Reg<u32, _CHNL_PRIORITY_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHNL_PRIORITY_SET;
#[doc = "`read()` method returns [chnl_priority_set::R](chnl_priority_set::R) reader structure"]
impl crate::Readable for CHNL_PRIORITY_SET {}
#[doc = "`write(|w| ..)` method takes [chnl_priority_set::W](chnl_priority_set::W) writer structure"]
impl crate::Writable for CHNL_PRIORITY_SET {}
#[doc = "Register set the priority channel"]
pub mod chnl_priority_set;
#[doc = "Register clear the priority channel\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [chnl_priority_clr](chnl_priority_clr) module"]
pub type CHNL_PRIORITY_CLR = crate::Reg<u32, _CHNL_PRIORITY_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHNL_PRIORITY_CLR;
#[doc = "`write(|w| ..)` method takes [chnl_priority_clr::W](chnl_priority_clr::W) writer structure"]
impl crate::Writable for CHNL_PRIORITY_CLR {}
#[doc = "Register clear the priority channel"]
pub mod chnl_priority_clr;
#[doc = "Flag error clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [err_clr](err_clr) module"]
pub type ERR_CLR = crate::Reg<u32, _ERR_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ERR_CLR;
#[doc = "`read()` method returns [err_clr::R](err_clr::R) reader structure"]
impl crate::Readable for ERR_CLR {}
#[doc = "`write(|w| ..)` method takes [err_clr::W](err_clr::W) writer structure"]
impl crate::Writable for ERR_CLR {}
#[doc = "Flag error clear register"]
pub mod err_clr;
