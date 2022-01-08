#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - On POR Reset Domain"]
    pub misc_por_0: crate::Reg<misc_por_0::MISC_POR_0_SPEC>,
    #[doc = "0x04 - On POR Reset Domain"]
    pub misc_por_1: crate::Reg<misc_por_1::MISC_POR_1_SPEC>,
    #[doc = "0x08 - On POR Reset Domain"]
    pub misc_por_2: crate::Reg<misc_por_2::MISC_POR_2_SPEC>,
    #[doc = "0x0c - On POR Reset Domain"]
    pub misc_por_3: crate::Reg<misc_por_3::MISC_POR_3_SPEC>,
    #[doc = "0x10 - Reserved"]
    pub rst_ctrl_0: crate::Reg<rst_ctrl_0::RST_CTRL_0_SPEC>,
    #[doc = "0x14 - Reserved"]
    pub rst_ctrl_1: crate::Reg<rst_ctrl_1::RST_CTRL_1_SPEC>,
    #[doc = "0x18 - Reserved"]
    pub chip_sta_0: crate::Reg<chip_sta_0::CHIP_STA_0_SPEC>,
    #[doc = "0x1c - Chip Status register"]
    pub chip_sta_1: crate::Reg<chip_sta_1::CHIP_STA_1_SPEC>,
    #[doc = "0x20 - Wake-up Interrupt Controller control register"]
    pub wic_ctrl: crate::Reg<wic_ctrl::WIC_CTRL_SPEC>,
    #[doc = "0x24 - Wake-up Interrupt Controller Status register"]
    pub wic_status: crate::Reg<wic_status::WIC_STATUS_SPEC>,
    _reserved10: [u8; 0x08],
    #[doc = "0x30 - Power Down Scheme configuration"]
    pub pwr_dwn_sch: crate::Reg<pwr_dwn_sch::PWR_DWN_SCH_SPEC>,
    _reserved11: [u8; 0x0c],
    #[doc = "0x40 - Control the power state of Oscillator once the M4 is in Power Saving Mode"]
    pub pwr_off_osc: crate::Reg<pwr_off_osc::PWR_OFF_OSC_SPEC>,
    #[doc = "0x44 - Configure the external wakeup event source. Turn on the OSC once PMUT is time out or GPIO INT is triggering."]
    pub ext_waking_up_src:
        crate::Reg<ext_waking_up_src::EXT_WAKING_UP_SRC_SPEC>,
    _reserved13: [u8; 0x28],
    #[doc = "0x70 - SDMA status register"]
    pub sdma_status: crate::Reg<sdma_status::SDMA_STATUS_SPEC>,
    #[doc = "0x74 - Register for SDMA Power Mode configuration"]
    pub sdma_power_mode_cfg:
        crate::Reg<sdma_power_mode_cfg::SDMA_POWER_MODE_CFG_SPEC>,
    #[doc = "0x78 - Register for controlling if power down event will be masked"]
    pub sdma_pd_src_mask_n:
        crate::Reg<sdma_pd_src_mask_n::SDMA_PD_SRC_MASK_N_SPEC>,
    #[doc = "0x7c - Reserved"]
    pub sdma_wu_src_mask_n:
        crate::Reg<sdma_wu_src_mask_n::SDMA_WU_SRC_MASK_N_SPEC>,
    _reserved_17_m4_status: [u8; 0x04],
    #[doc = "0x84 - Configuration for the M4 power domain"]
    pub m4_pwr_mode_cfg: crate::Reg<m4_pwr_mode_cfg::M4_PWR_MODE_CFG_SPEC>,
    #[doc = "0x88 - Reserved"]
    pub m4_pd_src_maskk_n:
        crate::Reg<m4_pd_src_maskk_n::M4_PD_SRC_MASKK_N_SPEC>,
    #[doc = "0x8c - Reserved"]
    pub m4_wu: crate::Reg<m4_wu::M4_WU_SPEC>,
    #[doc = "0x90 - Status of the Flexible Fusion Engine"]
    pub ffe_status: crate::Reg<ffe_status::FFE_STATUS_SPEC>,
    #[doc = "0x94 - Power Mode configuration for the Flexible Fusion Engine Power Domain"]
    pub ffe_pwr_mode_cfg: crate::Reg<ffe_pwr_mode_cfg::FFE_PWR_MODE_CFG_SPEC>,
    #[doc = "0x98 - Control masking of busy signal. The falling edge of any of the above signals (non-mask) will put the FFE into Power saving mode base on the Power Mode Cfg. Note: These signals used to indicate the BUSY status, so they must be level signals."]
    pub ffe_pd_src_mask_n:
        crate::Reg<ffe_pd_src_mask_n::FFE_PD_SRC_MASK_N_SPEC>,
    #[doc = "0x9c - Control the masking of the Flexible Fusion Engine wake-up event triggers"]
    pub ffe_wu_src_mask_n:
        crate::Reg<ffe_wu_src_mask_n::FFE_WU_SRC_MASK_N_SPEC>,
    #[doc = "0xa0 - FPGA Fabric Power domain status"]
    pub fb_status: crate::Reg<fb_status::FB_STATUS_SPEC>,
    #[doc = "0xa4 - Power mode configuration for the FPGA Fabric Power domain"]
    pub fb_pwr_mode_cfg: crate::Reg<fb_pwr_mode_cfg::FB_PWR_MODE_CFG_SPEC>,
    #[doc = "0xa8 - Control masking of power down event signals for the FPGA Fabric power domain. The falling edge of any of the listed signals (non-mask) will put the FB into Power saving mode base on the Power Mode Cfg. Note: These signals used to indicate the BUSY status, so they must be level signals."]
    pub fb_pd_src_mask_n: crate::Reg<fb_pd_src_mask_n::FB_PD_SRC_MASK_N_SPEC>,
    #[doc = "0xac - Control the masking of the FPGA FAbric wake-up event triggers"]
    pub fb_wu_src_mask_n: crate::Reg<fb_wu_src_mask_n::FB_WU_SRC_MASK_N_SPEC>,
    _reserved29: [u8; 0x04],
    #[doc = "0xb4 - Power mode configuration for the PF SRAM Power domain"]
    pub pf_pwr_mode_cfg: crate::Reg<pf_pwr_mode_cfg::PF_PWR_MODE_CFG_SPEC>,
    #[doc = "0xb8 - Reserved"]
    pub pf_pd_src_mask_n: crate::Reg<pf_pd_src_mask_n::PF_PD_SRC_MASK_N_SPEC>,
    #[doc = "0xbc - Reserved"]
    pub pf_wu_src_mask_n: crate::Reg<pf_wu_src_mask_n::PF_WU_SRC_MASK_N_SPEC>,
    #[doc = "0xc0 - M4S0 SRAM Power Domain status"]
    pub m4s0_sram_status: crate::Reg<m4s0_sram_status::M4S0_SRAM_STATUS_SPEC>,
    #[doc = "0xc4 - Power mode configuration for the M4S0 SRAM power domain"]
    pub m4s0_pwr_mode_cfg:
        crate::Reg<m4s0_pwr_mode_cfg::M4S0_PWR_MODE_CFG_SPEC>,
    #[doc = "0xc8 - Control masking of power-down event triggers for the M4S0 SRAM domain"]
    pub m4s0_pd_src_mask_n:
        crate::Reg<m4s0_pd_src_mask_n::M4S0_PD_SRC_MASK_N_SPEC>,
    #[doc = "0xcc - Control masking of wake-up event triggers for the M4S0 SRAM domain"]
    pub m4s0_wu_src_mask_n:
        crate::Reg<m4s0_wu_src_mask_n::M4S0_WU_SRC_MASK_N_SPEC>,
    #[doc = "0xd0 - Status of the A1 power domain"]
    pub a1_status: crate::Reg<a1_status::A1_STATUS_SPEC>,
    #[doc = "0xd4 - Power mode configuration for the A1 power domain"]
    pub a1_pwr_mode_cfg: crate::Reg<a1_pwr_mode_cfg::A1_PWR_MODE_CFG_SPEC>,
    #[doc = "0xd8 - Reserved"]
    pub a1_pd_src_mask_n: crate::Reg<a1_pd_src_mask_n::A1_PD_SRC_MASK_N_SPEC>,
    #[doc = "0xdc - Control masking of wake-up event triggers for the A1 domain"]
    pub a1_wu_src_mask_n: crate::Reg<a1_wu_src_mask_n::A1_WU_SRC_MASK_N_SPEC>,
    #[doc = "0xe0 - I2S Power info"]
    pub misc_status: crate::Reg<misc_status::MISC_STATUS_SPEC>,
    #[doc = "0xe4 - Audio power domain status"]
    pub audio_status: crate::Reg<audio_status::AUDIO_STATUS_SPEC>,
    #[doc = "0xe8 - M4 SRAM Power domain status"]
    pub m4_sram_status: crate::Reg<m4_sram_status::M4_SRAM_STATUS_SPEC>,
    #[doc = "0xec - Control masking of wake-up event triggers for the Audio domains"]
    pub audio_wu_src_mask_n:
        crate::Reg<audio_wu_src_mask_n::AUDIO_WU_SRC_MASK_N_SPEC>,
    _reserved44: [u8; 0x10],
    #[doc = "0x100 - Control DS pins for different SRAM instances on the M4 subsystem. For each instance: 1'b1 : Enable the Deep Sleep funciton of SRAM Macro, Memory content will be kept. While M4 access the memory in Deep Sleep mode, the HW will clear the corresponding bit."]
    pub m4_mem_ctrl_0: crate::Reg<m4_mem_ctrl_0::M4_MEM_CTRL_0_SPEC>,
    #[doc = "0x104 - Control Shutdown pin for various instances of SRAM on the M4 subsystem. For each instance: 1'b1 : Enable the Shutdown funciton of SRAM Macro, Memory content will be lost. While M4 access the memory in Shutdown mode, the HW will clear the corresponding bit."]
    pub m4_mem_ctrl_1: crate::Reg<m4_mem_ctrl_1::M4_MEM_CTRL_1_SPEC>,
    #[doc = "0x108 - RESERVED"]
    pub pf_mem_ctrl_0: crate::Reg<pf_mem_ctrl_0::PF_MEM_CTRL_0_SPEC>,
    #[doc = "0x10c - Control Shut Down pin of various FIFOs intances in the PF subsystem. For each one: 1'b1 : Enable the Shut Down function of SRAM Macro, Memory content will be lost"]
    pub pf_mem_ctrl_1: crate::Reg<pf_mem_ctrl_1::PF_MEM_CTRL_1_SPEC>,
    #[doc = "0x110 - Control the Deep Sleep pin of various elements in the Flexible Fusion Engine power domain. For each: 1'b1 : Enable the Deep Sleep function of SRAM Macro, Memory content will be kept."]
    pub ffe_mem_ctrl_0: crate::Reg<ffe_mem_ctrl_0::FFE_MEM_CTRL_0_SPEC>,
    #[doc = "0x114 - Control the Shut Down pin of various elements in the Flexible Fusion Engine power domain. For each: 1'b1 : Enable the Deep Sleep function of SRAM Macro, Memory content will be kept."]
    pub ffe_mem_ctrl_1: crate::Reg<ffe_mem_ctrl_1::FFE_MEM_CTRL_1_SPEC>,
    #[doc = "0x118 - Control the Deep Sleep pin of Audio channels. For each: 1'b1 : Enable the Deep Sleep function of SRAM Macro, Memory content will be kept."]
    pub audio_mem_ctrl_0: crate::Reg<audio_mem_ctrl_0::AUDIO_MEM_CTRL_0_SPEC>,
    #[doc = "0x11c - Control the shut down pin of Audio channels. For each: 1'b1 : Enable the Deep Sleep function of SRAM Macro, Memory content will be kept."]
    pub audio_mem_ctrl_1: crate::Reg<audio_mem_ctrl_1::AUDIO_MEM_CTRL_1_SPEC>,
    #[doc = "0x120 - Reserved"]
    pub m4_mem_cfg: crate::Reg<m4_mem_cfg::M4_MEM_CFG_SPEC>,
    #[doc = "0x124 - Reserved"]
    pub pf_mem_cfg: crate::Reg<pf_mem_cfg::PF_MEM_CFG_SPEC>,
    #[doc = "0x128 - Control Light Sleep pin of different FFE SRAM power domains"]
    pub ffe_mem_cfg: crate::Reg<ffe_mem_cfg::FFE_MEM_CFG_SPEC>,
    #[doc = "0x12c - Reserved"]
    pub audio_mem_cfg: crate::Reg<audio_mem_cfg::AUDIO_MEM_CFG_SPEC>,
    #[doc = "0x130 - Reserved"]
    pub m4_mem_ctrl_pwr_0:
        crate::Reg<m4_mem_ctrl_pwr_0::M4_MEM_CTRL_PWR_0_SPEC>,
    #[doc = "0x134 - Reserved"]
    pub m4_mem_ctrl_pwr_1:
        crate::Reg<m4_mem_ctrl_pwr_1::M4_MEM_CTRL_PWR_1_SPEC>,
    #[doc = "0x138 - Reserved"]
    pub m4_mem_ctrl_pwr_2:
        crate::Reg<m4_mem_ctrl_pwr_2::M4_MEM_CTRL_PWR_2_SPEC>,
    _reserved59: [u8; 0x04],
    #[doc = "0x140 - Control the Deep Sleep function of SRAM Macro for the SDMA power domain"]
    pub sdma_mem_ctrl_0: crate::Reg<sdma_mem_ctrl_0::SDMA_MEM_CTRL_0_SPEC>,
    #[doc = "0x144 - Control the Shut Down function of SRAM Macro for the SDMA power domain"]
    pub sdma_mem_ctrl_1: crate::Reg<sdma_mem_ctrl_1::SDMA_MEM_CTRL_1_SPEC>,
    _reserved61: [u8; 0x38],
    #[doc = "0x180 - Memory Power Down Control"]
    pub mem_pwr_dwn_ctrl: crate::Reg<mem_pwr_dwn_ctrl::MEM_PWR_DWN_CTRL_SPEC>,
    #[doc = "0x184 - Configuration for the PMU timer time-out period"]
    pub pmu_timer_cfg_0: crate::Reg<pmu_timer_cfg_0::PMU_TIMER_CFG_0_SPEC>,
    #[doc = "0x188 - Control wether the PMU timer is enabled or disabled"]
    pub pmu_timer_cfg_1: crate::Reg<pmu_timer_cfg_1::PMU_TIMER_CFG_1_SPEC>,
    #[doc = "0x18c - Control the delay for power-on after wake-up event. Applies to all power domains"]
    pub pdwu_timer_cfg: crate::Reg<pdwu_timer_cfg::PDWU_TIMER_CFG_SPEC>,
    _reserved65: [u8; 0x70],
    #[doc = "0x200 - Registers for triggering power-down events in the FFE, FB and PF power domains."]
    pub ffe_fb_pf_sw_pd: crate::Reg<ffe_fb_pf_sw_pd::FFE_FB_PF_SW_PD_SPEC>,
    #[doc = "0x204 - Register for triggering power-down events in M4 SRAM power domains. (RWHC)"]
    pub m4_sram_sw_pd: crate::Reg<m4_sram_sw_pd::M4_SRAM_SW_PD_SPEC>,
    #[doc = "0x208 - Register for triggering power down events in MISC power domains + some general purpose SFR's (RWHC)"]
    pub misc_sw_pd: crate::Reg<misc_sw_pd::MISC_SW_PD_SPEC>,
    #[doc = "0x20c - Register for triggering power-down events in Audio power domains. (RWHC)"]
    pub audio_sw_pd: crate::Reg<audio_sw_pd::AUDIO_SW_PD_SPEC>,
    #[doc = "0x210 - Registers for triggering wake-up events in the FFE, FB and PF power domains."]
    pub ffe_fb_pf_sw_wu: crate::Reg<ffe_fb_pf_sw_wu::FFE_FB_PF_SW_WU_SPEC>,
    #[doc = "0x214 - Register for triggering wake-up events in M4 SRAM power domains. (RWHC)"]
    pub m4_sram_sw_wu: crate::Reg<m4_sram_sw_wu::M4_SRAM_SW_WU_SPEC>,
    #[doc = "0x218 - Register for triggering wake up events in MISC power domains + some general purpose SFR's (RWHC)"]
    pub misc_sw_wu: crate::Reg<misc_sw_wu::MISC_SW_WU_SPEC>,
    #[doc = "0x21c - Register for triggering wake-up events in Audio power domains. (RWHC)"]
    pub audio_sram_sw_wu: crate::Reg<audio_sram_sw_wu::AUDIO_SRAM_SW_WU_SPEC>,
    #[doc = "0x220 - Power Management Unit Software Test Mode priority control"]
    pub pmu_stm_priority: crate::Reg<pmu_stm_priority::PMU_STM_PRIORITY_SPEC>,
    _reserved74: [u8; 0x0c],
    #[doc = "0x230 - Control for M4SRAM power domain light sleep mode"]
    pub m4sram_ssw_lpmf: crate::Reg<m4sram_ssw_lpmf::M4SRAM_SSW_LPMF_SPEC>,
    #[doc = "0x234 - Control masking for the LPMH (Low Power Mode header - deep sleep circuit)"]
    pub m4sram_ssw_lpmh_mask_n:
        crate::Reg<m4sram_ssw_lpmh_mask_n::M4SRAM_SSW_LPMH_MASK_N_SPEC>,
    _reserved76: [u8; 0x01b0],
    #[doc = "0x3e8 - Configuration for the amount of IDLE cycles before powering on the FB domain"]
    pub fbvlpmin_width: crate::Reg<fbvlpmin_width::FBVLPMINWIDTH_SPEC>,
    #[doc = "0x3ec - Indicates if AP nees to reload the code to SRAM"]
    pub apreboot_status: crate::Reg<apreboot_status::APREBOOTSTATUS_SPEC>,
    _reserved_78_fb_isolation: [u8; 0x08],
}
impl RegisterBlock {
    #[doc = "0x80 - PF SRAM Power Domain status"]
    #[inline(always)]
    pub fn pf_status(&self) -> &crate::Reg<pf_status::PF_STATUS_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(128usize)
                as *const crate::Reg<pf_status::PF_STATUS_SPEC>)
        }
    }
    #[doc = "0x80 - Status of the M4 Power Domain"]
    #[inline(always)]
    pub fn m4_status(&self) -> &crate::Reg<m4_status::M4_STATUS_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(128usize)
                as *const crate::Reg<m4_status::M4_STATUS_SPEC>)
        }
    }
    #[doc = "0x3f0 - Configure FB config enable and wether Audio SRAM can be put into Deep Sleep by the Audio hardware"]
    #[inline(always)]
    pub fn gen_purpose_0(
        &self,
    ) -> &crate::Reg<gen_purpose_0::GEN_PURPOSE_0_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1008usize)
                as *const crate::Reg<gen_purpose_0::GEN_PURPOSE_0_SPEC>)
        }
    }
    #[doc = "0x3f3 - Control for: Wether ext-interrupt can be used to wake up FFE, and clock switching for FFE/M4 power domains"]
    #[inline(always)]
    pub fn gen_purpose_1(
        &self,
    ) -> &crate::Reg<gen_purpose_1::GEN_PURPOSE_1_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1011usize)
                as *const crate::Reg<gen_purpose_1::GEN_PURPOSE_1_SPEC>)
        }
    }
    #[doc = "0x3f4 - Control the FB Isolation"]
    #[inline(always)]
    pub fn fb_isolation(&self) -> &crate::Reg<fb_isolation::FB_ISOLATION_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1012usize)
                as *const crate::Reg<fb_isolation::FB_ISOLATION_SPEC>)
        }
    }
}
#[doc = "MISC_POR_0 register accessor: an alias for `Reg<MISC_POR_0_SPEC>`"]
pub type MISC_POR_0 = crate::Reg<misc_por_0::MISC_POR_0_SPEC>;
#[doc = "On POR Reset Domain"]
pub mod misc_por_0;
#[doc = "MISC_POR_1 register accessor: an alias for `Reg<MISC_POR_1_SPEC>`"]
pub type MISC_POR_1 = crate::Reg<misc_por_1::MISC_POR_1_SPEC>;
#[doc = "On POR Reset Domain"]
pub mod misc_por_1;
#[doc = "MISC_POR_2 register accessor: an alias for `Reg<MISC_POR_2_SPEC>`"]
pub type MISC_POR_2 = crate::Reg<misc_por_2::MISC_POR_2_SPEC>;
#[doc = "On POR Reset Domain"]
pub mod misc_por_2;
#[doc = "MISC_POR_3 register accessor: an alias for `Reg<MISC_POR_3_SPEC>`"]
pub type MISC_POR_3 = crate::Reg<misc_por_3::MISC_POR_3_SPEC>;
#[doc = "On POR Reset Domain"]
pub mod misc_por_3;
#[doc = "RST_CTRL_0 register accessor: an alias for `Reg<RST_CTRL_0_SPEC>`"]
pub type RST_CTRL_0 = crate::Reg<rst_ctrl_0::RST_CTRL_0_SPEC>;
#[doc = "Reserved"]
pub mod rst_ctrl_0;
#[doc = "RST_CTRL_1 register accessor: an alias for `Reg<RST_CTRL_1_SPEC>`"]
pub type RST_CTRL_1 = crate::Reg<rst_ctrl_1::RST_CTRL_1_SPEC>;
#[doc = "Reserved"]
pub mod rst_ctrl_1;
#[doc = "CHIP_STA_0 register accessor: an alias for `Reg<CHIP_STA_0_SPEC>`"]
pub type CHIP_STA_0 = crate::Reg<chip_sta_0::CHIP_STA_0_SPEC>;
#[doc = "Reserved"]
pub mod chip_sta_0;
#[doc = "CHIP_STA_1 register accessor: an alias for `Reg<CHIP_STA_1_SPEC>`"]
pub type CHIP_STA_1 = crate::Reg<chip_sta_1::CHIP_STA_1_SPEC>;
#[doc = "Chip Status register"]
pub mod chip_sta_1;
#[doc = "WIC_CTRL register accessor: an alias for `Reg<WIC_CTRL_SPEC>`"]
pub type WIC_CTRL = crate::Reg<wic_ctrl::WIC_CTRL_SPEC>;
#[doc = "Wake-up Interrupt Controller control register"]
pub mod wic_ctrl;
#[doc = "WIC_STATUS register accessor: an alias for `Reg<WIC_STATUS_SPEC>`"]
pub type WIC_STATUS = crate::Reg<wic_status::WIC_STATUS_SPEC>;
#[doc = "Wake-up Interrupt Controller Status register"]
pub mod wic_status;
#[doc = "PWR_DWN_SCH register accessor: an alias for `Reg<PWR_DWN_SCH_SPEC>`"]
pub type PWR_DWN_SCH = crate::Reg<pwr_dwn_sch::PWR_DWN_SCH_SPEC>;
#[doc = "Power Down Scheme configuration"]
pub mod pwr_dwn_sch;
#[doc = "PWR_OFF_OSC register accessor: an alias for `Reg<PWR_OFF_OSC_SPEC>`"]
pub type PWR_OFF_OSC = crate::Reg<pwr_off_osc::PWR_OFF_OSC_SPEC>;
#[doc = "Control the power state of Oscillator once the M4 is in Power Saving Mode"]
pub mod pwr_off_osc;
#[doc = "EXT_WAKING_UP_SRC register accessor: an alias for `Reg<EXT_WAKING_UP_SRC_SPEC>`"]
pub type EXT_WAKING_UP_SRC =
    crate::Reg<ext_waking_up_src::EXT_WAKING_UP_SRC_SPEC>;
