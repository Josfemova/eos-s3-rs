#[doc = "Register `MSTICR` reader"]
pub struct R(crate::R<MSTICR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSTICR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSTICR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSTICR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TXOICR` reader - Clear Multi-Master Contention Interrupt. This register reflects the status of the interrupt. read from this register clears the ssi_mst_intr interrupt; writing has no effect."]
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
    #[doc = "Bit 0 - Clear Multi-Master Contention Interrupt. This register reflects the status of the interrupt. read from this register clears the ssi_mst_intr interrupt; writing has no effect."]
    #[inline(always)]
    pub fn txoicr(&self) -> TXOICR_R {
        TXOICR_R::new(self.bits != 0)
    }
}
#[doc = "Multi-Master Interrupt Clear Register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [msticr](index.html) module"]
pub struct MSTICR_SPEC;
impl crate::RegisterSpec for MSTICR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [msticr::R](R) reader structure"]
impl crate::Readable for MSTICR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MSTICR to value 0"]
impl crate::Resettable for MSTICR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
