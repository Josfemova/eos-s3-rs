#[doc = "Register `M4S0_SRAM_STATUS` reader"]
pub struct R(crate::R<M4S0_SRAM_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<M4S0_SRAM_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<M4S0_SRAM_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<M4S0_SRAM_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `M4S0_Active` reader - PF SRAM Power domain is active."]
pub struct M4S0_ACTIVE_R(crate::FieldReader<bool, bool>);
impl M4S0_ACTIVE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        M4S0_ACTIVE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M4S0_ACTIVE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M4S0_Deep_Sleep` reader - Reserved. Deep Sleep not supported for this power domain."]
pub struct M4S0_DEEP_SLEEP_R(crate::FieldReader<bool, bool>);
impl M4S0_DEEP_SLEEP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        M4S0_DEEP_SLEEP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M4S0_DEEP_SLEEP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M4S0_Shut_Down` reader - PF SRAM Fabric Power domain is in mode."]
pub struct M4S0_SHUT_DOWN_R(crate::FieldReader<bool, bool>);
impl M4S0_SHUT_DOWN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        M4S0_SHUT_DOWN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M4S0_SHUT_DOWN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M4S0_Clock_Gating` reader - Reserved"]
pub struct M4S0_CLOCK_GATING_R(crate::FieldReader<bool, bool>);
impl M4S0_CLOCK_GATING_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        M4S0_CLOCK_GATING_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M4S0_CLOCK_GATING_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - PF SRAM Power domain is active."]
    #[inline(always)]
    pub fn m4s0_active(&self) -> M4S0_ACTIVE_R {
        M4S0_ACTIVE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Reserved. Deep Sleep not supported for this power domain."]
    #[inline(always)]
    pub fn m4s0_deep_sleep(&self) -> M4S0_DEEP_SLEEP_R {
        M4S0_DEEP_SLEEP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PF SRAM Fabric Power domain is in mode."]
    #[inline(always)]
    pub fn m4s0_shut_down(&self) -> M4S0_SHUT_DOWN_R {
        M4S0_SHUT_DOWN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn m4s0_clock_gating(&self) -> M4S0_CLOCK_GATING_R {
        M4S0_CLOCK_GATING_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
#[doc = "M4S0 SRAM Power Domain status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m4s0_sram_status](index.html) module"]
pub struct M4S0_SRAM_STATUS_SPEC;
impl crate::RegisterSpec for M4S0_SRAM_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [m4s0_sram_status::R](R) reader structure"]
impl crate::Readable for M4S0_SRAM_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets M4S0_SRAM_STATUS to value 0"]
impl crate::Resettable for M4S0_SRAM_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
