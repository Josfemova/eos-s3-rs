#[doc = "Register `UART_PCellID4` reader"]
pub struct R(crate::R<UART_PCELLID4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_PCELLID4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_PCELLID4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_PCELLID4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `UART_PCellID4` reader - UART PCell ID 4 register"]
pub struct UART_PCELLID4_R(crate::FieldReader<u8, u8>);
impl UART_PCELLID4_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        UART_PCELLID4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART_PCELLID4_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - UART PCell ID 4 register"]
    #[inline(always)]
    pub fn uart_pcell_id4(&self) -> UART_PCELLID4_R {
        UART_PCELLID4_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "UART PCell ID 4 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_pcell_id4](index.html) module"]
pub struct UART_PCELLID4_SPEC;
impl crate::RegisterSpec for UART_PCELLID4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_pcell_id4::R](R) reader structure"]
impl crate::Readable for UART_PCELLID4_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UART_PCellID4 to value 0xb1"]
impl crate::Resettable for UART_PCELLID4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xb1
    }
}
