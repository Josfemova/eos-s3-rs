#[doc = "Register `DMA_ACTIVE_REG` reader"]
pub struct R(crate::R<DMA_ACTIVE_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_ACTIVE_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_ACTIVE_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_ACTIVE_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `dma_active` reader - dma_active signal status from System DMA"]
pub struct DMA_ACTIVE_R(crate::FieldReader<u16, u16>);
impl DMA_ACTIVE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        DMA_ACTIVE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_ACTIVE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:10 - dma_active signal status from System DMA"]
    #[inline(always)]
    pub fn dma_active(&self) -> DMA_ACTIVE_R {
        DMA_ACTIVE_R::new((self.bits & 0x07ff) as u16)
    }
}
#[doc = "dma_active signal status from System DMA\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_active_reg](index.html) module"]
pub struct DMA_ACTIVE_REG_SPEC;
impl crate::RegisterSpec for DMA_ACTIVE_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_active_reg::R](R) reader structure"]
impl crate::Readable for DMA_ACTIVE_REG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMA_ACTIVE_REG to value 0"]
impl crate::Resettable for DMA_ACTIVE_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
