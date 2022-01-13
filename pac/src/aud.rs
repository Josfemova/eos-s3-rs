#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Audio system configure register"]
    pub voice_config: crate::Reg<voice_config::VOICE_CONFIG_SPEC>,
    #[doc = "0x04 - LPSD config register"]
    pub lpsd_config: crate::Reg<lpsd_config::LPSD_CONFIG_SPEC>,
    #[doc = "0x08 - Audio DMAC configure register"]
    pub voice_dma_config: crate::Reg<voice_dma_config::VOICE_DMA_CONFIG_SPEC>,
    #[doc = "0x0c - Audio DMAC length register"]
    pub voice_dmac_len: crate::Reg<voice_dmac_len::VOICE_DMAC_LEN_SPEC>,
    #[doc = "0x10 - Audio DMAC Buffer offset"]
    pub voice_dmac_fifo: crate::Reg<voice_dmac_fifo::VOICE_DMAC_FIFO_SPEC>,
    #[doc = "0x14 - DMA0 dest address for the first buffer"]
    pub voice_dmac_dst_addr0:
        crate::Reg<voice_dmac_dst_addr0::VOICE_DMAC_DST_ADDR0_SPEC>,
    #[doc = "0x18 - DMA1 dest address for the first buffer"]
    pub voice_dmac_dst_addr1:
        crate::Reg<voice_dmac_dst_addr1::VOICE_DMAC_DST_ADDR1_SPEC>,
    #[doc = "0x1c - PDM2PCM core configure register"]
    pub pdm_core_config: crate::Reg<pdm_core_config::PDM_CORE_CONFIG_SPEC>,
    #[doc = "0x20 - Audio Status Register"]
    pub voice_status: crate::Reg<voice_status::VOICE_STATUS_SPEC>,
    #[doc = "0x24 - I2S master configure register"]
    pub i2s_config: crate::Reg<i2s_config::I2S_CONFIG_SPEC>,
    #[doc = "0x28 - FIFO SRAM configure register"]
    pub fifo_sram_cfg: crate::Reg<fifo_sram_cfg::FIFO_SRAM_CFG_SPEC>,
    #[doc = "0x2c - PDM core SRAM configure register"]
    pub pdma_sram_cfg: crate::Reg<pdma_sram_cfg::PDMA_SRAM_CFG_SPEC>,
    #[doc = "0x30 - Audio Debug Register"]
    pub dbg_mux_cfg: crate::Reg<dbg_mux_cfg::DBG_MUX_CFG_SPEC>,
}
#[doc = "VOICE_CONFIG register accessor: an alias for `Reg<VOICE_CONFIG_SPEC>`"]
pub type VOICE_CONFIG = crate::Reg<voice_config::VOICE_CONFIG_SPEC>;
#[doc = "Audio system configure register"]
pub mod voice_config;
#[doc = "LPSD_CONFIG register accessor: an alias for `Reg<LPSD_CONFIG_SPEC>`"]
pub type LPSD_CONFIG = crate::Reg<lpsd_config::LPSD_CONFIG_SPEC>;
#[doc = "LPSD config register"]
pub mod lpsd_config;
#[doc = "VOICE_DMA_CONFIG register accessor: an alias for `Reg<VOICE_DMA_CONFIG_SPEC>`"]
pub type VOICE_DMA_CONFIG = crate::Reg<voice_dma_config::VOICE_DMA_CONFIG_SPEC>;
#[doc = "Audio DMAC configure register"]
pub mod voice_dma_config;
#[doc = "VOICE_DMAC_LEN register accessor: an alias for `Reg<VOICE_DMAC_LEN_SPEC>`"]
pub type VOICE_DMAC_LEN = crate::Reg<voice_dmac_len::VOICE_DMAC_LEN_SPEC>;
#[doc = "Audio DMAC length register"]
pub mod voice_dmac_len;
#[doc = "VOICE_DMAC_FIFO register accessor: an alias for `Reg<VOICE_DMAC_FIFO_SPEC>`"]
pub type VOICE_DMAC_FIFO = crate::Reg<voice_dmac_fifo::VOICE_DMAC_FIFO_SPEC>;
#[doc = "Audio DMAC Buffer offset"]
pub mod voice_dmac_fifo;
#[doc = "VOICE_DMAC_DST_ADDR0 register accessor: an alias for `Reg<VOICE_DMAC_DST_ADDR0_SPEC>`"]
pub type VOICE_DMAC_DST_ADDR0 =
    crate::Reg<voice_dmac_dst_addr0::VOICE_DMAC_DST_ADDR0_SPEC>;
#[doc = "DMA0 dest address for the first buffer"]
pub mod voice_dmac_dst_addr0;
#[doc = "VOICE_DMAC_DST_ADDR1 register accessor: an alias for `Reg<VOICE_DMAC_DST_ADDR1_SPEC>`"]
pub type VOICE_DMAC_DST_ADDR1 =
    crate::Reg<voice_dmac_dst_addr1::VOICE_DMAC_DST_ADDR1_SPEC>;
#[doc = "DMA1 dest address for the first buffer"]
pub mod voice_dmac_dst_addr1;
#[doc = "PDM_CORE_CONFIG register accessor: an alias for `Reg<PDM_CORE_CONFIG_SPEC>`"]
pub type PDM_CORE_CONFIG = crate::Reg<pdm_core_config::PDM_CORE_CONFIG_SPEC>;
#[doc = "PDM2PCM core configure register"]
pub mod pdm_core_config;
#[doc = "VOICE_STATUS register accessor: an alias for `Reg<VOICE_STATUS_SPEC>`"]
pub type VOICE_STATUS = crate::Reg<voice_status::VOICE_STATUS_SPEC>;
#[doc = "Audio Status Register"]
pub mod voice_status;
#[doc = "I2S_CONFIG register accessor: an alias for `Reg<I2S_CONFIG_SPEC>`"]
pub type I2S_CONFIG = crate::Reg<i2s_config::I2S_CONFIG_SPEC>;
#[doc = "I2S master configure register"]
pub mod i2s_config;
#[doc = "FIFO_SRAM_CFG register accessor: an alias for `Reg<FIFO_SRAM_CFG_SPEC>`"]
pub type FIFO_SRAM_CFG = crate::Reg<fifo_sram_cfg::FIFO_SRAM_CFG_SPEC>;
#[doc = "FIFO SRAM configure register"]
pub mod fifo_sram_cfg;
#[doc = "PDMA_SRAM_CFG register accessor: an alias for `Reg<PDMA_SRAM_CFG_SPEC>`"]
pub type PDMA_SRAM_CFG = crate::Reg<pdma_sram_cfg::PDMA_SRAM_CFG_SPEC>;
#[doc = "PDM core SRAM configure register"]
pub mod pdma_sram_cfg;
#[doc = "DBG_MUX_CFG register accessor: an alias for `Reg<DBG_MUX_CFG_SPEC>`"]
pub type DBG_MUX_CFG = crate::Reg<dbg_mux_cfg::DBG_MUX_CFG_SPEC>;
#[doc = "Audio Debug Register"]
pub mod dbg_mux_cfg;
