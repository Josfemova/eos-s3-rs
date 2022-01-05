#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x8c - PAD_%s control register"]
    pub pad__ctrl: [crate::Reg<pad__ctrl::PAD__CTRL_SPEC>; 35],
    #[doc = "0x8c - PAD_%s control register(ffe exclusive)"]
    pub pad_35_ctrl_ffe: crate::Reg<pad__ctrl_ffe::PAD__CTRL_FFE_SPEC>,
    #[doc = "0x90 - PAD_%s control register(ffe exclusive)"]
    pub pad_36_ctrl_ffe: crate::Reg<pad__ctrl_ffe::PAD__CTRL_FFE_SPEC>,
    #[doc = "0x94 - PAD_%s control register(ffe exclusive)"]
    pub pad_37_ctrl_ffe: crate::Reg<pad__ctrl_ffe::PAD__CTRL_FFE_SPEC>,
    #[doc = "0x98 - PAD_%s control register(ffe exclusive)"]
    pub pad_38_ctrl_ffe: crate::Reg<pad__ctrl_ffe::PAD__CTRL_FFE_SPEC>,
    #[doc = "0x9c - PAD_%s control register(ffe exclusive)"]
    pub pad_39_ctrl_ffe: crate::Reg<pad__ctrl_ffe::PAD__CTRL_FFE_SPEC>,
    #[doc = "0xa0 - PAD_%s control register(ffe exclusive)"]
    pub pad_40_ctrl_ffe: crate::Reg<pad__ctrl_ffe::PAD__CTRL_FFE_SPEC>,
    #[doc = "0xa4 - PAD_%s control register(ffe exclusive)"]
    pub pad_41_ctrl_ffe: crate::Reg<pad__ctrl_ffe::PAD__CTRL_FFE_SPEC>,
    #[doc = "0xa8 - PAD_%s control register(ffe exclusive)"]
    pub pad_42_ctrl_ffe: crate::Reg<pad__ctrl_ffe::PAD__CTRL_FFE_SPEC>,
    #[doc = "0xac - PAD_%s control register(ffe exclusive)"]
    pub pad_43_ctrl_ffe: crate::Reg<pad__ctrl_ffe::PAD__CTRL_FFE_SPEC>,
    #[doc = "0xb0 - PAD_%s control register(ffe exclusive)"]
    pub pad_44_ctrl_ffe: crate::Reg<pad__ctrl_ffe::PAD__CTRL_FFE_SPEC>,
    #[doc = "0xb4 - PAD_%s control register(ffe exclusive)"]
    pub pad_45_ctrl_ffe: crate::Reg<pad__ctrl_ffe::PAD__CTRL_FFE_SPEC>,
    _reserved12: [u8; 0x48],
    #[doc = "0x100 - Select pad for SDA function in I2C0 (only pad 1 is selectable)"]
    pub sda0_sel_reg: crate::Reg<sda0_sel_reg::SDA0_SEL_REG_SPEC>,
    #[doc = "0x104 - Select pad for SDA function in I2C1 (3 pads available)"]
    pub sda1_sel_reg: crate::Reg<sda1_sel_reg::SDA1_SEL_REG_SPEC>,
    #[doc = "0x108 - Select pad for SDA function in I2C2 (only pad 41 is selectable)"]
    pub sda2_sel_reg: crate::Reg<sda2_sel_reg::SDA2_SEL_REG_SPEC>,
    #[doc = "0x10c - Select pad for SCL function in I2C0 (only pad 0 is selectable)"]
    pub scl0_sel_reg: crate::Reg<scl0_sel_reg::SCL0_SEL_REG_SPEC>,
    #[doc = "0x110 - Select pad for SCL function in I2C1 (3 pads available)"]
    pub scl1_sel_reg: crate::Reg<scl1_sel_reg::SCL1_SEL_REG_SPEC>,
    #[doc = "0x114 - Select pad for SCL function in I2C2 (only pad 40 is selectable)"]
    pub scl2_sel_reg: crate::Reg<scl2_sel_reg::SCL2_SEL_REG_SPEC>,
    #[doc = "0x118 - Select pad for SPI CLK function (only pad 16 is selectable)"]
    pub spis_clk_sel: crate::Reg<spis_clk_sel::SPIS_CLK_SEL_SPEC>,
    #[doc = "0x11c - Select pad for SPI SS function (only pad 20 is selectable)"]
    pub spis_ssn_sel: crate::Reg<spis_ssn_sel::SPIS_SSN_SEL_SPEC>,
    #[doc = "0x120 - Select pad for SPI MOSI function (only pad 19 is selectable)"]
    pub spis_mosi_sel: crate::Reg<spis_mosi_sel::SPIS_MOSI_SEL_SPEC>,
    #[doc = "0x124 - Select pad for SPI MISO function (only pad 36 is selectable)"]
    pub spis_miso_sel: crate::Reg<spis_miso_sel::SPIS_MISO_SEL_SPEC>,
    #[doc = "0x128 - Select pad for PDM microphone data line"]
    pub pdm_data_sele: crate::Reg<pdm_data_sele::PDM_DATA_SELE_SPEC>,
    #[doc = "0x12c - Select pad for the I2S Serial Data (SD)"]
    pub i2s_data_select: crate::Reg<i2s_data_select::I2S_DATA_SELECT_SPEC>,
    _reserved24: [u8; 0x04],
    #[doc = "0x134 - a"]
    pub uart_rxd_sel: crate::Reg<uart_rxd_sel::UART_RXD_SEL_SPEC>,
    #[doc = "0x138 - Select pad for SIREN in function"]
    pub ir_da_sirin_sel: crate::Reg<ir_da_sirin_sel::IRDA_SIRIN_SEL_SPEC>,
    #[doc = "0x13c - Select pad that triggers GPIO interrupt 0"]
    pub s_intr_0_sel_reg: crate::Reg<s_intr_0_sel_reg::S_INTR_0_SEL_REG_SPEC>,
    #[doc = "0x140 - Select pad that triggers GPIO interrupt 1"]
    pub s_intr_1_sel_reg: crate::Reg<s_intr_1_sel_reg::S_INTR_1_SEL_REG_SPEC>,
    #[doc = "0x144 - Select pad that triggers GPIO interrupt 2"]
    pub s_intr_2_sel: crate::Reg<s_intr_2_sel::S_INTR_2_SEL_SPEC>,
    #[doc = "0x148 - Select pad that triggers GPIO interrupt 3"]
    pub s_intr_3_sel: crate::Reg<s_intr_3_sel::S_INTR_3_SEL_SPEC>,
    #[doc = "0x14c - Select pad that triggers GPIO interrupt 4"]
    pub s_intr_4_sel: crate::Reg<s_intr_4_sel::S_INTR_4_SEL_SPEC>,
    #[doc = "0x150 - Select pad that triggers GPIO interrupt 5"]
    pub s_intr_5_sel: crate::Reg<s_intr_5_sel::S_INTR_5_SEL_SPEC>,
    #[doc = "0x154 - Select pad that triggers GPIO interrupt 6"]
    pub s_intr_6_sel: crate::Reg<s_intr_6_sel::S_INTR_6_SEL_SPEC>,
    #[doc = "0x158 - Select pad that triggers GPIO interrupt 7"]
    pub s_intr_7_sel: crate::Reg<s_intr_7_sel::S_INTR_7_SEL_SPEC>,
    #[doc = "0x15c - Pad Selection for the CTS nUART function"]
    pub nuartcts_sel: crate::Reg<nuartcts_sel::NUARTCTS_SEL_SPEC>,
    #[doc = "0x160 - Selects which IO input will be registered (Pads used as GPIOS)"]
    pub io_reg_sel: crate::Reg<io_reg_sel::IO_REG_SEL_SPEC>,
    _reserved36: [u8; 0x0c],
    #[doc = "0x170 - Selection for SWD clock pad (SCK)"]
    pub sw_clk_sel: crate::Reg<sw_clk_sel::SW_CLK_SEL_SPEC>,
    #[doc = "0x174 - Selection for SWD IO pad (SDIO)"]
    pub sw_io_sel: crate::Reg<sw_io_sel::SW_IO_SEL_SPEC>,
    _reserved38: [u8; 0x08],
    #[doc = "0x180 - Lacking proper documentation. Configuration of pins 0-31 related to Fabric"]
    pub fbio_sel_1: crate::Reg<fbio_sel_1::FBIO_SEL_1_SPEC>,
    #[doc = "0x184 - Lacking proper documentation. Configuration of pins 32-45 related to Fabric"]
    pub fbio_sel_2: crate::Reg<fbio_sel_2::FBIO_SEL_2_SPEC>,
    _reserved40: [u8; 0x08],
    #[doc = "0x190 - Selects pad for MISO function for the sensor SPI"]
    pub spi_sensor_miso_sel:
        crate::Reg<spi_sensor_miso_sel::SPI_SENSOR_MISO_SEL_SPEC>,
    #[doc = "0x194 - Selects pad for MOSI function for the sensor SPI"]
    pub spi_sensor_mosi_sel:
        crate::Reg<spi_sensor_mosi_sel::SPI_SENSOR_MOSI_SEL_SPEC>,
    _reserved42: [u8; 0x08],
    #[doc = "0x1a0 - Selects pad for the I2S Data-Word Select (WS)"]
    pub i2s_wd_clkin_sel: crate::Reg<i2s_wd_clkin_sel::I2S_WD_CLKIN_SEL_SPEC>,
    #[doc = "0x1a4 - Selects pad for I2S clock (SCK)"]
    pub i2s_clkin_sel: crate::Reg<i2s_clkin_sel::I2S_CLKIN_SEL_SPEC>,
    #[doc = "0x1a8 - Selects pin for PDM STATUS_IN function"]
    pub pdm_stat_in_sel: crate::Reg<pdm_stat_in_sel::PDM_STAT_IN_SEL_SPEC>,
    #[doc = "0x1ac - Selects pin for PDM CLKIN source (AP PDM CKO IN)"]
    pub pdm_clkin_sel: crate::Reg<pdm_clkin_sel::PDM_CLKIN_SEL_SPEC>,
}
#[doc = "PAD__CTRL register accessor: an alias for `Reg<PAD__CTRL_SPEC>`"]
pub type PAD__CTRL = crate::Reg<pad__ctrl::PAD__CTRL_SPEC>;
#[doc = "PAD_%s control register"]
pub mod pad__ctrl;
#[doc = "PAD__CTRL_FFE register accessor: an alias for `Reg<PAD__CTRL_FFE_SPEC>`"]
pub type PAD__CTRL_FFE = crate::Reg<pad__ctrl_ffe::PAD__CTRL_FFE_SPEC>;
#[doc = "PAD_%s control register(ffe exclusive)"]
pub mod pad__ctrl_ffe;
#[doc = "SDA0_SEL_REG register accessor: an alias for `Reg<SDA0_SEL_REG_SPEC>`"]
pub type SDA0_SEL_REG = crate::Reg<sda0_sel_reg::SDA0_SEL_REG_SPEC>;
#[doc = "Select pad for SDA function in I2C0 (only pad 1 is selectable)"]
pub mod sda0_sel_reg;
#[doc = "SDA1_SEL_REG register accessor: an alias for `Reg<SDA1_SEL_REG_SPEC>`"]
pub type SDA1_SEL_REG = crate::Reg<sda1_sel_reg::SDA1_SEL_REG_SPEC>;
#[doc = "Select pad for SDA function in I2C1 (3 pads available)"]
pub mod sda1_sel_reg;
#[doc = "SDA2_SEL_REG register accessor: an alias for `Reg<SDA2_SEL_REG_SPEC>`"]
pub type SDA2_SEL_REG = crate::Reg<sda2_sel_reg::SDA2_SEL_REG_SPEC>;
#[doc = "Select pad for SDA function in I2C2 (only pad 41 is selectable)"]
pub mod sda2_sel_reg;
#[doc = "SCL0_SEL_REG register accessor: an alias for `Reg<SCL0_SEL_REG_SPEC>`"]
pub type SCL0_SEL_REG = crate::Reg<scl0_sel_reg::SCL0_SEL_REG_SPEC>;
#[doc = "Select pad for SCL function in I2C0 (only pad 0 is selectable)"]
pub mod scl0_sel_reg;
#[doc = "SCL1_SEL_REG register accessor: an alias for `Reg<SCL1_SEL_REG_SPEC>`"]
pub type SCL1_SEL_REG = crate::Reg<scl1_sel_reg::SCL1_SEL_REG_SPEC>;
#[doc = "Select pad for SCL function in I2C1 (3 pads available)"]
pub mod scl1_sel_reg;
#[doc = "SCL2_SEL_REG register accessor: an alias for `Reg<SCL2_SEL_REG_SPEC>`"]
pub type SCL2_SEL_REG = crate::Reg<scl2_sel_reg::SCL2_SEL_REG_SPEC>;
#[doc = "Select pad for SCL function in I2C2 (only pad 40 is selectable)"]
pub mod scl2_sel_reg;
#[doc = "SPIs_CLK_SEL register accessor: an alias for `Reg<SPIS_CLK_SEL_SPEC>`"]
pub type SPIS_CLK_SEL = crate::Reg<spis_clk_sel::SPIS_CLK_SEL_SPEC>;
#[doc = "Select pad for SPI CLK function (only pad 16 is selectable)"]
pub mod spis_clk_sel;
#[doc = "SPIs_SSn_SEL register accessor: an alias for `Reg<SPIS_SSN_SEL_SPEC>`"]
pub type SPIS_SSN_SEL = crate::Reg<spis_ssn_sel::SPIS_SSN_SEL_SPEC>;
#[doc = "Select pad for SPI SS function (only pad 20 is selectable)"]
pub mod spis_ssn_sel;
#[doc = "SPIs_MOSI_SEL register accessor: an alias for `Reg<SPIS_MOSI_SEL_SPEC>`"]
pub type SPIS_MOSI_SEL = crate::Reg<spis_mosi_sel::SPIS_MOSI_SEL_SPEC>;
#[doc = "Select pad for SPI MOSI function (only pad 19 is selectable)"]
pub mod spis_mosi_sel;
#[doc = "SPIs_MISO_SEL register accessor: an alias for `Reg<SPIS_MISO_SEL_SPEC>`"]
pub type SPIS_MISO_SEL = crate::Reg<spis_miso_sel::SPIS_MISO_SEL_SPEC>;
#[doc = "Select pad for SPI MISO function (only pad 36 is selectable)"]
pub mod spis_miso_sel;
#[doc = "PDM_DATA_SELE register accessor: an alias for `Reg<PDM_DATA_SELE_SPEC>`"]
pub type PDM_DATA_SELE = crate::Reg<pdm_data_sele::PDM_DATA_SELE_SPEC>;
#[doc = "Select pad for PDM microphone data line"]
pub mod pdm_data_sele;
#[doc = "I2S_DATA_SELECT register accessor: an alias for `Reg<I2S_DATA_SELECT_SPEC>`"]
pub type I2S_DATA_SELECT = crate::Reg<i2s_data_select::I2S_DATA_SELECT_SPEC>;
#[doc = "Select pad for the I2S Serial Data (SD)"]
pub mod i2s_data_select;
#[doc = "UART_rxd_SEL register accessor: an alias for `Reg<UART_RXD_SEL_SPEC>`"]
pub type UART_RXD_SEL = crate::Reg<uart_rxd_sel::UART_RXD_SEL_SPEC>;
#[doc = "a"]
pub mod uart_rxd_sel;
#[doc = "IrDA_Sirin_SEL register accessor: an alias for `Reg<IRDA_SIRIN_SEL_SPEC>`"]
pub type IRDA_SIRIN_SEL = crate::Reg<ir_da_sirin_sel::IRDA_SIRIN_SEL_SPEC>;
#[doc = "Select pad for SIREN in function"]
pub mod ir_da_sirin_sel;
#[doc = "S_INTR_0_SEL_REG register accessor: an alias for `Reg<S_INTR_0_SEL_REG_SPEC>`"]
pub type S_INTR_0_SEL_REG = crate::Reg<s_intr_0_sel_reg::S_INTR_0_SEL_REG_SPEC>;
#[doc = "Select pad that triggers GPIO interrupt 0"]
pub mod s_intr_0_sel_reg;
#[doc = "S_INTR_1_SEL_REG register accessor: an alias for `Reg<S_INTR_1_SEL_REG_SPEC>`"]
pub type S_INTR_1_SEL_REG = crate::Reg<s_intr_1_sel_reg::S_INTR_1_SEL_REG_SPEC>;
#[doc = "Select pad that triggers GPIO interrupt 1"]
pub mod s_intr_1_sel_reg;
#[doc = "S_INTR_2_SEL register accessor: an alias for `Reg<S_INTR_2_SEL_SPEC>`"]
pub type S_INTR_2_SEL = crate::Reg<s_intr_2_sel::S_INTR_2_SEL_SPEC>;
#[doc = "Select pad that triggers GPIO interrupt 2"]
pub mod s_intr_2_sel;
#[doc = "S_INTR_3_SEL register accessor: an alias for `Reg<S_INTR_3_SEL_SPEC>`"]
pub type S_INTR_3_SEL = crate::Reg<s_intr_3_sel::S_INTR_3_SEL_SPEC>;
#[doc = "Select pad that triggers GPIO interrupt 3"]
pub mod s_intr_3_sel;
#[doc = "S_INTR_4_SEL register accessor: an alias for `Reg<S_INTR_4_SEL_SPEC>`"]
pub type S_INTR_4_SEL = crate::Reg<s_intr_4_sel::S_INTR_4_SEL_SPEC>;
#[doc = "Select pad that triggers GPIO interrupt 4"]
pub mod s_intr_4_sel;
#[doc = "S_INTR_5_SEL register accessor: an alias for `Reg<S_INTR_5_SEL_SPEC>`"]
pub type S_INTR_5_SEL = crate::Reg<s_intr_5_sel::S_INTR_5_SEL_SPEC>;
#[doc = "Select pad that triggers GPIO interrupt 5"]
pub mod s_intr_5_sel;
#[doc = "S_INTR_6_SEL register accessor: an alias for `Reg<S_INTR_6_SEL_SPEC>`"]
pub type S_INTR_6_SEL = crate::Reg<s_intr_6_sel::S_INTR_6_SEL_SPEC>;
#[doc = "Select pad that triggers GPIO interrupt 6"]
pub mod s_intr_6_sel;
#[doc = "S_INTR_7_SEL register accessor: an alias for `Reg<S_INTR_7_SEL_SPEC>`"]
pub type S_INTR_7_SEL = crate::Reg<s_intr_7_sel::S_INTR_7_SEL_SPEC>;
#[doc = "Select pad that triggers GPIO interrupt 7"]
pub mod s_intr_7_sel;
#[doc = "NUARTCTS_SEL register accessor: an alias for `Reg<NUARTCTS_SEL_SPEC>`"]
pub type NUARTCTS_SEL = crate::Reg<nuartcts_sel::NUARTCTS_SEL_SPEC>;
#[doc = "Pad Selection for the CTS nUART function"]
pub mod nuartcts_sel;
#[doc = "IO_REG_SEL register accessor: an alias for `Reg<IO_REG_SEL_SPEC>`"]
pub type IO_REG_SEL = crate::Reg<io_reg_sel::IO_REG_SEL_SPEC>;
#[doc = "Selects which IO input will be registered (Pads used as GPIOS)"]
pub mod io_reg_sel;
#[doc = "SW_CLK_SEL register accessor: an alias for `Reg<SW_CLK_SEL_SPEC>`"]
pub type SW_CLK_SEL = crate::Reg<sw_clk_sel::SW_CLK_SEL_SPEC>;
#[doc = "Selection for SWD clock pad (SCK)"]
pub mod sw_clk_sel;
#[doc = "SW_IO_SEL register accessor: an alias for `Reg<SW_IO_SEL_SPEC>`"]
pub type SW_IO_SEL = crate::Reg<sw_io_sel::SW_IO_SEL_SPEC>;
#[doc = "Selection for SWD IO pad (SDIO)"]
pub mod sw_io_sel;
#[doc = "FBIO_SEL_1 register accessor: an alias for `Reg<FBIO_SEL_1_SPEC>`"]
pub type FBIO_SEL_1 = crate::Reg<fbio_sel_1::FBIO_SEL_1_SPEC>;
#[doc = "Lacking proper documentation. Configuration of pins 0-31 related to Fabric"]
pub mod fbio_sel_1;
#[doc = "FBIO_SEL_2 register accessor: an alias for `Reg<FBIO_SEL_2_SPEC>`"]
pub type FBIO_SEL_2 = crate::Reg<fbio_sel_2::FBIO_SEL_2_SPEC>;
#[doc = "Lacking proper documentation. Configuration of pins 32-45 related to Fabric"]
pub mod fbio_sel_2;
#[doc = "SPI_SENSOR_MISO_SEL register accessor: an alias for `Reg<SPI_SENSOR_MISO_SEL_SPEC>`"]
pub type SPI_SENSOR_MISO_SEL =
    crate::Reg<spi_sensor_miso_sel::SPI_SENSOR_MISO_SEL_SPEC>;
