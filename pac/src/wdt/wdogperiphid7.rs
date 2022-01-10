#[doc = "Register `WDOGPERIPHID7` reader"]
pub struct R(crate::R<WDOGPERIPHID7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDOGPERIPHID7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDOGPERIPHID7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDOGPERIPHID7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WDOGPERIPHID7` reader - Peripheral ID Register 7."]
pub struct WDOGPERIPHID7_R(crate::FieldReader<u8, u8>);
impl WDOGPERIPHID7_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WDOGPERIPHID7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDOGPERIPHID7_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Peripheral ID Register 7."]
    #[inline(always)]
    pub fn wdogperiphid7(&self) -> WDOGPERIPHID7_R {
        WDOGPERIPHID7_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Peripheral ID Register 7.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdogperiphid7](index.html) module"]
pub struct WDOGPERIPHID7_SPEC;
impl crate::RegisterSpec for WDOGPERIPHID7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdogperiphid7::R](R) reader structure"]
impl crate::Readable for WDOGPERIPHID7_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets WDOGPERIPHID7 to value 0"]
impl crate::Resettable for WDOGPERIPHID7_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
