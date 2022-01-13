#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA Status register"]
    pub dma_status: crate::Reg<dma_status::DMA_STATUS_SPEC>,
    #[doc = "0x04 - DMA configuration register"]
    pub dma_cfg: crate::Reg<dma_cfg::DMA_CFG_SPEC>,
    #[doc = "0x08 - Control the pointer to the base address of the primary data structure"]
    pub ctrl_base_ptr: crate::Reg<ctrl_base_ptr::CTRL_BASE_PTR_SPEC>,
    #[doc = "0x0c - Base address of the alternate data structure."]
    pub alt_ctrl_base_ptr:
        crate::Reg<alt_ctrl_base_ptr::ALT_CTRL_BASE_PTR_SPEC>,
    #[doc = "0x10 - Channel wait on request status"]
    pub dma_waitonreq_status:
        crate::Reg<dma_waitonreq_status::DMA_WAITONREQ_STATUS_SPEC>,
    #[doc = "0x14 - Registers to generate a software DMA request in one of the 16 DMA channels"]
    pub chnl_sw_req: crate::Reg<chnl_sw_req::CHNL_SW_REQ_SPEC>,
    #[doc = "0x18 - Returns the useburst status, or disables dma_sreq\\[Channel\\]
from generating DMA requests"]
    pub chnl_use_burst_set:
        crate::Reg<chnl_use_burst_set::CHNL_USE_BURST_SET_SPEC>,
    #[doc = "0x1c - Set the appropriate bit to enable dma_sreq\\[Channel\\]
to generate requests."]
    pub chnl_useburst_set:
        crate::Reg<chnl_useburst_set::CHNL_USEBURST_SET_SPEC>,
    #[doc = "0x20 - Returns the request mask status of dma_req\\[\\]
and dma_sreq\\[\\], or disables the corresponding channel from generating DMA requests."]
    pub chnl_req_mask_set:
        crate::Reg<chnl_req_mask_set::CHNL_REQ_MASK_SET_SPEC>,
    #[doc = "0x24 - Set the appropriate bit to enable DMA requests for the channel corresponding to dma_req\\[C\\]
