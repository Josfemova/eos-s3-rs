#[doc = "Register `FFE_STATUS` reader"]
pub struct R(crate::R<FFE_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FFE_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FFE_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FFE_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FFE_Active` reader - FFE Power Domain is active"]
pub struct FFE_ACTIVE_R(crate::FieldReader<bool, bool>);
impl FFE_ACTIVE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FFE_ACTIVE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FFE_ACTIVE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FFE_Deep_Sleep` reader - FFE Power Domain is in Deep Sleep mode"]
pub struct FFE_DEEP_SLEEP_R(crate::FieldReader<bool, bool>);
impl FFE_DEEP_SLEEP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FFE_DEEP_SLEEP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FFE_DEEP_SLEEP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FFE_Shut_Down` reader - FFE Power Domain is in Shut_Down mode"]
pub struct FFE_SHUT_DOWN_R(crate::FieldReader<bool, bool>);
impl FFE_SHUT_DOWN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FFE_SHUT_DOWN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FFE_SHUT_DOWN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FFE_Clock_Gating` reader - Power Domain is alive, Onlt related clock is gated off -- This mode is for debugging only"]
pub struct FFE_CLOCK_GATING_R(crate::FieldReader<bool, bool>);
impl FFE_CLOCK_GATING_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FFE_CLOCK_GATING_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FFE_CLOCK_GATING_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - FFE Power Domain is active"]
    #[inline(always)]
    pub fn ffe_active(&self) -> FFE_ACTIVE_R {
        FFE_ACTIVE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - FFE Power Domain is in Deep Sleep mode"]
    #[inline(always)]
    pub fn ffe_deep_sleep(&self) -> FFE_DEEP_SLEEP_R {
        FFE_DEEP_SLEEP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - FFE Power Domain is in Shut_Down mode"]
    #[inline(always)]
    pub fn ffe_shut_down(&self) -> FFE_SHUT_DOWN_R {
        FFE_SHUT_DOWN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Power Domain is alive, Onlt related clock is gated off -- This mode is for debugging only"]
    #[inline(always)]
    pub fn ffe_clock_gating(&self) -> FFE_CLOCK_GATING_R {
        FFE_CLOCK_GATING_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
#[doc = "Status of the Flexible Fusion Engine\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ffe_status](index.html) module"]
pub struct FFE_STATUS_SPEC;
impl crate::RegisterSpec for FFE_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ffe_status::R](R) reader structure"]
impl crate::Readable for FFE_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FFE_STATUS to value 0"]
impl crate::Resettable for FFE_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
