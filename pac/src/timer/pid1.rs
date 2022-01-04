#[doc = "Register `PID1` reader"]
pub struct R(crate::R<PID1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PID1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PID1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PID1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PID` reader - Peripheral ID"]
pub struct PID_R(crate::FieldReader<u8, u8>);
impl PID_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PID_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Peripheral ID"]
    #[inline(always)]
    pub fn pid(&self) -> PID_R {
        PID_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Peripheral ID register 1: \\[7:0\\]
jep106_id_3_0. \\[3:0\\]
Par number\\[11:8\\].\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pid1](index.html) module"]
pub struct PID1_SPEC;
impl crate::RegisterSpec for PID1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pid1::R](R) reader structure"]
impl crate::Readable for PID1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PID1 to value 0"]
impl crate::Resettable for PID1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