#[doc = "Configure the external wakeup event source. Turn on the OSC once PMUT is time out or GPIO INT is triggering."]
pub mod ext_waking_up_src;
#[doc = "SDMA_STATUS register accessor: an alias for `Reg<SDMA_STATUS_SPEC>`"]
pub type SDMA_STATUS = crate::Reg<sdma_status::SDMA_STATUS_SPEC>;
#[doc = "SDMA status register"]
pub mod sdma_status;
#[doc = "SDMA_POWER_MODE_CFG register accessor: an alias for `Reg<SDMA_POWER_MODE_CFG_SPEC>`"]
pub type SDMA_POWER_MODE_CFG =
    crate::Reg<sdma_power_mode_cfg::SDMA_POWER_MODE_CFG_SPEC>;
#[doc = "Register for SDMA Power Mode configuration"]
pub mod sdma_power_mode_cfg;
#[doc = "SDMA_PD_SRC_MASK_N register accessor: an alias for `Reg<SDMA_PD_SRC_MASK_N_SPEC>`"]
pub type SDMA_PD_SRC_MASK_N =
    crate::Reg<sdma_pd_src_mask_n::SDMA_PD_SRC_MASK_N_SPEC>;
#[doc = "Register for controlling if power down event will be masked"]
pub mod sdma_pd_src_mask_n;
#[doc = "SDMA_WU_SRC_MASK_N register accessor: an alias for `Reg<SDMA_WU_SRC_MASK_N_SPEC>`"]
pub type SDMA_WU_SRC_MASK_N =
    crate::Reg<sdma_wu_src_mask_n::SDMA_WU_SRC_MASK_N_SPEC>;
