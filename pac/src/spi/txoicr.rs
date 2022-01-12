#[doc = "Register `TXOICR` reader"]
pub struct R(crate::R<TXOICR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXOICR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXOICR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXOICR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TXOICR` reader - Clear Transmit FIFO Overflow Interrupt. This register reflects the status of the interrupt. A read from this register clears the ssi_txo_intr interrupt; writing has no effect."]
pub struct TXOICR_R(crate::FieldReader<bool, bool>);
impl TXOICR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXOICR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXOICR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Clear Transmit FIFO Overflow Interrupt. This register reflects the status of the interrupt. A read from this register clears the ssi_txo_intr interrupt; writing has no effect."]
    #[inline(always)]
    pub fn txoicr(&self) -> TXOICR_R {
        TXOICR_R::new(self.bits != 0)
    }
}
#[doc = "Transmit FIFO Overflow Interrupt Clear Register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txoicr](index.html) module"]
pub struct TXOICR_SPEC;
impl crate::RegisterSpec for TXOICR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [txoicr::R](R) reader structure"]
impl crate::Readable for TXOICR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TXOICR to value 0"]
impl crate::Resettable for TXOICR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
