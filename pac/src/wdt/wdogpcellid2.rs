#[doc = "Register `WDOGPCELLID2` reader"]
pub struct R(crate::R<WDOGPCELLID2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDOGPCELLID2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDOGPCELLID2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDOGPCELLID2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WDOGPCELLID2` reader - Component ID Register 2."]
pub struct WDOGPCELLID2_R(crate::FieldReader<u8, u8>);
impl WDOGPCELLID2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WDOGPCELLID2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDOGPCELLID2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Component ID Register 2."]
    #[inline(always)]
    pub fn wdogpcellid2(&self) -> WDOGPCELLID2_R {
        WDOGPCELLID2_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Component ID Register 2.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdogpcellid2](index.html) module"]
pub struct WDOGPCELLID2_SPEC;
impl crate::RegisterSpec for WDOGPCELLID2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdogpcellid2::R](R) reader structure"]
impl crate::Readable for WDOGPCELLID2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets WDOGPCELLID2 to value 0x05"]
impl crate::Resettable for WDOGPCELLID2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x05
    }
}
