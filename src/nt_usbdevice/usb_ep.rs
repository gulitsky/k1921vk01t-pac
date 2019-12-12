#[doc = "EndPoint Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [data_buf](data_buf) module"]
pub type DATA_BUF = crate::Reg<u32, _DATA_BUF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA_BUF;
#[doc = "`read()` method returns [data_buf::R](data_buf::R) reader structure"]
impl crate::Readable for DATA_BUF {}
#[doc = "`write(|w| ..)` method takes [data_buf::W](data_buf::W) writer structure"]
impl crate::Writable for DATA_BUF {}
#[doc = "EndPoint Data Register"]
pub mod data_buf;
#[doc = "Interrupt flag register no control buffer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [irq_stat](irq_stat) module"]
pub type IRQ_STAT = crate::Reg<u32, _IRQ_STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQ_STAT;
#[doc = "`read()` method returns [irq_stat::R](irq_stat::R) reader structure"]
impl crate::Readable for IRQ_STAT {}
#[doc = "`write(|w| ..)` method takes [irq_stat::W](irq_stat::W) writer structure"]
impl crate::Writable for IRQ_STAT {}
#[doc = "Interrupt flag register no control buffer"]
pub mod irq_stat;
#[doc = "Enable register Interrupt no control buffer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [irq_enb](irq_enb) module"]
pub type IRQ_ENB = crate::Reg<u32, _IRQ_ENB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQ_ENB;
#[doc = "`read()` method returns [irq_enb::R](irq_enb::R) reader structure"]
impl crate::Readable for IRQ_ENB {}
#[doc = "`write(|w| ..)` method takes [irq_enb::W](irq_enb::W) writer structure"]
impl crate::Writable for IRQ_ENB {}
#[doc = "Enable register Interrupt no control buffer"]
pub mod irq_enb;
#[doc = "EndPoint Available count register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [avail_cnt](avail_cnt) module"]
pub type AVAIL_CNT = crate::Reg<u32, _AVAIL_CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AVAIL_CNT;
#[doc = "`read()` method returns [avail_cnt::R](avail_cnt::R) reader structure"]
impl crate::Readable for AVAIL_CNT {}
#[doc = "EndPoint Available count register"]
pub mod avail_cnt;
#[doc = "EndPoint Response Set/Clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rsp_sc](rsp_sc) module"]
pub type RSP_SC = crate::Reg<u32, _RSP_SC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSP_SC;
#[doc = "`read()` method returns [rsp_sc::R](rsp_sc::R) reader structure"]
impl crate::Readable for RSP_SC {}
#[doc = "`write(|w| ..)` method takes [rsp_sc::W](rsp_sc::W) writer structure"]
impl crate::Writable for RSP_SC {}
#[doc = "EndPoint Response Set/Clear register"]
pub mod rsp_sc;
#[doc = "EndPoint maximum packet size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mps](mps) module"]
pub type MPS = crate::Reg<u32, _MPS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPS;
#[doc = "`read()` method returns [mps::R](mps::R) reader structure"]
impl crate::Readable for MPS {}
#[doc = "`write(|w| ..)` method takes [mps::W](mps::W) writer structure"]
impl crate::Writable for MPS {}
#[doc = "EndPoint maximum packet size register"]
pub mod mps;
#[doc = "EndPoint Transfer count register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cnt](cnt) module"]
pub type CNT = crate::Reg<u32, _CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNT;
#[doc = "`read()` method returns [cnt::R](cnt::R) reader structure"]
impl crate::Readable for CNT {}
#[doc = "`write(|w| ..)` method takes [cnt::W](cnt::W) writer structure"]
impl crate::Writable for CNT {}
#[doc = "EndPoint Transfer count register"]
pub mod cnt;
#[doc = "EndPoint configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [usb_ep_cfg](usb_ep_cfg) module"]
pub type USB_EP_CFG = crate::Reg<u32, _USB_EP_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_EP_CFG;
#[doc = "`read()` method returns [usb_ep_cfg::R](usb_ep_cfg::R) reader structure"]
impl crate::Readable for USB_EP_CFG {}
#[doc = "`write(|w| ..)` method takes [usb_ep_cfg::W](usb_ep_cfg::W) writer structure"]
impl crate::Writable for USB_EP_CFG {}
#[doc = "EndPoint configuration register"]
pub mod usb_ep_cfg;
#[doc = "EndPoint RAM start addres register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [start_addr](start_addr) module"]
pub type START_ADDR = crate::Reg<u32, _START_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _START_ADDR;
#[doc = "`read()` method returns [start_addr::R](start_addr::R) reader structure"]
impl crate::Readable for START_ADDR {}
#[doc = "`write(|w| ..)` method takes [start_addr::W](start_addr::W) writer structure"]
impl crate::Writable for START_ADDR {}
#[doc = "EndPoint RAM start addres register"]
pub mod start_addr;
#[doc = "EndPoint RAM end addres register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [end_addr](end_addr) module"]
pub type END_ADDR = crate::Reg<u32, _END_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _END_ADDR;
#[doc = "`read()` method returns [end_addr::R](end_addr::R) reader structure"]
impl crate::Readable for END_ADDR {}
#[doc = "`write(|w| ..)` method takes [end_addr::W](end_addr::W) writer structure"]
impl crate::Writable for END_ADDR {}
#[doc = "EndPoint RAM end addres register"]
pub mod end_addr;