#[doc = "Reserved"]
pub mod sdma_wu_src_mask_n;
#[doc = "M4_STATUS register accessor: an alias for `Reg<M4_STATUS_SPEC>`"]
pub type M4_STATUS = crate::Reg<m4_status::M4_STATUS_SPEC>;
#[doc = "Status of the M4 Power Domain"]
pub mod m4_status;
#[doc = "M4_PWR_MODE_CFG register accessor: an alias for `Reg<M4_PWR_MODE_CFG_SPEC>`"]
pub type M4_PWR_MODE_CFG = crate::Reg<m4_pwr_mode_cfg::M4_PWR_MODE_CFG_SPEC>;
#[doc = "Configuration for the M4 power domain"]
pub mod m4_pwr_mode_cfg;
#[doc = "M4_PD_SRC_MASKk_N register accessor: an alias for `Reg<M4_PD_SRC_MASKK_N_SPEC>`"]
pub type M4_PD_SRC_MASKK_N =
    crate::Reg<m4_pd_src_maskk_n::M4_PD_SRC_MASKK_N_SPEC>;
#[doc = "Reserved"]
pub mod m4_pd_src_maskk_n;
#[doc = "M4_WU register accessor: an alias for `Reg<M4_WU_SPEC>`"]
pub type M4_WU = crate::Reg<m4_wu::M4_WU_SPEC>;
#[doc = "Reserved"]
pub mod m4_wu;
#[doc = "FFE_STATUS register accessor: an alias for `Reg<FFE_STATUS_SPEC>`"]
pub type FFE_STATUS = crate::Reg<ffe_status::FFE_STATUS_SPEC>;
#[doc = "Status of the Flexible Fusion Engine"]
pub mod ffe_status;
#[doc = "FFE_PWR_MODE_CFG register accessor: an alias for `Reg<FFE_PWR_MODE_CFG_SPEC>`"]
pub type FFE_PWR_MODE_CFG = crate::Reg<ffe_pwr_mode_cfg::FFE_PWR_MODE_CFG_SPEC>;
#[doc = "Power Mode configuration for the Flexible Fusion Engine Power Domain"]
pub mod ffe_pwr_mode_cfg;
#[doc = "FFE_PD_SRC_MASK_N register accessor: an alias for `Reg<FFE_PD_SRC_MASK_N_SPEC>`"]
pub type FFE_PD_SRC_MASK_N =
    crate::Reg<ffe_pd_src_mask_n::FFE_PD_SRC_MASK_N_SPEC>;
