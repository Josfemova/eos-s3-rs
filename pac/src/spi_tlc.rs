#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Device ID, Read Only. 0x21, the Protocol of accessing this SFR is different, See Device ID read Page for detail"]
    pub deviceidbyte: crate::Reg<deviceidbyte::DEVICEIDBYTE_SPEC>,
    _reserved1: [u8; 0x08],
    #[doc = "0x09 - PF Bank FIFO 0 Read Port"]
    pub cm_fifo_0_data: crate::Reg<cm_fifo_0_data::CM_FIFO_0_DATA_SPEC>,
    #[doc = "0x0a - PF Bank FIFO 1 Read Port"]
    pub cm_fifo_1_data: crate::Reg<cm_fifo_1_data::CM_FIFO_1_DATA_SPEC>,
    #[doc = "0x0b - PF Bank FIFO 2 Read Port"]
    pub cm_fifo_2_data: crate::Reg<cm_fifo_2_data::CM_FIFO_2_DATA_SPEC>,
    #[doc = "0x0c - PF Bank FIFO 3 Read Port"]
    pub cm_fifo_3_data: crate::Reg<cm_fifo_3_data::CM_FIFO_3_DATA_SPEC>,
    _reserved5: [u8; 0x13],
    #[doc = "0x20 - Memory Address, It is representing AHB Byte Address bit \\[7:0\\]. \n Bit 7:0 R/W, Default All 0 \n Bit \\[1:0\\]
will be ignored since only Support Word Access"]
    pub mem_addr_byte0: crate::Reg<mem_addr_byte0::MEMADDRBYTE0_SPEC>,
    #[doc = "0x21 - AHB Memory Address, It is representing AHB Byte Address bit \\[15:8\\]. Bit 7:0 R/W, Default All 0 \n Once write to this SFR, an AHB memory Read Could be Trigger. See 'TLC AHB Memory Read Trigger' worksheet for detail."]
    pub mem_addr_byte1: crate::Reg<mem_addr_byte1::MEMADDRBYTE1_SPEC>,
    #[doc = "0x22 - AHB Memory Address, It is representing AHB Byte Address bit \\[23:16\\]. Bit 7:0 R/W, Default All 0"]
    pub mem_addr_byte2: crate::Reg<mem_addr_byte2::MEMADDRBYTE2_SPEC>,
    #[doc = "0x23 - AHB Memory Address, It is representing AHB Byte Address bit \\[31:24\\]. Bit 7:0 R/W,Default All 0"]
    pub mem_addr_byte3: crate::Reg<mem_addr_byte3::MEMADDRBYTE3_SPEC>,
    _reserved9: [u8; 0x04],
    #[doc = "0x28 - First Byte (LSB) of AHB memory data"]
    pub mem_data_byte0: crate::Reg<mem_data_byte0::MEMDATABYTE0_SPEC>,
    #[doc = "0x29 - Second byte of AHB memory data"]
    pub mem_data_byte1: crate::Reg<mem_data_byte1::MEMDATABYTE1_SPEC>,
    #[doc = "0x2a - Third byte of AHB memory data"]
    pub mem_data_byte2: crate::Reg<mem_data_byte2::MEMDATABYTE2_SPEC>,
    #[doc = "0x2b - Forth byte of AHB memory data, once write to this SFR \n 1. Trigger a AHB memory Write \n 2. Auto Increment the AHB memory address (MemAddrByte0/MemAddrByte1) by 4 since AHB memory Address is in Byte Granunality. (offset 0x20~0x21, 64KB range)"]
    pub mem_data_byte3: crate::Reg<mem_data_byte3::MEMDATABYTE3_SPEC>,
    _reserved13: [u8; 0x03],
    #[doc = "0x2f - AHB status register"]
    pub ahbstatus: crate::Reg<ahbstatus::AHBSTATUS_SPEC>,
    #[doc = "0x30 - AHB access control register"]
    pub ahbaccessctl: crate::Reg<ahbaccessctl::AHBACCESSCTL_SPEC>,
    #[doc = "0x31 - General Purpose Registers, R/W, Default 0"]
    pub scratchbyte: crate::Reg<scratchbyte::SCRATCHBYTE_SPEC>,
    #[doc = "0x32 - ???"]
    pub tamarstatus: crate::Reg<tamarstatus::TAMARSTATUS_SPEC>,
    _reserved17: [u8; 0x03],
    #[doc = "0x36 - Set bit to clear the DMA FIFO. Firmware can only set this bit to 1 after set DmaClear bit to 1 and Program this bit to 0 after clear DmaClear bit"]
    pub dmadebugctl0: crate::Reg<dmadebugctl0::DMADEBUGCTL0_SPEC>,
    #[doc = "0x37 - Set bit to Reset the DMA engineer. Firmware needs to clear it before kick off the new DMA Transfer. Need to do a dummy Read on this SFR after program this bit"]
    pub dmadebugctl1: crate::Reg<dmadebugctl1::DMADEBUGCTL1_SPEC>,
    #[doc = "0x38 - DMA Starting Address, It is representing AHB Byte Address bit \\[7:0\\]. Bit \\[1:0\\]
