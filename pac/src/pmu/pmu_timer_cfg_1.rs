#[doc = "Register `PMU_TIMER_CFG_1` reader"]
pub struct R(crate::R<PMU_TIMER_CFG_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMU_TIMER_CFG_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMU_TIMER_CFG_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMU_TIMER_CFG_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMU_TIMER_CFG_1` writer"]
pub struct W(crate::W<PMU_TIMER_CFG_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMU_TIMER_CFG_1_SPEC>;
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
impl From<crate::W<PMU_TIMER_CFG_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMU_TIMER_CFG_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PMU_Timer_Enable` reader - Set to enable PMU timer. Once disable, the PMU timer will be reset to all 0. FW need to disable the PMU timer once PMUT INT trigger. Please Note: It may take Up to 64uS to disable the time since this enable signal needs to sync to 32KHz clock domain from C10/C01 clock domain"]
pub struct PMU_TIMER_ENABLE_R(crate::FieldReader<bool, bool>);
impl PMU_TIMER_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PMU_TIMER_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PMU_TIMER_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMU_Timer_Enable` writer - Set to enable PMU timer. Once disable, the PMU timer will be reset to all 0. FW need to disable the PMU timer once PMUT INT trigger. Please Note: It may take Up to 64uS to disable the time since this enable signal needs to sync to 32KHz clock domain from C10/C01 clock domain"]
pub struct PMU_TIMER_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> PMU_TIMER_ENABLE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Set to enable PMU timer. Once disable, the PMU timer will be reset to all 0. FW need to disable the PMU timer once PMUT INT trigger. Please Note: It may take Up to 64uS to disable the time since this enable signal needs to sync to 32KHz clock domain from C10/C01 clock domain"]
    #[inline(always)]
    pub fn pmu_timer_enable(&self) -> PMU_TIMER_ENABLE_R {
        PMU_TIMER_ENABLE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set to enable PMU timer. Once disable, the PMU timer will be reset to all 0. FW need to disable the PMU timer once PMUT INT trigger. Please Note: It may take Up to 64uS to disable the time since this enable signal needs to sync to 32KHz clock domain from C10/C01 clock domain"]
    #[inline(always)]
    pub fn pmu_timer_enable(&mut self) -> PMU_TIMER_ENABLE_W {
        PMU_TIMER_ENABLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control wether the PMU timer is enabled or disabled\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmu_timer_cfg_1](index.html) module"]
pub struct PMU_TIMER_CFG_1_SPEC;
impl crate::RegisterSpec for PMU_TIMER_CFG_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pmu_timer_cfg_1::R](R) reader structure"]
impl crate::Readable for PMU_TIMER_CFG_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmu_timer_cfg_1::W](W) writer structure"]
impl crate::Writable for PMU_TIMER_CFG_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PMU_TIMER_CFG_1 to value 0"]
impl crate::Resettable for PMU_TIMER_CFG_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
