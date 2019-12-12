#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Host controller USB version register"]
    pub version: VERSION,
    #[doc = "0x04 - HC Control register 1"]
    pub usbcmd_sts_intr: USBCMD_STS_INTR,
    #[doc = "0x08 - Register version"]
    pub frame_reg: FRAME_REG,
    _reserved3: [u8; 4usize],
    #[doc = "0x10 - HC Control register 2"]
    pub port_sts: PORT_STS,
    #[doc = "0x14 - Transmission Control register A"]
    pub usb_pkt_fields_a: USB_PKT_FIELDS_A,
    #[doc = "0x18 - Transmission Control register B"]
    pub usb_pkt_fields_b: USB_PKT_FIELDS_B,
    #[doc = "0x1c - Bufer start address register"]
    pub buff_start_addr: BUFF_START_ADDR,
    #[doc = "0x20 - Transmission data register"]
    pub tbt: TBT,
    #[doc = "0x24 - XFR STATUS register"]
    pub usb_status: USB_STATUS,
    _reserved9: [u8; 4usize],
    #[doc = "0x2c - Data buffer register"]
    pub data_buffer: DATA_BUFFER,
    #[doc = "0x30 - Register received data"]
    pub slave_in_count: SLAVE_IN_COUNT,
}
#[doc = "Host controller USB version register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [version](version) module"]
pub type VERSION = crate::Reg<u32, _VERSION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VERSION;
#[doc = "`read()` method returns [version::R](version::R) reader structure"]
impl crate::Readable for VERSION {}
#[doc = "Host controller USB version register"]
pub mod version;
#[doc = "HC Control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [usbcmd_sts_intr](usbcmd_sts_intr) module"]
pub type USBCMD_STS_INTR = crate::Reg<u32, _USBCMD_STS_INTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBCMD_STS_INTR;
#[doc = "`read()` method returns [usbcmd_sts_intr::R](usbcmd_sts_intr::R) reader structure"]
impl crate::Readable for USBCMD_STS_INTR {}
#[doc = "`write(|w| ..)` method takes [usbcmd_sts_intr::W](usbcmd_sts_intr::W) writer structure"]
impl crate::Writable for USBCMD_STS_INTR {}
#[doc = "HC Control register 1"]
pub mod usbcmd_sts_intr;
#[doc = "Register version\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [frame_reg](frame_reg) module"]
pub type FRAME_REG = crate::Reg<u32, _FRAME_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FRAME_REG;
#[doc = "`read()` method returns [frame_reg::R](frame_reg::R) reader structure"]
impl crate::Readable for FRAME_REG {}
#[doc = "`write(|w| ..)` method takes [frame_reg::W](frame_reg::W) writer structure"]
impl crate::Writable for FRAME_REG {}
#[doc = "Register version"]
pub mod frame_reg;
#[doc = "HC Control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [port_sts](port_sts) module"]
pub type PORT_STS = crate::Reg<u32, _PORT_STS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PORT_STS;
#[doc = "`read()` method returns [port_sts::R](port_sts::R) reader structure"]
impl crate::Readable for PORT_STS {}
#[doc = "`write(|w| ..)` method takes [port_sts::W](port_sts::W) writer structure"]
impl crate::Writable for PORT_STS {}
#[doc = "HC Control register 2"]
pub mod port_sts;
#[doc = "Transmission Control register A\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [usb_pkt_fields_a](usb_pkt_fields_a) module"]
pub type USB_PKT_FIELDS_A = crate::Reg<u32, _USB_PKT_FIELDS_A>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_PKT_FIELDS_A;
#[doc = "`read()` method returns [usb_pkt_fields_a::R](usb_pkt_fields_a::R) reader structure"]
impl crate::Readable for USB_PKT_FIELDS_A {}
#[doc = "`write(|w| ..)` method takes [usb_pkt_fields_a::W](usb_pkt_fields_a::W) writer structure"]
impl crate::Writable for USB_PKT_FIELDS_A {}
#[doc = "Transmission Control register A"]
pub mod usb_pkt_fields_a;
#[doc = "Transmission Control register B\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [usb_pkt_fields_b](usb_pkt_fields_b) module"]
pub type USB_PKT_FIELDS_B = crate::Reg<u32, _USB_PKT_FIELDS_B>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_PKT_FIELDS_B;
#[doc = "`read()` method returns [usb_pkt_fields_b::R](usb_pkt_fields_b::R) reader structure"]
impl crate::Readable for USB_PKT_FIELDS_B {}
#[doc = "`write(|w| ..)` method takes [usb_pkt_fields_b::W](usb_pkt_fields_b::W) writer structure"]
impl crate::Writable for USB_PKT_FIELDS_B {}
#[doc = "Transmission Control register B"]
pub mod usb_pkt_fields_b;
#[doc = "Bufer start address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [buff_start_addr](buff_start_addr) module"]
pub type BUFF_START_ADDR = crate::Reg<u32, _BUFF_START_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BUFF_START_ADDR;
#[doc = "`read()` method returns [buff_start_addr::R](buff_start_addr::R) reader structure"]
impl crate::Readable for BUFF_START_ADDR {}
#[doc = "`write(|w| ..)` method takes [buff_start_addr::W](buff_start_addr::W) writer structure"]
impl crate::Writable for BUFF_START_ADDR {}
#[doc = "Bufer start address register"]
pub mod buff_start_addr;
#[doc = "Transmission data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tbt](tbt) module"]
pub type TBT = crate::Reg<u32, _TBT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TBT;
#[doc = "`read()` method returns [tbt::R](tbt::R) reader structure"]
impl crate::Readable for TBT {}
#[doc = "`write(|w| ..)` method takes [tbt::W](tbt::W) writer structure"]
impl crate::Writable for TBT {}
#[doc = "Transmission data register"]
pub mod tbt;
#[doc = "XFR STATUS register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [usb_status](usb_status) module"]
pub type USB_STATUS = crate::Reg<u32, _USB_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_STATUS;
#[doc = "`read()` method returns [usb_status::R](usb_status::R) reader structure"]
impl crate::Readable for USB_STATUS {}
#[doc = "XFR STATUS register"]
pub mod usb_status;
#[doc = "Data buffer register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [data_buffer](data_buffer) module"]
pub type DATA_BUFFER = crate::Reg<u32, _DATA_BUFFER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA_BUFFER;
#[doc = "`read()` method returns [data_buffer::R](data_buffer::R) reader structure"]
impl crate::Readable for DATA_BUFFER {}
#[doc = "`write(|w| ..)` method takes [data_buffer::W](data_buffer::W) writer structure"]
impl crate::Writable for DATA_BUFFER {}
#[doc = "Data buffer register"]
pub mod data_buffer;
#[doc = "Register received data\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slave_in_count](slave_in_count) module"]
pub type SLAVE_IN_COUNT = crate::Reg<u32, _SLAVE_IN_COUNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLAVE_IN_COUNT;
#[doc = "`read()` method returns [slave_in_count::R](slave_in_count::R) reader structure"]
impl crate::Readable for SLAVE_IN_COUNT {}
#[doc = "Register received data"]
pub mod slave_in_count;
