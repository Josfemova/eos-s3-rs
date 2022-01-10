#[doc = "Register `WDOGPCELLID1` reader"]
pub struct R(crate::R<WDOGPCELLID1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDOGPCELLID1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDOGPCELLID1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDOGPCELLID1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WDOGPCELLID1` reader - Component ID Register 1."]
pub struct WDOGPCELLID1_R(crate::FieldReader<u8, u8>);
impl WDOGPCELLID1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WDOGPCELLID1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDOGPCELLID1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Component ID Register 1."]
    #[inline(always)]
    pub fn wdogpcellid1(&self) -> WDOGPCELLID1_R {
        WDOGPCELLID1_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Component ID Register 1.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdogpcellid1](index.html) module"]
pub struct WDOGPCELLID1_SPEC;
impl crate::RegisterSpec for WDOGPCELLID1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdogpcellid1::R](R) reader structure"]
impl crate::Readable for WDOGPCELLID1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets WDOGPCELLID1 to value 0xf0"]
impl crate::Resettable for WDOGPCELLID1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xf0
    }
}
