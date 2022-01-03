#[doc = "Register `RTC_STA_0` reader"]
pub struct R(crate::R<RTC_STA_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_STA_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_STA_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_STA_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `alarm` reader - Please refer to the Technical Reference Manual for detail, NO SYNC, FW need to read it twice to ensure the value."]
pub struct ALARM_R(crate::FieldReader<bool, bool>);
impl ALARM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ALARM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALARM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `testreq` reader - Please refer to the Technical Reference Manual for detail, NO SYNC, FW need to read it twice to ensure the value."]
pub struct TESTREQ_R(crate::FieldReader<bool, bool>);
impl TESTREQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TESTREQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TESTREQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `digtestbus` reader - Please refer to the Technical Reference Manual for detail, NO SYNC, FW need to read it twice to ensure the value."]
pub struct DIGTESTBUS_R(crate::FieldReader<bool, bool>);
impl DIGTESTBUS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIGTESTBUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIGTESTBUS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `oscok` reader - Please refer to the Technical Reference Manual for detail, NO SYNC, FW need to read it twice to ensure the value."]
pub struct OSCOK_R(crate::FieldReader<bool, bool>);
impl OSCOK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OSCOK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OSCOK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Please refer to the Technical Reference Manual for detail, NO SYNC, FW need to read it twice to ensure the value."]
    #[inline(always)]
    pub fn alarm(&self) -> ALARM_R {
        ALARM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Please refer to the Technical Reference Manual for detail, NO SYNC, FW need to read it twice to ensure the value."]
    #[inline(always)]
    pub fn testreq(&self) -> TESTREQ_R {
        TESTREQ_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Please refer to the Technical Reference Manual for detail, NO SYNC, FW need to read it twice to ensure the value."]
    #[inline(always)]
    pub fn digtestbus(&self) -> DIGTESTBUS_R {
        DIGTESTBUS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Please refer to the Technical Reference Manual for detail, NO SYNC, FW need to read it twice to ensure the value."]
    #[inline(always)]
    pub fn oscok(&self) -> OSCOK_R {
        OSCOK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
#[doc = "Incomplete information. Probably related to AIP RTC hardware status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_sta_0](index.html) module"]
pub struct RTC_STA_0_SPEC;
impl crate::RegisterSpec for RTC_STA_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_sta_0::R](R) reader structure"]
impl crate::Readable for RTC_STA_0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RTC_STA_0 to value 0"]
impl crate::Resettable for RTC_STA_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
