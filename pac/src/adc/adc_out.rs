#[doc = "Register `ADC_OUT` reader"]
pub struct R(crate::R<ADC_OUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_OUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_OUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_OUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OUT` reader - Stored last sampled value"]
pub struct OUT_R(crate::FieldReader<u16, u16>);
impl OUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:11 - Stored last sampled value"]
    #[inline(always)]
    pub fn out(&self) -> OUT_R {
        OUT_R::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "Last sampled value\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_out](index.html) module"]
pub struct ADC_OUT_SPEC;
impl crate::RegisterSpec for ADC_OUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_out::R](R) reader structure"]
impl crate::Readable for ADC_OUT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ADC_OUT to value 0"]
impl crate::Resettable for ADC_OUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
