#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - I2S Enable Register"]
    pub ier: crate::Reg<ier::IER_SPEC>,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - I2S Transmitter Block Enable Register"]
    pub iter: crate::Reg<iter::ITER_SPEC>,
    _reserved2: [u8; 0x0c],
    #[doc = "0x18 - Transmitter Block FIFO Reset Register"]
    pub txffr: crate::Reg<txffr::TXFFR_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x20 - Left Transmit Holding Register"]
    pub lthr0: crate::Reg<lthr0::LTHR0_SPEC>,
    #[doc = "0x24 - Right Transmit Holding Register"]
    pub rthr0: crate::Reg<rthr0::RTHR0_SPEC>,
    _reserved5: [u8; 0x04],
    #[doc = "0x2c - Transmit Enable Register"]
    pub ter0: crate::Reg<ter0::TER0_SPEC>,
    _reserved6: [u8; 0x04],
    #[doc = "0x34 - Transmit Configuration Register"]
    pub tcr0: crate::Reg<tcr0::TCR0_SPEC>,
    #[doc = "0x38 - Interrupt Status Register"]
    pub isr0: crate::Reg<isr0::ISR0_SPEC>,
    #[doc = "0x3c - Interrupt Mask Register"]
    pub imr0: crate::Reg<imr0::IMR0_SPEC>,
    _reserved9: [u8; 0x04],
    #[doc = "0x44 - Transmit Overrun Register"]
    pub tor0: crate::Reg<tor0::TOR0_SPEC>,
    _reserved10: [u8; 0x04],
    #[doc = "0x4c - Transmit FIFO Configuration Register"]
    pub tfcr0: crate::Reg<tfcr0::TFCR0_SPEC>,
    _reserved11: [u8; 0x04],
    #[doc = "0x54 - Transmit FIFO Flush"]
    pub tff0: crate::Reg<tff0::TFF0_SPEC>,
    _reserved12: [u8; 0x0170],
    #[doc = "0x1c8 - Transmitter Block DMA Register"]
    pub txdma: crate::Reg<txdma::TXDMA_SPEC>,
    #[doc = "0x1cc - Reset Transmitter Block DMA Register"]
    pub rtxdma: crate::Reg<rtxdma::RTXDMA_SPEC>,
    _reserved14: [u8; 0x24],
    #[doc = "0x1f4 - Component Parameter Register 1"]
    pub i2s_comp_param_1: crate::Reg<i2s_comp_param_1::I2S_COMP_PARAM_1_SPEC>,
    #[doc = "0x1f8 - Component version of the I2S peripheral."]
    pub i2s_comp_version: crate::Reg<i2s_comp_version::I2S_COMP_VERSION_SPEC>,
    #[doc = "0x1fc - Component type of the I2S peripheral."]
    pub i2s_comp_type: crate::Reg<i2s_comp_type::I2S_COMP_TYPE_SPEC>,
    _reserved17: [u8; 0x01f8],
    #[doc = "0x3f8 - Sound channel mode (mono or stereo)"]
    pub i2s_stereo_en: crate::Reg<i2s_stereo_en::I2S_STEREO_EN_SPEC>,
}
#[doc = "IER register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "I2S Enable Register"]
pub mod ier;
#[doc = "ITER register accessor: an alias for `Reg<ITER_SPEC>`"]
pub type ITER = crate::Reg<iter::ITER_SPEC>;
#[doc = "I2S Transmitter Block Enable Register"]
pub mod iter;
#[doc = "TXFFR register accessor: an alias for `Reg<TXFFR_SPEC>`"]
pub type TXFFR = crate::Reg<txffr::TXFFR_SPEC>;
#[doc = "Transmitter Block FIFO Reset Register"]
pub mod txffr;
#[doc = "LTHR0 register accessor: an alias for `Reg<LTHR0_SPEC>`"]
pub type LTHR0 = crate::Reg<lthr0::LTHR0_SPEC>;
#[doc = "Left Transmit Holding Register"]
pub mod lthr0;
#[doc = "RTHR0 register accessor: an alias for `Reg<RTHR0_SPEC>`"]
pub type RTHR0 = crate::Reg<rthr0::RTHR0_SPEC>;
#[doc = "Right Transmit Holding Register"]
pub mod rthr0;
#[doc = "TER0 register accessor: an alias for `Reg<TER0_SPEC>`"]
pub type TER0 = crate::Reg<ter0::TER0_SPEC>;
#[doc = "Transmit Enable Register"]
pub mod ter0;
#[doc = "TCR0 register accessor: an alias for `Reg<TCR0_SPEC>`"]
pub type TCR0 = crate::Reg<tcr0::TCR0_SPEC>;
#[doc = "Transmit Configuration Register"]
pub mod tcr0;
#[doc = "ISR0 register accessor: an alias for `Reg<ISR0_SPEC>`"]
pub type ISR0 = crate::Reg<isr0::ISR0_SPEC>;
#[doc = "Interrupt Status Register"]
pub mod isr0;
#[doc = "IMR0 register accessor: an alias for `Reg<IMR0_SPEC>`"]
pub type IMR0 = crate::Reg<imr0::IMR0_SPEC>;
#[doc = "Interrupt Mask Register"]
pub mod imr0;
#[doc = "TOR0 register accessor: an alias for `Reg<TOR0_SPEC>`"]
pub type TOR0 = crate::Reg<tor0::TOR0_SPEC>;
#[doc = "Transmit Overrun Register"]
pub mod tor0;
#[doc = "TFCR0 register accessor: an alias for `Reg<TFCR0_SPEC>`"]
pub type TFCR0 = crate::Reg<tfcr0::TFCR0_SPEC>;
#[doc = "Transmit FIFO Configuration Register"]
pub mod tfcr0;
#[doc = "TFF0 register accessor: an alias for `Reg<TFF0_SPEC>`"]
pub type TFF0 = crate::Reg<tff0::TFF0_SPEC>;
#[doc = "Transmit FIFO Flush"]
pub mod tff0;
#[doc = "TXDMA register accessor: an alias for `Reg<TXDMA_SPEC>`"]
pub type TXDMA = crate::Reg<txdma::TXDMA_SPEC>;
#[doc = "Transmitter Block DMA Register"]
pub mod txdma;
#[doc = "RTXDMA register accessor: an alias for `Reg<RTXDMA_SPEC>`"]
pub type RTXDMA = crate::Reg<rtxdma::RTXDMA_SPEC>;
#[doc = "Reset Transmitter Block DMA Register"]
pub mod rtxdma;
#[doc = "I2S_COMP_PARAM_1 register accessor: an alias for `Reg<I2S_COMP_PARAM_1_SPEC>`"]
pub type I2S_COMP_PARAM_1 = crate::Reg<i2s_comp_param_1::I2S_COMP_PARAM_1_SPEC>;
#[doc = "Component Parameter Register 1"]
pub mod i2s_comp_param_1;
#[doc = "I2S_COMP_VERSION register accessor: an alias for `Reg<I2S_COMP_VERSION_SPEC>`"]
pub type I2S_COMP_VERSION = crate::Reg<i2s_comp_version::I2S_COMP_VERSION_SPEC>;
#[doc = "Component version of the I2S peripheral."]
pub mod i2s_comp_version;
#[doc = "I2S_COMP_TYPE register accessor: an alias for `Reg<I2S_COMP_TYPE_SPEC>`"]
pub type I2S_COMP_TYPE = crate::Reg<i2s_comp_type::I2S_COMP_TYPE_SPEC>;
#[doc = "Component type of the I2S peripheral."]
pub mod i2s_comp_type;
#[doc = "I2S_STEREO_EN register accessor: an alias for `Reg<I2S_STEREO_EN_SPEC>`"]
pub type I2S_STEREO_EN = crate::Reg<i2s_stereo_en::I2S_STEREO_EN_SPEC>;
#[doc = "Sound channel mode (mono or stereo)"]
pub mod i2s_stereo_en;
