#[doc = "Register `M4_STATUS` reader"]
pub struct R(crate::R<M4_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<M4_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<M4_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<M4_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `M4_Active` reader - M4 Power Domain is active"]
pub struct M4_ACTIVE_R(crate::FieldReader<bool, bool>);
impl M4_ACTIVE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        M4_ACTIVE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M4_ACTIVE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M4_Deep_Sleep` reader - Reserved"]
pub struct M4_DEEP_SLEEP_R(crate::FieldReader<bool, bool>);
impl M4_DEEP_SLEEP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        M4_DEEP_SLEEP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M4_DEEP_SLEEP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M4_Shut_Down` reader - M4 Power Domain is in Shut Down mode"]
pub struct M4_SHUT_DOWN_R(crate::FieldReader<bool, bool>);
impl M4_SHUT_DOWN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        M4_SHUT_DOWN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M4_SHUT_DOWN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M4_Clock_Gating` reader - Reserved"]
pub struct M4_CLOCK_GATING_R(crate::FieldReader<bool, bool>);
impl M4_CLOCK_GATING_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        M4_CLOCK_GATING_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M4_CLOCK_GATING_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - M4 Power Domain is active"]
    #[inline(always)]
    pub fn m4_active(&self) -> M4_ACTIVE_R {
        M4_ACTIVE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Reserved"]
    #[inline(always)]
    pub fn m4_deep_sleep(&self) -> M4_DEEP_SLEEP_R {
        M4_DEEP_SLEEP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - M4 Power Domain is in Shut Down mode"]
    #[inline(always)]
    pub fn m4_shut_down(&self) -> M4_SHUT_DOWN_R {
        M4_SHUT_DOWN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn m4_clock_gating(&self) -> M4_CLOCK_GATING_R {
        M4_CLOCK_GATING_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
#[doc = "Status of the M4 Power Domain\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m4_status](index.html) module"]
pub struct M4_STATUS_SPEC;
impl crate::RegisterSpec for M4_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [m4_status::R](R) reader structure"]
impl crate::Readable for M4_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets M4_STATUS to value 0"]
impl crate::Resettable for M4_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
