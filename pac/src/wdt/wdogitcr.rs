#[doc = "Register `WDOGITCR` reader"]
pub struct R(crate::R<WDOGITCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDOGITCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDOGITCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDOGITCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WDOGITCR` writer"]
pub struct W(crate::W<WDOGITCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDOGITCR_SPEC>;
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
impl From<crate::W<WDOGITCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDOGITCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDOGITCR` reader - The WDOGITCR Register enables integration test mode. When in this more, the test output register directly controls the masted interrup output, WDOGINT, and reset output, WDOGRES. Integration Test mode Enable \\[0\\]
When set HIGH, places th watchdog into integration test mode."]
pub struct WDOGITCR_R(crate::FieldReader<bool, bool>);
impl WDOGITCR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WDOGITCR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDOGITCR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDOGITCR` writer - The WDOGITCR Register enables integration test mode. When in this more, the test output register directly controls the masted interrup output, WDOGINT, and reset output, WDOGRES. Integration Test mode Enable \\[0\\]
When set HIGH, places th watchdog into integration test mode."]
pub struct WDOGITCR_W<'a> {
    w: &'a mut W,
}
impl<'a> WDOGITCR_W<'a> {
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
    #[doc = "Bit 0 - The WDOGITCR Register enables integration test mode. When in this more, the test output register directly controls the masted interrup output, WDOGINT, and reset output, WDOGRES. Integration Test mode Enable \\[0\\]
When set HIGH, places th watchdog into integration test mode."]
    #[inline(always)]
    pub fn wdogitcr(&self) -> WDOGITCR_R {
        WDOGITCR_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The WDOGITCR Register enables integration test mode. When in this more, the test output register directly controls the masted interrup output, WDOGINT, and reset output, WDOGRES. Integration Test mode Enable \\[0\\]
When set HIGH, places th watchdog into integration test mode."]
    #[inline(always)]
    pub fn wdogitcr(&mut self) -> WDOGITCR_W {
        WDOGITCR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The WDOGITCR Register enables integration test mode. When in this more, the test output register directly controls the masted interrup output, WDOGINT, and reset output, WDOGRES. Integration Test mode Enable \\[0\\]
When set HIGH, places th watchdog into integration test mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdogitcr](index.html) module"]
pub struct WDOGITCR_SPEC;
impl crate::RegisterSpec for WDOGITCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdogitcr::R](R) reader structure"]
impl crate::Readable for WDOGITCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdogitcr::W](W) writer structure"]
impl crate::Writable for WDOGITCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WDOGITCR to value 0"]
impl crate::Resettable for WDOGITCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
