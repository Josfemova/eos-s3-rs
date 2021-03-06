#[doc = "Register `PMU_TIMER_CFG_0` reader"]
pub struct R(crate::R<PMU_TIMER_CFG_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMU_TIMER_CFG_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMU_TIMER_CFG_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMU_TIMER_CFG_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMU_TIMER_CFG_0` writer"]
pub struct W(crate::W<PMU_TIMER_CFG_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMU_TIMER_CFG_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<PMU_TIMER_CFG_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMU_TIMER_CFG_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PMU_Time_Out_period` reader - PMU timer is 16 bits running at 32KHZ. Once PMU timer bit 15:6 match the value of this register, PMUT INT will trigger and Timer will stop. 0 is a reserved value, min must be load value is 1."]
pub struct PMU_TIME_OUT_PERIOD_R(crate::FieldReader<u16, u16>);
impl PMU_TIME_OUT_PERIOD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        PMU_TIME_OUT_PERIOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PMU_TIME_OUT_PERIOD_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMU_Time_Out_period` writer - PMU timer is 16 bits running at 32KHZ. Once PMU timer bit 15:6 match the value of this register, PMUT INT will trigger and Timer will stop. 0 is a reserved value, min must be load value is 1."]
pub struct PMU_TIME_OUT_PERIOD_W<'a> {
    w: &'a mut W,
}
impl<'a> PMU_TIME_OUT_PERIOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - PMU timer is 16 bits running at 32KHZ. Once PMU timer bit 15:6 match the value of this register, PMUT INT will trigger and Timer will stop. 0 is a reserved value, min must be load value is 1."]
    #[inline(always)]
    pub fn pmu_time_out_period(&self) -> PMU_TIME_OUT_PERIOD_R {
        PMU_TIME_OUT_PERIOD_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - PMU timer is 16 bits running at 32KHZ. Once PMU timer bit 15:6 match the value of this register, PMUT INT will trigger and Timer will stop. 0 is a reserved value, min must be load value is 1."]
    #[inline(always)]
    pub fn pmu_time_out_period(&mut self) -> PMU_TIME_OUT_PERIOD_W {
        PMU_TIME_OUT_PERIOD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration for the PMU timer time-out period\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmu_timer_cfg_0](index.html) module"]
pub struct PMU_TIMER_CFG_0_SPEC;
impl crate::RegisterSpec for PMU_TIMER_CFG_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pmu_timer_cfg_0::R](R) reader structure"]
impl crate::Readable for PMU_TIMER_CFG_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmu_timer_cfg_0::W](W) writer structure"]
impl crate::Writable for PMU_TIMER_CFG_0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PMU_TIMER_CFG_0 to value 0"]
impl crate::Resettable for PMU_TIMER_CFG_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