#[doc = "Selects pad for MISO function for the sensor SPI"]
pub mod spi_sensor_miso_sel;
#[doc = "SPI_SENSOR_MOSI_SEL register accessor: an alias for `Reg<SPI_SENSOR_MOSI_SEL_SPEC>`"]
pub type SPI_SENSOR_MOSI_SEL =
    crate::Reg<spi_sensor_mosi_sel::SPI_SENSOR_MOSI_SEL_SPEC>;
#[doc = "Selects pad for MOSI function for the sensor SPI"]
pub mod spi_sensor_mosi_sel;
#[doc = "I2S_WD_CLKIN_SEL register accessor: an alias for `Reg<I2S_WD_CLKIN_SEL_SPEC>`"]
pub type I2S_WD_CLKIN_SEL = crate::Reg<i2s_wd_clkin_sel::I2S_WD_CLKIN_SEL_SPEC>;
#[doc = "Selects pad for the I2S Data-Word Select (WS)"]
pub mod i2s_wd_clkin_sel;
#[doc = "I2S_CLKIN_SEL register accessor: an alias for `Reg<I2S_CLKIN_SEL_SPEC>`"]
pub type I2S_CLKIN_SEL = crate::Reg<i2s_clkin_sel::I2S_CLKIN_SEL_SPEC>;
#[doc = "Selects pad for I2S clock (SCK)"]
pub mod i2s_clkin_sel;
#[doc = "PDM_STAT_IN_SEL register accessor: an alias for `Reg<PDM_STAT_IN_SEL_SPEC>`"]
pub type PDM_STAT_IN_SEL = crate::Reg<pdm_stat_in_sel::PDM_STAT_IN_SEL_SPEC>;
#[doc = "Selects pin for PDM STATUS_IN function"]
pub mod pdm_stat_in_sel;
#[doc = "PDM_CLKIN_SEL register accessor: an alias for `Reg<PDM_CLKIN_SEL_SPEC>`"]
pub type PDM_CLKIN_SEL = crate::Reg<pdm_clkin_sel::PDM_CLKIN_SEL_SPEC>;
#[doc = "Selects pin for PDM CLKIN source (AP PDM CKO IN)"]
pub mod pdm_clkin_sel;
