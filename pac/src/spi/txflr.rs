#[doc = "Register `TXFLR` reader"]
pub struct R(crate::R<TXFLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXFLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXFLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXFLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TXTFL` reader - Transmit FIFO Level. Contains the number of valid data entries in the transmit FIFO."]
pub struct TXTFL_R(crate::FieldReader<u8, u8>);
impl TXTFL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TXTFL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXTFL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:2 - Transmit FIFO Level. Contains the number of valid data entries in the transmit FIFO."]
    #[inline(always)]
    pub fn txtfl(&self) -> TXTFL_R {
        TXTFL_R::new((self.bits & 0x07) as u8)
    }
}
#[doc = "Transmit FIFO Level Register: This register contains the number of valid data entries in the transmit FIFO memory.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txflr](index.html) module"]
pub struct TXFLR_SPEC;
impl crate::RegisterSpec for TXFLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txflr::R](R) reader structure"]
impl crate::Readable for TXFLR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TXFLR to value 0"]
impl crate::Resettable for TXFLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
