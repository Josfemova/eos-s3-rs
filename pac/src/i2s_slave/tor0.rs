#[doc = "Register `TOR0` reader"]
pub struct R(crate::R<TOR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TOR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TOR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TOR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TXCHO` reader - Read this bit to clear the TX FIFO Data Overrun interrupt."]
pub struct TXCHO_R(crate::FieldReader<bool, bool>);
impl TXCHO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXCHO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXCHO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Read this bit to clear the TX FIFO Data Overrun interrupt."]
    #[inline(always)]
    pub fn txcho(&self) -> TXCHO_R {
        TXCHO_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "Transmit Overrun Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tor0](index.html) module"]
pub struct TOR0_SPEC;
impl crate::RegisterSpec for TOR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tor0::R](R) reader structure"]
impl crate::Readable for TOR0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TOR0 to value 0"]
impl crate::Resettable for TOR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
