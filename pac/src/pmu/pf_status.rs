#[doc = "Register `PF_STATUS` reader"]
pub struct R(crate::R<PF_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PF_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PF_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PF_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FB_Active` reader - PF SRAM Power domain is active"]
pub struct FB_ACTIVE_R(crate::FieldReader<bool, bool>);
impl FB_ACTIVE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FB_ACTIVE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FB_ACTIVE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FB_Deep_Sleep` reader - PF SRAM Fabric Power domain is in mode"]
pub struct FB_DEEP_SLEEP_R(crate::FieldReader<bool, bool>);
impl FB_DEEP_SLEEP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FB_DEEP_SLEEP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FB_DEEP_SLEEP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FB_Shut_Down` reader - PF SRAM Fabric Power domain is in mode"]
pub struct FB_SHUT_DOWN_R(crate::FieldReader<bool, bool>);
impl FB_SHUT_DOWN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FB_SHUT_DOWN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FB_SHUT_DOWN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FB_Clock_Gating` reader - Reserved"]
pub struct FB_CLOCK_GATING_R(crate::FieldReader<bool, bool>);
impl FB_CLOCK_GATING_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FB_CLOCK_GATING_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FB_CLOCK_GATING_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - PF SRAM Power domain is active"]
    #[inline(always)]
    pub fn fb_active(&self) -> FB_ACTIVE_R {
        FB_ACTIVE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PF SRAM Fabric Power domain is in mode"]
    #[inline(always)]
    pub fn fb_deep_sleep(&self) -> FB_DEEP_SLEEP_R {
        FB_DEEP_SLEEP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PF SRAM Fabric Power domain is in mode"]
    #[inline(always)]
    pub fn fb_shut_down(&self) -> FB_SHUT_DOWN_R {
        FB_SHUT_DOWN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn fb_clock_gating(&self) -> FB_CLOCK_GATING_R {
        FB_CLOCK_GATING_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
#[doc = "PF SRAM Power Domain status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pf_status](index.html) module"]
pub struct PF_STATUS_SPEC;
impl crate::RegisterSpec for PF_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pf_status::R](R) reader structure"]
impl crate::Readable for PF_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PF_STATUS to value 0"]
impl crate::Resettable for PF_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
