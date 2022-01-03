#[doc = "Register `RTC_STA_1` reader"]
pub struct R(crate::R<RTC_STA_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_STA_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_STA_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_STA_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PO` reader - Please refer to the Technical Reference Manual for detail, NO SYNC, FW need to read it twice to ensure the value."]
pub struct PO_R(crate::FieldReader<u32, u32>);
impl PO_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        PO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PO_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Please refer to the Technical Reference Manual for detail, NO SYNC, FW need to read it twice to ensure the value."]
    #[inline(always)]
    pub fn po(&self) -> PO_R {
        PO_R::new(self.bits as u32)
    }
}
#[doc = "Incomplete information. Probably related to AIP RTC hardware status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_sta_1](index.html) module"]
pub struct RTC_STA_1_SPEC;
impl crate::RegisterSpec for RTC_STA_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_sta_1::R](R) reader structure"]
impl crate::Readable for RTC_STA_1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RTC_STA_1 to value 0"]
impl crate::Resettable for RTC_STA_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
