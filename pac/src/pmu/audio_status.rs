#[doc = "Register `AUDIO_STATUS` reader"]
pub struct R(crate::R<AUDIO_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AUDIO_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AUDIO_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AUDIO_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `AD0` reader - Bit is set if AD0 is on"]
pub struct AD0_R(crate::FieldReader<bool, bool>);
impl AD0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AD0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AD0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AD1` reader - Bit is set if AD1 power domain is on"]
pub struct AD1_R(crate::FieldReader<bool, bool>);
impl AD1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AD1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AD1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AD2` reader - Bit is set if AD2 power domain is on"]
pub struct AD2_R(crate::FieldReader<bool, bool>);
impl AD2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AD2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AD2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AD3` reader - Bit is set if AD3 power domain is on"]
pub struct AD3_R(crate::FieldReader<bool, bool>);
impl AD3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AD3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AD3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AD4` reader - Bit is set if AD4 power domain is on"]
pub struct AD4_R(crate::FieldReader<bool, bool>);
impl AD4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AD4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AD4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AD5` reader - Bit is set if AD5 power domain is on"]
pub struct AD5_R(crate::FieldReader<bool, bool>);
impl AD5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AD5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AD5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Bit is set if AD0 is on"]
    #[inline(always)]
    pub fn ad0(&self) -> AD0_R {
        AD0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Bit is set if AD1 power domain is on"]
    #[inline(always)]
    pub fn ad1(&self) -> AD1_R {
        AD1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Bit is set if AD2 power domain is on"]
    #[inline(always)]
    pub fn ad2(&self) -> AD2_R {
        AD2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Bit is set if AD3 power domain is on"]
    #[inline(always)]
    pub fn ad3(&self) -> AD3_R {
        AD3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Bit is set if AD4 power domain is on"]
    #[inline(always)]
    pub fn ad4(&self) -> AD4_R {
        AD4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Bit is set if AD5 power domain is on"]
    #[inline(always)]
    pub fn ad5(&self) -> AD5_R {
        AD5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
#[doc = "Audio power domain status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [audio_status](index.html) module"]
pub struct AUDIO_STATUS_SPEC;
impl crate::RegisterSpec for AUDIO_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [audio_status::R](R) reader structure"]
impl crate::Readable for AUDIO_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets AUDIO_STATUS to value 0"]
impl crate::Resettable for AUDIO_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
