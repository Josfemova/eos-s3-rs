#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Primary pointer to the end address of the source data of channel 0"]
    pub src_data_end_ptr_ch0:
        crate::Reg<src_data_end_ptr_ch0::SRC_DATA_END_PTR_CH0_SPEC>,
    #[doc = "0x04 - Primary pointer to the end address of the destination data of channel 0"]
    pub dst_data_end_ptr_ch0:
        crate::Reg<dst_data_end_ptr_ch0::DST_DATA_END_PTR_CH0_SPEC>,
    #[doc = "0x08 - Primary configuration for channel 0"]
    pub ch_cfg_ch0: crate::Reg<ch_cfg_ch0::CH_CFG_CH0_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - Primary pointer to the end address of the source data of channel 1"]
    pub src_data_end_ptr_ch1:
        crate::Reg<src_data_end_ptr_ch1::SRC_DATA_END_PTR_CH1_SPEC>,
    #[doc = "0x14 - Primary pointer to the end address of the destination data of channel 1"]
    pub dst_data_end_ptr_ch1:
        crate::Reg<dst_data_end_ptr_ch1::DST_DATA_END_PTR_CH1_SPEC>,
    #[doc = "0x18 - Primary configuration for channel 1"]
    pub ch_cfg_ch1: crate::Reg<ch_cfg_ch1::CH_CFG_CH1_SPEC>,
    _reserved6: [u8; 0x04],
    #[doc = "0x20 - Primary pointer to the end address of the source data of channel 2"]
    pub src_data_end_ptr_ch2:
        crate::Reg<src_data_end_ptr_ch2::SRC_DATA_END_PTR_CH2_SPEC>,
    #[doc = "0x24 - Primary pointer to the end address of the destination data of channel 2"]
    pub dst_data_end_ptr_ch2:
        crate::Reg<dst_data_end_ptr_ch2::DST_DATA_END_PTR_CH2_SPEC>,
    #[doc = "0x28 - Primary configuration for channel 2"]
    pub ch_cfg_ch2: crate::Reg<ch_cfg_ch2::CH_CFG_CH2_SPEC>,
    _reserved9: [u8; 0x04],
    #[doc = "0x30 - Primary pointer to the end address of the source data of channel 3"]
    pub src_data_end_ptr_ch3:
        crate::Reg<src_data_end_ptr_ch3::SRC_DATA_END_PTR_CH3_SPEC>,
    #[doc = "0x34 - Primary pointer to the end address of the destination data of channel 3"]
    pub dst_data_end_ptr_ch3:
        crate::Reg<dst_data_end_ptr_ch3::DST_DATA_END_PTR_CH3_SPEC>,
    #[doc = "0x38 - Primary configuration for channel 3"]
    pub ch_cfg_ch3: crate::Reg<ch_cfg_ch3::CH_CFG_CH3_SPEC>,
    _reserved12: [u8; 0x04],
    #[doc = "0x40 - Primary pointer to the end address of the source data of channel 4"]
    pub src_data_end_ptr_ch4:
        crate::Reg<src_data_end_ptr_ch4::SRC_DATA_END_PTR_CH4_SPEC>,
    #[doc = "0x44 - Primary pointer to the end address of the destination data of channel 4"]
    pub dst_data_end_ptr_ch4:
        crate::Reg<dst_data_end_ptr_ch4::DST_DATA_END_PTR_CH4_SPEC>,
    #[doc = "0x48 - Primary configuration for channel 4"]
    pub ch_cfg_ch4: crate::Reg<ch_cfg_ch4::CH_CFG_CH4_SPEC>,
    _reserved15: [u8; 0x04],
    #[doc = "0x50 - Primary pointer to the end address of the source data of channel 5"]
    pub src_data_end_ptr_ch5:
        crate::Reg<src_data_end_ptr_ch5::SRC_DATA_END_PTR_CH5_SPEC>,
    #[doc = "0x54 - Primary pointer to the end address of the destination data of channel 5"]
    pub dst_data_end_ptr_ch5:
        crate::Reg<dst_data_end_ptr_ch5::DST_DATA_END_PTR_CH5_SPEC>,
    #[doc = "0x58 - Primary configuration for channel 5"]
    pub ch_cfg_ch5: crate::Reg<ch_cfg_ch5::CH_CFG_CH5_SPEC>,
    _reserved18: [u8; 0x04],
    #[doc = "0x60 - Primary pointer to the end address of the source data of channel 6"]
    pub src_data_end_ptr_ch6:
        crate::Reg<src_data_end_ptr_ch6::SRC_DATA_END_PTR_CH6_SPEC>,
    #[doc = "0x64 - Primary pointer to the end address of the destination data of channel 6"]
    pub dst_data_end_ptr_ch6:
        crate::Reg<dst_data_end_ptr_ch6::DST_DATA_END_PTR_CH6_SPEC>,
    #[doc = "0x68 - Primary configuration for channel 6"]
    pub ch_cfg_ch6: crate::Reg<ch_cfg_ch6::CH_CFG_CH6_SPEC>,
    _reserved21: [u8; 0x04],
    #[doc = "0x70 - Primary pointer to the end address of the source data of channel 7"]
    pub src_data_end_ptr_ch7:
        crate::Reg<src_data_end_ptr_ch7::SRC_DATA_END_PTR_CH7_SPEC>,
    #[doc = "0x74 - Primary pointer to the end address of the destination data of channel 7"]
    pub dst_data_end_ptr_ch7:
        crate::Reg<dst_data_end_ptr_ch7::DST_DATA_END_PTR_CH7_SPEC>,
    #[doc = "0x78 - Primary configuration for channel 7"]
    pub ch_cfg_ch7: crate::Reg<ch_cfg_ch7::CH_CFG_CH7_SPEC>,
    _reserved24: [u8; 0x04],
    #[doc = "0x80 - Primary pointer to the end address of the source data of channel 8"]
    pub src_data_end_ptr_ch8:
        crate::Reg<src_data_end_ptr_ch8::SRC_DATA_END_PTR_CH8_SPEC>,
    #[doc = "0x84 - Primary pointer to the end address of the destination data of channel 8"]
    pub dst_data_end_ptr_ch8:
        crate::Reg<dst_data_end_ptr_ch8::DST_DATA_END_PTR_CH8_SPEC>,
    #[doc = "0x88 - Primary configuration for channel 8"]
    pub ch_cfg_ch8: crate::Reg<ch_cfg_ch8::CH_CFG_CH8_SPEC>,
    _reserved27: [u8; 0x04],
    #[doc = "0x90 - Primary pointer to the end address of the source data of channel 9"]
    pub src_data_end_ptr_ch9:
        crate::Reg<src_data_end_ptr_ch9::SRC_DATA_END_PTR_CH9_SPEC>,
    #[doc = "0x94 - Primary pointer to the end address of the destination data of channel 9"]
    pub dst_data_end_ptr_ch9:
        crate::Reg<dst_data_end_ptr_ch9::DST_DATA_END_PTR_CH9_SPEC>,
    #[doc = "0x98 - Primary configuration for channel 9"]
    pub ch_cfg_ch9: crate::Reg<ch_cfg_ch9::CH_CFG_CH9_SPEC>,
    _reserved30: [u8; 0x04],
    #[doc = "0xa0 - Primary pointer to the end address of the source data of channel 10"]
    pub src_data_end_ptr_ch10:
        crate::Reg<src_data_end_ptr_ch10::SRC_DATA_END_PTR_CH10_SPEC>,
    #[doc = "0xa4 - Primary pointer to the end address of the destination data of channel 10"]
    pub dst_data_end_ptr_ch10:
        crate::Reg<dst_data_end_ptr_ch10::DST_DATA_END_PTR_CH10_SPEC>,
    #[doc = "0xa8 - Primary configuration for channel 10"]
    pub ch_cfg_ch10: crate::Reg<ch_cfg_ch10::CH_CFG_CH10_SPEC>,
    _reserved33: [u8; 0x04],
    #[doc = "0xb0 - Primary pointer to the end address of the source data of channel 11"]
    pub src_data_end_ptr_ch11:
        crate::Reg<src_data_end_ptr_ch11::SRC_DATA_END_PTR_CH11_SPEC>,
    #[doc = "0xb4 - Primary pointer to the end address of the destination data of channel 11"]
    pub dst_data_end_ptr_ch11:
        crate::Reg<dst_data_end_ptr_ch11::DST_DATA_END_PTR_CH11_SPEC>,
    #[doc = "0xb8 - Primary configuration for channel 11"]
    pub ch_cfg_ch11: crate::Reg<ch_cfg_ch11::CH_CFG_CH11_SPEC>,
    _reserved36: [u8; 0x04],
    #[doc = "0xc0 - Primary pointer to the end address of the source data of channel 12"]
    pub src_data_end_ptr_ch12:
        crate::Reg<src_data_end_ptr_ch12::SRC_DATA_END_PTR_CH12_SPEC>,
    #[doc = "0xc4 - Primary pointer to the end address of the destination data of channel 12"]
    pub dst_data_end_ptr_ch12:
        crate::Reg<dst_data_end_ptr_ch12::DST_DATA_END_PTR_CH12_SPEC>,
    #[doc = "0xc8 - Primary configuration for channel 12"]
    pub ch_cfg_ch12: crate::Reg<ch_cfg_ch12::CH_CFG_CH12_SPEC>,
    _reserved39: [u8; 0x04],
    #[doc = "0xd0 - Primary pointer to the end address of the source data of channel 13"]
    pub src_data_end_ptr_ch13:
        crate::Reg<src_data_end_ptr_ch13::SRC_DATA_END_PTR_CH13_SPEC>,
    #[doc = "0xd4 - Primary pointer to the end address of the destination data of channel 13"]
    pub dst_data_end_ptr_ch13:
        crate::Reg<dst_data_end_ptr_ch13::DST_DATA_END_PTR_CH13_SPEC>,
    #[doc = "0xd8 - Primary configuration for channel 13"]
    pub ch_cfg_ch13: crate::Reg<ch_cfg_ch13::CH_CFG_CH13_SPEC>,
    _reserved42: [u8; 0x04],
    #[doc = "0xe0 - Primary pointer to the end address of the source data of channel 14"]
    pub src_data_end_ptr_ch14:
        crate::Reg<src_data_end_ptr_ch14::SRC_DATA_END_PTR_CH14_SPEC>,
    #[doc = "0xe4 - Primary pointer to the end address of the destination data of channel 14"]
    pub dst_data_end_ptr_ch14:
        crate::Reg<dst_data_end_ptr_ch14::DST_DATA_END_PTR_CH14_SPEC>,
    #[doc = "0xe8 - Primary configuration for channel 14"]
    pub ch_cfg_ch14: crate::Reg<ch_cfg_ch14::CH_CFG_CH14_SPEC>,
    _reserved45: [u8; 0x04],
    #[doc = "0xf0 - Primary pointer to the end address of the source data of channel 15"]
    pub src_data_end_ptr_ch15:
        crate::Reg<src_data_end_ptr_ch15::SRC_DATA_END_PTR_CH15_SPEC>,
    #[doc = "0xf4 - Primary pointer to the end address of the destination data of channel 15"]
    pub dst_data_end_ptr_ch15:
        crate::Reg<dst_data_end_ptr_ch15::DST_DATA_END_PTR_CH15_SPEC>,
    #[doc = "0xf8 - Primary configuration for channel 15"]
    pub ch_cfg_ch15: crate::Reg<ch_cfg_ch15::CH_CFG_CH15_SPEC>,
    _reserved48: [u8; 0x04],
    #[doc = "0x100 - Alternate pointer to the end address of the source data of channel 0"]
    pub alt_src_data_end_ptr_ch0:
        crate::Reg<alt_src_data_end_ptr_ch0::ALT_SRC_DATA_END_PTR_CH0_SPEC>,
    #[doc = "0x104 - Alternate pointer to the end address of the destination data of channel 0"]
    pub alt_dst_data_end_ptr_ch0:
        crate::Reg<alt_dst_data_end_ptr_ch0::ALT_DST_DATA_END_PTR_CH0_SPEC>,
    #[doc = "0x108 - Primary configuration for channel 0"]
    pub alt_chn_cfg_ch0: crate::Reg<alt_chn_cfg_ch0::ALT_CHN_CFG_CH0_SPEC>,
    _reserved51: [u8; 0x04],
    #[doc = "0x110 - Alternate pointer to the end address of the source data of channel 1"]
    pub alt_src_data_end_ptr_ch1:
        crate::Reg<alt_src_data_end_ptr_ch1::ALT_SRC_DATA_END_PTR_CH1_SPEC>,
    #[doc = "0x114 - Alternate pointer to the end address of the destination data of channel 1"]
    pub alt_dst_data_end_ptr_ch1:
        crate::Reg<alt_dst_data_end_ptr_ch1::ALT_DST_DATA_END_PTR_CH1_SPEC>,
    #[doc = "0x118 - Primary configuration for channel 1"]
    pub alt_chn_cfg_ch1: crate::Reg<alt_chn_cfg_ch1::ALT_CHN_CFG_CH1_SPEC>,
    _reserved54: [u8; 0x04],
    #[doc = "0x120 - Alternate pointer to the end address of the source data of channel 2"]
    pub alt_src_data_end_ptr_ch2:
        crate::Reg<alt_src_data_end_ptr_ch2::ALT_SRC_DATA_END_PTR_CH2_SPEC>,
    #[doc = "0x124 - Alternate pointer to the end address of the destination data of channel 2"]
    pub alt_dst_data_end_ptr_ch2:
        crate::Reg<alt_dst_data_end_ptr_ch2::ALT_DST_DATA_END_PTR_CH2_SPEC>,
    #[doc = "0x128 - Primary configuration for channel 2"]
    pub alt_chn_cfg_ch2: crate::Reg<alt_chn_cfg_ch2::ALT_CHN_CFG_CH2_SPEC>,
    _reserved57: [u8; 0x04],
    #[doc = "0x130 - Alternate pointer to the end address of the source data of channel 3"]
    pub alt_src_data_end_ptr_ch3:
        crate::Reg<alt_src_data_end_ptr_ch3::ALT_SRC_DATA_END_PTR_CH3_SPEC>,
    #[doc = "0x134 - Alternate pointer to the end address of the destination data of channel 3"]
    pub alt_dst_data_end_ptr_ch3:
        crate::Reg<alt_dst_data_end_ptr_ch3::ALT_DST_DATA_END_PTR_CH3_SPEC>,
    #[doc = "0x138 - Primary configuration for channel 3"]
    pub alt_chn_cfg_ch3: crate::Reg<alt_chn_cfg_ch3::ALT_CHN_CFG_CH3_SPEC>,
    _reserved60: [u8; 0x04],
    #[doc = "0x140 - Alternate pointer to the end address of the source data of channel 4"]
    pub alt_src_data_end_ptr_ch4:
        crate::Reg<alt_src_data_end_ptr_ch4::ALT_SRC_DATA_END_PTR_CH4_SPEC>,
    #[doc = "0x144 - Alternate pointer to the end address of the destination data of channel 4"]
    pub alt_dst_data_end_ptr_ch4:
        crate::Reg<alt_dst_data_end_ptr_ch4::ALT_DST_DATA_END_PTR_CH4_SPEC>,
    #[doc = "0x148 - Primary configuration for channel 4"]
    pub alt_chn_cfg_ch4: crate::Reg<alt_chn_cfg_ch4::ALT_CHN_CFG_CH4_SPEC>,
    _reserved63: [u8; 0x04],
    #[doc = "0x150 - Alternate pointer to the end address of the source data of channel 5"]
    pub alt_src_data_end_ptr_ch5:
        crate::Reg<alt_src_data_end_ptr_ch5::ALT_SRC_DATA_END_PTR_CH5_SPEC>,
    #[doc = "0x154 - Alternate pointer to the end address of the destination data of channel 5"]
    pub alt_dst_data_end_ptr_ch5:
        crate::Reg<alt_dst_data_end_ptr_ch5::ALT_DST_DATA_END_PTR_CH5_SPEC>,
    #[doc = "0x158 - Primary configuration for channel 5"]
    pub alt_chn_cfg_ch5: crate::Reg<alt_chn_cfg_ch5::ALT_CHN_CFG_CH5_SPEC>,
    _reserved66: [u8; 0x04],
    #[doc = "0x160 - Alternate pointer to the end address of the source data of channel 6"]
    pub alt_src_data_end_ptr_ch6:
        crate::Reg<alt_src_data_end_ptr_ch6::ALT_SRC_DATA_END_PTR_CH6_SPEC>,
    #[doc = "0x164 - Alternate pointer to the end address of the destination data of channel 6"]
    pub alt_dst_data_end_ptr_ch6:
        crate::Reg<alt_dst_data_end_ptr_ch6::ALT_DST_DATA_END_PTR_CH6_SPEC>,
    #[doc = "0x168 - Primary configuration for channel 6"]
    pub alt_chn_cfg_ch6: crate::Reg<alt_chn_cfg_ch6::ALT_CHN_CFG_CH6_SPEC>,
    _reserved69: [u8; 0x04],
    #[doc = "0x170 - Alternate pointer to the end address of the source data of channel 7"]
    pub alt_src_data_end_ptr_ch7:
        crate::Reg<alt_src_data_end_ptr_ch7::ALT_SRC_DATA_END_PTR_CH7_SPEC>,
    #[doc = "0x174 - Alternate pointer to the end address of the destination data of channel 7"]
    pub alt_dst_data_end_ptr_ch7:
        crate::Reg<alt_dst_data_end_ptr_ch7::ALT_DST_DATA_END_PTR_CH7_SPEC>,
    #[doc = "0x178 - Primary configuration for channel 7"]
    pub alt_chn_cfg_ch7: crate::Reg<alt_chn_cfg_ch7::ALT_CHN_CFG_CH7_SPEC>,
    _reserved72: [u8; 0x04],
    #[doc = "0x180 - Alternate pointer to the end address of the source data of channel 8"]
    pub alt_src_data_end_ptr_ch8:
        crate::Reg<alt_src_data_end_ptr_ch8::ALT_SRC_DATA_END_PTR_CH8_SPEC>,
    #[doc = "0x184 - Alternate pointer to the end address of the destination data of channel 8"]
    pub alt_dst_data_end_ptr_ch8:
        crate::Reg<alt_dst_data_end_ptr_ch8::ALT_DST_DATA_END_PTR_CH8_SPEC>,
    #[doc = "0x188 - Primary configuration for channel 8"]
    pub alt_chn_cfg_ch8: crate::Reg<alt_chn_cfg_ch8::ALT_CHN_CFG_CH8_SPEC>,
    _reserved75: [u8; 0x04],
    #[doc = "0x190 - Alternate pointer to the end address of the source data of channel 9"]
    pub alt_src_data_end_ptr_ch9:
        crate::Reg<alt_src_data_end_ptr_ch9::ALT_SRC_DATA_END_PTR_CH9_SPEC>,
    #[doc = "0x194 - Alternate pointer to the end address of the destination data of channel 9"]
    pub alt_dst_data_end_ptr_ch9:
        crate::Reg<alt_dst_data_end_ptr_ch9::ALT_DST_DATA_END_PTR_CH9_SPEC>,
    #[doc = "0x198 - Primary configuration for channel 9"]
    pub alt_chn_cfg_ch9: crate::Reg<alt_chn_cfg_ch9::ALT_CHN_CFG_CH9_SPEC>,
    _reserved78: [u8; 0x04],
    #[doc = "0x1a0 - Alternate pointer to the end address of the source data of channel 10"]
    pub alt_src_data_end_ptr_ch10:
        crate::Reg<alt_src_data_end_ptr_ch10::ALT_SRC_DATA_END_PTR_CH10_SPEC>,
    #[doc = "0x1a4 - Alternate pointer to the end address of the destination data of channel 10"]
    pub alt_dst_data_end_ptr_ch10:
        crate::Reg<alt_dst_data_end_ptr_ch10::ALT_DST_DATA_END_PTR_CH10_SPEC>,
    #[doc = "0x1a8 - Primary configuration for channel 10"]
    pub alt_chn_cfg_ch10: crate::Reg<alt_chn_cfg_ch10::ALT_CHN_CFG_CH10_SPEC>,
    _reserved81: [u8; 0x04],
    #[doc = "0x1b0 - Alternate pointer to the end address of the source data of channel 11"]
    pub alt_src_data_end_ptr_ch11:
        crate::Reg<alt_src_data_end_ptr_ch11::ALT_SRC_DATA_END_PTR_CH11_SPEC>,
    #[doc = "0x1b4 - Alternate pointer to the end address of the destination data of channel 11"]
    pub alt_dst_data_end_ptr_ch11:
        crate::Reg<alt_dst_data_end_ptr_ch11::ALT_DST_DATA_END_PTR_CH11_SPEC>,
    #[doc = "0x1b8 - Primary configuration for channel 11"]
    pub alt_chn_cfg_ch11: crate::Reg<alt_chn_cfg_ch11::ALT_CHN_CFG_CH11_SPEC>,
    _reserved84: [u8; 0x04],
    #[doc = "0x1c0 - Alternate pointer to the end address of the source data of channel 12"]
    pub alt_src_data_end_ptr_ch12:
        crate::Reg<alt_src_data_end_ptr_ch12::ALT_SRC_DATA_END_PTR_CH12_SPEC>,
    #[doc = "0x1c4 - Alternate pointer to the end address of the destination data of channel 12"]
    pub alt_dst_data_end_ptr_ch12:
        crate::Reg<alt_dst_data_end_ptr_ch12::ALT_DST_DATA_END_PTR_CH12_SPEC>,
    #[doc = "0x1c8 - Primary configuration for channel 12"]
    pub alt_chn_cfg_ch12: crate::Reg<alt_chn_cfg_ch12::ALT_CHN_CFG_CH12_SPEC>,
    _reserved87: [u8; 0x04],
    #[doc = "0x1d0 - Alternate pointer to the end address of the source data of channel 13"]
    pub alt_src_data_end_ptr_ch13:
        crate::Reg<alt_src_data_end_ptr_ch13::ALT_SRC_DATA_END_PTR_CH13_SPEC>,
    #[doc = "0x1d4 - Alternate pointer to the end address of the destination data of channel 13"]
    pub alt_dst_data_end_ptr_ch13:
        crate::Reg<alt_dst_data_end_ptr_ch13::ALT_DST_DATA_END_PTR_CH13_SPEC>,
    #[doc = "0x1d8 - Primary configuration for channel 13"]
    pub alt_chn_cfg_ch13: crate::Reg<alt_chn_cfg_ch13::ALT_CHN_CFG_CH13_SPEC>,
    _reserved90: [u8; 0x04],
    #[doc = "0x1e0 - Alternate pointer to the end address of the source data of channel 14"]
    pub alt_src_data_end_ptr_ch14:
        crate::Reg<alt_src_data_end_ptr_ch14::ALT_SRC_DATA_END_PTR_CH14_SPEC>,
    #[doc = "0x1e4 - Alternate pointer to the end address of the destination data of channel 14"]
    pub alt_dst_data_end_ptr_ch14:
        crate::Reg<alt_dst_data_end_ptr_ch14::ALT_DST_DATA_END_PTR_CH14_SPEC>,
    #[doc = "0x1e8 - Primary configuration for channel 14"]
    pub alt_chn_cfg_ch14: crate::Reg<alt_chn_cfg_ch14::ALT_CHN_CFG_CH14_SPEC>,
    _reserved93: [u8; 0x04],
    #[doc = "0x1f0 - Alternate pointer to the end address of the source data of channel 15"]
    pub alt_src_data_end_ptr_ch15:
        crate::Reg<alt_src_data_end_ptr_ch15::ALT_SRC_DATA_END_PTR_CH15_SPEC>,
    #[doc = "0x1f4 - Alternate pointer to the end address of the destination data of channel 15"]
    pub alt_dst_data_end_ptr_ch15:
        crate::Reg<alt_dst_data_end_ptr_ch15::ALT_DST_DATA_END_PTR_CH15_SPEC>,
    #[doc = "0x1f8 - Primary configuration for channel 15"]
    pub alt_chn_cfg_ch15: crate::Reg<alt_chn_cfg_ch15::ALT_CHN_CFG_CH15_SPEC>,
}
#[doc = "SRC_DATA_END_PTR_CH0 register accessor: an alias for `Reg<SRC_DATA_END_PTR_CH0_SPEC>`"]
pub type SRC_DATA_END_PTR_CH0 =
    crate::Reg<src_data_end_ptr_ch0::SRC_DATA_END_PTR_CH0_SPEC>;
