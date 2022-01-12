#[doc = "Register `TIMER_VALUE` reader"]
pub struct R(crate::R<TIMER_VALUE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER_VALUE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMER_VALUE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMER_VALUE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TIMER_VALUE` reader - Return the Value of 30-bits, in 1mS resoultion. This is the RTC value"]
pub struct TIMER_VALUE_R(crate::FieldReader<u32, u32>);
impl TIMER_VALUE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        TIMER_VALUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER_VALUE_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:29 - Return the Value of 30-bits, in 1mS resoultion. This is the RTC value"]
    #[inline(always)]
    pub fn timer_value(&self) -> TIMER_VALUE_R {
        TIMER_VALUE_R::new((self.bits & 0x3fff_ffff) as u32)
    }
}
#[doc = "Return the Value of 30-bits, in 1mS resoultion. This is the RTC value\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer_value](index.html) module"]
pub struct TIMER_VALUE_SPEC;
impl crate::RegisterSpec for TIMER_VALUE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer_value::R](R) reader structure"]
impl crate::Readable for TIMER_VALUE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TIMER_VALUE to value 0"]
impl crate::Resettable for TIMER_VALUE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
