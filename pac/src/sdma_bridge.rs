#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA request"]
    pub dma_req: crate::Reg<dma_req::DMA_REQ_SPEC>,
    #[doc = "0x04 - DMA wait on request register"]
    pub dma_waitonreq_reg:
        crate::Reg<dma_waitonreq_reg::DMA_WAITONREQ_REG_SPEC>,
    #[doc = "0x08 - dma_active signal status from System DMA"]
    pub dma_active_reg: crate::Reg<dma_active_reg::DMA_ACTIVE_REG_SPEC>,
    #[doc = "0x0c - sdma power down event threshold. If sdma stays in idle cycles longer than the threshold, sdma will be automaticlly put into power down to save power."]
    pub sdma_pwrd_cnt: crate::Reg<sdma_pwrd_cnt::SDMA_PWRD_CNT_SPEC>,
    #[doc = "0x10 - Control register for System DMA SRAM"]
    pub sdma_sram_ctl: crate::Reg<sdma_sram_ctl::SDMA_SRAM_CTL_SPEC>,
}
#[doc = "DMA_REQ register accessor: an alias for `Reg<DMA_REQ_SPEC>`"]
pub type DMA_REQ = crate::Reg<dma_req::DMA_REQ_SPEC>;
#[doc = "DMA request"]
pub mod dma_req;
#[doc = "DMA_WAITONREQ_REG register accessor: an alias for `Reg<DMA_WAITONREQ_REG_SPEC>`"]
pub type DMA_WAITONREQ_REG =
    crate::Reg<dma_waitonreq_reg::DMA_WAITONREQ_REG_SPEC>;
#[doc = "DMA wait on request register"]
pub mod dma_waitonreq_reg;
#[doc = "DMA_ACTIVE_REG register accessor: an alias for `Reg<DMA_ACTIVE_REG_SPEC>`"]
pub type DMA_ACTIVE_REG = crate::Reg<dma_active_reg::DMA_ACTIVE_REG_SPEC>;
#[doc = "dma_active signal status from System DMA"]
pub mod dma_active_reg;
#[doc = "SDMA_PWRD_CNT register accessor: an alias for `Reg<SDMA_PWRD_CNT_SPEC>`"]
pub type SDMA_PWRD_CNT = crate::Reg<sdma_pwrd_cnt::SDMA_PWRD_CNT_SPEC>;
#[doc = "sdma power down event threshold. If sdma stays in idle cycles longer than the threshold, sdma will be automaticlly put into power down to save power."]
pub mod sdma_pwrd_cnt;
#[doc = "SDMA_SRAM_CTL register accessor: an alias for `Reg<SDMA_SRAM_CTL_SPEC>`"]
pub type SDMA_SRAM_CTL = crate::Reg<sdma_sram_ctl::SDMA_SRAM_CTL_SPEC>;
#[doc = "Control register for System DMA SRAM"]
pub mod sdma_sram_ctl;