#[doc = "Primary pointer to the end address of the source data of channel 0"]
pub mod src_data_end_ptr_ch0;
#[doc = "DST_DATA_END_PTR_CH0 register accessor: an alias for `Reg<DST_DATA_END_PTR_CH0_SPEC>`"]
pub type DST_DATA_END_PTR_CH0 =
    crate::Reg<dst_data_end_ptr_ch0::DST_DATA_END_PTR_CH0_SPEC>;
#[doc = "Primary pointer to the end address of the destination data of channel 0"]
pub mod dst_data_end_ptr_ch0;
#[doc = "CH_CFG_CH0 register accessor: an alias for `Reg<CH_CFG_CH0_SPEC>`"]
pub type CH_CFG_CH0 = crate::Reg<ch_cfg_ch0::CH_CFG_CH0_SPEC>;
#[doc = "Primary configuration for channel 0"]
pub mod ch_cfg_ch0;
#[doc = "SRC_DATA_END_PTR_CH1 register accessor: an alias for `Reg<SRC_DATA_END_PTR_CH1_SPEC>`"]
pub type SRC_DATA_END_PTR_CH1 =
    crate::Reg<src_data_end_ptr_ch1::SRC_DATA_END_PTR_CH1_SPEC>;
#[doc = "Primary pointer to the end address of the source data of channel 1"]
pub mod src_data_end_ptr_ch1;
#[doc = "DST_DATA_END_PTR_CH1 register accessor: an alias for `Reg<DST_DATA_END_PTR_CH1_SPEC>`"]
pub type DST_DATA_END_PTR_CH1 =
    crate::Reg<dst_data_end_ptr_ch1::DST_DATA_END_PTR_CH1_SPEC>;
