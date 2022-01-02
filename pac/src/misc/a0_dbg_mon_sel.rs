#[doc = "Register `A0_DBG_MON_SEL` reader"]
pub struct R(crate::R<A0_DBG_MON_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<A0_DBG_MON_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<A0_DBG_MON_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<A0_DBG_MON_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `A0_DBG_MON_SEL` writer"]
pub struct W(crate::W<A0_DBG_MON_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<A0_DBG_MON_SEL_SPEC>;
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
impl From<crate::W<A0_DBG_MON_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<A0_DBG_MON_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Select debug monitors\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum A0_DEBUG_MON_SEL_A {
    #[doc = "0: Selects the PMU monitor as the monitor for the A0 subsystem"]
    USE_PMU_MONITOR = 0,
    #[doc = "1: Selects the CRU monitor as the monitor for the A0 subsystem"]
    USE_CRU_MONITOR = 1,
}
impl From<A0_DEBUG_MON_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: A0_DEBUG_MON_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `A0_DEBUG_MON_SEL` reader - Select debug monitors"]
pub struct A0_DEBUG_MON_SEL_R(crate::FieldReader<u8, A0_DEBUG_MON_SEL_A>);
impl A0_DEBUG_MON_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        A0_DEBUG_MON_SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<A0_DEBUG_MON_SEL_A> {
        match self.bits {
            0 => Some(A0_DEBUG_MON_SEL_A::USE_PMU_MONITOR),
            1 => Some(A0_DEBUG_MON_SEL_A::USE_CRU_MONITOR),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `USE_PMU_MONITOR`"]
    #[inline(always)]
    pub fn is_use_pmu_monitor(&self) -> bool {
        **self == A0_DEBUG_MON_SEL_A::USE_PMU_MONITOR
    }
    #[doc = "Checks if the value of the field is `USE_CRU_MONITOR`"]
    #[inline(always)]
    pub fn is_use_cru_monitor(&self) -> bool {
        **self == A0_DEBUG_MON_SEL_A::USE_CRU_MONITOR
    }
}
impl core::ops::Deref for A0_DEBUG_MON_SEL_R {
    type Target = crate::FieldReader<u8, A0_DEBUG_MON_SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `A0_DEBUG_MON_SEL` writer - Select debug monitors"]
pub struct A0_DEBUG_MON_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> A0_DEBUG_MON_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: A0_DEBUG_MON_SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Selects the PMU monitor as the monitor for the A0 subsystem"]
    #[inline(always)]
    pub fn use_pmu_monitor(self) -> &'a mut W {
        self.variant(A0_DEBUG_MON_SEL_A::USE_PMU_MONITOR)
    }
    #[doc = "Selects the CRU monitor as the monitor for the A0 subsystem"]
    #[inline(always)]
    pub fn use_cru_monitor(self) -> &'a mut W {
        self.variant(A0_DEBUG_MON_SEL_A::USE_CRU_MONITOR)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Select debug monitors"]
    #[inline(always)]
    pub fn a0_debug_mon_sel(&self) -> A0_DEBUG_MON_SEL_R {
        A0_DEBUG_MON_SEL_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Select debug monitors"]
    #[inline(always)]
    pub fn a0_debug_mon_sel(&mut self) -> A0_DEBUG_MON_SEL_W {
        A0_DEBUG_MON_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Select A0 debug monitors\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [a0_dbg_mon_sel](index.html) module"]
pub struct A0_DBG_MON_SEL_SPEC;
impl crate::RegisterSpec for A0_DBG_MON_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [a0_dbg_mon_sel::R](R) reader structure"]
impl crate::Readable for A0_DBG_MON_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [a0_dbg_mon_sel::W](W) writer structure"]
impl crate::Writable for A0_DBG_MON_SEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets A0_DBG_MON_SEL to value 0"]
impl crate::Resettable for A0_DBG_MON_SEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
