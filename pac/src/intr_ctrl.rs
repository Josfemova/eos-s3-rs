#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Indicators of interrupt triggers detected"]
    pub gpio_intr: crate::Reg<gpio_intr::GPIO_INTR_SPEC>,
    #[doc = "0x04 - GPIO raw interrupt indicators"]
    pub gpio_intr_raw: crate::Reg<gpio_intr_raw::GPIO_INTR_RAW_SPEC>,
    #[doc = "0x08 - Indicators of interrupt trigger types"]
    pub gpio_intr_type: crate::Reg<gpio_intr_type::GPIO_INTR_TYPE_SPEC>,
    #[doc = "0x0c - Indicators of interrupt trigger polarities (will depend on the type of interrupt)"]
    pub gpio_intr_pol: crate::Reg<gpio_intr_pol::GPIO_INTR_POL_SPEC>,
    #[doc = "0x10 - GPIO interrupt enable for AP"]
    pub gpio_intr_en_ap: crate::Reg<gpio_intr_en_ap::GPIO_INTR_EN_AP_SPEC>,
    #[doc = "0x14 - GPIO interrupt enable for M4"]
    pub gpio_intr_en_m4: crate::Reg<gpio_intr_en_m4::GPIO_INTR_EN_M4_SPEC>,
    #[doc = "0x18 - GPIO interrupt enable for FFE0"]
    pub gpio_intr_en_ffe0:
        crate::Reg<gpio_intr_en_ffe0::GPIO_INTR_EN_FFE0_SPEC>,
    #[doc = "0x1c - GPIO interrupt enable for FFE1"]
    pub gpio_intr_en_ffe1:
        crate::Reg<gpio_intr_en_ffe1::GPIO_INTR_EN_FFE1_SPEC>,
    _reserved8: [u8; 0x10],
    #[doc = "0x30 - Indicators of interrupt triggers detected"]
    pub other_intr: crate::Reg<other_intr::OTHER_INTR_SPEC>,
    #[doc = "0x34 - Various interrupt enable for AP"]
    pub other_intr_en_ap: crate::Reg<other_intr_en_ap::OTHER_INTR_EN_AP_SPEC>,
    #[doc = "0x38 - Various interrupt enable for M4"]
    pub other_intr_en_m4: crate::Reg<other_intr_en_m4::OTHER_INTR_EN_M4_SPEC>,
    _reserved11: [u8; 0x04],
    #[doc = "0x40 - Indicators of General purpose software interrupt 1 triggers detected"]
    pub software_intr_1: crate::Reg<software_intr_1::SOFTWARE_INTR_1_SPEC>,
    #[doc = "0x44 - General purpose Software interrupt 1 enable for AP"]
    pub software_intr_1_en_ap:
        crate::Reg<software_intr_1_en_ap::SOFTWARE_INTR_1_EN_AP_SPEC>,
    _reserved13: [u8; 0x08],
    #[doc = "0x50 - Indicators of General purpose software interrupt 2 triggers detected"]
    pub software_intr_2: crate::Reg<software_intr_2::SOFTWARE_INTR_2_SPEC>,
    #[doc = "0x54 - General purpose Software interrupt 2 enable for AP"]
    pub software_intr_2_en_ap:
        crate::Reg<software_intr_2_en_ap::SOFTWARE_INTR_2_EN_AP_SPEC>,
    #[doc = "0x58 - General purpose Software interrupt 2 enable for M4"]
    pub software_intr_en_m4:
        crate::Reg<software_intr_en_m4::SOFTWARE_INTR_EN_M4_SPEC>,
    _reserved16: [u8; 0x04],
    #[doc = "0x60 - Indicators of FFE0 interrupt triggers detected"]
    pub ffe_intr: crate::Reg<ffe_intr::FFE_INTR_SPEC>,
    #[doc = "0x64 - FFE0 interrupt enable for AP"]
    pub ffe_intr_en_ap: crate::Reg<ffe_intr_en_ap::FFE_INTR_EN_AP_SPEC>,
    #[doc = "0x68 - FFE0 interrupt enable for M4"]
    pub ffe_intr_en_m4: crate::Reg<ffe_intr_en_m4::FFE_INTR_EN_M4_SPEC>,
    _reserved19: [u8; 0x14],
    #[doc = "0x80 - Indicators of interrupt triggers detected"]
    pub fb_intr: crate::Reg<fb_intr::FB_INTR_SPEC>,
    #[doc = "0x84 - FB raw interrupt indicators"]
    pub fb_intr_raw: crate::Reg<fb_intr_raw::FB_INTR_RAW_SPEC>,
    #[doc = "0x88 - Indicators of interrupt trigger types"]
    pub fb_intr_type: crate::Reg<fb_intr_type::FB_INTR_TYPE_SPEC>,
    #[doc = "0x8c - Indicators of interrupt trigger polarities (will depend on the type of interrupt)"]
    pub fb_intr_pol: crate::Reg<fb_intr_pol::FB_INTR_POL_SPEC>,
    #[doc = "0x90 - FB interrupt enable for AP"]
    pub fb_intr_en_ap: crate::Reg<fb_intr_en_ap::FB_INTR_EN_AP_SPEC>,
    #[doc = "0x94 - FB interrupt enable for M4"]
    pub fb_intr_en_m4: crate::Reg<fb_intr_en_m4::FB_INTR_EN_M4_SPEC>,
    _reserved25: [u8; 0x08],
    #[doc = "0xa0 - Indicator of detected SRAM access while shut down or sleep mode interrupts"]
    pub m4_mem_aon_intr: crate::Reg<m4_mem_aon_intr::M4_MEM_AON_INTR_SPEC>,
    #[doc = "0xa4 - Interrupt enable for SRAM access while in shut down or deep sleep modes"]
    pub m4_mem_aon_intr_en:
        crate::Reg<m4_mem_aon_intr_en::M4_MEM_AON_INTR_EN_SPEC>,
}
#[doc = "GPIO_INTR register accessor: an alias for `Reg<GPIO_INTR_SPEC>`"]
pub type GPIO_INTR = crate::Reg<gpio_intr::GPIO_INTR_SPEC>;
#[doc = "Indicators of interrupt triggers detected"]
pub mod gpio_intr;
#[doc = "GPIO_INTR_RAW register accessor: an alias for `Reg<GPIO_INTR_RAW_SPEC>`"]
pub type GPIO_INTR_RAW = crate::Reg<gpio_intr_raw::GPIO_INTR_RAW_SPEC>;
#[doc = "GPIO raw interrupt indicators"]
pub mod gpio_intr_raw;
#[doc = "GPIO_INTR_TYPE register accessor: an alias for `Reg<GPIO_INTR_TYPE_SPEC>`"]
pub type GPIO_INTR_TYPE = crate::Reg<gpio_intr_type::GPIO_INTR_TYPE_SPEC>;
#[doc = "Indicators of interrupt trigger types"]
pub mod gpio_intr_type;
#[doc = "GPIO_INTR_POL register accessor: an alias for `Reg<GPIO_INTR_POL_SPEC>`"]
pub type GPIO_INTR_POL = crate::Reg<gpio_intr_pol::GPIO_INTR_POL_SPEC>;
#[doc = "Indicators of interrupt trigger polarities (will depend on the type of interrupt)"]
pub mod gpio_intr_pol;
#[doc = "GPIO_INTR_EN_AP register accessor: an alias for `Reg<GPIO_INTR_EN_AP_SPEC>`"]
pub type GPIO_INTR_EN_AP = crate::Reg<gpio_intr_en_ap::GPIO_INTR_EN_AP_SPEC>;
#[doc = "GPIO interrupt enable for AP"]
pub mod gpio_intr_en_ap;
#[doc = "GPIO_INTR_EN_M4 register accessor: an alias for `Reg<GPIO_INTR_EN_M4_SPEC>`"]
pub type GPIO_INTR_EN_M4 = crate::Reg<gpio_intr_en_m4::GPIO_INTR_EN_M4_SPEC>;
#[doc = "GPIO interrupt enable for M4"]
pub mod gpio_intr_en_m4;
#[doc = "GPIO_INTR_EN_FFE0 register accessor: an alias for `Reg<GPIO_INTR_EN_FFE0_SPEC>`"]
pub type GPIO_INTR_EN_FFE0 =
    crate::Reg<gpio_intr_en_ffe0::GPIO_INTR_EN_FFE0_SPEC>;