#[doc = "Primary pointer to the end address of the destination data of channel 1"]
pub mod dst_data_end_ptr_ch1;
#[doc = "CH_CFG_CH1 register accessor: an alias for `Reg<CH_CFG_CH1_SPEC>`"]
pub type CH_CFG_CH1 = crate::Reg<ch_cfg_ch1::CH_CFG_CH1_SPEC>;
#[doc = "Primary configuration for channel 1"]
pub mod ch_cfg_ch1;
#[doc = "SRC_DATA_END_PTR_CH2 register accessor: an alias for `Reg<SRC_DATA_END_PTR_CH2_SPEC>`"]
pub type SRC_DATA_END_PTR_CH2 =
    crate::Reg<src_data_end_ptr_ch2::SRC_DATA_END_PTR_CH2_SPEC>;
#[doc = "Primary pointer to the end address of the source data of channel 2"]
pub mod src_data_end_ptr_ch2;
#[doc = "DST_DATA_END_PTR_CH2 register accessor: an alias for `Reg<DST_DATA_END_PTR_CH2_SPEC>`"]
pub type DST_DATA_END_PTR_CH2 =
    crate::Reg<dst_data_end_ptr_ch2::DST_DATA_END_PTR_CH2_SPEC>;
#[doc = "Primary pointer to the end address of the destination data of channel 2"]
pub mod dst_data_end_ptr_ch2;
#[doc = "CH_CFG_CH2 register accessor: an alias for `Reg<CH_CFG_CH2_SPEC>`"]
pub type CH_CFG_CH2 = crate::Reg<ch_cfg_ch2::CH_CFG_CH2_SPEC>;
#[doc = "Primary configuration for channel 2"]
pub mod ch_cfg_ch2;
#[doc = "SRC_DATA_END_PTR_CH3 register accessor: an alias for `Reg<SRC_DATA_END_PTR_CH3_SPEC>`"]
pub type SRC_DATA_END_PTR_CH3 =
    crate::Reg<src_data_end_ptr_ch3::SRC_DATA_END_PTR_CH3_SPEC>;
