#[doc = "Register `DMA_STATUS` reader"]
pub struct R(crate::R<DMA_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `master_enable` reader - Set if controller is enabled"]
pub struct MASTER_ENABLE_R(crate::FieldReader<bool, bool>);
impl MASTER_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MASTER_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MASTER_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Current state of the control state machine.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum STATE_A {
    #[doc = "0: idle state"]
    IDLE = 0,
    #[doc = "1: reading channel controller data."]
    READ_CH_CNTRL_DATA = 1,
    #[doc = "2: reading source data end pointer"]
    READ_SRC_DATA_END_PTR = 2,
    #[doc = "3: reading destination data end pointer"]
    READ_DEST_DATA_END_PTR = 3,
    #[doc = "4: reading source data"]
    READ_SRC_DATA = 4,
    #[doc = "5: writing destination data"]
    WRITE_DEST_DATA = 5,
    #[doc = "6: waiting for DMA request to clear"]
    WAIT_DMA_REQ_CLEAR = 6,
    #[doc = "7: writing channel controller data"]
    WRITE_CH_CTRL_DATA = 7,
    #[doc = "8: stalled state"]
    STALLED = 8,
    #[doc = "9: done state"]
    DONE = 9,
    #[doc = "10: peripheral scatter-gather transition"]
    PERIPH_SCATTER_GATHER_TRANS = 10,
}
impl From<STATE_A> for u8 {
    #[inline(always)]
    fn from(variant: STATE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `state` reader - Current state of the control state machine."]
pub struct STATE_R(crate::FieldReader<u8, STATE_A>);
impl STATE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        STATE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<STATE_A> {
        match self.bits {
            0 => Some(STATE_A::IDLE),
            1 => Some(STATE_A::READ_CH_CNTRL_DATA),
            2 => Some(STATE_A::READ_SRC_DATA_END_PTR),
            3 => Some(STATE_A::READ_DEST_DATA_END_PTR),
            4 => Some(STATE_A::READ_SRC_DATA),
            5 => Some(STATE_A::WRITE_DEST_DATA),
            6 => Some(STATE_A::WAIT_DMA_REQ_CLEAR),
            7 => Some(STATE_A::WRITE_CH_CTRL_DATA),
            8 => Some(STATE_A::STALLED),
            9 => Some(STATE_A::DONE),
            10 => Some(STATE_A::PERIPH_SCATTER_GATHER_TRANS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        **self == STATE_A::IDLE
    }
    #[doc = "Checks if the value of the field is `READ_CH_CNTRL_DATA`"]
    #[inline(always)]
    pub fn is_read_ch_cntrl_data(&self) -> bool {
        **self == STATE_A::READ_CH_CNTRL_DATA
    }
    #[doc = "Checks if the value of the field is `READ_SRC_DATA_END_PTR`"]
    #[inline(always)]
    pub fn is_read_src_data_end_ptr(&self) -> bool {
        **self == STATE_A::READ_SRC_DATA_END_PTR
    }
    #[doc = "Checks if the value of the field is `READ_DEST_DATA_END_PTR`"]
    #[inline(always)]
    pub fn is_read_dest_data_end_ptr(&self) -> bool {
        **self == STATE_A::READ_DEST_DATA_END_PTR
    }
    #[doc = "Checks if the value of the field is `READ_SRC_DATA`"]
    #[inline(always)]
    pub fn is_read_src_data(&self) -> bool {
        **self == STATE_A::READ_SRC_DATA
    }
    #[doc = "Checks if the value of the field is `WRITE_DEST_DATA`"]
    #[inline(always)]
    pub fn is_write_dest_data(&self) -> bool {
        **self == STATE_A::WRITE_DEST_DATA
    }
    #[doc = "Checks if the value of the field is `WAIT_DMA_REQ_CLEAR`"]
    #[inline(always)]
    pub fn is_wait_dma_req_clear(&self) -> bool {
        **self == STATE_A::WAIT_DMA_REQ_CLEAR
    }
    #[doc = "Checks if the value of the field is `WRITE_CH_CTRL_DATA`"]
    #[inline(always)]
    pub fn is_write_ch_ctrl_data(&self) -> bool {
        **self == STATE_A::WRITE_CH_CTRL_DATA
    }
    #[doc = "Checks if the value of the field is `STALLED`"]
    #[inline(always)]
    pub fn is_stalled(&self) -> bool {
        **self == STATE_A::STALLED
    }
    #[doc = "Checks if the value of the field is `DONE`"]
    #[inline(always)]
    pub fn is_done(&self) -> bool {
        **self == STATE_A::DONE
    }
    #[doc = "Checks if the value of the field is `PERIPH_SCATTER_GATHER_TRANS`"]
    #[inline(always)]
    pub fn is_periph_scatter_gather_trans(&self) -> bool {
        **self == STATE_A::PERIPH_SCATTER_GATHER_TRANS
    }
}
impl core::ops::Deref for STATE_R {
    type Target = crate::FieldReader<u8, STATE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `chnls_minus1` reader - Number of available DMA channels (value in register + 1)"]
pub struct CHNLS_MINUS1_R(crate::FieldReader<u8, u8>);
impl CHNLS_MINUS1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CHNLS_MINUS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHNLS_MINUS1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `test_status` reader - To reduce the gate count you can configure the controller to exclude the integration test logic. If value = 1, the integration test logic is included. If 0, test logic is not included. Any other value is undefined"]
pub struct TEST_STATUS_R(crate::FieldReader<u8, u8>);
impl TEST_STATUS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TEST_STATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEST_STATUS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Set if controller is enabled"]
    #[inline(always)]
    pub fn master_enable(&self) -> MASTER_ENABLE_R {
        MASTER_ENABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 4:7 - Current state of the control state machine."]
    #[inline(always)]
    pub fn state(&self) -> STATE_R {
        STATE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 16:20 - Number of available DMA channels (value in register + 1)"]
    #[inline(always)]
    pub fn chnls_minus1(&self) -> CHNLS_MINUS1_R {
        CHNLS_MINUS1_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 28:31 - To reduce the gate count you can configure the controller to exclude the integration test logic. If value = 1, the integration test logic is included. If 0, test logic is not included. Any other value is undefined"]
    #[inline(always)]
    pub fn test_status(&self) -> TEST_STATUS_R {
        TEST_STATUS_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[doc = "DMA Status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_status](index.html) module"]
pub struct DMA_STATUS_SPEC;
impl crate::RegisterSpec for DMA_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_status::R](R) reader structure"]
impl crate::Readable for DMA_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMA_STATUS to value 0x100f_0000"]
impl crate::Resettable for DMA_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x100f_0000
    }
}
