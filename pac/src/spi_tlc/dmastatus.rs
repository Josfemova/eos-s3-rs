#[doc = "Register `DMASTATUS` reader"]
pub struct R(crate::R<DMASTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMASTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMASTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMASTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DmaFIFO_Empty` reader - Set if DMA FIFO is Empty"]
pub struct DMAFIFO_EMPTY_R(crate::FieldReader<bool, bool>);
impl DMAFIFO_EMPTY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMAFIFO_EMPTY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMAFIFO_EMPTY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DmaFIFO_underflow` reader - Set if DMA hit underflow condition"]
pub struct DMAFIFO_UNDERFLOW_R(crate::FieldReader<bool, bool>);
impl DMAFIFO_UNDERFLOW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMAFIFO_UNDERFLOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMAFIFO_UNDERFLOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Set if DMA FIFO is Empty"]
    #[inline(always)]
    pub fn dma_fifo_empty(&self) -> DMAFIFO_EMPTY_R {
        DMAFIFO_EMPTY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Set if DMA hit underflow condition"]
    #[inline(always)]
    pub fn dma_fifo_underflow(&self) -> DMAFIFO_UNDERFLOW_R {
        DMAFIFO_UNDERFLOW_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
#[doc = "DMA Status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmastatus](index.html) module"]
pub struct DMASTATUS_SPEC;
impl crate::RegisterSpec for DMASTATUS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [dmastatus::R](R) reader structure"]
impl crate::Readable for DMASTATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMASTATUS to value 0"]
impl crate::Resettable for DMASTATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
