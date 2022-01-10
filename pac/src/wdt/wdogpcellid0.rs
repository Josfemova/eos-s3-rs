#[doc = "Register `WDOGPCELLID0` reader"]
pub struct R(crate::R<WDOGPCELLID0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDOGPCELLID0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDOGPCELLID0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDOGPCELLID0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WDOGPCELLID0` reader - Component ID Register 0."]
pub struct WDOGPCELLID0_R(crate::FieldReader<u8, u8>);
impl WDOGPCELLID0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WDOGPCELLID0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDOGPCELLID0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Component ID Register 0."]
    #[inline(always)]
    pub fn wdogpcellid0(&self) -> WDOGPCELLID0_R {
        WDOGPCELLID0_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Component ID Register 0.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdogpcellid0](index.html) module"]
pub struct WDOGPCELLID0_SPEC;
impl crate::RegisterSpec for WDOGPCELLID0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdogpcellid0::R](R) reader structure"]
impl crate::Readable for WDOGPCELLID0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets WDOGPCELLID0 to value 0x0d"]
impl crate::Resettable for WDOGPCELLID0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0d
    }
}