#[doc = "Control masking of busy signal. The falling edge of any of the above signals (non-mask) will put the FFE into Power saving mode base on the Power Mode Cfg. Note: These signals used to indicate the BUSY status, so they must be level signals."]
pub mod ffe_pd_src_mask_n;
#[doc = "FFE_WU_SRC_MASK_N register accessor: an alias for `Reg<FFE_WU_SRC_MASK_N_SPEC>`"]
pub type FFE_WU_SRC_MASK_N =
    crate::Reg<ffe_wu_src_mask_n::FFE_WU_SRC_MASK_N_SPEC>;
#[doc = "Control the masking of the Flexible Fusion Engine wake-up event triggers"]
pub mod ffe_wu_src_mask_n;
#[doc = "FB_STATUS register accessor: an alias for `Reg<FB_STATUS_SPEC>`"]
pub type FB_STATUS = crate::Reg<fb_status::FB_STATUS_SPEC>;
#[doc = "FPGA Fabric Power domain status"]
pub mod fb_status;
#[doc = "FB_PWR_MODE_CFG register accessor: an alias for `Reg<FB_PWR_MODE_CFG_SPEC>`"]
pub type FB_PWR_MODE_CFG = crate::Reg<fb_pwr_mode_cfg::FB_PWR_MODE_CFG_SPEC>;
#[doc = "Power mode configuration for the FPGA Fabric Power domain"]
pub mod fb_pwr_mode_cfg;
#[doc = "FB_PD_SRC_MASK_N register accessor: an alias for `Reg<FB_PD_SRC_MASK_N_SPEC>`"]
pub type FB_PD_SRC_MASK_N = crate::Reg<fb_pd_src_mask_n::FB_PD_SRC_MASK_N_SPEC>;
#[doc = "Control masking of power down event signals for the FPGA Fabric power domain. The falling edge of any of the listed signals (non-mask) will put the FB into Power saving mode base on the Power Mode Cfg. Note: These signals used to indicate the BUSY status, so they must be level signals."]
pub mod fb_pd_src_mask_n;
#[doc = "FB_WU_SRC_MASK_N register accessor: an alias for `Reg<FB_WU_SRC_MASK_N_SPEC>`"]
pub type FB_WU_SRC_MASK_N = crate::Reg<fb_wu_src_mask_n::FB_WU_SRC_MASK_N_SPEC>;
#[doc = "Control the masking of the FPGA FAbric wake-up event triggers"]
pub mod fb_wu_src_mask_n;
#[doc = "PF_STATUS register accessor: an alias for `Reg<PF_STATUS_SPEC>`"]
pub type PF_STATUS = crate::Reg<pf_status::PF_STATUS_SPEC>;
#[doc = "PF SRAM Power Domain status"]
pub mod pf_status;
#[doc = "PF_PWR_MODE_CFG register accessor: an alias for `Reg<PF_PWR_MODE_CFG_SPEC>`"]
pub type PF_PWR_MODE_CFG = crate::Reg<pf_pwr_mode_cfg::PF_PWR_MODE_CFG_SPEC>;
#[doc = "Power mode configuration for the PF SRAM Power domain"]
pub mod pf_pwr_mode_cfg;
#[doc = "PF_PD_SRC_MASK_N register accessor: an alias for `Reg<PF_PD_SRC_MASK_N_SPEC>`"]
pub type PF_PD_SRC_MASK_N = crate::Reg<pf_pd_src_mask_n::PF_PD_SRC_MASK_N_SPEC>;
#[doc = "Reserved"]
pub mod pf_pd_src_mask_n;
#[doc = "PF_WU_SRC_MASK_N register accessor: an alias for `Reg<PF_WU_SRC_MASK_N_SPEC>`"]
pub type PF_WU_SRC_MASK_N = crate::Reg<pf_wu_src_mask_n::PF_WU_SRC_MASK_N_SPEC>;
#[doc = "Reserved"]
pub mod pf_wu_src_mask_n;
#[doc = "M4S0_SRAM_STATUS register accessor: an alias for `Reg<M4S0_SRAM_STATUS_SPEC>`"]
pub type M4S0_SRAM_STATUS = crate::Reg<m4s0_sram_status::M4S0_SRAM_STATUS_SPEC>;
#[doc = "M4S0 SRAM Power Domain status"]
pub mod m4s0_sram_status;
#[doc = "M4S0_PWR_MODE_CFG register accessor: an alias for `Reg<M4S0_PWR_MODE_CFG_SPEC>`"]
pub type M4S0_PWR_MODE_CFG =
    crate::Reg<m4s0_pwr_mode_cfg::M4S0_PWR_MODE_CFG_SPEC>;
