#[doc = "Register `PDWU_Timer_CFG` reader"]
pub struct R(crate::R<PDWU_TIMER_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDWU_TIMER_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDWU_TIMER_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDWU_TIMER_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDWU_Timer_CFG` writer"]
pub struct W(crate::W<PDWU_TIMER_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDWU_TIMER_CFG_SPEC>;
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
impl From<crate::W<PDWU_TIMER_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDWU_TIMER_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PDWU_Timer_period` reader - This is to define the additional IDLE cycles after Power Gating cell is off (Power is ON). This configuration applied to all power domain. Waits for reg value + 1 cycles, i.e. 0 - 1 cycle, 2 - 3 cycles, etc."]
pub struct PDWU_TIMER_PERIOD_R(crate::FieldReader<u8, u8>);
impl PDWU_TIMER_PERIOD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PDWU_TIMER_PERIOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDWU_TIMER_PERIOD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDWU_Timer_period` writer - This is to define the additional IDLE cycles after Power Gating cell is off (Power is ON). This configuration applied to all power domain. Waits for reg value + 1 cycles, i.e. 0 - 1 cycle, 2 - 3 cycles, etc."]
pub struct PDWU_TIMER_PERIOD_W<'a> {
    w: &'a mut W,
}
impl<'a> PDWU_TIMER_PERIOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - This is to define the additional IDLE cycles after Power Gating cell is off (Power is ON). This configuration applied to all power domain. Waits for reg value + 1 cycles, i.e. 0 - 1 cycle, 2 - 3 cycles, etc."]
    #[inline(always)]
    pub fn pdwu_timer_period(&self) -> PDWU_TIMER_PERIOD_R {
        PDWU_TIMER_PERIOD_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - This is to define the additional IDLE cycles after Power Gating cell is off (Power is ON). This configuration applied to all power domain. Waits for reg value + 1 cycles, i.e. 0 - 1 cycle, 2 - 3 cycles, etc."]
    #[inline(always)]
    pub fn pdwu_timer_period(&mut self) -> PDWU_TIMER_PERIOD_W {
        PDWU_TIMER_PERIOD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control the delay for power-on after wake-up event. Applies to all power domains\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdwu_timer_cfg](index.html) module"]
pub struct PDWU_TIMER_CFG_SPEC;
impl crate::RegisterSpec for PDWU_TIMER_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdwu_timer_cfg::R](R) reader structure"]
impl crate::Readable for PDWU_TIMER_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdwu_timer_cfg::W](W) writer structure"]
impl crate::Writable for PDWU_TIMER_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDWU_Timer_CFG to value 0"]
impl crate::Resettable for PDWU_TIMER_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
