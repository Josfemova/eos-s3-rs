#[doc = "Register `WDOGPERIPHID5` reader"]
pub struct R(crate::R<WDOGPERIPHID5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDOGPERIPHID5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDOGPERIPHID5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDOGPERIPHID5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WDOGPERIPHID5` reader - Peripheral ID Register 5."]
pub struct WDOGPERIPHID5_R(crate::FieldReader<u8, u8>);
impl WDOGPERIPHID5_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WDOGPERIPHID5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDOGPERIPHID5_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Peripheral ID Register 5."]
    #[inline(always)]
    pub fn wdogperiphid5(&self) -> WDOGPERIPHID5_R {
        WDOGPERIPHID5_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Peripheral ID Register 5.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdogperiphid5](index.html) module"]
pub struct WDOGPERIPHID5_SPEC;
impl crate::RegisterSpec for WDOGPERIPHID5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdogperiphid5::R](R) reader structure"]
impl crate::Readable for WDOGPERIPHID5_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets WDOGPERIPHID5 to value 0"]
impl crate::Resettable for WDOGPERIPHID5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