#[doc = "GPIO interrupt enable for FFE0"]
pub mod gpio_intr_en_ffe0;
#[doc = "GPIO_INTR_EN_FFE1 register accessor: an alias for `Reg<GPIO_INTR_EN_FFE1_SPEC>`"]
pub type GPIO_INTR_EN_FFE1 =
    crate::Reg<gpio_intr_en_ffe1::GPIO_INTR_EN_FFE1_SPEC>;
#[doc = "GPIO interrupt enable for FFE1"]
pub mod gpio_intr_en_ffe1;
#[doc = "OTHER_INTR register accessor: an alias for `Reg<OTHER_INTR_SPEC>`"]
pub type OTHER_INTR = crate::Reg<other_intr::OTHER_INTR_SPEC>;
#[doc = "Indicators of interrupt triggers detected"]
pub mod other_intr;
#[doc = "OTHER_INTR_EN_AP register accessor: an alias for `Reg<OTHER_INTR_EN_AP_SPEC>`"]
pub type OTHER_INTR_EN_AP = crate::Reg<other_intr_en_ap::OTHER_INTR_EN_AP_SPEC>;
#[doc = "Various interrupt enable for AP"]
pub mod other_intr_en_ap;
#[doc = "OTHER_INTR_EN_M4 register accessor: an alias for `Reg<OTHER_INTR_EN_M4_SPEC>`"]
pub type OTHER_INTR_EN_M4 = crate::Reg<other_intr_en_m4::OTHER_INTR_EN_M4_SPEC>;
#[doc = "Various interrupt enable for M4"]
pub mod other_intr_en_m4;
#[doc = "SOFTWARE_INTR_1 register accessor: an alias for `Reg<SOFTWARE_INTR_1_SPEC>`"]
pub type SOFTWARE_INTR_1 = crate::Reg<software_intr_1::SOFTWARE_INTR_1_SPEC>;
#[doc = "Indicators of General purpose software interrupt 1 triggers detected"]
pub mod software_intr_1;
#[doc = "SOFTWARE_INTR_1_EN_AP register accessor: an alias for `Reg<SOFTWARE_INTR_1_EN_AP_SPEC>`"]
pub type SOFTWARE_INTR_1_EN_AP =
    crate::Reg<software_intr_1_en_ap::SOFTWARE_INTR_1_EN_AP_SPEC>;
