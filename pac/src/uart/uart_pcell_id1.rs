#[doc = "Register `UART_PCellID1` reader"]
pub struct R(crate::R<UART_PCELLID1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_PCELLID1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_PCELLID1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_PCELLID1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `UART_PCellID1` reader - UART PCell ID 1 register"]
pub struct UART_PCELLID1_R(crate::FieldReader<u8, u8>);
impl UART_PCELLID1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        UART_PCELLID1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART_PCELLID1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - UART PCell ID 1 register"]
    #[inline(always)]
    pub fn uart_pcell_id1(&self) -> UART_PCELLID1_R {
        UART_PCELLID1_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "UART PCell ID 1 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_pcell_id1](index.html) module"]
pub struct UART_PCELLID1_SPEC;
impl crate::RegisterSpec for UART_PCELLID1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_pcell_id1::R](R) reader structure"]
impl crate::Readable for UART_PCELLID1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UART_PCellID1 to value 0xf0"]
impl crate::Resettable for UART_PCELLID1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xf0
    }
}