will be ignored since only Support Word Access"]
    pub dmaaddr0: crate::Reg<dmaaddr0::DMAADDR0_SPEC>,
    #[doc = "0x39 - DMA Starting Address, It is representing AHB Byte Address bit \\[15:8\\]."]
    pub dmaaddr1: crate::Reg<dmaaddr1::DMAADDR1_SPEC>,
    #[doc = "0x3a - DMA Starting Address, It is representing AHB Byte Address bit \\[23:16\\]"]
    pub dmaaddr2: crate::Reg<dmaaddr2::DMAADDR2_SPEC>,
    #[doc = "0x3b - DMA Starting Address, It is representing AHB Byte Address bit \\[31:24\\]"]
    pub dmaaddr3: crate::Reg<dmaaddr3::DMAADDR3_SPEC>,
    #[doc = "0x3c - DMA transfer size indicates the number of bytes to be read out ( X ). The minimum transfer size is 4 bytes. \n Program the value for number of bytes to read minus 4 bytes ( X -4 ), into the 2 registers. \n DmaBurstSize0 register represents the BurstSize\\[7:0\\]. \n DmaBurstSize1 register represents the BurstSize\\[15:8\\]. \n Lower 2 bits are ignored by hardware, since it only supports Word Access. This means only multiples of 4 are supported. \n For example: \n To read 4 bytes, you would write: \n DmaBurstSize0 = 0, DmaBurstSize1 = 0 \n To read 8 bytes, you would write: \n DmaBurstSize0 = 4, DmaBurstSize1 = 0 \n … \n To read 256 bytes, you would write: \n DmaBurstSize0 = FC, DmaBurstSize1 = 0 \n To read 260 bytes, you would write: \n DmaBurstSize0 = 0, DmaBurstSize1 = 1 \n ... and so on, etc."]
    pub dmaburstsize0: crate::Reg<dmaburstsize0::DMABURSTSIZE0_SPEC>,
    #[doc = "0x3d - MSB Byte of DMA transfer size. it is representing BurstSize\\[15:8\\]. Max transfer size is 64KB, Once it is written, DMA will be kickoff unless DmaBurstSize0\\[1:0\\]
