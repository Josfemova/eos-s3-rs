#[doc = "Register `FFE_DBG_COMBINED` reader"]
pub struct R(crate::R<FFE_DBG_COMBINED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FFE_DBG_COMBINED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FFE_DBG_COMBINED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FFE_DBG_COMBINED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `sm0_SM_debug` reader - Sensor memory 0 debug signals"]
pub struct SM0_SM_DEBUG_R(crate::FieldReader<u8, u8>);
impl SM0_SM_DEBUG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SM0_SM_DEBUG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SM0_SM_DEBUG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sm1_SM_debug` reader - Sensor memory 1 debug signals"]
pub struct SM1_SM_DEBUG_R(crate::FieldReader<u8, u8>);
impl SM1_SM_DEBUG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SM1_SM_DEBUG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SM1_SM_DEBUG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ffe0_debug` reader - ffe0 debug signals"]
pub struct FFE0_DEBUG_R(crate::FieldReader<u8, u8>);
impl FFE0_DEBUG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FFE0_DEBUG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FFE0_DEBUG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Sensor memory 0 debug signals"]
    #[inline(always)]
    pub fn sm0_sm_debug(&self) -> SM0_SM_DEBUG_R {
        SM0_SM_DEBUG_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Sensor memory 1 debug signals"]
    #[inline(always)]
    pub fn sm1_sm_debug(&self) -> SM1_SM_DEBUG_R {
        SM1_SM_DEBUG_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - ffe0 debug signals"]
    #[inline(always)]
    pub fn ffe0_debug(&self) -> FFE0_DEBUG_R {
        FFE0_DEBUG_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[doc = "Combined Flexible Fusion Engine debug signals\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ffe_dbg_combined](index.html) module"]
pub struct FFE_DBG_COMBINED_SPEC;
impl crate::RegisterSpec for FFE_DBG_COMBINED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ffe_dbg_combined::R](R) reader structure"]
impl crate::Readable for FFE_DBG_COMBINED_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FFE_DBG_COMBINED to value 0"]
impl crate::Resettable for FFE_DBG_COMBINED_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
