#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - For Clock 10 (SYNC Up on A0 and AHB Interface of Batching Memory, AUDIO DMA, M4 SRAMs,M4 Bus Matrix and Trace block)"]
    pub clk_ctrl_a_0: crate::Reg<clk_ctrl_a_0::CLK_CTRL_A_0_SPEC>,
    #[doc = "0x04 - For Clock 10 (SYNC Up on A0 and AHB Interface of Batching Memory, AUDIO DMA, M4 SRAMs,M4 Bus Matrix and Trace block)"]
    pub clk_ctrl_a_1: crate::Reg<clk_ctrl_a_1::CLK_CTRL_A_1_SPEC>,
    #[doc = "0x08 - For Clock 2 (FB, A1 (Including CFGSM))"]
    pub clk_ctrl_b_0: crate::Reg<clk_ctrl_b_0::CLK_CTRL_B_0_SPEC>,
    #[doc = "0x0c - For Clock 2 (FB, A1 (Including CFGSM))"]
    pub clk_ctrl_b_1: crate::Reg<clk_ctrl_b_1::CLK_CTRL_B_1_SPEC>,
    #[doc = "0x10 - For Clock 8 X4 (FFE X4 clk)"]
    pub clk_ctrl_c_0: crate::Reg<clk_ctrl_c_0::CLK_CTRL_C_0_SPEC>,
    #[doc = "0x14 - For Clock 11 (To M4 peripherals - AHB/APB bridge, UART, WDT and TIMER)"]
    pub clk_ctrl_d_0: crate::Reg<clk_ctrl_d_0::CLK_CTRL_D_0_SPEC>,
    #[doc = "0x18 - For Clock 12 - Reserved"]
    pub clk_ctrl_e_0: crate::Reg<clk_ctrl_e_0::CLK_CTRL_E_0_SPEC>,
    #[doc = "0x1c - For Clock 12"]
    pub clk_ctrl_e_1: crate::Reg<clk_ctrl_e_1::CLK_CTRL_E_1_SPEC>,
    #[doc = "0x20 - For Clock 16 (FB)"]
    pub clk_ctrl_f_0: crate::Reg<clk_ctrl_f_0::CLK_CTRL_F_0_SPEC>,
    #[doc = "0x24 - For Clock 16 (FB)"]
    pub clk_ctrl_f_1: crate::Reg<clk_ctrl_f_1::CLK_CTRL_F_1_SPEC>,
    #[doc = "0x28 - For Clock 30 (PDM LEFT/RIGHT clk, I2S Master clk)"]
    pub clk_ctrl_g_0: crate::Reg<clk_ctrl_g_0::CLK_CTRL_G_0_SPEC>,
    #[doc = "0x2c - For Clock 19 (ADC)"]
    pub clk_ctrl_h_0: crate::Reg<clk_ctrl_h_0::CLK_CTRL_H_0_SPEC>,
    #[doc = "0x30 - For Clock 19 (ADC)"]
    pub clk_ctrl_h_1: crate::Reg<clk_ctrl_h_1::CLK_CTRL_H_1_SPEC>,
    #[doc = "0x34 - For Clock 21 (FB - additional clk)"]
    pub clk_ctrl_i_0: crate::Reg<clk_ctrl_i_0::CLK_CTRL_I_0_SPEC>,
    #[doc = "0x38 - For Clock 21 (FB - additional clk)"]
    pub clk_ctrl_i_1: crate::Reg<clk_ctrl_i_1::CLK_CTRL_I_1_SPEC>,
    _reserved15: [u8; 0x04],
    #[doc = "0x40 - Gating control for Clock 1"]
    pub c01_clk_gate: crate::Reg<c01_clk_gate::C01_CLK_GATE_SPEC>,
    #[doc = "0x44 - Gating control for Clock 2"]
    pub c02_clk_gate: crate::Reg<c02_clk_gate::C02_CLK_GATE_SPEC>,
    #[doc = "0x48 - Gating control for FFE X4 clock"]
    pub c08_x4_clk_gate: crate::Reg<c08_x4_clk_gate::C08_X4_CLK_GATE_SPEC>,
    #[doc = "0x4c - Gating control for FFE X1 clock"]
    pub c08_x1_clk_gate: crate::Reg<c08_x1_clk_gate::C08_X1_CLK_GATE_SPEC>,
    #[doc = "0x50 - Gating control for Clock 10"]
    pub c10_fclk_gate: crate::Reg<c10_fclk_gate::C10_FCLK_GATE_SPEC>,
    #[doc = "0x54 - Gating control for clock 11"]
    pub c11_clk_gate: crate::Reg<c11_clk_gate::C11_CLK_GATE_SPEC>,
    #[doc = "0x58 - Gating control for clock 12"]
    pub c12_clk_gate_reserved: crate::Reg<c12_clk_gate_reserved::C12_CLK_GATE_RESERVED_SPEC>,
    #[doc = "0x5c - Gating control for SWD CS clock U"]
    pub cs_clk_gate: crate::Reg<cs_clk_gate::CS_CLK_GATE_SPEC>,
    #[doc = "0x60 - Not specified"]
    pub cu_clk_gate_reserved: crate::Reg<cu_clk_gate_reserved::CU_CLK_GATE_RESERVED_SPEC>,
    #[doc = "0x64 - Gating control for FB clock 16"]
    pub c16_clk_gate: crate::Reg<c16_clk_gate::C16_CLK_GATE_SPEC>,
    #[doc = "0x68 - Not specified"]
    pub clk_reserved_0: crate::Reg<clk_reserved_0::CLK_RESERVED_0_SPEC>,
    #[doc = "0x6c - Gating control for ADC clock 19"]
    pub c19_clk_gate: crate::Reg<c19_clk_gate::C19_CLK_GATE_SPEC>,
    #[doc = "0x70 - Gating control for FB additional clock 21"]
    pub c21_clk_gate: crate::Reg<c21_clk_gate::C21_CLK_GATE_SPEC>,
    _reserved28: [u8; 0x0c],
    #[doc = "0x80 - Packet FIFO power domain Software Reset.Once Program the SW Reset Bit to 1, the corresponding reset will be asserted immediately. Once Program the SW Reset Bit to 0, the corresponding reset will be de-asserted synchronous even the corresponding clock is not running. (Turn off by Clock gating cell)"]
    pub pf_sw_reset: crate::Reg<pf_sw_reset::PF_SW_RESET_SPEC>,
    #[doc = "0x84 - Once Program the SW Reset Bit to 1, the corresponding reset will be asserted immediately. Once Program the SW Reset Bit to 0, the corresponding reset will be de-asserted synchronous even the corresponding clock is not running. (Turn off by Clock gating cell)"]
    pub ffe_sw_reset: crate::Reg<ffe_sw_reset::FFE_SW_RESET_SPEC>,
    #[doc = "0x88 - Once Program the SW Reset Bit to 1, the corresponding reset will be asserted immediately. Once Program the SW Reset Bit to 0, the corresponding reset will be de-asserted synchronous even the corresponding clock is not running. (Turn off by Clock gating cell)"]
    pub fb_sw_reset: crate::Reg<fb_sw_reset::FB_SW_RESET_SPEC>,
    #[doc = "0x8c - Once Program the SW Reset Bit to 1, the corresponding reset will be asserted immediately. Once Program the SW Reset Bit to 0, the corresponding reset will be de-asserted synchronous even the corresponding clock is not running. (Turn off by Clock gating cell)"]
    pub a1_sw_reset: crate::Reg<a1_sw_reset::A1_SW_RESET_SPEC>,
    #[doc = "0x90 - Once Program the SW Reset Bit to 1, the corresponding reset will be asserted immediately. Once Program the SW Reset Bit to 0, the corresponding reset will be de-asserted synchronous even the corresponding clock is not running. (Turn off by Clock gating cell)"]
    pub audio_misc_sw_reset: crate::Reg<audio_misc_sw_reset::AUDIO_MISC_SW_RESET_SPEC>,
    #[doc = "0x94 - Not specified. FAFIFO1, AHBWB Software Reset control"]
    pub fb_misc_sw_rst_ctl: crate::Reg<fb_misc_sw_rst_ctl::FB_MISC_SW_RST_CTL_SPEC>,
    _reserved34: [u8; 0x68],
    #[doc = "0x100 - This Clock is used to delay the Clock gating control signals from PMU. The PMU will monitor the feedback/delayed Clock Gating Control signals to ensure the clocks are OFF before jump to next state. The Firmware needs to Configure this divider to ensure there delay time is longer enough. C23 Clock needs to be 2/3 of the lowest clock frequency of other clocks. For Example, if the Lowest clock frequency of other clocks are 5, then C23 should be lower than 3.33MHz (Or the clock frequency of other clocks should be at least 1.5 times faster than C23.)"]
    pub clk_ctrl_pmu: crate::Reg<clk_ctrl_pmu::CLK_CTRL_PMU_SPEC>,
    #[doc = "0x104 - General reg and SPI ALWAYS ON control"]
    pub cru_general: crate::Reg<cru_general::CRU_GENERAL_SPEC>,
    #[doc = "0x108 - CRU Debug registers"]
    pub cru_debug: crate::Reg<cru_debug::CRU_DEBUG_SPEC>,
    _reserved37: [u8; 0x04],
    #[doc = "0x110 - Source Clock is C10 (CLK to SDMA,I2S module Inside A1, AHB2APB Bridge /CFG DMA Bridge inside A1 , FFE, Packet FIFO,SDMA,A0 ) If Bit 4 is 0, any change on Bit 3:0 will not take effect. And bit 4 and bit 3:0 can not change at same time."]
    pub c01_clk_div: crate::Reg<c01_clk_div::C01_CLK_DIV_SPEC>,
    #[doc = "0x114 - Source Clock is C10 (CLK to Voice APB interface, PIF, FB). If Bit 4 is 0, any change on Bit 3:0 will not take effect. And bit 4 and bit 3:0 can not change at same time."]
    pub c09_clk_div: crate::Reg<c09_clk_div::C09_CLK_DIV_SPEC>,
    #[doc = "0x118 - Source Clock is C30 (LPSD CLK).If Bit 4 is 0, any change on Bit 3:0 will not take effect. And bit 4 and bit 3:0 can not change at same time."]
    pub c31_clk_div: crate::Reg<c31_clk_div::C31_CLK_DIV_SPEC>,
    #[doc = "0x11c - Gating control for clock 09"]
    pub c09_clk_gate: crate::Reg<c09_clk_gate::C09_CLK_GATE_SPEC>,
    #[doc = "0x120 - Gating control for clocks 30-31"]
    pub c30_c31_clk_gate: crate::Reg<c30_c31_clk_gate::C30_C31_CLK_GATE_SPEC>,
    #[doc = "0x124 - Control for divider gates in different clocks"]
    pub clk_divider_clk_gating: crate::Reg<clk_divider_clk_gating::CLK_DIVIDER_CLK_GATING_SPEC>,
    _reserved43: [u8; 0x08],
    #[doc = "0x130 - For Clock 2 (FB, A1 (Including CFGSM))"]
    pub clk_switch_for_b: crate::Reg<clk_switch_for_b::CLK_SWITCH_FOR_B_SPEC>,
    #[doc = "0x134 - For Clock 8 X4 (FFE X4 clk)"]
    pub clk_switch_for_c: crate::Reg<clk_switch_for_c::CLK_SWITCH_FOR_C_SPEC>,
    #[doc = "0x138 - For Clock 11 (To M4 peripherals - AHB/APB bridge, UART, WDT and TIMER)"]
    pub clk_switch_for_d: crate::Reg<clk_switch_for_d::CLK_SWITCH_FOR_D_SPEC>,
    #[doc = "0x13c - For Clock 19 (ADC)"]
    pub clk_switch_for_h: crate::Reg<clk_switch_for_h::CLK_SWITCH_FOR_H_SPEC>,
    #[doc = "0x140 - For CLK 23 (PMU clk gating control)"]
    pub clk_switch_for_j: crate::Reg<clk_switch_for_j::CLK_SWITCH_FOR_J_SPEC>,
    #[doc = "0x144 - To C30(PDM LEFT/RIGHT Clk, I2S_MASTER)"]
    pub clk_switch_for_g: crate::Reg<clk_switch_for_g::CLK_SWITCH_FOR_G_SPEC>,
}
#[doc = "CLK_CTRL_A_0 register accessor: an alias for `Reg<CLK_CTRL_A_0_SPEC>`"]
pub type CLK_CTRL_A_0 = crate::Reg<clk_ctrl_a_0::CLK_CTRL_A_0_SPEC>;
#[doc = "For Clock 10 (SYNC Up on A0 and AHB Interface of Batching Memory, AUDIO DMA, M4 SRAMs,M4 Bus Matrix and Trace block)"]
pub mod clk_ctrl_a_0;
#[doc = "CLK_CTRL_A_1 register accessor: an alias for `Reg<CLK_CTRL_A_1_SPEC>`"]
pub type CLK_CTRL_A_1 = crate::Reg<clk_ctrl_a_1::CLK_CTRL_A_1_SPEC>;
#[doc = "For Clock 10 (SYNC Up on A0 and AHB Interface of Batching Memory, AUDIO DMA, M4 SRAMs,M4 Bus Matrix and Trace block)"]
pub mod clk_ctrl_a_1;
#[doc = "CLK_CTRL_B_0 register accessor: an alias for `Reg<CLK_CTRL_B_0_SPEC>`"]
pub type CLK_CTRL_B_0 = crate::Reg<clk_ctrl_b_0::CLK_CTRL_B_0_SPEC>;
#[doc = "For Clock 2 (FB, A1 (Including CFGSM))"]
pub mod clk_ctrl_b_0;
#[doc = "CLK_CTRL_B_1 register accessor: an alias for `Reg<CLK_CTRL_B_1_SPEC>`"]
pub type CLK_CTRL_B_1 = crate::Reg<clk_ctrl_b_1::CLK_CTRL_B_1_SPEC>;
#[doc = "For Clock 2 (FB, A1 (Including CFGSM))"]
pub mod clk_ctrl_b_1;
#[doc = "CLK_CTRL_C_0 register accessor: an alias for `Reg<CLK_CTRL_C_0_SPEC>`"]
pub type CLK_CTRL_C_0 = crate::Reg<clk_ctrl_c_0::CLK_CTRL_C_0_SPEC>;
#[doc = "For Clock 8 X4 (FFE X4 clk)"]
pub mod clk_ctrl_c_0;
#[doc = "CLK_CTRL_D_0 register accessor: an alias for `Reg<CLK_CTRL_D_0_SPEC>`"]
pub type CLK_CTRL_D_0 = crate::Reg<clk_ctrl_d_0::CLK_CTRL_D_0_SPEC>;
#[doc = "For Clock 11 (To M4 peripherals - AHB/APB bridge, UART, WDT and TIMER)"]
pub mod clk_ctrl_d_0;
#[doc = "CLK_CTRL_E_0 register accessor: an alias for `Reg<CLK_CTRL_E_0_SPEC>`"]
pub type CLK_CTRL_E_0 = crate::Reg<clk_ctrl_e_0::CLK_CTRL_E_0_SPEC>;
#[doc = "For Clock 12 - Reserved"]
pub mod clk_ctrl_e_0;
#[doc = "CLK_CTRL_E_1 register accessor: an alias for `Reg<CLK_CTRL_E_1_SPEC>`"]
pub type CLK_CTRL_E_1 = crate::Reg<clk_ctrl_e_1::CLK_CTRL_E_1_SPEC>;
#[doc = "For Clock 12"]
pub mod clk_ctrl_e_1;
#[doc = "CLK_CTRL_F_0 register accessor: an alias for `Reg<CLK_CTRL_F_0_SPEC>`"]
pub type CLK_CTRL_F_0 = crate::Reg<clk_ctrl_f_0::CLK_CTRL_F_0_SPEC>;
#[doc = "For Clock 16 (FB)"]
pub mod clk_ctrl_f_0;
#[doc = "CLK_CTRL_F_1 register accessor: an alias for `Reg<CLK_CTRL_F_1_SPEC>`"]
pub type CLK_CTRL_F_1 = crate::Reg<clk_ctrl_f_1::CLK_CTRL_F_1_SPEC>;
#[doc = "For Clock 16 (FB)"]
pub mod clk_ctrl_f_1;
#[doc = "CLK_CTRL_G_0 register accessor: an alias for `Reg<CLK_CTRL_G_0_SPEC>`"]
pub type CLK_CTRL_G_0 = crate::Reg<clk_ctrl_g_0::CLK_CTRL_G_0_SPEC>;
#[doc = "For Clock 30 (PDM LEFT/RIGHT clk, I2S Master clk)"]
pub mod clk_ctrl_g_0;
#[doc = "CLK_CTRL_H_0 register accessor: an alias for `Reg<CLK_CTRL_H_0_SPEC>`"]
pub type CLK_CTRL_H_0 = crate::Reg<clk_ctrl_h_0::CLK_CTRL_H_0_SPEC>;
#[doc = "For Clock 19 (ADC)"]
pub mod clk_ctrl_h_0;
#[doc = "CLK_CTRL_H_1 register accessor: an alias for `Reg<CLK_CTRL_H_1_SPEC>`"]
pub type CLK_CTRL_H_1 = crate::Reg<clk_ctrl_h_1::CLK_CTRL_H_1_SPEC>;
#[doc = "For Clock 19 (ADC)"]
pub mod clk_ctrl_h_1;
#[doc = "CLK_CTRL_I_0 register accessor: an alias for `Reg<CLK_CTRL_I_0_SPEC>`"]
pub type CLK_CTRL_I_0 = crate::Reg<clk_ctrl_i_0::CLK_CTRL_I_0_SPEC>;
#[doc = "For Clock 21 (FB - additional clk)"]
pub mod clk_ctrl_i_0;
#[doc = "CLK_CTRL_I_1 register accessor: an alias for `Reg<CLK_CTRL_I_1_SPEC>`"]
pub type CLK_CTRL_I_1 = crate::Reg<clk_ctrl_i_1::CLK_CTRL_I_1_SPEC>;
#[doc = "For Clock 21 (FB - additional clk)"]
pub mod clk_ctrl_i_1;
#[doc = "C01_CLK_GATE register accessor: an alias for `Reg<C01_CLK_GATE_SPEC>`"]
pub type C01_CLK_GATE = crate::Reg<c01_clk_gate::C01_CLK_GATE_SPEC>;
#[doc = "Gating control for Clock 1"]
pub mod c01_clk_gate;
#[doc = "C02_CLK_GATE register accessor: an alias for `Reg<C02_CLK_GATE_SPEC>`"]
pub type C02_CLK_GATE = crate::Reg<c02_clk_gate::C02_CLK_GATE_SPEC>;
#[doc = "Gating control for Clock 2"]
pub mod c02_clk_gate;
#[doc = "C08_X4_CLK_GATE register accessor: an alias for `Reg<C08_X4_CLK_GATE_SPEC>`"]
pub type C08_X4_CLK_GATE = crate::Reg<c08_x4_clk_gate::C08_X4_CLK_GATE_SPEC>;
#[doc = "Gating control for FFE X4 clock"]
pub mod c08_x4_clk_gate;
#[doc = "C08_X1_CLK_GATE register accessor: an alias for `Reg<C08_X1_CLK_GATE_SPEC>`"]
pub type C08_X1_CLK_GATE = crate::Reg<c08_x1_clk_gate::C08_X1_CLK_GATE_SPEC>;
#[doc = "Gating control for FFE X1 clock"]
pub mod c08_x1_clk_gate;
#[doc = "C10_FCLK_GATE register accessor: an alias for `Reg<C10_FCLK_GATE_SPEC>`"]
pub type C10_FCLK_GATE = crate::Reg<c10_fclk_gate::C10_FCLK_GATE_SPEC>;
#[doc = "Gating control for Clock 10"]
pub mod c10_fclk_gate;
#[doc = "C11_CLK_GATE register accessor: an alias for `Reg<C11_CLK_GATE_SPEC>`"]
pub type C11_CLK_GATE = crate::Reg<c11_clk_gate::C11_CLK_GATE_SPEC>;
#[doc = "Gating control for clock 11"]
pub mod c11_clk_gate;
#[doc = "C12_CLK_GATE_Reserved register accessor: an alias for `Reg<C12_CLK_GATE_RESERVED_SPEC>`"]
pub type C12_CLK_GATE_RESERVED = crate::Reg<c12_clk_gate_reserved::C12_CLK_GATE_RESERVED_SPEC>;
#[doc = "Gating control for clock 12"]
pub mod c12_clk_gate_reserved;
#[doc = "CS_CLK_GATE register accessor: an alias for `Reg<CS_CLK_GATE_SPEC>`"]
pub type CS_CLK_GATE = crate::Reg<cs_clk_gate::CS_CLK_GATE_SPEC>;
#[doc = "Gating control for SWD CS clock U"]
pub mod cs_clk_gate;
#[doc = "CU_CLK_GATE_Reserved register accessor: an alias for `Reg<CU_CLK_GATE_RESERVED_SPEC>`"]
pub type CU_CLK_GATE_RESERVED = crate::Reg<cu_clk_gate_reserved::CU_CLK_GATE_RESERVED_SPEC>;
#[doc = "Not specified"]
pub mod cu_clk_gate_reserved;
#[doc = "C16_CLK_GATE register accessor: an alias for `Reg<C16_CLK_GATE_SPEC>`"]
pub type C16_CLK_GATE = crate::Reg<c16_clk_gate::C16_CLK_GATE_SPEC>;
#[doc = "Gating control for FB clock 16"]
pub mod c16_clk_gate;
#[doc = "CLK_RESERVED_0 register accessor: an alias for `Reg<CLK_RESERVED_0_SPEC>`"]
pub type CLK_RESERVED_0 = crate::Reg<clk_reserved_0::CLK_RESERVED_0_SPEC>;
#[doc = "Not specified"]
pub mod clk_reserved_0;
#[doc = "C19_CLK_GATE register accessor: an alias for `Reg<C19_CLK_GATE_SPEC>`"]
pub type C19_CLK_GATE = crate::Reg<c19_clk_gate::C19_CLK_GATE_SPEC>;
#[doc = "Gating control for ADC clock 19"]
pub mod c19_clk_gate;
#[doc = "C21_CLK_GATE register accessor: an alias for `Reg<C21_CLK_GATE_SPEC>`"]
pub type C21_CLK_GATE = crate::Reg<c21_clk_gate::C21_CLK_GATE_SPEC>;
#[doc = "Gating control for FB additional clock 21"]
pub mod c21_clk_gate;
#[doc = "PF_SW_RESET register accessor: an alias for `Reg<PF_SW_RESET_SPEC>`"]
pub type PF_SW_RESET = crate::Reg<pf_sw_reset::PF_SW_RESET_SPEC>;
#[doc = "Packet FIFO power domain Software Reset.Once Program the SW Reset Bit to 1, the corresponding reset will be asserted immediately. Once Program the SW Reset Bit to 0, the corresponding reset will be de-asserted synchronous even the corresponding clock is not running. (Turn off by Clock gating cell)"]
pub mod pf_sw_reset;
#[doc = "FFE_SW_RESET register accessor: an alias for `Reg<FFE_SW_RESET_SPEC>`"]
pub type FFE_SW_RESET = crate::Reg<ffe_sw_reset::FFE_SW_RESET_SPEC>;
#[doc = "Once Program the SW Reset Bit to 1, the corresponding reset will be asserted immediately. Once Program the SW Reset Bit to 0, the corresponding reset will be de-asserted synchronous even the corresponding clock is not running. (Turn off by Clock gating cell)"]
pub mod ffe_sw_reset;
#[doc = "FB_SW_RESET register accessor: an alias for `Reg<FB_SW_RESET_SPEC>`"]
pub type FB_SW_RESET = crate::Reg<fb_sw_reset::FB_SW_RESET_SPEC>;
#[doc = "Once Program the SW Reset Bit to 1, the corresponding reset will be asserted immediately. Once Program the SW Reset Bit to 0, the corresponding reset will be de-asserted synchronous even the corresponding clock is not running. (Turn off by Clock gating cell)"]
pub mod fb_sw_reset;
#[doc = "A1_SW_RESET register accessor: an alias for `Reg<A1_SW_RESET_SPEC>`"]
pub type A1_SW_RESET = crate::Reg<a1_sw_reset::A1_SW_RESET_SPEC>;
#[doc = "Once Program the SW Reset Bit to 1, the corresponding reset will be asserted immediately. Once Program the SW Reset Bit to 0, the corresponding reset will be de-asserted synchronous even the corresponding clock is not running. (Turn off by Clock gating cell)"]
pub mod a1_sw_reset;
#[doc = "AUDIO_MISC_SW_RESET register accessor: an alias for `Reg<AUDIO_MISC_SW_RESET_SPEC>`"]
pub type AUDIO_MISC_SW_RESET = crate::Reg<audio_misc_sw_reset::AUDIO_MISC_SW_RESET_SPEC>;
#[doc = "Once Program the SW Reset Bit to 1, the corresponding reset will be asserted immediately. Once Program the SW Reset Bit to 0, the corresponding reset will be de-asserted synchronous even the corresponding clock is not running. (Turn off by Clock gating cell)"]
pub mod audio_misc_sw_reset;
#[doc = "FB_MISC_SW_RST_CTL register accessor: an alias for `Reg<FB_MISC_SW_RST_CTL_SPEC>`"]
pub type FB_MISC_SW_RST_CTL = crate::Reg<fb_misc_sw_rst_ctl::FB_MISC_SW_RST_CTL_SPEC>;
#[doc = "Not specified. FAFIFO1, AHBWB Software Reset control"]
pub mod fb_misc_sw_rst_ctl;
#[doc = "CLK_CTRL_PMU register accessor: an alias for `Reg<CLK_CTRL_PMU_SPEC>`"]
pub type CLK_CTRL_PMU = crate::Reg<clk_ctrl_pmu::CLK_CTRL_PMU_SPEC>;
#[doc = "This Clock is used to delay the Clock gating control signals from PMU. The PMU will monitor the feedback/delayed Clock Gating Control signals to ensure the clocks are OFF before jump to next state. The Firmware needs to Configure this divider to ensure there delay time is longer enough. C23 Clock needs to be 2/3 of the lowest clock frequency of other clocks. For Example, if the Lowest clock frequency of other clocks are 5, then C23 should be lower than 3.33MHz (Or the clock frequency of other clocks should be at least 1.5 times faster than C23.)"]
pub mod clk_ctrl_pmu;
#[doc = "CRU_GENERAL register accessor: an alias for `Reg<CRU_GENERAL_SPEC>`"]
pub type CRU_GENERAL = crate::Reg<cru_general::CRU_GENERAL_SPEC>;
#[doc = "General reg and SPI ALWAYS ON control"]
pub mod cru_general;
#[doc = "CRU_DEBUG register accessor: an alias for `Reg<CRU_DEBUG_SPEC>`"]
pub type CRU_DEBUG = crate::Reg<cru_debug::CRU_DEBUG_SPEC>;
#[doc = "CRU Debug registers"]
pub mod cru_debug;
#[doc = "C01_CLK_DIV register accessor: an alias for `Reg<C01_CLK_DIV_SPEC>`"]
pub type C01_CLK_DIV = crate::Reg<c01_clk_div::C01_CLK_DIV_SPEC>;
#[doc = "Source Clock is C10 (CLK to SDMA,I2S module Inside A1, AHB2APB Bridge /CFG DMA Bridge inside A1 , FFE, Packet FIFO,SDMA,A0 ) If Bit 4 is 0, any change on Bit 3:0 will not take effect. And bit 4 and bit 3:0 can not change at same time."]
pub mod c01_clk_div;
#[doc = "C09_CLK_DIV register accessor: an alias for `Reg<C09_CLK_DIV_SPEC>`"]
pub type C09_CLK_DIV = crate::Reg<c09_clk_div::C09_CLK_DIV_SPEC>;
#[doc = "Source Clock is C10 (CLK to Voice APB interface, PIF, FB). If Bit 4 is 0, any change on Bit 3:0 will not take effect. And bit 4 and bit 3:0 can not change at same time."]
pub mod c09_clk_div;
#[doc = "C31_CLK_DIV register accessor: an alias for `Reg<C31_CLK_DIV_SPEC>`"]
pub type C31_CLK_DIV = crate::Reg<c31_clk_div::C31_CLK_DIV_SPEC>;
#[doc = "Source Clock is C30 (LPSD CLK).If Bit 4 is 0, any change on Bit 3:0 will not take effect. And bit 4 and bit 3:0 can not change at same time."]
pub mod c31_clk_div;
#[doc = "C09_CLK_GATE register accessor: an alias for `Reg<C09_CLK_GATE_SPEC>`"]
pub type C09_CLK_GATE = crate::Reg<c09_clk_gate::C09_CLK_GATE_SPEC>;
#[doc = "Gating control for clock 09"]
pub mod c09_clk_gate;
#[doc = "C30_C31_CLK_GATE register accessor: an alias for `Reg<C30_C31_CLK_GATE_SPEC>`"]
pub type C30_C31_CLK_GATE = crate::Reg<c30_c31_clk_gate::C30_C31_CLK_GATE_SPEC>;
#[doc = "Gating control for clocks 30-31"]
pub mod c30_c31_clk_gate;
#[doc = "CLK_DIVIDER_CLK_GATING register accessor: an alias for `Reg<CLK_DIVIDER_CLK_GATING_SPEC>`"]
pub type CLK_DIVIDER_CLK_GATING = crate::Reg<clk_divider_clk_gating::CLK_DIVIDER_CLK_GATING_SPEC>;
#[doc = "Control for divider gates in different clocks"]
pub mod clk_divider_clk_gating;
#[doc = "CLK_SWITCH_FOR_B register accessor: an alias for `Reg<CLK_SWITCH_FOR_B_SPEC>`"]
pub type CLK_SWITCH_FOR_B = crate::Reg<clk_switch_for_b::CLK_SWITCH_FOR_B_SPEC>;
#[doc = "For Clock 2 (FB, A1 (Including CFGSM))"]
pub mod clk_switch_for_b;
#[doc = "CLK_SWITCH_FOR_C register accessor: an alias for `Reg<CLK_SWITCH_FOR_C_SPEC>`"]
pub type CLK_SWITCH_FOR_C = crate::Reg<clk_switch_for_c::CLK_SWITCH_FOR_C_SPEC>;
#[doc = "For Clock 8 X4 (FFE X4 clk)"]
pub mod clk_switch_for_c;
#[doc = "CLK_SWITCH_FOR_D register accessor: an alias for `Reg<CLK_SWITCH_FOR_D_SPEC>`"]
pub type CLK_SWITCH_FOR_D = crate::Reg<clk_switch_for_d::CLK_SWITCH_FOR_D_SPEC>;
#[doc = "For Clock 11 (To M4 peripherals - AHB/APB bridge, UART, WDT and TIMER)"]
pub mod clk_switch_for_d;
#[doc = "CLK_SWITCH_FOR_H register accessor: an alias for `Reg<CLK_SWITCH_FOR_H_SPEC>`"]
pub type CLK_SWITCH_FOR_H = crate::Reg<clk_switch_for_h::CLK_SWITCH_FOR_H_SPEC>;
#[doc = "For Clock 19 (ADC)"]
pub mod clk_switch_for_h;
#[doc = "CLK_SWITCH_FOR_J register accessor: an alias for `Reg<CLK_SWITCH_FOR_J_SPEC>`"]
pub type CLK_SWITCH_FOR_J = crate::Reg<clk_switch_for_j::CLK_SWITCH_FOR_J_SPEC>;
#[doc = "For CLK 23 (PMU clk gating control)"]
pub mod clk_switch_for_j;
#[doc = "CLK_SWITCH_FOR_G register accessor: an alias for `Reg<CLK_SWITCH_FOR_G_SPEC>`"]
pub type CLK_SWITCH_FOR_G = crate::Reg<clk_switch_for_g::CLK_SWITCH_FOR_G_SPEC>;
#[doc = "To C30(PDM LEFT/RIGHT Clk, I2S_MASTER)"]
pub mod clk_switch_for_g;
