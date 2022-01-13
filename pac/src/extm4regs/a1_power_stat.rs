#[doc = "Register `A1_POWER_STAT` reader"]
pub struct R(crate::R<A1_POWER_STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<A1_POWER_STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<A1_POWER_STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<A1_POWER_STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `A1_POWER_STAT` reader - Status of the A1 subsystem power status"]
pub struct A1_POWER_STAT_R(crate::FieldReader<u8, u8>);
impl A1_POWER_STAT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        A1_POWER_STAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for A1_POWER_STAT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:1 - Status of the A1 subsystem power status"]
    #[inline(always)]
    pub fn a1_power_stat(&self) -> A1_POWER_STAT_R {
        A1_POWER_STAT_R::new((self.bits & 0x03) as u8)
    }
}
#[doc = "Status of the A1 subsystem power status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [a1_power_stat](index.html) module"]
pub struct A1_POWER_STAT_SPEC;
impl crate::RegisterSpec for A1_POWER_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [a1_power_stat::R](R) reader structure"]
impl crate::Readable for A1_POWER_STAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets A1_POWER_STAT to value 0"]
impl crate::Resettable for A1_POWER_STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
