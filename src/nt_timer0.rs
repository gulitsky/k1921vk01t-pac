#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Timer register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Current value timer register"]
    pub value: VALUE,
    #[doc = "0x08 - Reload value timer register"]
    pub reload: RELOAD,
    #[doc = "0x0c - No description"]
    pub intstatus_intclear: INTSTATUS_INTCLEAR,
}
#[doc = "Control Timer register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "Control Timer register"]
pub mod ctrl;
#[doc = "Current value timer register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [value](value) module"]
pub type VALUE = crate::Reg<u32, _VALUE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VALUE;
#[doc = "`read()` method returns [value::R](value::R) reader structure"]
impl crate::Readable for VALUE {}
#[doc = "`write(|w| ..)` method takes [value::W](value::W) writer structure"]
impl crate::Writable for VALUE {}
#[doc = "Current value timer register"]
pub mod value;
#[doc = "Reload value timer register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [reload](reload) module"]
pub type RELOAD = crate::Reg<u32, _RELOAD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RELOAD;
#[doc = "`read()` method returns [reload::R](reload::R) reader structure"]
impl crate::Readable for RELOAD {}
#[doc = "`write(|w| ..)` method takes [reload::W](reload::W) writer structure"]
impl crate::Writable for RELOAD {}
#[doc = "Reload value timer register"]
pub mod reload;
#[doc = "No description\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intstatus_intclear](intstatus_intclear) module"]
pub type INTSTATUS_INTCLEAR = crate::Reg<u32, _INTSTATUS_INTCLEAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTSTATUS_INTCLEAR;
#[doc = "`read()` method returns [intstatus_intclear::R](intstatus_intclear::R) reader structure"]
impl crate::Readable for INTSTATUS_INTCLEAR {}
#[doc = "`write(|w| ..)` method takes [intstatus_intclear::W](intstatus_intclear::W) writer structure"]
impl crate::Writable for INTSTATUS_INTCLEAR {}
#[doc = "No description"]
pub mod intstatus_intclear;