and dma_sreq\\[C\\]."]
    pub chnl_req_mask_clr:
        crate::Reg<chnl_req_mask_clr::CHNL_REQ_MASK_CLR_SPEC>,
    #[doc = "0x28 - Returns the enable status of the channels, or enables the corresponding channels."]
    pub chnl_enable_set: crate::Reg<chnl_enable_set::CHNL_ENABLE_SET_SPEC>,
    #[doc = "0x2c - Set the appropriate bit to disable the corresponding DMA channel."]
    pub chnl_enable_clr: crate::Reg<chnl_enable_clr::CHNL_ENABLE_CLR_SPEC>,
    #[doc = "0x30 - Returns the channel control data structure status, or selects the alternate data structure for the corresponding DMA channel."]
    pub chnl_pri_alt_set: crate::Reg<chnl_pri_alt_set::CHNL_PRI_ALT_SET_SPEC>,
    #[doc = "0x34 - Set the appropriate bit to select the primary data structure for the corresponding DMA channel."]
    pub chnl_pri_alt_clr: crate::Reg<chnl_pri_alt_clr::CHNL_PRI_ALT_CLR_SPEC>,
    #[doc = "0x38 - Returns the channel priority mask status, or sets the channel priority to high."]
    pub chnl_priority_set:
        crate::Reg<chnl_priority_set::CHNL_PRIORITY_SET_SPEC>,
    #[doc = "0x3c - Set the appropriate bit to select the default priority level for the specified DMA channel."]
    pub chnl_priority_clear:
        crate::Reg<chnl_priority_clear::CHNL_PRIORITY_CLEAR_SPEC>,
    _reserved16: [u8; 0x0c],
    #[doc = "0x4c - Returns the status of dma_err, or sets the signal LOW."]
    pub err_clr: crate::Reg<err_clr::ERR_CLR_SPEC>,
    _reserved17: [u8; 0x0f80],
    #[doc = "0xfd0 - Peripheral identification 4"]
    pub periph_id_4: crate::Reg<periph_id_4::PERIPH_ID_4_SPEC>,
    _reserved18: [u8; 0x0c],
    #[doc = "0xfe0 - Peripheral identification 0"]
    pub periph_id_0: crate::Reg<periph_id_0::PERIPH_ID_0_SPEC>,
    #[doc = "0xfe4 - Peripheral identification 1"]
    pub periph_id_1: crate::Reg<periph_id_1::PERIPH_ID_1_SPEC>,
    #[doc = "0xfe8 - Peripheral identification 2"]
    pub periph_id_2: crate::Reg<periph_id_2::PERIPH_ID_2_SPEC>,
    #[doc = "0xfec - Peripheral identification 3"]
    pub periph_id_3: crate::Reg<periph_id_3::PERIPH_ID_3_SPEC>,
    _reserved_22_pcell_id_: [u8; 0x04],
}
impl RegisterBlock {
    #[doc = "0xff0 - PrimeCell identification 3"]
    #[inline(always)]
    pub fn pcell_id_3(&self) -> &crate::Reg<pcell_id_3::PCELL_ID_3_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4080usize)
                as *const crate::Reg<pcell_id_3::PCELL_ID_3_SPEC>)
        }
    }
    #[doc = "0xff0 - PrimeCell identification 2"]
    #[inline(always)]
    pub fn pcell_id_2(&self) -> &crate::Reg<pcell_id_2::PCELL_ID_2_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4080usize)
                as *const crate::Reg<pcell_id_2::PCELL_ID_2_SPEC>)
        }
    }
    #[doc = "0xff0 - PrimeCell identification 1"]
    #[inline(always)]
    pub fn pcell_id_1(&self) -> &crate::Reg<pcell_id_1::PCELL_ID_1_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4080usize)
                as *const crate::Reg<pcell_id_1::PCELL_ID_1_SPEC>)
        }
    }
    #[doc = "0xff0 - PrimeCell identification 0"]
    #[inline(always)]
    pub fn pcell_id_0(&self) -> &crate::Reg<pcell_id_0::PCELL_ID_0_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4080usize)
                as *const crate::Reg<pcell_id_0::PCELL_ID_0_SPEC>)
        }
    }
}
#[doc = "DMA_STATUS register accessor: an alias for `Reg<DMA_STATUS_SPEC>`"]
pub type DMA_STATUS = crate::Reg<dma_status::DMA_STATUS_SPEC>;
#[doc = "DMA Status register"]
pub mod dma_status;
#[doc = "DMA_CFG register accessor: an alias for `Reg<DMA_CFG_SPEC>`"]
pub type DMA_CFG = crate::Reg<dma_cfg::DMA_CFG_SPEC>;
#[doc = "DMA configuration register"]
pub mod dma_cfg;
#[doc = "CTRL_BASE_PTR register accessor: an alias for `Reg<CTRL_BASE_PTR_SPEC>`"]
pub type CTRL_BASE_PTR = crate::Reg<ctrl_base_ptr::CTRL_BASE_PTR_SPEC>;
#[doc = "Control the pointer to the base address of the primary data structure"]
pub mod ctrl_base_ptr;
#[doc = "ALT_CTRL_BASE_PTR register accessor: an alias for `Reg<ALT_CTRL_BASE_PTR_SPEC>`"]
pub type ALT_CTRL_BASE_PTR =
    crate::Reg<alt_ctrl_base_ptr::ALT_CTRL_BASE_PTR_SPEC>;
#[doc = "Base address of the alternate data structure."]
pub mod alt_ctrl_base_ptr;
#[doc = "DMA_WAITONREQ_STATUS register accessor: an alias for `Reg<DMA_WAITONREQ_STATUS_SPEC>`"]
pub type DMA_WAITONREQ_STATUS =
    crate::Reg<dma_waitonreq_status::DMA_WAITONREQ_STATUS_SPEC>;