#[doc = "Primary pointer to the end address of the source data of channel 3"]
pub mod src_data_end_ptr_ch3;
#[doc = "DST_DATA_END_PTR_CH3 register accessor: an alias for `Reg<DST_DATA_END_PTR_CH3_SPEC>`"]
pub type DST_DATA_END_PTR_CH3 =
    crate::Reg<dst_data_end_ptr_ch3::DST_DATA_END_PTR_CH3_SPEC>;
#[doc = "Primary pointer to the end address of the destination data of channel 3"]
pub mod dst_data_end_ptr_ch3;
#[doc = "CH_CFG_CH3 register accessor: an alias for `Reg<CH_CFG_CH3_SPEC>`"]
pub type CH_CFG_CH3 = crate::Reg<ch_cfg_ch3::CH_CFG_CH3_SPEC>;
#[doc = "Primary configuration for channel 3"]
pub mod ch_cfg_ch3;
#[doc = "SRC_DATA_END_PTR_CH4 register accessor: an alias for `Reg<SRC_DATA_END_PTR_CH4_SPEC>`"]
pub type SRC_DATA_END_PTR_CH4 =
    crate::Reg<src_data_end_ptr_ch4::SRC_DATA_END_PTR_CH4_SPEC>;
#[doc = "Primary pointer to the end address of the source data of channel 4"]
pub mod src_data_end_ptr_ch4;
#[doc = "DST_DATA_END_PTR_CH4 register accessor: an alias for `Reg<DST_DATA_END_PTR_CH4_SPEC>`"]
pub type DST_DATA_END_PTR_CH4 =
    crate::Reg<dst_data_end_ptr_ch4::DST_DATA_END_PTR_CH4_SPEC>;
#[doc = "Primary pointer to the end address of the destination data of channel 4"]
pub mod dst_data_end_ptr_ch4;
#[doc = "CH_CFG_CH4 register accessor: an alias for `Reg<CH_CFG_CH4_SPEC>`"]
pub type CH_CFG_CH4 = crate::Reg<ch_cfg_ch4::CH_CFG_CH4_SPEC>;
#[doc = "Primary configuration for channel 4"]
pub mod ch_cfg_ch4;
#[doc = "SRC_DATA_END_PTR_CH5 register accessor: an alias for `Reg<SRC_DATA_END_PTR_CH5_SPEC>`"]
pub type SRC_DATA_END_PTR_CH5 =
    crate::Reg<src_data_end_ptr_ch5::SRC_DATA_END_PTR_CH5_SPEC>;
#[doc = "Primary pointer to the end address of the source data of channel 5"]
pub mod src_data_end_ptr_ch5;
#[doc = "DST_DATA_END_PTR_CH5 register accessor: an alias for `Reg<DST_DATA_END_PTR_CH5_SPEC>`"]
pub type DST_DATA_END_PTR_CH5 =
    crate::Reg<dst_data_end_ptr_ch5::DST_DATA_END_PTR_CH5_SPEC>;
#[doc = "Primary pointer to the end address of the destination data of channel 5"]
pub mod dst_data_end_ptr_ch5;
#[doc = "CH_CFG_CH5 register accessor: an alias for `Reg<CH_CFG_CH5_SPEC>`"]
pub type CH_CFG_CH5 = crate::Reg<ch_cfg_ch5::CH_CFG_CH5_SPEC>;
#[doc = "Primary configuration for channel 5"]
pub mod ch_cfg_ch5;
#[doc = "SRC_DATA_END_PTR_CH6 register accessor: an alias for `Reg<SRC_DATA_END_PTR_CH6_SPEC>`"]
pub type SRC_DATA_END_PTR_CH6 =
    crate::Reg<src_data_end_ptr_ch6::SRC_DATA_END_PTR_CH6_SPEC>;
#[doc = "Primary pointer to the end address of the source data of channel 6"]
pub mod src_data_end_ptr_ch6;
#[doc = "DST_DATA_END_PTR_CH6 register accessor: an alias for `Reg<DST_DATA_END_PTR_CH6_SPEC>`"]
pub type DST_DATA_END_PTR_CH6 =
    crate::Reg<dst_data_end_ptr_ch6::DST_DATA_END_PTR_CH6_SPEC>;
#[doc = "Primary pointer to the end address of the destination data of channel 6"]
pub mod dst_data_end_ptr_ch6;
#[doc = "CH_CFG_CH6 register accessor: an alias for `Reg<CH_CFG_CH6_SPEC>`"]
pub type CH_CFG_CH6 = crate::Reg<ch_cfg_ch6::CH_CFG_CH6_SPEC>;
#[doc = "Primary configuration for channel 6"]
pub mod ch_cfg_ch6;
#[doc = "SRC_DATA_END_PTR_CH7 register accessor: an alias for `Reg<SRC_DATA_END_PTR_CH7_SPEC>`"]
pub type SRC_DATA_END_PTR_CH7 =
    crate::Reg<src_data_end_ptr_ch7::SRC_DATA_END_PTR_CH7_SPEC>;