#[doc = "General purpose Software interrupt 1 enable for AP"]
pub mod software_intr_1_en_ap;
#[doc = "SOFTWARE_INTR_2 register accessor: an alias for `Reg<SOFTWARE_INTR_2_SPEC>`"]
pub type SOFTWARE_INTR_2 = crate::Reg<software_intr_2::SOFTWARE_INTR_2_SPEC>;
#[doc = "Indicators of General purpose software interrupt 2 triggers detected"]
pub mod software_intr_2;
#[doc = "SOFTWARE_INTR_2_EN_AP register accessor: an alias for `Reg<SOFTWARE_INTR_2_EN_AP_SPEC>`"]
pub type SOFTWARE_INTR_2_EN_AP =
    crate::Reg<software_intr_2_en_ap::SOFTWARE_INTR_2_EN_AP_SPEC>;
#[doc = "General purpose Software interrupt 2 enable for AP"]
pub mod software_intr_2_en_ap;
#[doc = "SOFTWARE_INTR_EN_M4 register accessor: an alias for `Reg<SOFTWARE_INTR_EN_M4_SPEC>`"]
pub type SOFTWARE_INTR_EN_M4 =
    crate::Reg<software_intr_en_m4::SOFTWARE_INTR_EN_M4_SPEC>;
#[doc = "General purpose Software interrupt 2 enable for M4"]
pub mod software_intr_en_m4;
#[doc = "FFE_INTR register accessor: an alias for `Reg<FFE_INTR_SPEC>`"]
pub type FFE_INTR = crate::Reg<ffe_intr::FFE_INTR_SPEC>;
#[doc = "Indicators of FFE0 interrupt triggers detected"]
pub mod ffe_intr;
#[doc = "FFE_INTR_EN_AP register accessor: an alias for `Reg<FFE_INTR_EN_AP_SPEC>`"]
pub type FFE_INTR_EN_AP = crate::Reg<ffe_intr_en_ap::FFE_INTR_EN_AP_SPEC>;
#[doc = "FFE0 interrupt enable for AP"]
pub mod ffe_intr_en_ap;
#[doc = "FFE_INTR_EN_M4 register accessor: an alias for `Reg<FFE_INTR_EN_M4_SPEC>`"]
pub type FFE_INTR_EN_M4 = crate::Reg<ffe_intr_en_m4::FFE_INTR_EN_M4_SPEC>;
#[doc = "FFE0 interrupt enable for M4"]
pub mod ffe_intr_en_m4;
#[doc = "FB_INTR register accessor: an alias for `Reg<FB_INTR_SPEC>`"]
pub type FB_INTR = crate::Reg<fb_intr::FB_INTR_SPEC>;
#[doc = "Indicators of interrupt triggers detected"]
pub mod fb_intr;
#[doc = "FB_INTR_RAW register accessor: an alias for `Reg<FB_INTR_RAW_SPEC>`"]
pub type FB_INTR_RAW = crate::Reg<fb_intr_raw::FB_INTR_RAW_SPEC>;
#[doc = "FB raw interrupt indicators"]
pub mod fb_intr_raw;
#[doc = "FB_INTR_TYPE register accessor: an alias for `Reg<FB_INTR_TYPE_SPEC>`"]
pub type FB_INTR_TYPE = crate::Reg<fb_intr_type::FB_INTR_TYPE_SPEC>;
#[doc = "Indicators of interrupt trigger types"]
pub mod fb_intr_type;
#[doc = "FB_INTR_POL register accessor: an alias for `Reg<FB_INTR_POL_SPEC>`"]
pub type FB_INTR_POL = crate::Reg<fb_intr_pol::FB_INTR_POL_SPEC>;
#[doc = "Indicators of interrupt trigger polarities (will depend on the type of interrupt)"]
pub mod fb_intr_pol;
#[doc = "FB_INTR_EN_AP register accessor: an alias for `Reg<FB_INTR_EN_AP_SPEC>`"]
pub type FB_INTR_EN_AP = crate::Reg<fb_intr_en_ap::FB_INTR_EN_AP_SPEC>;
#[doc = "FB interrupt enable for AP"]
pub mod fb_intr_en_ap;
#[doc = "FB_INTR_EN_M4 register accessor: an alias for `Reg<FB_INTR_EN_M4_SPEC>`"]
pub type FB_INTR_EN_M4 = crate::Reg<fb_intr_en_m4::FB_INTR_EN_M4_SPEC>;
#[doc = "FB interrupt enable for M4"]
pub mod fb_intr_en_m4;
#[doc = "M4_MEM_AON_INTR register accessor: an alias for `Reg<M4_MEM_AON_INTR_SPEC>`"]
pub type M4_MEM_AON_INTR = crate::Reg<m4_mem_aon_intr::M4_MEM_AON_INTR_SPEC>;
#[doc = "Indicator of detected SRAM access while shut down or sleep mode interrupts"]
pub mod m4_mem_aon_intr;
#[doc = "M4_MEM_AON_INTR_EN register accessor: an alias for `Reg<M4_MEM_AON_INTR_EN_SPEC>`"]
pub type M4_MEM_AON_INTR_EN =
    crate::Reg<m4_mem_aon_intr_en::M4_MEM_AON_INTR_EN_SPEC>;
#[doc = "Interrupt enable for SRAM access while in shut down or deep sleep modes"]
pub mod m4_mem_aon_intr_en;
