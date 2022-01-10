#[doc = "Register `WDOGPERIPHID6` reader"]
pub struct R(crate::R<WDOGPERIPHID6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDOGPERIPHID6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDOGPERIPHID6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDOGPERIPHID6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WDOGPERIPHID6` reader - Peripheral ID Register 6."]
pub struct WDOGPERIPHID6_R(crate::FieldReader<u8, u8>);
impl WDOGPERIPHID6_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WDOGPERIPHID6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDOGPERIPHID6_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Peripheral ID Register 6."]
    #[inline(always)]
    pub fn wdogperiphid6(&self) -> WDOGPERIPHID6_R {
        WDOGPERIPHID6_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Peripheral ID Register 6.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdogperiphid6](index.html) module"]
pub struct WDOGPERIPHID6_SPEC;
impl crate::RegisterSpec for WDOGPERIPHID6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdogperiphid6::R](R) reader structure"]
impl crate::Readable for WDOGPERIPHID6_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets WDOGPERIPHID6 to value 0"]
impl crate::Resettable for WDOGPERIPHID6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
