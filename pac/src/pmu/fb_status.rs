#[doc = "Register `FB_STATUS` reader"]
pub struct R(crate::R<FB_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FB_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FB_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FB_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FB_Active` reader - FPGA Fabric Power domain is active"]
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
#[doc = "Field `FB_Deep_Sleep` reader - FPGA Fabric Power domain is in mode"]
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
#[doc = "Field `FB_Shut_Down` reader - FPGA Fabric Power domain is in mode"]
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
impl R {
    #[doc = "Bit 0 - FPGA Fabric Power domain is active"]
    #[inline(always)]
    pub fn fb_active(&self) -> FB_ACTIVE_R {
        FB_ACTIVE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - FPGA Fabric Power domain is in mode"]
    #[inline(always)]
    pub fn fb_deep_sleep(&self) -> FB_DEEP_SLEEP_R {
        FB_DEEP_SLEEP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - FPGA Fabric Power domain is in mode"]
    #[inline(always)]
    pub fn fb_shut_down(&self) -> FB_SHUT_DOWN_R {
        FB_SHUT_DOWN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
#[doc = "FPGA Fabric Power domain status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fb_status](index.html) module"]
pub struct FB_STATUS_SPEC;
impl crate::RegisterSpec for FB_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fb_status::R](R) reader structure"]
impl crate::Readable for FB_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FB_STATUS to value 0"]
impl crate::Resettable for FB_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