#[doc = "Power mode configuration for the M4S0 SRAM power domain"]
pub mod m4s0_pwr_mode_cfg;
#[doc = "M4S0_PD_SRC_MASK_N register accessor: an alias for `Reg<M4S0_PD_SRC_MASK_N_SPEC>`"]
pub type M4S0_PD_SRC_MASK_N =
    crate::Reg<m4s0_pd_src_mask_n::M4S0_PD_SRC_MASK_N_SPEC>;
#[doc = "Control masking of power-down event triggers for the M4S0 SRAM domain"]
pub mod m4s0_pd_src_mask_n;
#[doc = "M4S0_WU_SRC_MASK_N register accessor: an alias for `Reg<M4S0_WU_SRC_MASK_N_SPEC>`"]
pub type M4S0_WU_SRC_MASK_N =
    crate::Reg<m4s0_wu_src_mask_n::M4S0_WU_SRC_MASK_N_SPEC>;
#[doc = "Control masking of wake-up event triggers for the M4S0 SRAM domain"]
pub mod m4s0_wu_src_mask_n;
#[doc = "A1_STATUS register accessor: an alias for `Reg<A1_STATUS_SPEC>`"]
pub type A1_STATUS = crate::Reg<a1_status::A1_STATUS_SPEC>;
#[doc = "Status of the A1 power domain"]
pub mod a1_status;
#[doc = "A1_PWR_MODE_CFG register accessor: an alias for `Reg<A1_PWR_MODE_CFG_SPEC>`"]
pub type A1_PWR_MODE_CFG = crate::Reg<a1_pwr_mode_cfg::A1_PWR_MODE_CFG_SPEC>;
#[doc = "Power mode configuration for the A1 power domain"]
pub mod a1_pwr_mode_cfg;
#[doc = "A1_PD_SRC_MASK_N register accessor: an alias for `Reg<A1_PD_SRC_MASK_N_SPEC>`"]
pub type A1_PD_SRC_MASK_N = crate::Reg<a1_pd_src_mask_n::A1_PD_SRC_MASK_N_SPEC>;
#[doc = "Reserved"]
pub mod a1_pd_src_mask_n;
#[doc = "A1_WU_SRC_MASK_N register accessor: an alias for `Reg<A1_WU_SRC_MASK_N_SPEC>`"]
pub type A1_WU_SRC_MASK_N = crate::Reg<a1_wu_src_mask_n::A1_WU_SRC_MASK_N_SPEC>;
#[doc = "Control masking of wake-up event triggers for the A1 domain"]
pub mod a1_wu_src_mask_n;
#[doc = "MISC_STATUS register accessor: an alias for `Reg<MISC_STATUS_SPEC>`"]
pub type MISC_STATUS = crate::Reg<misc_status::MISC_STATUS_SPEC>;
#[doc = "I2S Power info"]
pub mod misc_status;
#[doc = "AUDIO_STATUS register accessor: an alias for `Reg<AUDIO_STATUS_SPEC>`"]
pub type AUDIO_STATUS = crate::Reg<audio_status::AUDIO_STATUS_SPEC>;
#[doc = "Audio power domain status"]
pub mod audio_status;
#[doc = "M4_SRAM_STATUS register accessor: an alias for `Reg<M4_SRAM_STATUS_SPEC>`"]
pub type M4_SRAM_STATUS = crate::Reg<m4_sram_status::M4_SRAM_STATUS_SPEC>;
#[doc = "M4 SRAM Power domain status"]
pub mod m4_sram_status;
#[doc = "AUDIO_WU_SRC_MASK_N register accessor: an alias for `Reg<AUDIO_WU_SRC_MASK_N_SPEC>`"]
pub type AUDIO_WU_SRC_MASK_N =
    crate::Reg<audio_wu_src_mask_n::AUDIO_WU_SRC_MASK_N_SPEC>;
