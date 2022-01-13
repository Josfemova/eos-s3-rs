#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Configuration register for the simple periodic timer"]
    pub spt_cfg: crate::Reg<spt_cfg::SPT_CFG_SPEC>,
    #[doc = "0x04 - Allows blocking the PMU and FFE kickoff signal"]
    pub sleep_mode: crate::Reg<sleep_mode::SLEEP_MODE_SPEC>,
    #[doc = "0x08 - 40 msec increment error compensation register"]
    pub error_cmp_40m: crate::Reg<error_cmp_40m::ERROR_CMP_40M_SPEC>,
    #[doc = "0x0c - 1 sec Increment Error Compensation 0 Register"]
    pub error_cmp_1s_0: crate::Reg<error_cmp_1s_0::ERROR_CMP_1S_0_SPEC>,
    #[doc = "0x10 - 1 sec Incremente Error Compensation 1 Register"]
    pub error_cmp_1s_1: crate::Reg<error_cmp_1s_1::ERROR_CMP_1S_1_SPEC>,
    #[doc = "0x14 - 1 sec Incremente Error Compensation 2 Register"]
    pub error_cmp_1s_2: crate::Reg<error_cmp_1s_2::ERROR_CMP_1S_2_SPEC>,
    #[doc = "0x18 - 1 sec Incremente Error Compensation 3 Register"]
    pub error_cmp_1s_3: crate::Reg<error_cmp_1s_3::ERROR_CMP_1S_3_SPEC>,
    #[doc = "0x1c - 2, 4, 6, 8, 16 sec Increment Error Compensation Register"]
    pub error_cmp_rtc_0: crate::Reg<error_cmp_rtc_0::ERROR_CMP_RTC_0_SPEC>,
    #[doc = "0x20 - 32, 64, 128, 256 sec Increment Error Compensation Register"]
    pub error_cmp_rtc_1: crate::Reg<error_cmp_rtc_1::ERROR_CMP_RTC_1_SPEC>,
    #[doc = "0x24 - 512, 1024, 2048, 4096 sec Increment Error Compensation Register."]
    pub error_cmp_rtc_2: crate::Reg<error_cmp_rtc_2::ERROR_CMP_RTC_2_SPEC>,
    #[doc = "0x28 - 8192, 16384 sec Increment Error Compensation Register"]
    pub error_cmp_rtc_3: crate::Reg<error_cmp_rtc_3::ERROR_CMP_RTC_3_SPEC>,
    #[doc = "0x2c - Update the 30-Bit Timer once write. Note: Please programmed SPT_EN (0x000, bit 0) to 0 before write this register to avoid any potential issue"]
    pub update_tmr_val: crate::Reg<update_tmr_val::UPDATE_TMR_VAL_SPEC>,
    #[doc = "0x30 - Not documented"]
    pub spare_bits: crate::Reg<spare_bits::SPARE_BITS_SPEC>,
    #[doc = "0x34 - Return the Value of 30-bits, in 1mS resoultion. This is the RTC value"]
    pub timer_value: crate::Reg<timer_value::TIMER_VALUE_SPEC>,
    _reserved_14_ms_cnt_value: [u8; 0x04],
}
impl RegisterBlock {
    #[doc = "0x38 - Return the Value of the 1mS counter which is generating the 1mS event. It is an downcount counter. Default is 0x40 (65-1). For 32KHz input, the resoultion is ~15uS. For 16KHz input, the resoultion is ~30uS"]
    #[inline(always)]
    pub fn ms_cnt_value(&self) -> &crate::Reg<ms_cnt_value::MS_CNT_VALUE_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(56usize)
                as *const crate::Reg<ms_cnt_value::MS_CNT_VALUE_SPEC>)
        }
    }
    #[doc = "0x38 - Return the Value of the Event counter generating FFE Time out event It is an upcount counter, in 1mS resoultion."]
    #[inline(always)]
    pub fn event_cnt_value(
        &self,
    ) -> &crate::Reg<event_cnt_value::EVENT_CNT_VALUE_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(56usize)
                as *const crate::Reg<event_cnt_value::EVENT_CNT_VALUE_SPEC>)
        }
    }
}
#[doc = "SPT_CFG register accessor: an alias for `Reg<SPT_CFG_SPEC>`"]
pub type SPT_CFG = crate::Reg<spt_cfg::SPT_CFG_SPEC>;
#[doc = "Configuration register for the simple periodic timer"]
pub mod spt_cfg;
#[doc = "SLEEP_MODE register accessor: an alias for `Reg<SLEEP_MODE_SPEC>`"]
pub type SLEEP_MODE = crate::Reg<sleep_mode::SLEEP_MODE_SPEC>;
#[doc = "Allows blocking the PMU and FFE kickoff signal"]
pub mod sleep_mode;
#[doc = "ERROR_CMP_40M register accessor: an alias for `Reg<ERROR_CMP_40M_SPEC>`"]
pub type ERROR_CMP_40M = crate::Reg<error_cmp_40m::ERROR_CMP_40M_SPEC>;
#[doc = "40 msec increment error compensation register"]
pub mod error_cmp_40m;
#[doc = "ERROR_CMP_1S_0 register accessor: an alias for `Reg<ERROR_CMP_1S_0_SPEC>`"]
pub type ERROR_CMP_1S_0 = crate::Reg<error_cmp_1s_0::ERROR_CMP_1S_0_SPEC>;
#[doc = "1 sec Increment Error Compensation 0 Register"]
pub mod error_cmp_1s_0;
#[doc = "ERROR_CMP_1S_1 register accessor: an alias for `Reg<ERROR_CMP_1S_1_SPEC>`"]
pub type ERROR_CMP_1S_1 = crate::Reg<error_cmp_1s_1::ERROR_CMP_1S_1_SPEC>;
#[doc = "1 sec Incremente Error Compensation 1 Register"]
pub mod error_cmp_1s_1;
#[doc = "ERROR_CMP_1S_2 register accessor: an alias for `Reg<ERROR_CMP_1S_2_SPEC>`"]
pub type ERROR_CMP_1S_2 = crate::Reg<error_cmp_1s_2::ERROR_CMP_1S_2_SPEC>;
#[doc = "1 sec Incremente Error Compensation 2 Register"]
pub mod error_cmp_1s_2;
#[doc = "ERROR_CMP_1S_3 register accessor: an alias for `Reg<ERROR_CMP_1S_3_SPEC>`"]
pub type ERROR_CMP_1S_3 = crate::Reg<error_cmp_1s_3::ERROR_CMP_1S_3_SPEC>;
#[doc = "1 sec Incremente Error Compensation 3 Register"]
pub mod error_cmp_1s_3;
#[doc = "ERROR_CMP_RTC_0 register accessor: an alias for `Reg<ERROR_CMP_RTC_0_SPEC>`"]
pub type ERROR_CMP_RTC_0 = crate::Reg<error_cmp_rtc_0::ERROR_CMP_RTC_0_SPEC>;
#[doc = "2, 4, 6, 8, 16 sec Increment Error Compensation Register"]
pub mod error_cmp_rtc_0;
#[doc = "ERROR_CMP_RTC_1 register accessor: an alias for `Reg<ERROR_CMP_RTC_1_SPEC>`"]
pub type ERROR_CMP_RTC_1 = crate::Reg<error_cmp_rtc_1::ERROR_CMP_RTC_1_SPEC>;
#[doc = "32, 64, 128, 256 sec Increment Error Compensation Register"]
pub mod error_cmp_rtc_1;
#[doc = "ERROR_CMP_RTC_2 register accessor: an alias for `Reg<ERROR_CMP_RTC_2_SPEC>`"]
pub type ERROR_CMP_RTC_2 = crate::Reg<error_cmp_rtc_2::ERROR_CMP_RTC_2_SPEC>;
#[doc = "512, 1024, 2048, 4096 sec Increment Error Compensation Register."]
pub mod error_cmp_rtc_2;
#[doc = "ERROR_CMP_RTC_3 register accessor: an alias for `Reg<ERROR_CMP_RTC_3_SPEC>`"]
pub type ERROR_CMP_RTC_3 = crate::Reg<error_cmp_rtc_3::ERROR_CMP_RTC_3_SPEC>;
#[doc = "8192, 16384 sec Increment Error Compensation Register"]
pub mod error_cmp_rtc_3;
#[doc = "UPDATE_TMR_VAL register accessor: an alias for `Reg<UPDATE_TMR_VAL_SPEC>`"]
pub type UPDATE_TMR_VAL = crate::Reg<update_tmr_val::UPDATE_TMR_VAL_SPEC>;
#[doc = "Update the 30-Bit Timer once write. Note: Please programmed SPT_EN (0x000, bit 0) to 0 before write this register to avoid any potential issue"]
pub mod update_tmr_val;
#[doc = "SPARE_BITS register accessor: an alias for `Reg<SPARE_BITS_SPEC>`"]
pub type SPARE_BITS = crate::Reg<spare_bits::SPARE_BITS_SPEC>;
#[doc = "Not documented"]
pub mod spare_bits;
#[doc = "TIMER_VALUE register accessor: an alias for `Reg<TIMER_VALUE_SPEC>`"]
pub type TIMER_VALUE = crate::Reg<timer_value::TIMER_VALUE_SPEC>;
#[doc = "Return the Value of 30-bits, in 1mS resoultion. This is the RTC value"]
pub mod timer_value;
#[doc = "EVENT_CNT_VALUE register accessor: an alias for `Reg<EVENT_CNT_VALUE_SPEC>`"]
pub type EVENT_CNT_VALUE = crate::Reg<event_cnt_value::EVENT_CNT_VALUE_SPEC>;
#[doc = "Return the Value of the Event counter generating FFE Time out event It is an upcount counter, in 1mS resoultion."]
pub mod event_cnt_value;
#[doc = "MS_CNT_VALUE register accessor: an alias for `Reg<MS_CNT_VALUE_SPEC>`"]
pub type MS_CNT_VALUE = crate::Reg<ms_cnt_value::MS_CNT_VALUE_SPEC>;
#[doc = "Return the Value of the 1mS counter which is generating the 1mS event. It is an downcount counter. Default is 0x40 (65-1). For 32KHz input, the resoultion is ~15uS. For 16KHz input, the resoultion is ~30uS"]
pub mod ms_cnt_value;