#[doc = "Primary pointer to the end address of the source data of channel 7"]
pub mod src_data_end_ptr_ch7;
#[doc = "DST_DATA_END_PTR_CH7 register accessor: an alias for `Reg<DST_DATA_END_PTR_CH7_SPEC>`"]
pub type DST_DATA_END_PTR_CH7 =
    crate::Reg<dst_data_end_ptr_ch7::DST_DATA_END_PTR_CH7_SPEC>;
#[doc = "Primary pointer to the end address of the destination data of channel 7"]
pub mod dst_data_end_ptr_ch7;
#[doc = "CH_CFG_CH7 register accessor: an alias for `Reg<CH_CFG_CH7_SPEC>`"]
pub type CH_CFG_CH7 = crate::Reg<ch_cfg_ch7::CH_CFG_CH7_SPEC>;
#[doc = "Primary configuration for channel 7"]
pub mod ch_cfg_ch7;
#[doc = "SRC_DATA_END_PTR_CH8 register accessor: an alias for `Reg<SRC_DATA_END_PTR_CH8_SPEC>`"]
pub type SRC_DATA_END_PTR_CH8 =
    crate::Reg<src_data_end_ptr_ch8::SRC_DATA_END_PTR_CH8_SPEC>;
#[doc = "Primary pointer to the end address of the source data of channel 8"]
pub mod src_data_end_ptr_ch8;
#[doc = "DST_DATA_END_PTR_CH8 register accessor: an alias for `Reg<DST_DATA_END_PTR_CH8_SPEC>`"]
pub type DST_DATA_END_PTR_CH8 =
    crate::Reg<dst_data_end_ptr_ch8::DST_DATA_END_PTR_CH8_SPEC>;
#[doc = "Primary pointer to the end address of the destination data of channel 8"]
pub mod dst_data_end_ptr_ch8;
#[doc = "CH_CFG_CH8 register accessor: an alias for `Reg<CH_CFG_CH8_SPEC>`"]
pub type CH_CFG_CH8 = crate::Reg<ch_cfg_ch8::CH_CFG_CH8_SPEC>;
#[doc = "Primary configuration for channel 8"]
pub mod ch_cfg_ch8;
#[doc = "SRC_DATA_END_PTR_CH9 register accessor: an alias for `Reg<SRC_DATA_END_PTR_CH9_SPEC>`"]
pub type SRC_DATA_END_PTR_CH9 =
    crate::Reg<src_data_end_ptr_ch9::SRC_DATA_END_PTR_CH9_SPEC>;
#[doc = "Primary pointer to the end address of the source data of channel 9"]
pub mod src_data_end_ptr_ch9;
#[doc = "DST_DATA_END_PTR_CH9 register accessor: an alias for `Reg<DST_DATA_END_PTR_CH9_SPEC>`"]
pub type DST_DATA_END_PTR_CH9 =
    crate::Reg<dst_data_end_ptr_ch9::DST_DATA_END_PTR_CH9_SPEC>;
#[doc = "Primary pointer to the end address of the destination data of channel 9"]
pub mod dst_data_end_ptr_ch9;
#[doc = "CH_CFG_CH9 register accessor: an alias for `Reg<CH_CFG_CH9_SPEC>`"]
pub type CH_CFG_CH9 = crate::Reg<ch_cfg_ch9::CH_CFG_CH9_SPEC>;
#[doc = "Primary configuration for channel 9"]
pub mod ch_cfg_ch9;
#[doc = "SRC_DATA_END_PTR_CH10 register accessor: an alias for `Reg<SRC_DATA_END_PTR_CH10_SPEC>`"]
pub type SRC_DATA_END_PTR_CH10 =
    crate::Reg<src_data_end_ptr_ch10::SRC_DATA_END_PTR_CH10_SPEC>;
#[doc = "Primary pointer to the end address of the source data of channel 10"]
pub mod src_data_end_ptr_ch10;
#[doc = "DST_DATA_END_PTR_CH10 register accessor: an alias for `Reg<DST_DATA_END_PTR_CH10_SPEC>`"]
pub type DST_DATA_END_PTR_CH10 =
    crate::Reg<dst_data_end_ptr_ch10::DST_DATA_END_PTR_CH10_SPEC>;
#[doc = "Primary pointer to the end address of the destination data of channel 10"]
pub mod dst_data_end_ptr_ch10;
#[doc = "CH_CFG_CH10 register accessor: an alias for `Reg<CH_CFG_CH10_SPEC>`"]
pub type CH_CFG_CH10 = crate::Reg<ch_cfg_ch10::CH_CFG_CH10_SPEC>;
#[doc = "Primary configuration for channel 10"]
pub mod ch_cfg_ch10;
#[doc = "SRC_DATA_END_PTR_CH11 register accessor: an alias for `Reg<SRC_DATA_END_PTR_CH11_SPEC>`"]
pub type SRC_DATA_END_PTR_CH11 =
    crate::Reg<src_data_end_ptr_ch11::SRC_DATA_END_PTR_CH11_SPEC>;
#[doc = "Primary pointer to the end address of the source data of channel 11"]
pub mod src_data_end_ptr_ch11;
#[doc = "DST_DATA_END_PTR_CH11 register accessor: an alias for `Reg<DST_DATA_END_PTR_CH11_SPEC>`"]
pub type DST_DATA_END_PTR_CH11 =
    crate::Reg<dst_data_end_ptr_ch11::DST_DATA_END_PTR_CH11_SPEC>;
#[doc = "Primary pointer to the end address of the destination data of channel 11"]
pub mod dst_data_end_ptr_ch11;
#[doc = "CH_CFG_CH11 register accessor: an alias for `Reg<CH_CFG_CH11_SPEC>`"]
pub type CH_CFG_CH11 = crate::Reg<ch_cfg_ch11::CH_CFG_CH11_SPEC>;
#[doc = "Primary configuration for channel 11"]
pub mod ch_cfg_ch11;
#[doc = "SRC_DATA_END_PTR_CH12 register accessor: an alias for `Reg<SRC_DATA_END_PTR_CH12_SPEC>`"]
pub type SRC_DATA_END_PTR_CH12 =
    crate::Reg<src_data_end_ptr_ch12::SRC_DATA_END_PTR_CH12_SPEC>;
#[doc = "Primary pointer to the end address of the source data of channel 12"]
pub mod src_data_end_ptr_ch12;
#[doc = "DST_DATA_END_PTR_CH12 register accessor: an alias for `Reg<DST_DATA_END_PTR_CH12_SPEC>`"]
pub type DST_DATA_END_PTR_CH12 =
    crate::Reg<dst_data_end_ptr_ch12::DST_DATA_END_PTR_CH12_SPEC>;
#[doc = "Primary pointer to the end address of the destination data of channel 12"]
pub mod dst_data_end_ptr_ch12;
#[doc = "CH_CFG_CH12 register accessor: an alias for `Reg<CH_CFG_CH12_SPEC>`"]
pub type CH_CFG_CH12 = crate::Reg<ch_cfg_ch12::CH_CFG_CH12_SPEC>;
#[doc = "Primary configuration for channel 12"]
pub mod ch_cfg_ch12;
#[doc = "SRC_DATA_END_PTR_CH13 register accessor: an alias for `Reg<SRC_DATA_END_PTR_CH13_SPEC>`"]
pub type SRC_DATA_END_PTR_CH13 =
    crate::Reg<src_data_end_ptr_ch13::SRC_DATA_END_PTR_CH13_SPEC>;
#[doc = "Primary pointer to the end address of the source data of channel 13"]
pub mod src_data_end_ptr_ch13;
#[doc = "DST_DATA_END_PTR_CH13 register accessor: an alias for `Reg<DST_DATA_END_PTR_CH13_SPEC>`"]
pub type DST_DATA_END_PTR_CH13 =
    crate::Reg<dst_data_end_ptr_ch13::DST_DATA_END_PTR_CH13_SPEC>;
#[doc = "Primary pointer to the end address of the destination data of channel 13"]
pub mod dst_data_end_ptr_ch13;
#[doc = "CH_CFG_CH13 register accessor: an alias for `Reg<CH_CFG_CH13_SPEC>`"]
pub type CH_CFG_CH13 = crate::Reg<ch_cfg_ch13::CH_CFG_CH13_SPEC>;
#[doc = "Primary configuration for channel 13"]
pub mod ch_cfg_ch13;
#[doc = "SRC_DATA_END_PTR_CH14 register accessor: an alias for `Reg<SRC_DATA_END_PTR_CH14_SPEC>`"]
pub type SRC_DATA_END_PTR_CH14 =
    crate::Reg<src_data_end_ptr_ch14::SRC_DATA_END_PTR_CH14_SPEC>;