#[doc = "Control masking of wake-up event triggers for the Audio domains"]
pub mod audio_wu_src_mask_n;
#[doc = "M4_MEM_CTRL_0 register accessor: an alias for `Reg<M4_MEM_CTRL_0_SPEC>`"]
pub type M4_MEM_CTRL_0 = crate::Reg<m4_mem_ctrl_0::M4_MEM_CTRL_0_SPEC>;
#[doc = "Control DS pins for different SRAM instances on the M4 subsystem. For each instance: 1'b1 : Enable the Deep Sleep funciton of SRAM Macro, Memory content will be kept. While M4 access the memory in Deep Sleep mode, the HW will clear the corresponding bit."]
pub mod m4_mem_ctrl_0;
#[doc = "M4_MEM_CTRL_1 register accessor: an alias for `Reg<M4_MEM_CTRL_1_SPEC>`"]
pub type M4_MEM_CTRL_1 = crate::Reg<m4_mem_ctrl_1::M4_MEM_CTRL_1_SPEC>;
#[doc = "Control Shutdown pin for various instances of SRAM on the M4 subsystem. For each instance: 1'b1 : Enable the Shutdown funciton of SRAM Macro, Memory content will be lost. While M4 access the memory in Shutdown mode, the HW will clear the corresponding bit."]
pub mod m4_mem_ctrl_1;
#[doc = "PF_MEM_CTRL_0 register accessor: an alias for `Reg<PF_MEM_CTRL_0_SPEC>`"]
pub type PF_MEM_CTRL_0 = crate::Reg<pf_mem_ctrl_0::PF_MEM_CTRL_0_SPEC>;
#[doc = "RESERVED"]
pub mod pf_mem_ctrl_0;
#[doc = "PF_MEM_CTRL_1 register accessor: an alias for `Reg<PF_MEM_CTRL_1_SPEC>`"]
pub type PF_MEM_CTRL_1 = crate::Reg<pf_mem_ctrl_1::PF_MEM_CTRL_1_SPEC>;
#[doc = "Control Shut Down pin of various FIFOs intances in the PF subsystem. For each one: 1'b1 : Enable the Shut Down function of SRAM Macro, Memory content will be lost"]
pub mod pf_mem_ctrl_1;
#[doc = "FFE_MEM_CTRL_0 register accessor: an alias for `Reg<FFE_MEM_CTRL_0_SPEC>`"]
pub type FFE_MEM_CTRL_0 = crate::Reg<ffe_mem_ctrl_0::FFE_MEM_CTRL_0_SPEC>;
#[doc = "Control the Deep Sleep pin of various elements in the Flexible Fusion Engine power domain. For each: 1'b1 : Enable the Deep Sleep function of SRAM Macro, Memory content will be kept."]
pub mod ffe_mem_ctrl_0;
#[doc = "FFE_MEM_CTRL_1 register accessor: an alias for `Reg<FFE_MEM_CTRL_1_SPEC>`"]
pub type FFE_MEM_CTRL_1 = crate::Reg<ffe_mem_ctrl_1::FFE_MEM_CTRL_1_SPEC>;
#[doc = "Control the Shut Down pin of various elements in the Flexible Fusion Engine power domain. For each: 1'b1 : Enable the Deep Sleep function of SRAM Macro, Memory content will be kept."]
pub mod ffe_mem_ctrl_1;
#[doc = "AUDIO_MEM_CTRL_0 register accessor: an alias for `Reg<AUDIO_MEM_CTRL_0_SPEC>`"]
pub type AUDIO_MEM_CTRL_0 = crate::Reg<audio_mem_ctrl_0::AUDIO_MEM_CTRL_0_SPEC>;
#[doc = "Control the Deep Sleep pin of Audio channels. For each: 1'b1 : Enable the Deep Sleep function of SRAM Macro, Memory content will be kept."]
pub mod audio_mem_ctrl_0;
#[doc = "AUDIO_MEM_CTRL_1 register accessor: an alias for `Reg<AUDIO_MEM_CTRL_1_SPEC>`"]
pub type AUDIO_MEM_CTRL_1 = crate::Reg<audio_mem_ctrl_1::AUDIO_MEM_CTRL_1_SPEC>;
#[doc = "Control the shut down pin of Audio channels. For each: 1'b1 : Enable the Deep Sleep function of SRAM Macro, Memory content will be kept."]
pub mod audio_mem_ctrl_1;
#[doc = "M4_MEM_CFG register accessor: an alias for `Reg<M4_MEM_CFG_SPEC>`"]
pub type M4_MEM_CFG = crate::Reg<m4_mem_cfg::M4_MEM_CFG_SPEC>;
#[doc = "Reserved"]
pub mod m4_mem_cfg;
#[doc = "PF_MEM_CFG register accessor: an alias for `Reg<PF_MEM_CFG_SPEC>`"]
pub type PF_MEM_CFG = crate::Reg<pf_mem_cfg::PF_MEM_CFG_SPEC>;
#[doc = "Reserved"]
pub mod pf_mem_cfg;
#[doc = "FFE_MEM_CFG register accessor: an alias for `Reg<FFE_MEM_CFG_SPEC>`"]
pub type FFE_MEM_CFG = crate::Reg<ffe_mem_cfg::FFE_MEM_CFG_SPEC>;
#[doc = "Control Light Sleep pin of different FFE SRAM power domains"]
pub mod ffe_mem_cfg;
#[doc = "AUDIO_MEM_CFG register accessor: an alias for `Reg<AUDIO_MEM_CFG_SPEC>`"]
pub type AUDIO_MEM_CFG = crate::Reg<audio_mem_cfg::AUDIO_MEM_CFG_SPEC>;
#[doc = "Reserved"]
pub mod audio_mem_cfg;
#[doc = "M4_MEM_CTRL_PWR_0 register accessor: an alias for `Reg<M4_MEM_CTRL_PWR_0_SPEC>`"]
pub type M4_MEM_CTRL_PWR_0 =
    crate::Reg<m4_mem_ctrl_pwr_0::M4_MEM_CTRL_PWR_0_SPEC>;
