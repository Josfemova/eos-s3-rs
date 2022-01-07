#[doc = "Register `SDMA_STATUS` reader"]
pub struct R(crate::R<SDMA_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDMA_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDMA_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDMA_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SDMA_Active` reader - SDMA power domain is active"]
pub struct SDMA_ACTIVE_R(crate::FieldReader<bool, bool>);
impl SDMA_ACTIVE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SDMA_ACTIVE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDMA_ACTIVE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDMA_Deep_Sleep` reader - SDMA power domain is in Deep Sleep Mode"]
pub struct SDMA_DEEP_SLEEP_R(crate::FieldReader<bool, bool>);
impl SDMA_DEEP_SLEEP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SDMA_DEEP_SLEEP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDMA_DEEP_SLEEP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDMA_Shut_Down` reader - SDMA power domain is in Shut Down mode"]
pub struct SDMA_SHUT_DOWN_R(crate::FieldReader<bool, bool>);
impl SDMA_SHUT_DOWN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SDMA_SHUT_DOWN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDMA_SHUT_DOWN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDMA_Clock_Gating` reader - Reserved"]
pub struct SDMA_CLOCK_GATING_R(crate::FieldReader<bool, bool>);
impl SDMA_CLOCK_GATING_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SDMA_CLOCK_GATING_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDMA_CLOCK_GATING_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - SDMA power domain is active"]
    #[inline(always)]
    pub fn sdma_active(&self) -> SDMA_ACTIVE_R {
        SDMA_ACTIVE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SDMA power domain is in Deep Sleep Mode"]
    #[inline(always)]
    pub fn sdma_deep_sleep(&self) -> SDMA_DEEP_SLEEP_R {
        SDMA_DEEP_SLEEP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SDMA power domain is in Shut Down mode"]
    #[inline(always)]
    pub fn sdma_shut_down(&self) -> SDMA_SHUT_DOWN_R {
        SDMA_SHUT_DOWN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn sdma_clock_gating(&self) -> SDMA_CLOCK_GATING_R {
        SDMA_CLOCK_GATING_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
#[doc = "SDMA status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdma_status](index.html) module"]
pub struct SDMA_STATUS_SPEC;
impl crate::RegisterSpec for SDMA_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdma_status::R](R) reader structure"]
impl crate::Readable for SDMA_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SDMA_STATUS to value 0"]
impl crate::Resettable for SDMA_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