#[doc = "Primary pointer to the end address of the source data of channel 14"]
pub mod src_data_end_ptr_ch14;
#[doc = "DST_DATA_END_PTR_CH14 register accessor: an alias for `Reg<DST_DATA_END_PTR_CH14_SPEC>`"]
pub type DST_DATA_END_PTR_CH14 =
    crate::Reg<dst_data_end_ptr_ch14::DST_DATA_END_PTR_CH14_SPEC>;
#[doc = "Primary pointer to the end address of the destination data of channel 14"]
pub mod dst_data_end_ptr_ch14;
#[doc = "CH_CFG_CH14 register accessor: an alias for `Reg<CH_CFG_CH14_SPEC>`"]
pub type CH_CFG_CH14 = crate::Reg<ch_cfg_ch14::CH_CFG_CH14_SPEC>;
#[doc = "Primary configuration for channel 14"]
pub mod ch_cfg_ch14;
#[doc = "SRC_DATA_END_PTR_CH15 register accessor: an alias for `Reg<SRC_DATA_END_PTR_CH15_SPEC>`"]
pub type SRC_DATA_END_PTR_CH15 =
    crate::Reg<src_data_end_ptr_ch15::SRC_DATA_END_PTR_CH15_SPEC>;
#[doc = "Primary pointer to the end address of the source data of channel 15"]
pub mod src_data_end_ptr_ch15;
#[doc = "DST_DATA_END_PTR_CH15 register accessor: an alias for `Reg<DST_DATA_END_PTR_CH15_SPEC>`"]
pub type DST_DATA_END_PTR_CH15 =
    crate::Reg<dst_data_end_ptr_ch15::DST_DATA_END_PTR_CH15_SPEC>;
#[doc = "Primary pointer to the end address of the destination data of channel 15"]
pub mod dst_data_end_ptr_ch15;
#[doc = "CH_CFG_CH15 register accessor: an alias for `Reg<CH_CFG_CH15_SPEC>`"]
pub type CH_CFG_CH15 = crate::Reg<ch_cfg_ch15::CH_CFG_CH15_SPEC>;
#[doc = "Primary configuration for channel 15"]
pub mod ch_cfg_ch15;
#[doc = "ALT_SRC_DATA_END_PTR_CH0 register accessor: an alias for `Reg<ALT_SRC_DATA_END_PTR_CH0_SPEC>`"]
pub type ALT_SRC_DATA_END_PTR_CH0 =
    crate::Reg<alt_src_data_end_ptr_ch0::ALT_SRC_DATA_END_PTR_CH0_SPEC>;
#[doc = "Alternate pointer to the end address of the source data of channel 0"]
pub mod alt_src_data_end_ptr_ch0;
#[doc = "ALT_DST_DATA_END_PTR_CH0 register accessor: an alias for `Reg<ALT_DST_DATA_END_PTR_CH0_SPEC>`"]
pub type ALT_DST_DATA_END_PTR_CH0 =
    crate::Reg<alt_dst_data_end_ptr_ch0::ALT_DST_DATA_END_PTR_CH0_SPEC>;
#[doc = "Alternate pointer to the end address of the destination data of channel 0"]
pub mod alt_dst_data_end_ptr_ch0;
#[doc = "ALT_CHN_CFG_CH0 register accessor: an alias for `Reg<ALT_CHN_CFG_CH0_SPEC>`"]
pub type ALT_CHN_CFG_CH0 = crate::Reg<alt_chn_cfg_ch0::ALT_CHN_CFG_CH0_SPEC>;
#[doc = "Primary configuration for channel 0"]
pub mod alt_chn_cfg_ch0;
#[doc = "ALT_SRC_DATA_END_PTR_CH1 register accessor: an alias for `Reg<ALT_SRC_DATA_END_PTR_CH1_SPEC>`"]
pub type ALT_SRC_DATA_END_PTR_CH1 =
    crate::Reg<alt_src_data_end_ptr_ch1::ALT_SRC_DATA_END_PTR_CH1_SPEC>;
#[doc = "Alternate pointer to the end address of the source data of channel 1"]
pub mod alt_src_data_end_ptr_ch1;
#[doc = "ALT_DST_DATA_END_PTR_CH1 register accessor: an alias for `Reg<ALT_DST_DATA_END_PTR_CH1_SPEC>`"]
pub type ALT_DST_DATA_END_PTR_CH1 =
    crate::Reg<alt_dst_data_end_ptr_ch1::ALT_DST_DATA_END_PTR_CH1_SPEC>;
#[doc = "Alternate pointer to the end address of the destination data of channel 1"]
pub mod alt_dst_data_end_ptr_ch1;
#[doc = "ALT_CHN_CFG_CH1 register accessor: an alias for `Reg<ALT_CHN_CFG_CH1_SPEC>`"]
pub type ALT_CHN_CFG_CH1 = crate::Reg<alt_chn_cfg_ch1::ALT_CHN_CFG_CH1_SPEC>;
#[doc = "Primary configuration for channel 1"]
pub mod alt_chn_cfg_ch1;
#[doc = "ALT_SRC_DATA_END_PTR_CH2 register accessor: an alias for `Reg<ALT_SRC_DATA_END_PTR_CH2_SPEC>`"]
pub type ALT_SRC_DATA_END_PTR_CH2 =
    crate::Reg<alt_src_data_end_ptr_ch2::ALT_SRC_DATA_END_PTR_CH2_SPEC>;
#[doc = "Alternate pointer to the end address of the source data of channel 2"]
pub mod alt_src_data_end_ptr_ch2;
#[doc = "ALT_DST_DATA_END_PTR_CH2 register accessor: an alias for `Reg<ALT_DST_DATA_END_PTR_CH2_SPEC>`"]
pub type ALT_DST_DATA_END_PTR_CH2 =
    crate::Reg<alt_dst_data_end_ptr_ch2::ALT_DST_DATA_END_PTR_CH2_SPEC>;
#[doc = "Alternate pointer to the end address of the destination data of channel 2"]
pub mod alt_dst_data_end_ptr_ch2;
#[doc = "ALT_CHN_CFG_CH2 register accessor: an alias for `Reg<ALT_CHN_CFG_CH2_SPEC>`"]
pub type ALT_CHN_CFG_CH2 = crate::Reg<alt_chn_cfg_ch2::ALT_CHN_CFG_CH2_SPEC>;
#[doc = "Primary configuration for channel 2"]
pub mod alt_chn_cfg_ch2;
#[doc = "ALT_SRC_DATA_END_PTR_CH3 register accessor: an alias for `Reg<ALT_SRC_DATA_END_PTR_CH3_SPEC>`"]
pub type ALT_SRC_DATA_END_PTR_CH3 =
    crate::Reg<alt_src_data_end_ptr_ch3::ALT_SRC_DATA_END_PTR_CH3_SPEC>;
#[doc = "Alternate pointer to the end address of the source data of channel 3"]
pub mod alt_src_data_end_ptr_ch3;
#[doc = "ALT_DST_DATA_END_PTR_CH3 register accessor: an alias for `Reg<ALT_DST_DATA_END_PTR_CH3_SPEC>`"]
pub type ALT_DST_DATA_END_PTR_CH3 =
    crate::Reg<alt_dst_data_end_ptr_ch3::ALT_DST_DATA_END_PTR_CH3_SPEC>;
#[doc = "Alternate pointer to the end address of the destination data of channel 3"]
pub mod alt_dst_data_end_ptr_ch3;
#[doc = "ALT_CHN_CFG_CH3 register accessor: an alias for `Reg<ALT_CHN_CFG_CH3_SPEC>`"]
pub type ALT_CHN_CFG_CH3 = crate::Reg<alt_chn_cfg_ch3::ALT_CHN_CFG_CH3_SPEC>;
#[doc = "Primary configuration for channel 3"]
pub mod alt_chn_cfg_ch3;
#[doc = "ALT_SRC_DATA_END_PTR_CH4 register accessor: an alias for `Reg<ALT_SRC_DATA_END_PTR_CH4_SPEC>`"]
pub type ALT_SRC_DATA_END_PTR_CH4 =
    crate::Reg<alt_src_data_end_ptr_ch4::ALT_SRC_DATA_END_PTR_CH4_SPEC>;
