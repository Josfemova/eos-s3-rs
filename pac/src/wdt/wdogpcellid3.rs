#[doc = "Register `WDOGPCELLID3` reader"]
pub struct R(crate::R<WDOGPCELLID3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDOGPCELLID3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDOGPCELLID3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDOGPCELLID3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WDOGPCELLID3` reader - Component ID Register 3."]
pub struct WDOGPCELLID3_R(crate::FieldReader<u8, u8>);
impl WDOGPCELLID3_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WDOGPCELLID3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDOGPCELLID3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Component ID Register 3."]
    #[inline(always)]
    pub fn wdogpcellid3(&self) -> WDOGPCELLID3_R {
        WDOGPCELLID3_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Component ID Register 3.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdogpcellid3](index.html) module"]
pub struct WDOGPCELLID3_SPEC;
impl crate::RegisterSpec for WDOGPCELLID3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdogpcellid3::R](R) reader structure"]
impl crate::Readable for WDOGPCELLID3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets WDOGPCELLID3 to value 0xb1"]
impl crate::Resettable for WDOGPCELLID3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xb1
    }
}
