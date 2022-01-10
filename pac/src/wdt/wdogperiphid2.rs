#[doc = "Register `WDOGPERIPHID2` reader"]
pub struct R(crate::R<WDOGPERIPHID2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDOGPERIPHID2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDOGPERIPHID2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDOGPERIPHID2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WDOGPERIPHID2` reader - Peripheral ID Register 2. \\[7:4\\]
Revision. \\[3\\]
jedec_used. \\[2:0\\]
jep106_id_6_4."]
pub struct WDOGPERIPHID2_R(crate::FieldReader<u8, u8>);
impl WDOGPERIPHID2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WDOGPERIPHID2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDOGPERIPHID2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Peripheral ID Register 2. \\[7:4\\]
Revision. \\[3\\]
jedec_used. \\[2:0\\]
jep106_id_6_4."]
    #[inline(always)]
    pub fn wdogperiphid2(&self) -> WDOGPERIPHID2_R {
        WDOGPERIPHID2_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Peripheral ID Register 2. \\[7:4\\]
Revision. \\[3\\]
jedec_used. \\[2:0\\]
jep106_id_6_4.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdogperiphid2](index.html) module"]
pub struct WDOGPERIPHID2_SPEC;
impl crate::RegisterSpec for WDOGPERIPHID2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdogperiphid2::R](R) reader structure"]
impl crate::Readable for WDOGPERIPHID2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets WDOGPERIPHID2 to value 0x1b"]
impl crate::Resettable for WDOGPERIPHID2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1b
    }
}
