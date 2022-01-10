#[doc = "Register `WDOGPERIPHID1` reader"]
pub struct R(crate::R<WDOGPERIPHID1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDOGPERIPHID1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDOGPERIPHID1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDOGPERIPHID1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WDOGPERIPHID1` reader - Peripheral ID Register 1. \\[7:4\\]
jep106_id_3_0. \\[3:0\\]
Part number \\[11:8\\]."]
pub struct WDOGPERIPHID1_R(crate::FieldReader<u8, u8>);
impl WDOGPERIPHID1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WDOGPERIPHID1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDOGPERIPHID1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Peripheral ID Register 1. \\[7:4\\]
jep106_id_3_0. \\[3:0\\]
Part number \\[11:8\\]."]
    #[inline(always)]
    pub fn wdogperiphid1(&self) -> WDOGPERIPHID1_R {
        WDOGPERIPHID1_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Peripheral ID Register 1. \\[7:4\\]
jep106_id_3_0. \\[3:0\\]
Part number \\[11:8\\].\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdogperiphid1](index.html) module"]
pub struct WDOGPERIPHID1_SPEC;
impl crate::RegisterSpec for WDOGPERIPHID1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdogperiphid1::R](R) reader structure"]
impl crate::Readable for WDOGPERIPHID1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets WDOGPERIPHID1 to value 0xb8"]
impl crate::Resettable for WDOGPERIPHID1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xb8
    }
}