#[doc = "Channel wait on request status"]
pub mod dma_waitonreq_status;
#[doc = "CHNL_SW_REQ register accessor: an alias for `Reg<CHNL_SW_REQ_SPEC>`"]
pub type CHNL_SW_REQ = crate::Reg<chnl_sw_req::CHNL_SW_REQ_SPEC>;
#[doc = "Registers to generate a software DMA request in one of the 16 DMA channels"]
pub mod chnl_sw_req;
#[doc = "CHNL_USE_BURST_SET register accessor: an alias for `Reg<CHNL_USE_BURST_SET_SPEC>`"]
pub type CHNL_USE_BURST_SET =
    crate::Reg<chnl_use_burst_set::CHNL_USE_BURST_SET_SPEC>;
#[doc = "Returns the useburst status, or disables dma_sreq\\[Channel\\]
from generating DMA requests"]
pub mod chnl_use_burst_set;
#[doc = "CHNL_USEBURST_SET register accessor: an alias for `Reg<CHNL_USEBURST_SET_SPEC>`"]
pub type CHNL_USEBURST_SET =
    crate::Reg<chnl_useburst_set::CHNL_USEBURST_SET_SPEC>;
#[doc = "Set the appropriate bit to enable dma_sreq\\[Channel\\]
to generate requests."]
pub mod chnl_useburst_set;
#[doc = "CHNL_REQ_MASK_SET register accessor: an alias for `Reg<CHNL_REQ_MASK_SET_SPEC>`"]
pub type CHNL_REQ_MASK_SET =
    crate::Reg<chnl_req_mask_set::CHNL_REQ_MASK_SET_SPEC>;
#[doc = "Returns the request mask status of dma_req\\[\\]
and dma_sreq\\[\\], or disables the corresponding channel from generating DMA requests."]
pub mod chnl_req_mask_set;
#[doc = "CHNL_REQ_MASK_CLR register accessor: an alias for `Reg<CHNL_REQ_MASK_CLR_SPEC>`"]
pub type CHNL_REQ_MASK_CLR =
    crate::Reg<chnl_req_mask_clr::CHNL_REQ_MASK_CLR_SPEC>;
#[doc = "Set the appropriate bit to enable DMA requests for the channel corresponding to dma_req\\[C\\]
and dma_sreq\\[C\\]."]
pub mod chnl_req_mask_clr;
#[doc = "CHNL_ENABLE_SET register accessor: an alias for `Reg<CHNL_ENABLE_SET_SPEC>`"]
pub type CHNL_ENABLE_SET = crate::Reg<chnl_enable_set::CHNL_ENABLE_SET_SPEC>;
#[doc = "Returns the enable status of the channels, or enables the corresponding channels."]
pub mod chnl_enable_set;
#[doc = "CHNL_ENABLE_CLR register accessor: an alias for `Reg<CHNL_ENABLE_CLR_SPEC>`"]
pub type CHNL_ENABLE_CLR = crate::Reg<chnl_enable_clr::CHNL_ENABLE_CLR_SPEC>;
#[doc = "Set the appropriate bit to disable the corresponding DMA channel."]
pub mod chnl_enable_clr;
#[doc = "CHNL_PRI_ALT_SET register accessor: an alias for `Reg<CHNL_PRI_ALT_SET_SPEC>`"]
pub type CHNL_PRI_ALT_SET = crate::Reg<chnl_pri_alt_set::CHNL_PRI_ALT_SET_SPEC>;
#[doc = "Returns the channel control data structure status, or selects the alternate data structure for the corresponding DMA channel."]
pub mod chnl_pri_alt_set;
#[doc = "CHNL_PRI_ALT_CLR register accessor: an alias for `Reg<CHNL_PRI_ALT_CLR_SPEC>`"]
pub type CHNL_PRI_ALT_CLR = crate::Reg<chnl_pri_alt_clr::CHNL_PRI_ALT_CLR_SPEC>;
#[doc = "Set the appropriate bit to select the primary data structure for the corresponding DMA channel."]
pub mod chnl_pri_alt_clr;
#[doc = "CHNL_PRIORITY_SET register accessor: an alias for `Reg<CHNL_PRIORITY_SET_SPEC>`"]
pub type CHNL_PRIORITY_SET =
    crate::Reg<chnl_priority_set::CHNL_PRIORITY_SET_SPEC>;
