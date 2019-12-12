#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Part seconds register"]
    pub pseconds: PSECONDS,
    #[doc = "0x04 - Seconds register"]
    pub second: SECOND,
    #[doc = "0x08 - Minute register"]
    pub minute: MINUTE,
    _reserved3: [u8; 4usize],
    #[doc = "0x10 - Hour register"]
    pub hour: HOUR,
    _reserved4: [u8; 4usize],
    #[doc = "0x18 - Day of week register"]
    pub wday: WDAY,
    _reserved5: [u8; 4usize],
    #[doc = "0x20 - Day register"]
    pub day: DAY,
    #[doc = "0x24 - Month register"]
    pub month: MONTH,
    #[doc = "0x28 - Year register"]
    pub year: YEAR,
    #[doc = "0x2c - Register Update shadow registers RTC"]
    pub shdw: SHDW,
    #[doc = "0x30 - General register Time"]
    pub time: TIME,
}
#[doc = "Part seconds register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pseconds](pseconds) module"]
pub type PSECONDS = crate::Reg<u32, _PSECONDS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PSECONDS;
#[doc = "`read()` method returns [pseconds::R](pseconds::R) reader structure"]
impl crate::Readable for PSECONDS {}
#[doc = "`write(|w| ..)` method takes [pseconds::W](pseconds::W) writer structure"]
impl crate::Writable for PSECONDS {}
#[doc = "Part seconds register"]
pub mod pseconds;
#[doc = "Seconds register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [second](second) module"]
pub type SECOND = crate::Reg<u32, _SECOND>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SECOND;
#[doc = "`read()` method returns [second::R](second::R) reader structure"]
impl crate::Readable for SECOND {}
#[doc = "`write(|w| ..)` method takes [second::W](second::W) writer structure"]
impl crate::Writable for SECOND {}
#[doc = "Seconds register"]
pub mod second;
#[doc = "Minute register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [minute](minute) module"]
pub type MINUTE = crate::Reg<u32, _MINUTE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MINUTE;
#[doc = "`read()` method returns [minute::R](minute::R) reader structure"]
impl crate::Readable for MINUTE {}
#[doc = "`write(|w| ..)` method takes [minute::W](minute::W) writer structure"]
impl crate::Writable for MINUTE {}
#[doc = "Minute register"]
pub mod minute;
#[doc = "Hour register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hour](hour) module"]
pub type HOUR = crate::Reg<u32, _HOUR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOUR;
#[doc = "`read()` method returns [hour::R](hour::R) reader structure"]
impl crate::Readable for HOUR {}
#[doc = "`write(|w| ..)` method takes [hour::W](hour::W) writer structure"]
impl crate::Writable for HOUR {}
#[doc = "Hour register"]
pub mod hour;
#[doc = "Day of week register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [wday](wday) module"]
pub type WDAY = crate::Reg<u32, _WDAY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDAY;
#[doc = "`read()` method returns [wday::R](wday::R) reader structure"]
impl crate::Readable for WDAY {}
#[doc = "`write(|w| ..)` method takes [wday::W](wday::W) writer structure"]
impl crate::Writable for WDAY {}
#[doc = "Day of week register"]
pub mod wday;
#[doc = "Day register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [day](day) module"]
pub type DAY = crate::Reg<u32, _DAY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DAY;
#[doc = "`read()` method returns [day::R](day::R) reader structure"]
impl crate::Readable for DAY {}
#[doc = "`write(|w| ..)` method takes [day::W](day::W) writer structure"]
impl crate::Writable for DAY {}
#[doc = "Day register"]
pub mod day;
#[doc = "Month register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [month](month) module"]
pub type MONTH = crate::Reg<u32, _MONTH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MONTH;
#[doc = "`read()` method returns [month::R](month::R) reader structure"]
impl crate::Readable for MONTH {}
#[doc = "`write(|w| ..)` method takes [month::W](month::W) writer structure"]
impl crate::Writable for MONTH {}
#[doc = "Month register"]
pub mod month;
#[doc = "Year register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [year](year) module"]
pub type YEAR = crate::Reg<u32, _YEAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _YEAR;
#[doc = "`read()` method returns [year::R](year::R) reader structure"]
impl crate::Readable for YEAR {}
#[doc = "`write(|w| ..)` method takes [year::W](year::W) writer structure"]
impl crate::Writable for YEAR {}
#[doc = "Year register"]
pub mod year;
#[doc = "Register Update shadow registers RTC\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [shdw](shdw) module"]
pub type SHDW = crate::Reg<u32, _SHDW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHDW;
#[doc = "`write(|w| ..)` method takes [shdw::W](shdw::W) writer structure"]
impl crate::Writable for SHDW {}
#[doc = "Register Update shadow registers RTC"]
pub mod shdw;
#[doc = "General register Time\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [time](time) module"]
pub type TIME = crate::Reg<u32, _TIME>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIME;
#[doc = "`read()` method returns [time::R](time::R) reader structure"]
impl crate::Readable for TIME {}
#[doc = "General register Time"]
pub mod time;