#[doc = "Alternate pointer to the end address of the source data of channel 4"]
pub mod alt_src_data_end_ptr_ch4;
#[doc = "ALT_DST_DATA_END_PTR_CH4 register accessor: an alias for `Reg<ALT_DST_DATA_END_PTR_CH4_SPEC>`"]
pub type ALT_DST_DATA_END_PTR_CH4 =
    crate::Reg<alt_dst_data_end_ptr_ch4::ALT_DST_DATA_END_PTR_CH4_SPEC>;
#[doc = "Alternate pointer to the end address of the destination data of channel 4"]
pub mod alt_dst_data_end_ptr_ch4;
#[doc = "ALT_CHN_CFG_CH4 register accessor: an alias for `Reg<ALT_CHN_CFG_CH4_SPEC>`"]
pub type ALT_CHN_CFG_CH4 = crate::Reg<alt_chn_cfg_ch4::ALT_CHN_CFG_CH4_SPEC>;
#[doc = "Primary configuration for channel 4"]
pub mod alt_chn_cfg_ch4;
#[doc = "ALT_SRC_DATA_END_PTR_CH5 register accessor: an alias for `Reg<ALT_SRC_DATA_END_PTR_CH5_SPEC>`"]
pub type ALT_SRC_DATA_END_PTR_CH5 =
    crate::Reg<alt_src_data_end_ptr_ch5::ALT_SRC_DATA_END_PTR_CH5_SPEC>;
#[doc = "Alternate pointer to the end address of the source data of channel 5"]
pub mod alt_src_data_end_ptr_ch5;
#[doc = "ALT_DST_DATA_END_PTR_CH5 register accessor: an alias for `Reg<ALT_DST_DATA_END_PTR_CH5_SPEC>`"]
pub type ALT_DST_DATA_END_PTR_CH5 =
    crate::Reg<alt_dst_data_end_ptr_ch5::ALT_DST_DATA_END_PTR_CH5_SPEC>;
#[doc = "Alternate pointer to the end address of the destination data of channel 5"]
pub mod alt_dst_data_end_ptr_ch5;
#[doc = "ALT_CHN_CFG_CH5 register accessor: an alias for `Reg<ALT_CHN_CFG_CH5_SPEC>`"]
pub type ALT_CHN_CFG_CH5 = crate::Reg<alt_chn_cfg_ch5::ALT_CHN_CFG_CH5_SPEC>;
#[doc = "Primary configuration for channel 5"]
pub mod alt_chn_cfg_ch5;
#[doc = "ALT_SRC_DATA_END_PTR_CH6 register accessor: an alias for `Reg<ALT_SRC_DATA_END_PTR_CH6_SPEC>`"]
pub type ALT_SRC_DATA_END_PTR_CH6 =
    crate::Reg<alt_src_data_end_ptr_ch6::ALT_SRC_DATA_END_PTR_CH6_SPEC>;
#[doc = "Alternate pointer to the end address of the source data of channel 6"]
pub mod alt_src_data_end_ptr_ch6;
#[doc = "ALT_DST_DATA_END_PTR_CH6 register accessor: an alias for `Reg<ALT_DST_DATA_END_PTR_CH6_SPEC>`"]
pub type ALT_DST_DATA_END_PTR_CH6 =
    crate::Reg<alt_dst_data_end_ptr_ch6::ALT_DST_DATA_END_PTR_CH6_SPEC>;
#[doc = "Alternate pointer to the end address of the destination data of channel 6"]
pub mod alt_dst_data_end_ptr_ch6;
#[doc = "ALT_CHN_CFG_CH6 register accessor: an alias for `Reg<ALT_CHN_CFG_CH6_SPEC>`"]
pub type ALT_CHN_CFG_CH6 = crate::Reg<alt_chn_cfg_ch6::ALT_CHN_CFG_CH6_SPEC>;
#[doc = "Primary configuration for channel 6"]
pub mod alt_chn_cfg_ch6;
#[doc = "ALT_SRC_DATA_END_PTR_CH7 register accessor: an alias for `Reg<ALT_SRC_DATA_END_PTR_CH7_SPEC>`"]
pub type ALT_SRC_DATA_END_PTR_CH7 =
    crate::Reg<alt_src_data_end_ptr_ch7::ALT_SRC_DATA_END_PTR_CH7_SPEC>;
#[doc = "Alternate pointer to the end address of the source data of channel 7"]
pub mod alt_src_data_end_ptr_ch7;
#[doc = "ALT_DST_DATA_END_PTR_CH7 register accessor: an alias for `Reg<ALT_DST_DATA_END_PTR_CH7_SPEC>`"]
pub type ALT_DST_DATA_END_PTR_CH7 =
    crate::Reg<alt_dst_data_end_ptr_ch7::ALT_DST_DATA_END_PTR_CH7_SPEC>;
#[doc = "Alternate pointer to the end address of the destination data of channel 7"]
pub mod alt_dst_data_end_ptr_ch7;
#[doc = "ALT_CHN_CFG_CH7 register accessor: an alias for `Reg<ALT_CHN_CFG_CH7_SPEC>`"]
pub type ALT_CHN_CFG_CH7 = crate::Reg<alt_chn_cfg_ch7::ALT_CHN_CFG_CH7_SPEC>;
#[doc = "Primary configuration for channel 7"]
pub mod alt_chn_cfg_ch7;
#[doc = "ALT_SRC_DATA_END_PTR_CH8 register accessor: an alias for `Reg<ALT_SRC_DATA_END_PTR_CH8_SPEC>`"]
pub type ALT_SRC_DATA_END_PTR_CH8 =
    crate::Reg<alt_src_data_end_ptr_ch8::ALT_SRC_DATA_END_PTR_CH8_SPEC>;
#[doc = "Alternate pointer to the end address of the source data of channel 8"]
pub mod alt_src_data_end_ptr_ch8;
#[doc = "ALT_DST_DATA_END_PTR_CH8 register accessor: an alias for `Reg<ALT_DST_DATA_END_PTR_CH8_SPEC>`"]
pub type ALT_DST_DATA_END_PTR_CH8 =
    crate::Reg<alt_dst_data_end_ptr_ch8::ALT_DST_DATA_END_PTR_CH8_SPEC>;
#[doc = "Alternate pointer to the end address of the destination data of channel 8"]
pub mod alt_dst_data_end_ptr_ch8;
#[doc = "ALT_CHN_CFG_CH8 register accessor: an alias for `Reg<ALT_CHN_CFG_CH8_SPEC>`"]
pub type ALT_CHN_CFG_CH8 = crate::Reg<alt_chn_cfg_ch8::ALT_CHN_CFG_CH8_SPEC>;
#[doc = "Primary configuration for channel 8"]
pub mod alt_chn_cfg_ch8;
#[doc = "ALT_SRC_DATA_END_PTR_CH9 register accessor: an alias for `Reg<ALT_SRC_DATA_END_PTR_CH9_SPEC>`"]
pub type ALT_SRC_DATA_END_PTR_CH9 =
    crate::Reg<alt_src_data_end_ptr_ch9::ALT_SRC_DATA_END_PTR_CH9_SPEC>;
#[doc = "Alternate pointer to the end address of the source data of channel 9"]
pub mod alt_src_data_end_ptr_ch9;
#[doc = "ALT_DST_DATA_END_PTR_CH9 register accessor: an alias for `Reg<ALT_DST_DATA_END_PTR_CH9_SPEC>`"]
pub type ALT_DST_DATA_END_PTR_CH9 =
    crate::Reg<alt_dst_data_end_ptr_ch9::ALT_DST_DATA_END_PTR_CH9_SPEC>;
#[doc = "Alternate pointer to the end address of the destination data of channel 9"]
pub mod alt_dst_data_end_ptr_ch9;
#[doc = "ALT_CHN_CFG_CH9 register accessor: an alias for `Reg<ALT_CHN_CFG_CH9_SPEC>`"]
pub type ALT_CHN_CFG_CH9 = crate::Reg<alt_chn_cfg_ch9::ALT_CHN_CFG_CH9_SPEC>;
#[doc = "Primary configuration for channel 9"]
pub mod alt_chn_cfg_ch9;
#[doc = "ALT_SRC_DATA_END_PTR_CH10 register accessor: an alias for `Reg<ALT_SRC_DATA_END_PTR_CH10_SPEC>`"]
pub type ALT_SRC_DATA_END_PTR_CH10 =
    crate::Reg<alt_src_data_end_ptr_ch10::ALT_SRC_DATA_END_PTR_CH10_SPEC>;
