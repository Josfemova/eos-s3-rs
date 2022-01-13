#[doc = "Register `DMA_WAITONREQ_STATUS` reader"]
pub struct R(crate::R<DMA_WAITONREQ_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_WAITONREQ_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_WAITONREQ_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_WAITONREQ_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `dma_waitonreq_status` reader - Channel wait on request status. \n \n Read as: \n \n Bit \\[C\\]
= 0 dma_waitonreq\\[C\\]
is LOW. \n \n Bit \\[C\\]
= 1 dma_waitonreq\\[C\\]
is HIGH."]
pub struct DMA_WAITONREQ_STATUS_R(crate::FieldReader<u16, u16>);
impl DMA_WAITONREQ_STATUS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        DMA_WAITONREQ_STATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_WAITONREQ_STATUS_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Channel wait on request status. \n \n Read as: \n \n Bit \\[C\\]
= 0 dma_waitonreq\\[C\\]
is LOW. \n \n Bit \\[C\\]
= 1 dma_waitonreq\\[C\\]
is HIGH."]
    #[inline(always)]
    pub fn dma_waitonreq_status(&self) -> DMA_WAITONREQ_STATUS_R {
        DMA_WAITONREQ_STATUS_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Channel wait on request status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_waitonreq_status](index.html) module"]
pub struct DMA_WAITONREQ_STATUS_SPEC;
impl crate::RegisterSpec for DMA_WAITONREQ_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_waitonreq_status::R](R) reader structure"]
impl crate::Readable for DMA_WAITONREQ_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMA_WAITONREQ_STATUS to value 0"]
impl crate::Resettable for DMA_WAITONREQ_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
