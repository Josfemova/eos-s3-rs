#[doc = "Register `DMA_INTR` reader"]
pub struct R(crate::R<DMA_INTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_INTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_INTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_INTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `dma_herror` reader - 1: hresp=1, 0: hresp didn't go to 1, write one to clr"]
pub struct DMA_HERROR_R(crate::FieldReader<bool, bool>);
impl DMA_HERROR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMA_HERROR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_HERROR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rx_data_available` reader - 1: rx threshold was hit, 0:threshold was not hit. This is before external mask bit."]
pub struct RX_DATA_AVAILABLE_R(crate::FieldReader<bool, bool>);
impl RX_DATA_AVAILABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_DATA_AVAILABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_DATA_AVAILABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ahb_bridge_fifo_overflow` reader - 1: A ahb FIFO bridge overflow occurred, 0: no overflow occurred"]
pub struct AHB_BRIDGE_FIFO_OVERFLOW_R(crate::FieldReader<bool, bool>);
impl AHB_BRIDGE_FIFO_OVERFLOW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AHB_BRIDGE_FIFO_OVERFLOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AHB_BRIDGE_FIFO_OVERFLOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `spim_ssi_txe_intr` reader - SPIM Transmit FIFO empty"]
pub struct SPIM_SSI_TXE_INTR_R(crate::FieldReader<bool, bool>);
impl SPIM_SSI_TXE_INTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPIM_SSI_TXE_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPIM_SSI_TXE_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `spim_ssi_txo_intr` reader - SPIM Transmit FIFO overflow"]
pub struct SPIM_SSI_TXO_INTR_R(crate::FieldReader<bool, bool>);
impl SPIM_SSI_TXO_INTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPIM_SSI_TXO_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPIM_SSI_TXO_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `spim_ssi_rxf_intr` reader - SPIM Receive FIFO threshold"]
pub struct SPIM_SSI_RXF_INTR_R(crate::FieldReader<bool, bool>);
impl SPIM_SSI_RXF_INTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPIM_SSI_RXF_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPIM_SSI_RXF_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `spim_ssi_rxo_intr` reader - SPIM Receive FIFO overflow"]
pub struct SPIM_SSI_RXO_INTR_R(crate::FieldReader<bool, bool>);
impl SPIM_SSI_RXO_INTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPIM_SSI_RXO_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPIM_SSI_RXO_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `spim_ssi_rxu_intr` reader - SPIM Receive FIFO underflow"]
pub struct SPIM_SSI_RXU_INTR_R(crate::FieldReader<bool, bool>);
impl SPIM_SSI_RXU_INTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPIM_SSI_RXU_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPIM_SSI_RXU_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `spim_ssi_mst_intr` reader - SPIM master interrupt"]
pub struct SPIM_SSI_MST_INTR_R(crate::FieldReader<bool, bool>);
impl SPIM_SSI_MST_INTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPIM_SSI_MST_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPIM_SSI_MST_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - 1: hresp=1, 0: hresp didn't go to 1, write one to clr"]
    #[inline(always)]
    pub fn dma_herror(&self) -> DMA_HERROR_R {
        DMA_HERROR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1: rx threshold was hit, 0:threshold was not hit. This is before external mask bit."]
    #[inline(always)]
    pub fn rx_data_available(&self) -> RX_DATA_AVAILABLE_R {
        RX_DATA_AVAILABLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 1: A ahb FIFO bridge overflow occurred, 0: no overflow occurred"]
    #[inline(always)]
    pub fn ahb_bridge_fifo_overflow(&self) -> AHB_BRIDGE_FIFO_OVERFLOW_R {
        AHB_BRIDGE_FIFO_OVERFLOW_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SPIM Transmit FIFO empty"]
    #[inline(always)]
    pub fn spim_ssi_txe_intr(&self) -> SPIM_SSI_TXE_INTR_R {
        SPIM_SSI_TXE_INTR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - SPIM Transmit FIFO overflow"]
    #[inline(always)]
    pub fn spim_ssi_txo_intr(&self) -> SPIM_SSI_TXO_INTR_R {
        SPIM_SSI_TXO_INTR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - SPIM Receive FIFO threshold"]
    #[inline(always)]
    pub fn spim_ssi_rxf_intr(&self) -> SPIM_SSI_RXF_INTR_R {
        SPIM_SSI_RXF_INTR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - SPIM Receive FIFO overflow"]
    #[inline(always)]
    pub fn spim_ssi_rxo_intr(&self) -> SPIM_SSI_RXO_INTR_R {
        SPIM_SSI_RXO_INTR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - SPIM Receive FIFO underflow"]
    #[inline(always)]
    pub fn spim_ssi_rxu_intr(&self) -> SPIM_SSI_RXU_INTR_R {
        SPIM_SSI_RXU_INTR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - SPIM master interrupt"]
    #[inline(always)]
    pub fn spim_ssi_mst_intr(&self) -> SPIM_SSI_MST_INTR_R {
        SPIM_SSI_MST_INTR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
#[doc = "DMA interrupts\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_intr](index.html) module"]
pub struct DMA_INTR_SPEC;
impl crate::RegisterSpec for DMA_INTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_intr::R](R) reader structure"]
impl crate::Readable for DMA_INTR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMA_INTR to value 0"]
impl crate::Resettable for DMA_INTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