#[doc = "Alternate pointer to the end address of the source data of channel 10"]
pub mod alt_src_data_end_ptr_ch10;
#[doc = "ALT_DST_DATA_END_PTR_CH10 register accessor: an alias for `Reg<ALT_DST_DATA_END_PTR_CH10_SPEC>`"]
pub type ALT_DST_DATA_END_PTR_CH10 =
    crate::Reg<alt_dst_data_end_ptr_ch10::ALT_DST_DATA_END_PTR_CH10_SPEC>;
#[doc = "Alternate pointer to the end address of the destination data of channel 10"]
pub mod alt_dst_data_end_ptr_ch10;
#[doc = "ALT_CHN_CFG_CH10 register accessor: an alias for `Reg<ALT_CHN_CFG_CH10_SPEC>`"]
pub type ALT_CHN_CFG_CH10 = crate::Reg<alt_chn_cfg_ch10::ALT_CHN_CFG_CH10_SPEC>;
#[doc = "Primary configuration for channel 10"]
pub mod alt_chn_cfg_ch10;
#[doc = "ALT_SRC_DATA_END_PTR_CH11 register accessor: an alias for `Reg<ALT_SRC_DATA_END_PTR_CH11_SPEC>`"]
pub type ALT_SRC_DATA_END_PTR_CH11 =
    crate::Reg<alt_src_data_end_ptr_ch11::ALT_SRC_DATA_END_PTR_CH11_SPEC>;
#[doc = "Alternate pointer to the end address of the source data of channel 11"]
pub mod alt_src_data_end_ptr_ch11;
#[doc = "ALT_DST_DATA_END_PTR_CH11 register accessor: an alias for `Reg<ALT_DST_DATA_END_PTR_CH11_SPEC>`"]
pub type ALT_DST_DATA_END_PTR_CH11 =
    crate::Reg<alt_dst_data_end_ptr_ch11::ALT_DST_DATA_END_PTR_CH11_SPEC>;
#[doc = "Alternate pointer to the end address of the destination data of channel 11"]
pub mod alt_dst_data_end_ptr_ch11;
#[doc = "ALT_CHN_CFG_CH11 register accessor: an alias for `Reg<ALT_CHN_CFG_CH11_SPEC>`"]
pub type ALT_CHN_CFG_CH11 = crate::Reg<alt_chn_cfg_ch11::ALT_CHN_CFG_CH11_SPEC>;
#[doc = "Primary configuration for channel 11"]
pub mod alt_chn_cfg_ch11;
#[doc = "ALT_SRC_DATA_END_PTR_CH12 register accessor: an alias for `Reg<ALT_SRC_DATA_END_PTR_CH12_SPEC>`"]
pub type ALT_SRC_DATA_END_PTR_CH12 =
    crate::Reg<alt_src_data_end_ptr_ch12::ALT_SRC_DATA_END_PTR_CH12_SPEC>;
#[doc = "Alternate pointer to the end address of the source data of channel 12"]
pub mod alt_src_data_end_ptr_ch12;
#[doc = "ALT_DST_DATA_END_PTR_CH12 register accessor: an alias for `Reg<ALT_DST_DATA_END_PTR_CH12_SPEC>`"]
pub type ALT_DST_DATA_END_PTR_CH12 =
    crate::Reg<alt_dst_data_end_ptr_ch12::ALT_DST_DATA_END_PTR_CH12_SPEC>;
#[doc = "Alternate pointer to the end address of the destination data of channel 12"]
pub mod alt_dst_data_end_ptr_ch12;
#[doc = "ALT_CHN_CFG_CH12 register accessor: an alias for `Reg<ALT_CHN_CFG_CH12_SPEC>`"]
pub type ALT_CHN_CFG_CH12 = crate::Reg<alt_chn_cfg_ch12::ALT_CHN_CFG_CH12_SPEC>;
#[doc = "Primary configuration for channel 12"]
pub mod alt_chn_cfg_ch12;
#[doc = "ALT_SRC_DATA_END_PTR_CH13 register accessor: an alias for `Reg<ALT_SRC_DATA_END_PTR_CH13_SPEC>`"]
pub type ALT_SRC_DATA_END_PTR_CH13 =
    crate::Reg<alt_src_data_end_ptr_ch13::ALT_SRC_DATA_END_PTR_CH13_SPEC>;
#[doc = "Alternate pointer to the end address of the source data of channel 13"]
pub mod alt_src_data_end_ptr_ch13;
#[doc = "ALT_DST_DATA_END_PTR_CH13 register accessor: an alias for `Reg<ALT_DST_DATA_END_PTR_CH13_SPEC>`"]
pub type ALT_DST_DATA_END_PTR_CH13 =
    crate::Reg<alt_dst_data_end_ptr_ch13::ALT_DST_DATA_END_PTR_CH13_SPEC>;
#[doc = "Alternate pointer to the end address of the destination data of channel 13"]
pub mod alt_dst_data_end_ptr_ch13;
#[doc = "ALT_CHN_CFG_CH13 register accessor: an alias for `Reg<ALT_CHN_CFG_CH13_SPEC>`"]
pub type ALT_CHN_CFG_CH13 = crate::Reg<alt_chn_cfg_ch13::ALT_CHN_CFG_CH13_SPEC>;
#[doc = "Primary configuration for channel 13"]
pub mod alt_chn_cfg_ch13;
#[doc = "ALT_SRC_DATA_END_PTR_CH14 register accessor: an alias for `Reg<ALT_SRC_DATA_END_PTR_CH14_SPEC>`"]
pub type ALT_SRC_DATA_END_PTR_CH14 =
    crate::Reg<alt_src_data_end_ptr_ch14::ALT_SRC_DATA_END_PTR_CH14_SPEC>;
#[doc = "Alternate pointer to the end address of the source data of channel 14"]
pub mod alt_src_data_end_ptr_ch14;
#[doc = "ALT_DST_DATA_END_PTR_CH14 register accessor: an alias for `Reg<ALT_DST_DATA_END_PTR_CH14_SPEC>`"]
pub type ALT_DST_DATA_END_PTR_CH14 =
    crate::Reg<alt_dst_data_end_ptr_ch14::ALT_DST_DATA_END_PTR_CH14_SPEC>;
#[doc = "Alternate pointer to the end address of the destination data of channel 14"]
pub mod alt_dst_data_end_ptr_ch14;
#[doc = "ALT_CHN_CFG_CH14 register accessor: an alias for `Reg<ALT_CHN_CFG_CH14_SPEC>`"]
pub type ALT_CHN_CFG_CH14 = crate::Reg<alt_chn_cfg_ch14::ALT_CHN_CFG_CH14_SPEC>;
#[doc = "Primary configuration for channel 14"]
pub mod alt_chn_cfg_ch14;
#[doc = "ALT_SRC_DATA_END_PTR_CH15 register accessor: an alias for `Reg<ALT_SRC_DATA_END_PTR_CH15_SPEC>`"]
pub type ALT_SRC_DATA_END_PTR_CH15 =
    crate::Reg<alt_src_data_end_ptr_ch15::ALT_SRC_DATA_END_PTR_CH15_SPEC>;
#[doc = "Alternate pointer to the end address of the source data of channel 15"]
pub mod alt_src_data_end_ptr_ch15;
#[doc = "ALT_DST_DATA_END_PTR_CH15 register accessor: an alias for `Reg<ALT_DST_DATA_END_PTR_CH15_SPEC>`"]
pub type ALT_DST_DATA_END_PTR_CH15 =
    crate::Reg<alt_dst_data_end_ptr_ch15::ALT_DST_DATA_END_PTR_CH15_SPEC>;
#[doc = "Alternate pointer to the end address of the destination data of channel 15"]
pub mod alt_dst_data_end_ptr_ch15;
#[doc = "ALT_CHN_CFG_CH15 register accessor: an alias for `Reg<ALT_CHN_CFG_CH15_SPEC>`"]
pub type ALT_CHN_CFG_CH15 = crate::Reg<alt_chn_cfg_ch15::ALT_CHN_CFG_CH15_SPEC>;
#[doc = "Primary configuration for channel 15"]
pub mod alt_chn_cfg_ch15;
