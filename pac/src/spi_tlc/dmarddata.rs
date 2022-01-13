#[doc = "Register `DMARDDATA` reader"]
pub struct R(crate::R<DMARDDATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMARDDATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMARDDATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMARDDATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DmaRdData` reader - DMA Read Data Port"]
pub struct DMARDDATA_R(crate::FieldReader<u8, u8>);
impl DMARDDATA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DMARDDATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMARDDATA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - DMA Read Data Port"]
    #[inline(always)]
    pub fn dma_rd_data(&self) -> DMARDDATA_R {
        DMARDDATA_R::new(self.bits as u8)
    }
}
#[doc = "DMA Read Data Port\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmarddata](index.html) module"]
pub struct DMARDDATA_SPEC;
impl crate::RegisterSpec for DMARDDATA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [dmarddata::R](R) reader structure"]
impl crate::Readable for DMARDDATA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMARDDATA to value 0"]
impl crate::Resettable for DMARDDATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
