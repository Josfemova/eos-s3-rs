#[doc = "Register `RXFLR` reader"]
pub struct R(crate::R<RXFLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXFLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXFLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXFLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXTFL` reader - Receive FIFO Level. Contains the number of valid data entries in the receive FIFO."]
pub struct RXTFL_R(crate::FieldReader<u8, u8>);
impl RXTFL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RXTFL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXTFL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:2 - Receive FIFO Level. Contains the number of valid data entries in the receive FIFO."]
    #[inline(always)]
    pub fn rxtfl(&self) -> RXTFL_R {
        RXTFL_R::new((self.bits & 0x07) as u8)
    }
}
#[doc = "Receive FIFO Level Register: This register contains the number of valid data entries in FIFO memory. This register can be read at any time.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxflr](index.html) module"]
pub struct RXFLR_SPEC;
impl crate::RegisterSpec for RXFLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxflr::R](R) reader structure"]
impl crate::Readable for RXFLR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RXFLR to value 0"]
impl crate::Resettable for RXFLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
