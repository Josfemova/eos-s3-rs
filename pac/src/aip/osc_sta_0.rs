#[doc = "Register `OSC_STA_0` reader"]
pub struct R(crate::R<OSC_STA_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OSC_STA_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OSC_STA_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OSC_STA_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `lock` reader - Please refer to the Technical Reference Manual for detail, NO SYNC, FW need to read it twice to ensure the value."]
pub struct LOCK_R(crate::FieldReader<bool, bool>);
impl LOCK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `anatestreq_` reader - Please refer to the Technical Reference Manual for detail, NO SYNC, FW need to read it twice to ensure the value."]
pub struct ANATESTREQ__R(crate::FieldReader<bool, bool>);
impl ANATESTREQ__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ANATESTREQ__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ANATESTREQ__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Please refer to the Technical Reference Manual for detail, NO SYNC, FW need to read it twice to ensure the value."]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Please refer to the Technical Reference Manual for detail, NO SYNC, FW need to read it twice to ensure the value."]
    #[inline(always)]
    pub fn anatestreq_(&self) -> ANATESTREQ__R {
        ANATESTREQ__R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
#[doc = "Contains information about oscilator status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osc_sta_0](index.html) module"]
pub struct OSC_STA_0_SPEC;
impl crate::RegisterSpec for OSC_STA_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [osc_sta_0::R](R) reader structure"]
impl crate::Readable for OSC_STA_0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets OSC_STA_0 to value 0"]
impl crate::Resettable for OSC_STA_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