#[doc = "Reserved"]
pub mod m4_mem_ctrl_pwr_0;
#[doc = "M4_MEM_CTRL_PWR_1 register accessor: an alias for `Reg<M4_MEM_CTRL_PWR_1_SPEC>`"]
pub type M4_MEM_CTRL_PWR_1 =
    crate::Reg<m4_mem_ctrl_pwr_1::M4_MEM_CTRL_PWR_1_SPEC>;
#[doc = "Reserved"]
pub mod m4_mem_ctrl_pwr_1;
#[doc = "M4_MEM_CTRL_PWR_2 register accessor: an alias for `Reg<M4_MEM_CTRL_PWR_2_SPEC>`"]
pub type M4_MEM_CTRL_PWR_2 =
    crate::Reg<m4_mem_ctrl_pwr_2::M4_MEM_CTRL_PWR_2_SPEC>;
#[doc = "Reserved"]
pub mod m4_mem_ctrl_pwr_2;
#[doc = "SDMA_MEM_CTRL_0 register accessor: an alias for `Reg<SDMA_MEM_CTRL_0_SPEC>`"]
pub type SDMA_MEM_CTRL_0 = crate::Reg<sdma_mem_ctrl_0::SDMA_MEM_CTRL_0_SPEC>;
#[doc = "Control the Deep Sleep function of SRAM Macro for the SDMA power domain"]
pub mod sdma_mem_ctrl_0;
#[doc = "SDMA_MEM_CTRL_1 register accessor: an alias for `Reg<SDMA_MEM_CTRL_1_SPEC>`"]
pub type SDMA_MEM_CTRL_1 = crate::Reg<sdma_mem_ctrl_1::SDMA_MEM_CTRL_1_SPEC>;
#[doc = "Control the Shut Down function of SRAM Macro for the SDMA power domain"]
pub mod sdma_mem_ctrl_1;
#[doc = "MEM_PWR_DWN_CTRL register accessor: an alias for `Reg<MEM_PWR_DWN_CTRL_SPEC>`"]
pub type MEM_PWR_DWN_CTRL = crate::Reg<mem_pwr_dwn_ctrl::MEM_PWR_DWN_CTRL_SPEC>;
#[doc = "Memory Power Down Control"]
pub mod mem_pwr_dwn_ctrl;
#[doc = "PMU_TIMER_CFG_0 register accessor: an alias for `Reg<PMU_TIMER_CFG_0_SPEC>`"]
pub type PMU_TIMER_CFG_0 = crate::Reg<pmu_timer_cfg_0::PMU_TIMER_CFG_0_SPEC>;
#[doc = "Configuration for the PMU timer time-out period"]
pub mod pmu_timer_cfg_0;
#[doc = "PMU_TIMER_CFG_1 register accessor: an alias for `Reg<PMU_TIMER_CFG_1_SPEC>`"]
pub type PMU_TIMER_CFG_1 = crate::Reg<pmu_timer_cfg_1::PMU_TIMER_CFG_1_SPEC>;
#[doc = "Control wether the PMU timer is enabled or disabled"]
pub mod pmu_timer_cfg_1;
#[doc = "PDWU_Timer_CFG register accessor: an alias for `Reg<PDWU_TIMER_CFG_SPEC>`"]
pub type PDWU_TIMER_CFG = crate::Reg<pdwu_timer_cfg::PDWU_TIMER_CFG_SPEC>;
#[doc = "Control the delay for power-on after wake-up event. Applies to all power domains"]
pub mod pdwu_timer_cfg;
#[doc = "FFE_FB_PF_SW_PD register accessor: an alias for `Reg<FFE_FB_PF_SW_PD_SPEC>`"]
pub type FFE_FB_PF_SW_PD = crate::Reg<ffe_fb_pf_sw_pd::FFE_FB_PF_SW_PD_SPEC>;
#[doc = "Registers for triggering power-down events in the FFE, FB and PF power domains."]
pub mod ffe_fb_pf_sw_pd;
#[doc = "M4_SRAM_SW_PD register accessor: an alias for `Reg<M4_SRAM_SW_PD_SPEC>`"]
pub type M4_SRAM_SW_PD = crate::Reg<m4_sram_sw_pd::M4_SRAM_SW_PD_SPEC>;
#[doc = "Register for triggering power-down events in M4 SRAM power domains. (RWHC)"]
pub mod m4_sram_sw_pd;
#[doc = "MISC_SW_PD register accessor: an alias for `Reg<MISC_SW_PD_SPEC>`"]
pub type MISC_SW_PD = crate::Reg<misc_sw_pd::MISC_SW_PD_SPEC>;
#[doc = "Register for triggering power down events in MISC power domains + some general purpose SFR's (RWHC)"]
pub mod misc_sw_pd;
#[doc = "AUDIO_SW_PD register accessor: an alias for `Reg<AUDIO_SW_PD_SPEC>`"]
pub type AUDIO_SW_PD = crate::Reg<audio_sw_pd::AUDIO_SW_PD_SPEC>;
#[doc = "Register for triggering power-down events in Audio power domains. (RWHC)"]
pub mod audio_sw_pd;
#[doc = "FFE_FB_PF_SW_WU register accessor: an alias for `Reg<FFE_FB_PF_SW_WU_SPEC>`"]
pub type FFE_FB_PF_SW_WU = crate::Reg<ffe_fb_pf_sw_wu::FFE_FB_PF_SW_WU_SPEC>;
#[doc = "Registers for triggering wake-up events in the FFE, FB and PF power domains."]
pub mod ffe_fb_pf_sw_wu;
#[doc = "M4_SRAM_SW_WU register accessor: an alias for `Reg<M4_SRAM_SW_WU_SPEC>`"]
pub type M4_SRAM_SW_WU = crate::Reg<m4_sram_sw_wu::M4_SRAM_SW_WU_SPEC>;
#[doc = "Register for triggering wake-up events in M4 SRAM power domains. (RWHC)"]
pub mod m4_sram_sw_wu;
#[doc = "MISC_SW_WU register accessor: an alias for `Reg<MISC_SW_WU_SPEC>`"]
pub type MISC_SW_WU = crate::Reg<misc_sw_wu::MISC_SW_WU_SPEC>;
#[doc = "Register for triggering wake up events in MISC power domains + some general purpose SFR's (RWHC)"]
pub mod misc_sw_wu;
#[doc = "AUDIO_SRAM_SW_WU register accessor: an alias for `Reg<AUDIO_SRAM_SW_WU_SPEC>`"]
pub type AUDIO_SRAM_SW_WU = crate::Reg<audio_sram_sw_wu::AUDIO_SRAM_SW_WU_SPEC>;
#[doc = "Register for triggering wake-up events in Audio power domains. (RWHC)"]
pub mod audio_sram_sw_wu;
#[doc = "PMU_STM_PRIORITY register accessor: an alias for `Reg<PMU_STM_PRIORITY_SPEC>`"]
pub type PMU_STM_PRIORITY = crate::Reg<pmu_stm_priority::PMU_STM_PRIORITY_SPEC>;
#[doc = "Power Management Unit Software Test Mode priority control"]
pub mod pmu_stm_priority;
#[doc = "M4SRAM_SSW_LPMF register accessor: an alias for `Reg<M4SRAM_SSW_LPMF_SPEC>`"]
pub type M4SRAM_SSW_LPMF = crate::Reg<m4sram_ssw_lpmf::M4SRAM_SSW_LPMF_SPEC>;
#[doc = "Control for M4SRAM power domain light sleep mode"]
pub mod m4sram_ssw_lpmf;
#[doc = "M4SRAM_SSW_LPMH_MASK_N register accessor: an alias for `Reg<M4SRAM_SSW_LPMH_MASK_N_SPEC>`"]
pub type M4SRAM_SSW_LPMH_MASK_N =
    crate::Reg<m4sram_ssw_lpmh_mask_n::M4SRAM_SSW_LPMH_MASK_N_SPEC>;
