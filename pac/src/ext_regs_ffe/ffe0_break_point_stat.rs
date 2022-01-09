#[doc = "Register `FFE0_BREAK_POINT_STAT` reader"]
pub struct R(crate::R<FFE0_BREAK_POINT_STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FFE0_BREAK_POINT_STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FFE0_BREAK_POINT_STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FFE0_BREAK_POINT_STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FFE0_BP_MATCH` reader - This signals that a 'break point' has been reached and FFE execution is paused."]
pub struct FFE0_BP_MATCH_R(crate::FieldReader<bool, bool>);
impl FFE0_BP_MATCH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FFE0_BP_MATCH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FFE0_BP_MATCH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SM0_BP_MATCH` reader - SM Break Point Match signal output to the host that notifies the host that the break point condition has been detected."]
pub struct SM0_BP_MATCH_R(crate::FieldReader<bool, bool>);
impl SM0_BP_MATCH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SM0_BP_MATCH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SM0_BP_MATCH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SM1_BP_MATCH` reader - SM Break Point Match signal output to the host that notifies the host that the break point condition has been detected."]
pub struct SM1_BP_MATCH_R(crate::FieldReader<bool, bool>);
impl SM1_BP_MATCH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SM1_BP_MATCH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SM1_BP_MATCH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - This signals that a 'break point' has been reached and FFE execution is paused."]
    #[inline(always)]
    pub fn ffe0_bp_match(&self) -> FFE0_BP_MATCH_R {
        FFE0_BP_MATCH_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SM Break Point Match signal output to the host that notifies the host that the break point condition has been detected."]
    #[inline(always)]
    pub fn sm0_bp_match(&self) -> SM0_BP_MATCH_R {
        SM0_BP_MATCH_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SM Break Point Match signal output to the host that notifies the host that the break point condition has been detected."]
    #[inline(always)]
    pub fn sm1_bp_match(&self) -> SM1_BP_MATCH_R {
        SM1_BP_MATCH_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
#[doc = "FFE break point status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ffe0_break_point_stat](index.html) module"]
pub struct FFE0_BREAK_POINT_STAT_SPEC;
impl crate::RegisterSpec for FFE0_BREAK_POINT_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ffe0_break_point_stat::R](R) reader structure"]
impl crate::Readable for FFE0_BREAK_POINT_STAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FFE0_BREAK_POINT_STAT to value 0"]
impl crate::Resettable for FFE0_BREAK_POINT_STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
