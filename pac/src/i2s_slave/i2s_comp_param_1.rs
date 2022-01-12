#[doc = "Register `I2S_COMP_PARAM_1` reader"]
pub struct R(crate::R<I2S_COMP_PARAM_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2S_COMP_PARAM_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2S_COMP_PARAM_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2S_COMP_PARAM_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Data width of the APB\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum APB_DATA_WIDTH_A {
    #[doc = "0: APB data width is 8 bit"]
    X8BIT = 0,
    #[doc = "1: APB data width is 16 bit"]
    X16BIT = 1,
    #[doc = "2: APB data width is 32 bit"]
    X32BIT = 2,
}
impl From<APB_DATA_WIDTH_A> for u8 {
    #[inline(always)]
    fn from(variant: APB_DATA_WIDTH_A) -> Self {
        variant as _
    }
}
#[doc = "Field `APB_DATA_WIDTH` reader - Data width of the APB"]
pub struct APB_DATA_WIDTH_R(crate::FieldReader<u8, APB_DATA_WIDTH_A>);
impl APB_DATA_WIDTH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        APB_DATA_WIDTH_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<APB_DATA_WIDTH_A> {
        match self.bits {
            0 => Some(APB_DATA_WIDTH_A::X8BIT),
            1 => Some(APB_DATA_WIDTH_A::X16BIT),
            2 => Some(APB_DATA_WIDTH_A::X32BIT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `X8BIT`"]
    #[inline(always)]
    pub fn is_x8bit(&self) -> bool {
        **self == APB_DATA_WIDTH_A::X8BIT
    }
    #[doc = "Checks if the value of the field is `X16BIT`"]
    #[inline(always)]
    pub fn is_x16bit(&self) -> bool {
        **self == APB_DATA_WIDTH_A::X16BIT
    }
    #[doc = "Checks if the value of the field is `X32BIT`"]
    #[inline(always)]
    pub fn is_x32bit(&self) -> bool {
        **self == APB_DATA_WIDTH_A::X32BIT
    }
}
impl core::ops::Deref for APB_DATA_WIDTH_R {
    type Target = crate::FieldReader<u8, APB_DATA_WIDTH_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Indicates the value of I2S_FIFO_DEPTH_GLOBAL\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum I2S_FIFO_DEPTH_GLOBAL_A {
    #[doc = "0: I2S_FIFO_DEPTH_GLOBAL equals 2"]
    DEPTH_2 = 0,
    #[doc = "1: I2S_FIFO_DEPTH_GLOBAL equals 4"]
    DEPTH_4 = 1,
    #[doc = "2: I2S_FIFO_DEPTH_GLOBAL equals 8"]
    DEPTH_8 = 2,
    #[doc = "3: I2S_FIFO_DEPTH_GLOBAL equals 16"]
    DEPTH_16 = 3,
}
impl From<I2S_FIFO_DEPTH_GLOBAL_A> for u8 {
    #[inline(always)]
    fn from(variant: I2S_FIFO_DEPTH_GLOBAL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `I2S_FIFO_DEPTH_GLOBAL` reader - Indicates the value of I2S_FIFO_DEPTH_GLOBAL"]
pub struct I2S_FIFO_DEPTH_GLOBAL_R(
    crate::FieldReader<u8, I2S_FIFO_DEPTH_GLOBAL_A>,
);
impl I2S_FIFO_DEPTH_GLOBAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        I2S_FIFO_DEPTH_GLOBAL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2S_FIFO_DEPTH_GLOBAL_A {
        match self.bits {
            0 => I2S_FIFO_DEPTH_GLOBAL_A::DEPTH_2,
            1 => I2S_FIFO_DEPTH_GLOBAL_A::DEPTH_4,
            2 => I2S_FIFO_DEPTH_GLOBAL_A::DEPTH_8,
            3 => I2S_FIFO_DEPTH_GLOBAL_A::DEPTH_16,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DEPTH_2`"]
    #[inline(always)]
    pub fn is_depth_2(&self) -> bool {
        **self == I2S_FIFO_DEPTH_GLOBAL_A::DEPTH_2
    }
    #[doc = "Checks if the value of the field is `DEPTH_4`"]
    #[inline(always)]
    pub fn is_depth_4(&self) -> bool {
        **self == I2S_FIFO_DEPTH_GLOBAL_A::DEPTH_4
    }
    #[doc = "Checks if the value of the field is `DEPTH_8`"]
    #[inline(always)]
    pub fn is_depth_8(&self) -> bool {
        **self == I2S_FIFO_DEPTH_GLOBAL_A::DEPTH_8
    }
    #[doc = "Checks if the value of the field is `DEPTH_16`"]
    #[inline(always)]
    pub fn is_depth_16(&self) -> bool {
        **self == I2S_FIFO_DEPTH_GLOBAL_A::DEPTH_16
    }
}
impl core::ops::Deref for I2S_FIFO_DEPTH_GLOBAL_R {
    type Target = crate::FieldReader<u8, I2S_FIFO_DEPTH_GLOBAL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Enabled mode for the I2S peripheral\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2S_MODE_EN_A {
    #[doc = "0: I2S operating in slave mode."]
    SLAVE = 0,
    #[doc = "1: I2S operating in master mode"]
    MASTER = 1,
}
impl From<I2S_MODE_EN_A> for bool {
    #[inline(always)]
    fn from(variant: I2S_MODE_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2S_MODE_EN` reader - Enabled mode for the I2S peripheral"]
pub struct I2S_MODE_EN_R(crate::FieldReader<bool, I2S_MODE_EN_A>);
impl I2S_MODE_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2S_MODE_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2S_MODE_EN_A {
        match self.bits {
            false => I2S_MODE_EN_A::SLAVE,
            true => I2S_MODE_EN_A::MASTER,
        }
    }
    #[doc = "Checks if the value of the field is `SLAVE`"]
    #[inline(always)]
    pub fn is_slave(&self) -> bool {
        **self == I2S_MODE_EN_A::SLAVE
    }
    #[doc = "Checks if the value of the field is `MASTER`"]
    #[inline(always)]
    pub fn is_master(&self) -> bool {
        **self == I2S_MODE_EN_A::MASTER
    }
}
impl core::ops::Deref for I2S_MODE_EN_R {
    type Target = crate::FieldReader<bool, I2S_MODE_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2S_TRANSMITTER_BLOCK` reader - "]
pub struct I2S_TRANSMITTER_BLOCK_R(crate::FieldReader<bool, bool>);
impl I2S_TRANSMITTER_BLOCK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2S_TRANSMITTER_BLOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2S_TRANSMITTER_BLOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Indicates how many I2S TX channels are active\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum I2S_TX_CHANNELS_A {
    #[doc = "0: 1 TX channel."]
    X1_CHANNEL = 0,
    #[doc = "1: 2 TX channel."]
    X2_CHANNEL = 1,
    #[doc = "2: 3 TX channel."]
    X3_CHANNEL = 2,
    #[doc = "3: 4 TX channel."]
    X4_CHANNEL = 3,
}
impl From<I2S_TX_CHANNELS_A> for u8 {
    #[inline(always)]
    fn from(variant: I2S_TX_CHANNELS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `I2S_TX_CHANNELS` reader - Indicates how many I2S TX channels are active"]
pub struct I2S_TX_CHANNELS_R(crate::FieldReader<u8, I2S_TX_CHANNELS_A>);
impl I2S_TX_CHANNELS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        I2S_TX_CHANNELS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2S_TX_CHANNELS_A {
        match self.bits {
            0 => I2S_TX_CHANNELS_A::X1_CHANNEL,
            1 => I2S_TX_CHANNELS_A::X2_CHANNEL,
            2 => I2S_TX_CHANNELS_A::X3_CHANNEL,
            3 => I2S_TX_CHANNELS_A::X4_CHANNEL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `X1_CHANNEL`"]
    #[inline(always)]
    pub fn is_x1_channel(&self) -> bool {
        **self == I2S_TX_CHANNELS_A::X1_CHANNEL
    }
    #[doc = "Checks if the value of the field is `X2_CHANNEL`"]
    #[inline(always)]
    pub fn is_x2_channel(&self) -> bool {
        **self == I2S_TX_CHANNELS_A::X2_CHANNEL
    }
    #[doc = "Checks if the value of the field is `X3_CHANNEL`"]
    #[inline(always)]
    pub fn is_x3_channel(&self) -> bool {
        **self == I2S_TX_CHANNELS_A::X3_CHANNEL
    }
    #[doc = "Checks if the value of the field is `X4_CHANNEL`"]
    #[inline(always)]
    pub fn is_x4_channel(&self) -> bool {
        **self == I2S_TX_CHANNELS_A::X4_CHANNEL
    }
}
impl core::ops::Deref for I2S_TX_CHANNELS_R {
    type Target = crate::FieldReader<u8, I2S_TX_CHANNELS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Indicates the wordsize of I2S TX packets\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum I2S_TX_WORDSIZE_0_A {
    #[doc = "0: TX configured with 12bit wordsize"]
    X12BIT = 0,
    #[doc = "1: TX configured with 16bit wordsize"]
    X16BIT = 1,
    #[doc = "2: TX configured with 20bit wordsize"]
    X20BIT = 2,
    #[doc = "3: TX configured with 24bit wordsize"]
    X24BIT = 3,
    #[doc = "4: TX configured with 32bit wordsize"]
    X32BIT = 4,
}
impl From<I2S_TX_WORDSIZE_0_A> for u8 {
    #[inline(always)]
    fn from(variant: I2S_TX_WORDSIZE_0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `I2S_TX_WORDSIZE_0` reader - Indicates the wordsize of I2S TX packets"]
pub struct I2S_TX_WORDSIZE_0_R(crate::FieldReader<u8, I2S_TX_WORDSIZE_0_A>);
impl I2S_TX_WORDSIZE_0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        I2S_TX_WORDSIZE_0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<I2S_TX_WORDSIZE_0_A> {
        match self.bits {
            0 => Some(I2S_TX_WORDSIZE_0_A::X12BIT),
            1 => Some(I2S_TX_WORDSIZE_0_A::X16BIT),
            2 => Some(I2S_TX_WORDSIZE_0_A::X20BIT),
            3 => Some(I2S_TX_WORDSIZE_0_A::X24BIT),
            4 => Some(I2S_TX_WORDSIZE_0_A::X32BIT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `X12BIT`"]
    #[inline(always)]
    pub fn is_x12bit(&self) -> bool {
        **self == I2S_TX_WORDSIZE_0_A::X12BIT
    }
    #[doc = "Checks if the value of the field is `X16BIT`"]
    #[inline(always)]
    pub fn is_x16bit(&self) -> bool {
        **self == I2S_TX_WORDSIZE_0_A::X16BIT
    }
    #[doc = "Checks if the value of the field is `X20BIT`"]
    #[inline(always)]
    pub fn is_x20bit(&self) -> bool {
        **self == I2S_TX_WORDSIZE_0_A::X20BIT
    }
    #[doc = "Checks if the value of the field is `X24BIT`"]
    #[inline(always)]
    pub fn is_x24bit(&self) -> bool {
        **self == I2S_TX_WORDSIZE_0_A::X24BIT
    }
    #[doc = "Checks if the value of the field is `X32BIT`"]
    #[inline(always)]
    pub fn is_x32bit(&self) -> bool {
        **self == I2S_TX_WORDSIZE_0_A::X32BIT
    }
}
impl core::ops::Deref for I2S_TX_WORDSIZE_0_R {
    type Target = crate::FieldReader<u8, I2S_TX_WORDSIZE_0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:1 - Data width of the APB"]
    #[inline(always)]
    pub fn apb_data_width(&self) -> APB_DATA_WIDTH_R {
        APB_DATA_WIDTH_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Indicates the value of I2S_FIFO_DEPTH_GLOBAL"]
    #[inline(always)]
    pub fn i2s_fifo_depth_global(&self) -> I2S_FIFO_DEPTH_GLOBAL_R {
        I2S_FIFO_DEPTH_GLOBAL_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 4 - Enabled mode for the I2S peripheral"]
    #[inline(always)]
    pub fn i2s_mode_en(&self) -> I2S_MODE_EN_R {
        I2S_MODE_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn i2s_transmitter_block(&self) -> I2S_TRANSMITTER_BLOCK_R {
        I2S_TRANSMITTER_BLOCK_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 9:10 - Indicates how many I2S TX channels are active"]
    #[inline(always)]
    pub fn i2s_tx_channels(&self) -> I2S_TX_CHANNELS_R {
        I2S_TX_CHANNELS_R::new(((self.bits >> 9) & 0x03) as u8)
    }
    #[doc = "Bits 16:18 - Indicates the wordsize of I2S TX packets"]
    #[inline(always)]
    pub fn i2s_tx_wordsize_0(&self) -> I2S_TX_WORDSIZE_0_R {
        I2S_TX_WORDSIZE_0_R::new(((self.bits >> 16) & 0x07) as u8)
    }
}
#[doc = "Component Parameter Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2s_comp_param_1](index.html) module"]
pub struct I2S_COMP_PARAM_1_SPEC;
impl crate::RegisterSpec for I2S_COMP_PARAM_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2s_comp_param_1::R](R) reader structure"]
impl crate::Readable for I2S_COMP_PARAM_1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets I2S_COMP_PARAM_1 to value 0x012a"]
impl crate::Resettable for I2S_COMP_PARAM_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x012a
    }
}