#[doc = "Control masking for the LPMH (Low Power Mode header - deep sleep circuit)"]
pub mod m4sram_ssw_lpmh_mask_n;
#[doc = "FBVLPMinWidth register accessor: an alias for `Reg<FBVLPMINWIDTH_SPEC>`"]
pub type FBVLPMINWIDTH = crate::Reg<fbvlpmin_width::FBVLPMINWIDTH_SPEC>;
#[doc = "Configuration for the amount of IDLE cycles before powering on the FB domain"]
pub mod fbvlpmin_width;
#[doc = "APRebootStatus register accessor: an alias for `Reg<APREBOOTSTATUS_SPEC>`"]
pub type APREBOOTSTATUS = crate::Reg<apreboot_status::APREBOOTSTATUS_SPEC>;
#[doc = "Indicates if AP nees to reload the code to SRAM"]
pub mod apreboot_status;
#[doc = "GEN_PURPOSE_0 register accessor: an alias for `Reg<GEN_PURPOSE_0_SPEC>`"]
pub type GEN_PURPOSE_0 = crate::Reg<gen_purpose_0::GEN_PURPOSE_0_SPEC>;
#[doc = "Configure FB config enable and wether Audio SRAM can be put into Deep Sleep by the Audio hardware"]
pub mod gen_purpose_0;
#[doc = "FB_ISOLATION register accessor: an alias for `Reg<FB_ISOLATION_SPEC>`"]
pub type FB_ISOLATION = crate::Reg<fb_isolation::FB_ISOLATION_SPEC>;
#[doc = "Control the FB Isolation"]
pub mod fb_isolation;
#[doc = "GEN_PURPOSE_1 register accessor: an alias for `Reg<GEN_PURPOSE_1_SPEC>`"]
pub type GEN_PURPOSE_1 = crate::Reg<gen_purpose_1::GEN_PURPOSE_1_SPEC>;
#[doc = "Control for: Wether ext-interrupt can be used to wake up FFE, and clock switching for FFE/M4 power domains"]
pub mod gen_purpose_1;