#[doc = "Returns the channel priority mask status, or sets the channel priority to high."]
pub mod chnl_priority_set;
#[doc = "CHNL_PRIORITY_CLEAR register accessor: an alias for `Reg<CHNL_PRIORITY_CLEAR_SPEC>`"]
pub type CHNL_PRIORITY_CLEAR =
    crate::Reg<chnl_priority_clear::CHNL_PRIORITY_CLEAR_SPEC>;
#[doc = "Set the appropriate bit to select the default priority level for the specified DMA channel."]
pub mod chnl_priority_clear;
#[doc = "ERR_CLR register accessor: an alias for `Reg<ERR_CLR_SPEC>`"]
pub type ERR_CLR = crate::Reg<err_clr::ERR_CLR_SPEC>;
#[doc = "Returns the status of dma_err, or sets the signal LOW."]
pub mod err_clr;
#[doc = "PERIPH_ID_4 register accessor: an alias for `Reg<PERIPH_ID_4_SPEC>`"]
pub type PERIPH_ID_4 = crate::Reg<periph_id_4::PERIPH_ID_4_SPEC>;
#[doc = "Peripheral identification 4"]
pub mod periph_id_4;
#[doc = "PERIPH_ID_0 register accessor: an alias for `Reg<PERIPH_ID_0_SPEC>`"]
pub type PERIPH_ID_0 = crate::Reg<periph_id_0::PERIPH_ID_0_SPEC>;
#[doc = "Peripheral identification 0"]
pub mod periph_id_0;
#[doc = "PERIPH_ID_1 register accessor: an alias for `Reg<PERIPH_ID_1_SPEC>`"]
pub type PERIPH_ID_1 = crate::Reg<periph_id_1::PERIPH_ID_1_SPEC>;
#[doc = "Peripheral identification 1"]
pub mod periph_id_1;
#[doc = "PERIPH_ID_2 register accessor: an alias for `Reg<PERIPH_ID_2_SPEC>`"]
pub type PERIPH_ID_2 = crate::Reg<periph_id_2::PERIPH_ID_2_SPEC>;
#[doc = "Peripheral identification 2"]
pub mod periph_id_2;
#[doc = "PERIPH_ID_3 register accessor: an alias for `Reg<PERIPH_ID_3_SPEC>`"]
pub type PERIPH_ID_3 = crate::Reg<periph_id_3::PERIPH_ID_3_SPEC>;
#[doc = "Peripheral identification 3"]
pub mod periph_id_3;
#[doc = "PCELL_ID_0 register accessor: an alias for `Reg<PCELL_ID_0_SPEC>`"]
pub type PCELL_ID_0 = crate::Reg<pcell_id_0::PCELL_ID_0_SPEC>;
#[doc = "PrimeCell identification 0"]
pub mod pcell_id_0;
#[doc = "PCELL_ID_1 register accessor: an alias for `Reg<PCELL_ID_1_SPEC>`"]
pub type PCELL_ID_1 = crate::Reg<pcell_id_1::PCELL_ID_1_SPEC>;
#[doc = "PrimeCell identification 1"]
pub mod pcell_id_1;
#[doc = "PCELL_ID_2 register accessor: an alias for `Reg<PCELL_ID_2_SPEC>`"]
pub type PCELL_ID_2 = crate::Reg<pcell_id_2::PCELL_ID_2_SPEC>;
#[doc = "PrimeCell identification 2"]
pub mod pcell_id_2;
#[doc = "PCELL_ID_3 register accessor: an alias for `Reg<PCELL_ID_3_SPEC>`"]
pub type PCELL_ID_3 = crate::Reg<pcell_id_3::PCELL_ID_3_SPEC>;
#[doc = "PrimeCell identification 3"]
pub mod pcell_id_3;
