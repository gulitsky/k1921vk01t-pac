#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Frequency control register"]
    pub clc: CLC,
    _reserved1: [u8; 4usize],
    #[doc = "0x08 - Identity register"]
    pub id: ID,
    #[doc = "0x0c - Register divider"]
    pub fdr: FDR,
    _reserved3: [u8; 240usize],
    #[doc = "0x100 - LIST"]
    pub list: [LIST; 8],
    _reserved4: [u8; 32usize],
    #[doc = "0x140 - MSPND"]
    pub mspnd: [MSPND; 8],
    _reserved5: [u8; 32usize],
    #[doc = "0x180 - MSID"]
    pub msid: [MSID; 8],
    _reserved6: [u8; 32usize],
    #[doc = "0x1c0 - Mask register message index"]
    pub msimask: MSIMASK,
    #[doc = "0x1c4 - Register command panel"]
    pub panctr: PANCTR,
    #[doc = "0x1c8 - No description"]
    pub mcr_reg: MCR_REG,
    #[doc = "0x1cc - Interrupt register"]
    pub mitr: MITR,
    _reserved10: [u8; 48usize],
    #[doc = "0x200 - CAN_Node"]
    pub can_node: [CAN_NODE; 2],
    _reserved11: [u8; 3072usize],
    #[doc = "0x1000 - CAN_Msg"]
    pub can_msg: [CAN_MSG; 256],
}
#[doc = r"Register block"]
#[repr(C)]
pub struct LIST {
    #[doc = "0x00 - Register list0"]
    pub list: self::list::LIST,
}
#[doc = r"Register block"]
#[doc = "LIST"]
pub mod list;
#[doc = r"Register block"]
#[repr(C)]
pub struct MSPND {
    #[doc = "0x00 - Register waiting interrupts0"]
    pub mspnd: self::mspnd::MSPND,
}
#[doc = r"Register block"]
#[doc = "MSPND"]
pub mod mspnd;
#[doc = r"Register block"]
#[repr(C)]
pub struct MSID {
    #[doc = "0x00 - Register messages index0"]
    pub msid: self::msid::MSID,
}
#[doc = r"Register block"]
#[doc = "MSID"]
pub mod msid;
#[doc = r"Register block"]
#[repr(C)]
pub struct CAN_NODE {
    #[doc = "0x00 - Register control node0"]
    pub ncr: self::can_node::NCR,
    #[doc = "0x04 - Register state node0"]
    pub nsr: self::can_node::NSR,
    #[doc = "0x08 - Interrupt pointer register node0"]
    pub nipr: self::can_node::NIPR,
    #[doc = "0x0c - Port control register node0"]
    pub npcr: self::can_node::NPCR,
    #[doc = "0x10 - Timing register bits 0"]
    pub nbtr: self::can_node::NBTR,
    #[doc = "0x14 - Counter error register node0"]
    pub necnt: self::can_node::NECNT,
    #[doc = "0x18 - Register message counter node0"]
    pub nfcr: self::can_node::NFCR,
    _reserved7: [u8; 224usize],
    #[doc = "0xfc - No description"]
    pub reserved: self::can_node::RESERVED,
}
#[doc = r"Register block"]
#[doc = "CAN_Node"]
pub mod can_node;
#[doc = r"Register block"]
#[repr(C)]
pub struct CAN_MSG {
    #[doc = "0x00 - Register control the operation of the message object 0"]
    pub mofcr: self::can_msg::MOFCR,
    #[doc = "0x04 - Pointer register FIFO / gateway message object 0"]
    pub mofgpr: self::can_msg::MOFGPR,
    #[doc = "0x08 - Pointer register interrupt message object 0"]
    pub moipr: self::can_msg::MOIPR,
    #[doc = "0x0c - Mask register message object 0"]
    pub moamr: self::can_msg::MOAMR,
    #[doc = "0x10 - Low data registers of the message object 0"]
    pub modatal: self::can_msg::MODATAL,
    #[doc = "0x14 - High data registers of the message object 0"]
    pub modatah: self::can_msg::MODATAH,
    #[doc = "0x18 - Register arbitration message object 0"]
    pub moar: self::can_msg::MOAR,
    _reserved_7_moctr: [u8; 4usize],
}
impl CAN_MSG {
    #[doc = "0x1c - Status register of the message object 0"]
    #[inline(always)]
    pub fn mostat(&self) -> &self::can_msg::MOSTAT {
        unsafe {
            &*(((self as *const Self) as *const u8).add(28usize) as *const self::can_msg::MOSTAT)
        }
    }
    #[doc = "0x1c - Status register of the message object 0"]
    #[inline(always)]
    pub fn mostat_mut(&self) -> &mut self::can_msg::MOSTAT {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(28usize) as *mut self::can_msg::MOSTAT)
        }
    }
    #[doc = "0x1c - Control register Message object 0"]
    #[inline(always)]
    pub fn moctr(&self) -> &self::can_msg::MOCTR {
        unsafe {
            &*(((self as *const Self) as *const u8).add(28usize) as *const self::can_msg::MOCTR)
        }
    }
    #[doc = "0x1c - Control register Message object 0"]
    #[inline(always)]
    pub fn moctr_mut(&self) -> &mut self::can_msg::MOCTR {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(28usize) as *mut self::can_msg::MOCTR)
        }
    }
}
#[doc = r"Register block"]
#[doc = "CAN_Msg"]
pub mod can_msg;
#[doc = "Frequency control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [clc](clc) module"]
pub type CLC = crate::Reg<u32, _CLC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLC;
#[doc = "`read()` method returns [clc::R](clc::R) reader structure"]
impl crate::Readable for CLC {}
#[doc = "`write(|w| ..)` method takes [clc::W](clc::W) writer structure"]
impl crate::Writable for CLC {}
#[doc = "Frequency control register"]
pub mod clc;
#[doc = "Identity register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [id](id) module"]
pub type ID = crate::Reg<u32, _ID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ID;
#[doc = "`read()` method returns [id::R](id::R) reader structure"]
impl crate::Readable for ID {}
#[doc = "`write(|w| ..)` method takes [id::W](id::W) writer structure"]
impl crate::Writable for ID {}
#[doc = "Identity register"]
pub mod id;
#[doc = "Register divider\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fdr](fdr) module"]
pub type FDR = crate::Reg<u32, _FDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FDR;
#[doc = "`read()` method returns [fdr::R](fdr::R) reader structure"]
impl crate::Readable for FDR {}
#[doc = "`write(|w| ..)` method takes [fdr::W](fdr::W) writer structure"]
impl crate::Writable for FDR {}
#[doc = "Register divider"]
pub mod fdr;
#[doc = "Mask register message index\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [msimask](msimask) module"]
pub type MSIMASK = crate::Reg<u32, _MSIMASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MSIMASK;
#[doc = "`read()` method returns [msimask::R](msimask::R) reader structure"]
impl crate::Readable for MSIMASK {}
#[doc = "`write(|w| ..)` method takes [msimask::W](msimask::W) writer structure"]
impl crate::Writable for MSIMASK {}
#[doc = "Mask register message index"]
pub mod msimask;
#[doc = "Register command panel\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [panctr](panctr) module"]
pub type PANCTR = crate::Reg<u32, _PANCTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PANCTR;
#[doc = "`read()` method returns [panctr::R](panctr::R) reader structure"]
impl crate::Readable for PANCTR {}
#[doc = "`write(|w| ..)` method takes [panctr::W](panctr::W) writer structure"]
impl crate::Writable for PANCTR {}
#[doc = "Register command panel"]
pub mod panctr;
#[doc = "No description\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcr_reg](mcr_reg) module"]
pub type MCR_REG = crate::Reg<u32, _MCR_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCR_REG;
#[doc = "`read()` method returns [mcr_reg::R](mcr_reg::R) reader structure"]
impl crate::Readable for MCR_REG {}
#[doc = "`write(|w| ..)` method takes [mcr_reg::W](mcr_reg::W) writer structure"]
impl crate::Writable for MCR_REG {}
#[doc = "No description"]
pub mod mcr_reg;
#[doc = "Interrupt register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mitr](mitr) module"]
pub type MITR = crate::Reg<u32, _MITR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MITR;
#[doc = "`write(|w| ..)` method takes [mitr::W](mitr::W) writer structure"]
impl crate::Writable for MITR {}
#[doc = "Interrupt register"]
pub mod mitr;