= 2'b10. Min. Transfer Size will be 4 Bytes once DMA is Kick Off ({DmaBurstSize0, DmaBurstSize1} == 0x0}"]
    pub dmaburstsize1: crate::Reg<dmaburstsize1::DMABURSTSIZE1_SPEC>,
    #[doc = "0x3e - For dummy write purpose."]
    pub reserved_dummy: crate::Reg<reserved_dummy::RESERVED_DUMMY_SPEC>,
    #[doc = "0x3f - DMA Status register"]
    pub dmastatus: crate::Reg<dmastatus::DMASTATUS_SPEC>,
    #[doc = "0x40 - DMA Read Data Port"]
    pub dmarddata: crate::Reg<dmarddata::DMARDDATA_SPEC>,
}
#[doc = "CM_FIFO_0_DATA register accessor: an alias for `Reg<CM_FIFO_0_DATA_SPEC>`"]
pub type CM_FIFO_0_DATA = crate::Reg<cm_fifo_0_data::CM_FIFO_0_DATA_SPEC>;
#[doc = "PF Bank FIFO 0 Read Port"]
pub mod cm_fifo_0_data;
#[doc = "CM_FIFO_1_DATA register accessor: an alias for `Reg<CM_FIFO_1_DATA_SPEC>`"]
pub type CM_FIFO_1_DATA = crate::Reg<cm_fifo_1_data::CM_FIFO_1_DATA_SPEC>;
#[doc = "PF Bank FIFO 1 Read Port"]
pub mod cm_fifo_1_data;
#[doc = "CM_FIFO_2_DATA register accessor: an alias for `Reg<CM_FIFO_2_DATA_SPEC>`"]
pub type CM_FIFO_2_DATA = crate::Reg<cm_fifo_2_data::CM_FIFO_2_DATA_SPEC>;
#[doc = "PF Bank FIFO 2 Read Port"]
pub mod cm_fifo_2_data;
#[doc = "CM_FIFO_3_DATA register accessor: an alias for `Reg<CM_FIFO_3_DATA_SPEC>`"]
pub type CM_FIFO_3_DATA = crate::Reg<cm_fifo_3_data::CM_FIFO_3_DATA_SPEC>;
#[doc = "PF Bank FIFO 3 Read Port"]
pub mod cm_fifo_3_data;
#[doc = "MemAddrByte0 register accessor: an alias for `Reg<MEMADDRBYTE0_SPEC>`"]
pub type MEMADDRBYTE0 = crate::Reg<mem_addr_byte0::MEMADDRBYTE0_SPEC>;
#[doc = "Memory Address, It is representing AHB Byte Address bit \\[7:0\\]. \n Bit 7:0 R/W, Default All 0 \n Bit \\[1:0\\]
will be ignored since only Support Word Access"]
pub mod mem_addr_byte0;
#[doc = "MemAddrByte1 register accessor: an alias for `Reg<MEMADDRBYTE1_SPEC>`"]
pub type MEMADDRBYTE1 = crate::Reg<mem_addr_byte1::MEMADDRBYTE1_SPEC>;
#[doc = "AHB Memory Address, It is representing AHB Byte Address bit \\[15:8\\]. Bit 7:0 R/W, Default All 0 \n Once write to this SFR, an AHB memory Read Could be Trigger. See 'TLC AHB Memory Read Trigger' worksheet for detail."]
pub mod mem_addr_byte1;
#[doc = "MemAddrByte2 register accessor: an alias for `Reg<MEMADDRBYTE2_SPEC>`"]
pub type MEMADDRBYTE2 = crate::Reg<mem_addr_byte2::MEMADDRBYTE2_SPEC>;
#[doc = "AHB Memory Address, It is representing AHB Byte Address bit \\[23:16\\]. Bit 7:0 R/W, Default All 0"]
pub mod mem_addr_byte2;
#[doc = "MemAddrByte3 register accessor: an alias for `Reg<MEMADDRBYTE3_SPEC>`"]
pub type MEMADDRBYTE3 = crate::Reg<mem_addr_byte3::MEMADDRBYTE3_SPEC>;
#[doc = "AHB Memory Address, It is representing AHB Byte Address bit \\[31:24\\]. Bit 7:0 R/W,Default All 0"]
pub mod mem_addr_byte3;
#[doc = "MemDataByte0 register accessor: an alias for `Reg<MEMDATABYTE0_SPEC>`"]
pub type MEMDATABYTE0 = crate::Reg<mem_data_byte0::MEMDATABYTE0_SPEC>;
#[doc = "First Byte (LSB) of AHB memory data"]
pub mod mem_data_byte0;
#[doc = "MemDataByte1 register accessor: an alias for `Reg<MEMDATABYTE1_SPEC>`"]
pub type MEMDATABYTE1 = crate::Reg<mem_data_byte1::MEMDATABYTE1_SPEC>;
#[doc = "Second byte of AHB memory data"]
pub mod mem_data_byte1;
#[doc = "MemDataByte2 register accessor: an alias for `Reg<MEMDATABYTE2_SPEC>`"]
pub type MEMDATABYTE2 = crate::Reg<mem_data_byte2::MEMDATABYTE2_SPEC>;
#[doc = "Third byte of AHB memory data"]
pub mod mem_data_byte2;
#[doc = "MemDataByte3 register accessor: an alias for `Reg<MEMDATABYTE3_SPEC>`"]
pub type MEMDATABYTE3 = crate::Reg<mem_data_byte3::MEMDATABYTE3_SPEC>;
#[doc = "Forth byte of AHB memory data, once write to this SFR \n 1. Trigger a AHB memory Write \n 2. Auto Increment the AHB memory address (MemAddrByte0/MemAddrByte1) by 4 since AHB memory Address is in Byte Granunality. (offset 0x20~0x21, 64KB range)"]
pub mod mem_data_byte3;
#[doc = "AHBSTATUS register accessor: an alias for `Reg<AHBSTATUS_SPEC>`"]
pub type AHBSTATUS = crate::Reg<ahbstatus::AHBSTATUS_SPEC>;
#[doc = "AHB status register"]
pub mod ahbstatus;
#[doc = "AHBACCESSCTL register accessor: an alias for `Reg<AHBACCESSCTL_SPEC>`"]
pub type AHBACCESSCTL = crate::Reg<ahbaccessctl::AHBACCESSCTL_SPEC>;
#[doc = "AHB access control register"]
pub mod ahbaccessctl;
#[doc = "SCRATCHBYTE register accessor: an alias for `Reg<SCRATCHBYTE_SPEC>`"]
pub type SCRATCHBYTE = crate::Reg<scratchbyte::SCRATCHBYTE_SPEC>;
#[doc = "General Purpose Registers, R/W, Default 0"]
pub mod scratchbyte;
#[doc = "TAMARSTATUS register accessor: an alias for `Reg<TAMARSTATUS_SPEC>`"]
pub type TAMARSTATUS = crate::Reg<tamarstatus::TAMARSTATUS_SPEC>;
#[doc = "???"]
pub mod tamarstatus;
#[doc = "DMADEBUGCTL0 register accessor: an alias for `Reg<DMADEBUGCTL0_SPEC>`"]
pub type DMADEBUGCTL0 = crate::Reg<dmadebugctl0::DMADEBUGCTL0_SPEC>;
#[doc = "Set bit to clear the DMA FIFO. Firmware can only set this bit to 1 after set DmaClear bit to 1 and Program this bit to 0 after clear DmaClear bit"]
pub mod dmadebugctl0;
#[doc = "DMADEBUGCTL1 register accessor: an alias for `Reg<DMADEBUGCTL1_SPEC>`"]
pub type DMADEBUGCTL1 = crate::Reg<dmadebugctl1::DMADEBUGCTL1_SPEC>;
#[doc = "Set bit to Reset the DMA engineer. Firmware needs to clear it before kick off the new DMA Transfer. Need to do a dummy Read on this SFR after program this bit"]
pub mod dmadebugctl1;
#[doc = "DMAADDR0 register accessor: an alias for `Reg<DMAADDR0_SPEC>`"]
pub type DMAADDR0 = crate::Reg<dmaaddr0::DMAADDR0_SPEC>;
#[doc = "DMA Starting Address, It is representing AHB Byte Address bit \\[7:0\\]. Bit \\[1:0\\]
will be ignored since only Support Word Access"]
pub mod dmaaddr0;
#[doc = "DMAADDR1 register accessor: an alias for `Reg<DMAADDR1_SPEC>`"]
pub type DMAADDR1 = crate::Reg<dmaaddr1::DMAADDR1_SPEC>;
#[doc = "DMA Starting Address, It is representing AHB Byte Address bit \\[15:8\\]."]
pub mod dmaaddr1;
#[doc = "DMAADDR2 register accessor: an alias for `Reg<DMAADDR2_SPEC>`"]
pub type DMAADDR2 = crate::Reg<dmaaddr2::DMAADDR2_SPEC>;
#[doc = "DMA Starting Address, It is representing AHB Byte Address bit \\[23:16\\]"]
pub mod dmaaddr2;
#[doc = "DMAADDR3 register accessor: an alias for `Reg<DMAADDR3_SPEC>`"]
pub type DMAADDR3 = crate::Reg<dmaaddr3::DMAADDR3_SPEC>;
#[doc = "DMA Starting Address, It is representing AHB Byte Address bit \\[31:24\\]"]
pub mod dmaaddr3;
#[doc = "DMABURSTSIZE0 register accessor: an alias for `Reg<DMABURSTSIZE0_SPEC>`"]
pub type DMABURSTSIZE0 = crate::Reg<dmaburstsize0::DMABURSTSIZE0_SPEC>;
#[doc = "DMA transfer size indicates the number of bytes to be read out ( X ). The minimum transfer size is 4 bytes. \n Program the value for number of bytes to read minus 4 bytes ( X -4 ), into the 2 registers. \n DmaBurstSize0 register represents the BurstSize\\[7:0\\]. \n DmaBurstSize1 register represents the BurstSize\\[15:8\\]. \n Lower 2 bits are ignored by hardware, since it only supports Word Access. This means only multiples of 4 are supported. \n For example: \n To read 4 bytes, you would write: \n DmaBurstSize0 = 0, DmaBurstSize1 = 0 \n To read 8 bytes, you would write: \n DmaBurstSize0 = 4, DmaBurstSize1 = 0 \n … \n To read 256 bytes, you would write: \n DmaBurstSize0 = FC, DmaBurstSize1 = 0 \n To read 260 bytes, you would write: \n DmaBurstSize0 = 0, DmaBurstSize1 = 1 \n ... and so on, etc."]
pub mod dmaburstsize0;
#[doc = "DMABURSTSIZE1 register accessor: an alias for `Reg<DMABURSTSIZE1_SPEC>`"]
pub type DMABURSTSIZE1 = crate::Reg<dmaburstsize1::DMABURSTSIZE1_SPEC>;
#[doc = "MSB Byte of DMA transfer size. it is representing BurstSize\\[15:8\\]. Max transfer size is 64KB, Once it is written, DMA will be kickoff unless DmaBurstSize0\\[1:0\\]
= 2'b10. Min. Transfer Size will be 4 Bytes once DMA is Kick Off ({DmaBurstSize0, DmaBurstSize1} == 0x0}"]
pub mod dmaburstsize1;
#[doc = "Reserved_dummy register accessor: an alias for `Reg<RESERVED_DUMMY_SPEC>`"]
pub type RESERVED_DUMMY = crate::Reg<reserved_dummy::RESERVED_DUMMY_SPEC>;
#[doc = "For dummy write purpose."]
pub mod reserved_dummy;
#[doc = "DMASTATUS register accessor: an alias for `Reg<DMASTATUS_SPEC>`"]
pub type DMASTATUS = crate::Reg<dmastatus::DMASTATUS_SPEC>;
#[doc = "DMA Status register"]
pub mod dmastatus;
#[doc = "DMARDDATA register accessor: an alias for `Reg<DMARDDATA_SPEC>`"]
pub type DMARDDATA = crate::Reg<dmarddata::DMARDDATA_SPEC>;
#[doc = "DMA Read Data Port"]
pub mod dmarddata;
#[doc = "DEVICEIDBYTE register accessor: an alias for `Reg<DEVICEIDBYTE_SPEC>`"]
pub type DEVICEIDBYTE = crate::Reg<deviceidbyte::DEVICEIDBYTE_SPEC>;
#[doc = "Device ID, Read Only. 0x21, the Protocol of accessing this SFR is different, See Device ID read Page for detail"]
pub mod deviceidbyte;
