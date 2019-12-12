#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Interrupt status register"]
    pub irq_stat_l: IRQ_STAT_L,
    _reserved1: [u8; 4usize],
    #[doc = "0x08 - Interrupt enable register"]
    pub irq_enb_l: IRQ_ENB_L,
    _reserved2: [u8; 4usize],
    #[doc = "0x10 - USB Interrupt status register"]
    pub usb_irq_stat: USB_IRQ_STAT,
    #[doc = "0x14 - USB Interrupt enable register"]
    pub usb_irq_enb: USB_IRQ_ENB,
    #[doc = "0x18 - USB Operations register"]
    pub usb_oper: USB_OPER,
    #[doc = "0x1c - USB Frame counter register"]
    pub usb_frame_cnt: USB_FRAME_CNT,
    #[doc = "0x20 - USB Address register"]
    pub usb_addr: USB_ADDR,
    _reserved7: [u8; 4usize],
    #[doc = "0x28 - Data buffer for transmission register of Control EndPoint"]
    pub cep_data_buf: CEP_DATA_BUF,
    #[doc = "0x2c - Control register and buffer status of Control EndPoint"]
    pub cep_ctrl_stat: CEP_CTRL_STAT,
    #[doc = "0x30 - Interrupt Enable buffer register of Control EndPoint"]
    pub cep_irq_enb: CEP_IRQ_ENB,
    #[doc = "0x34 - Buffer interrupt flag register of Control EndPoint"]
    pub cep_irq_stat: CEP_IRQ_STAT,
    #[doc = "0x38 - In transfer data count register of Control EndPoint"]
    pub cep_in_xfrcnt: CEP_IN_XFRCNT,
    #[doc = "0x3c - Out-transfer data count register of Control EndPoint"]
    pub cep_out_xfrcnt: CEP_OUT_XFRCNT,
    _reserved13: [u8; 4usize],
    #[doc = "0x44 - Register zero and first byte packet Setup of Control EndPoint"]
    pub cep_setup1_0: CEP_SETUP1_0,
    #[doc = "0x48 - Register 2nd and 3rd bytes of the packet Setup of Control EndPoint"]
    pub cep_setup3_2: CEP_SETUP3_2,
    #[doc = "0x4c - Register 4th and 5th bytes of the packet Setup of Control EndPoint"]
    pub cep_setup5_4: CEP_SETUP5_4,
    #[doc = "0x50 - Register on the 6th and 7th byte packet Setup of Control EndPoint"]
    pub cep_setup7_6: CEP_SETUP7_6,
    #[doc = "0x54 - Control EndPoint RAM start Addr register"]
    pub cep_start_addr: CEP_START_ADDR,
    #[doc = "0x58 - Control EndPoint RAM end Addr register"]
    pub cep_end_addr: CEP_END_ADDR,
    #[doc = "0x5c - Register control DMA"]
    pub dma_ctrl_sts: DMA_CTRL_STS,
    #[doc = "0x60 - DMA Count byte Register"]
    pub dma_cnt: DMA_CNT,
    #[doc = "0x64 - USB_EP"]
    pub usb_ep: [USB_EP; 2],
    _reserved22: [u8; 1612usize],
    #[doc = "0x700 - AHB addr register"]
    pub ahb_dma_addr: AHB_DMA_ADDR,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct USB_EP {
    #[doc = "0x00 - EndPoint Data Register"]
    pub data_buf: self::usb_ep::DATA_BUF,
    #[doc = "0x04 - Interrupt flag register no control buffer"]
    pub irq_stat: self::usb_ep::IRQ_STAT,
    #[doc = "0x08 - Enable register Interrupt no control buffer"]
    pub irq_enb: self::usb_ep::IRQ_ENB,
    #[doc = "0x0c - EndPoint Available count register"]
    pub avail_cnt: self::usb_ep::AVAIL_CNT,
    #[doc = "0x10 - EndPoint Response Set/Clear register"]
    pub rsp_sc: self::usb_ep::RSP_SC,
    #[doc = "0x14 - EndPoint maximum packet size register"]
    pub mps: self::usb_ep::MPS,
    #[doc = "0x18 - EndPoint Transfer count register"]
    pub cnt: self::usb_ep::CNT,
    #[doc = "0x1c - EndPoint configuration register"]
    pub usb_ep_cfg: self::usb_ep::USB_EP_CFG,
    #[doc = "0x20 - EndPoint RAM start addres register"]
    pub start_addr: self::usb_ep::START_ADDR,
    #[doc = "0x24 - EndPoint RAM end addres register"]
    pub end_addr: self::usb_ep::END_ADDR,
}
#[doc = r"Register block"]
#[doc = "USB_EP"]
pub mod usb_ep;
#[doc = "Interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [irq_stat_l](irq_stat_l) module"]
pub type IRQ_STAT_L = crate::Reg<u32, _IRQ_STAT_L>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQ_STAT_L;
#[doc = "`read()` method returns [irq_stat_l::R](irq_stat_l::R) reader structure"]
impl crate::Readable for IRQ_STAT_L {}
#[doc = "`write(|w| ..)` method takes [irq_stat_l::W](irq_stat_l::W) writer structure"]
impl crate::Writable for IRQ_STAT_L {}
#[doc = "Interrupt status register"]
pub mod irq_stat_l;
#[doc = "Interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [irq_enb_l](irq_enb_l) module"]
pub type IRQ_ENB_L = crate::Reg<u32, _IRQ_ENB_L>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQ_ENB_L;
#[doc = "`read()` method returns [irq_enb_l::R](irq_enb_l::R) reader structure"]
impl crate::Readable for IRQ_ENB_L {}
#[doc = "`write(|w| ..)` method takes [irq_enb_l::W](irq_enb_l::W) writer structure"]
impl crate::Writable for IRQ_ENB_L {}
#[doc = "Interrupt enable register"]
pub mod irq_enb_l;
#[doc = "USB Interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [usb_irq_stat](usb_irq_stat) module"]
pub type USB_IRQ_STAT = crate::Reg<u32, _USB_IRQ_STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_IRQ_STAT;
#[doc = "`read()` method returns [usb_irq_stat::R](usb_irq_stat::R) reader structure"]
impl crate::Readable for USB_IRQ_STAT {}
#[doc = "`write(|w| ..)` method takes [usb_irq_stat::W](usb_irq_stat::W) writer structure"]
impl crate::Writable for USB_IRQ_STAT {}
#[doc = "USB Interrupt status register"]
pub mod usb_irq_stat;
#[doc = "USB Interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [usb_irq_enb](usb_irq_enb) module"]
pub type USB_IRQ_ENB = crate::Reg<u32, _USB_IRQ_ENB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_IRQ_ENB;
#[doc = "`read()` method returns [usb_irq_enb::R](usb_irq_enb::R) reader structure"]
impl crate::Readable for USB_IRQ_ENB {}
#[doc = "`write(|w| ..)` method takes [usb_irq_enb::W](usb_irq_enb::W) writer structure"]
impl crate::Writable for USB_IRQ_ENB {}
#[doc = "USB Interrupt enable register"]
pub mod usb_irq_enb;
#[doc = "USB Operations register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [usb_oper](usb_oper) module"]
pub type USB_OPER = crate::Reg<u32, _USB_OPER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_OPER;
#[doc = "`read()` method returns [usb_oper::R](usb_oper::R) reader structure"]
impl crate::Readable for USB_OPER {}
#[doc = "`write(|w| ..)` method takes [usb_oper::W](usb_oper::W) writer structure"]
impl crate::Writable for USB_OPER {}
#[doc = "USB Operations register"]
pub mod usb_oper;
#[doc = "USB Frame counter register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [usb_frame_cnt](usb_frame_cnt) module"]
pub type USB_FRAME_CNT = crate::Reg<u32, _USB_FRAME_CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_FRAME_CNT;
#[doc = "`read()` method returns [usb_frame_cnt::R](usb_frame_cnt::R) reader structure"]
impl crate::Readable for USB_FRAME_CNT {}
#[doc = "USB Frame counter register"]
pub mod usb_frame_cnt;
#[doc = "USB Address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [usb_addr](usb_addr) module"]
pub type USB_ADDR = crate::Reg<u32, _USB_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_ADDR;
#[doc = "`read()` method returns [usb_addr::R](usb_addr::R) reader structure"]
impl crate::Readable for USB_ADDR {}
#[doc = "`write(|w| ..)` method takes [usb_addr::W](usb_addr::W) writer structure"]
impl crate::Writable for USB_ADDR {}
#[doc = "USB Address register"]
pub mod usb_addr;
#[doc = "Data buffer for transmission register of Control EndPoint\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cep_data_buf](cep_data_buf) module"]
pub type CEP_DATA_BUF = crate::Reg<u32, _CEP_DATA_BUF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CEP_DATA_BUF;
#[doc = "`read()` method returns [cep_data_buf::R](cep_data_buf::R) reader structure"]
impl crate::Readable for CEP_DATA_BUF {}
#[doc = "`write(|w| ..)` method takes [cep_data_buf::W](cep_data_buf::W) writer structure"]
impl crate::Writable for CEP_DATA_BUF {}
#[doc = "Data buffer for transmission register of Control EndPoint"]
pub mod cep_data_buf;
#[doc = "Control register and buffer status of Control EndPoint\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cep_ctrl_stat](cep_ctrl_stat) module"]
pub type CEP_CTRL_STAT = crate::Reg<u32, _CEP_CTRL_STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CEP_CTRL_STAT;
#[doc = "`read()` method returns [cep_ctrl_stat::R](cep_ctrl_stat::R) reader structure"]
impl crate::Readable for CEP_CTRL_STAT {}
#[doc = "`write(|w| ..)` method takes [cep_ctrl_stat::W](cep_ctrl_stat::W) writer structure"]
impl crate::Writable for CEP_CTRL_STAT {}
#[doc = "Control register and buffer status of Control EndPoint"]
pub mod cep_ctrl_stat;
#[doc = "Interrupt Enable buffer register of Control EndPoint\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cep_irq_enb](cep_irq_enb) module"]
pub type CEP_IRQ_ENB = crate::Reg<u32, _CEP_IRQ_ENB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CEP_IRQ_ENB;
#[doc = "`read()` method returns [cep_irq_enb::R](cep_irq_enb::R) reader structure"]
impl crate::Readable for CEP_IRQ_ENB {}
#[doc = "`write(|w| ..)` method takes [cep_irq_enb::W](cep_irq_enb::W) writer structure"]
impl crate::Writable for CEP_IRQ_ENB {}
#[doc = "Interrupt Enable buffer register of Control EndPoint"]
pub mod cep_irq_enb;
#[doc = "Buffer interrupt flag register of Control EndPoint\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cep_irq_stat](cep_irq_stat) module"]
pub type CEP_IRQ_STAT = crate::Reg<u32, _CEP_IRQ_STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CEP_IRQ_STAT;
#[doc = "`read()` method returns [cep_irq_stat::R](cep_irq_stat::R) reader structure"]
impl crate::Readable for CEP_IRQ_STAT {}
#[doc = "`write(|w| ..)` method takes [cep_irq_stat::W](cep_irq_stat::W) writer structure"]
impl crate::Writable for CEP_IRQ_STAT {}
#[doc = "Buffer interrupt flag register of Control EndPoint"]
pub mod cep_irq_stat;
#[doc = "In transfer data count register of Control EndPoint\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cep_in_xfrcnt](cep_in_xfrcnt) module"]
pub type CEP_IN_XFRCNT = crate::Reg<u32, _CEP_IN_XFRCNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CEP_IN_XFRCNT;
#[doc = "`read()` method returns [cep_in_xfrcnt::R](cep_in_xfrcnt::R) reader structure"]
impl crate::Readable for CEP_IN_XFRCNT {}
#[doc = "`write(|w| ..)` method takes [cep_in_xfrcnt::W](cep_in_xfrcnt::W) writer structure"]
impl crate::Writable for CEP_IN_XFRCNT {}
#[doc = "In transfer data count register of Control EndPoint"]
pub mod cep_in_xfrcnt;
#[doc = "Out-transfer data count register of Control EndPoint\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cep_out_xfrcnt](cep_out_xfrcnt) module"]
pub type CEP_OUT_XFRCNT = crate::Reg<u32, _CEP_OUT_XFRCNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CEP_OUT_XFRCNT;
#[doc = "`read()` method returns [cep_out_xfrcnt::R](cep_out_xfrcnt::R) reader structure"]
impl crate::Readable for CEP_OUT_XFRCNT {}
#[doc = "Out-transfer data count register of Control EndPoint"]
pub mod cep_out_xfrcnt;
#[doc = "Register zero and first byte packet Setup of Control EndPoint\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cep_setup1_0](cep_setup1_0) module"]
pub type CEP_SETUP1_0 = crate::Reg<u32, _CEP_SETUP1_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CEP_SETUP1_0;
#[doc = "`read()` method returns [cep_setup1_0::R](cep_setup1_0::R) reader structure"]
impl crate::Readable for CEP_SETUP1_0 {}
#[doc = "Register zero and first byte packet Setup of Control EndPoint"]
pub mod cep_setup1_0;
#[doc = "Register 2nd and 3rd bytes of the packet Setup of Control EndPoint\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cep_setup3_2](cep_setup3_2) module"]
pub type CEP_SETUP3_2 = crate::Reg<u32, _CEP_SETUP3_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CEP_SETUP3_2;
#[doc = "`read()` method returns [cep_setup3_2::R](cep_setup3_2::R) reader structure"]
impl crate::Readable for CEP_SETUP3_2 {}
#[doc = "Register 2nd and 3rd bytes of the packet Setup of Control EndPoint"]
pub mod cep_setup3_2;
#[doc = "Register 4th and 5th bytes of the packet Setup of Control EndPoint\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cep_setup5_4](cep_setup5_4) module"]
pub type CEP_SETUP5_4 = crate::Reg<u32, _CEP_SETUP5_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CEP_SETUP5_4;
#[doc = "`read()` method returns [cep_setup5_4::R](cep_setup5_4::R) reader structure"]
impl crate::Readable for CEP_SETUP5_4 {}
#[doc = "Register 4th and 5th bytes of the packet Setup of Control EndPoint"]
pub mod cep_setup5_4;
#[doc = "Register on the 6th and 7th byte packet Setup of Control EndPoint\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cep_setup7_6](cep_setup7_6) module"]
pub type CEP_SETUP7_6 = crate::Reg<u32, _CEP_SETUP7_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CEP_SETUP7_6;
#[doc = "`read()` method returns [cep_setup7_6::R](cep_setup7_6::R) reader structure"]
impl crate::Readable for CEP_SETUP7_6 {}
#[doc = "Register on the 6th and 7th byte packet Setup of Control EndPoint"]
pub mod cep_setup7_6;
#[doc = "Control EndPoint RAM start Addr register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cep_start_addr](cep_start_addr) module"]
pub type CEP_START_ADDR = crate::Reg<u32, _CEP_START_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CEP_START_ADDR;
#[doc = "`read()` method returns [cep_start_addr::R](cep_start_addr::R) reader structure"]
impl crate::Readable for CEP_START_ADDR {}
#[doc = "`write(|w| ..)` method takes [cep_start_addr::W](cep_start_addr::W) writer structure"]
impl crate::Writable for CEP_START_ADDR {}
#[doc = "Control EndPoint RAM start Addr register"]
pub mod cep_start_addr;
#[doc = "Control EndPoint RAM end Addr register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cep_end_addr](cep_end_addr) module"]
pub type CEP_END_ADDR = crate::Reg<u32, _CEP_END_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CEP_END_ADDR;
#[doc = "`read()` method returns [cep_end_addr::R](cep_end_addr::R) reader structure"]
impl crate::Readable for CEP_END_ADDR {}
#[doc = "`write(|w| ..)` method takes [cep_end_addr::W](cep_end_addr::W) writer structure"]
impl crate::Writable for CEP_END_ADDR {}
#[doc = "Control EndPoint RAM end Addr register"]
pub mod cep_end_addr;
#[doc = "Register control DMA\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dma_ctrl_sts](dma_ctrl_sts) module"]
pub type DMA_CTRL_STS = crate::Reg<u32, _DMA_CTRL_STS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_CTRL_STS;
#[doc = "`read()` method returns [dma_ctrl_sts::R](dma_ctrl_sts::R) reader structure"]
impl crate::Readable for DMA_CTRL_STS {}
#[doc = "`write(|w| ..)` method takes [dma_ctrl_sts::W](dma_ctrl_sts::W) writer structure"]
impl crate::Writable for DMA_CTRL_STS {}
#[doc = "Register control DMA"]
pub mod dma_ctrl_sts;
#[doc = "DMA Count byte Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dma_cnt](dma_cnt) module"]
pub type DMA_CNT = crate::Reg<u32, _DMA_CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_CNT;
#[doc = "`read()` method returns [dma_cnt::R](dma_cnt::R) reader structure"]
impl crate::Readable for DMA_CNT {}
#[doc = "`write(|w| ..)` method takes [dma_cnt::W](dma_cnt::W) writer structure"]
impl crate::Writable for DMA_CNT {}
#[doc = "DMA Count byte Register"]
pub mod dma_cnt;
#[doc = "AHB addr register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ahb_dma_addr](ahb_dma_addr) module"]
pub type AHB_DMA_ADDR = crate::Reg<u32, _AHB_DMA_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHB_DMA_ADDR;
#[doc = "`read()` method returns [ahb_dma_addr::R](ahb_dma_addr::R) reader structure"]
impl crate::Readable for AHB_DMA_ADDR {}
#[doc = "`write(|w| ..)` method takes [ahb_dma_addr::W](ahb_dma_addr::W) writer structure"]
impl crate::Writable for AHB_DMA_ADDR {}
#[doc = "AHB addr register"]
pub mod ahb_dma_addr;
