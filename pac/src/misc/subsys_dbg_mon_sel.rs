#[doc = "Register `SUBSYS_DBG_MON_SEL` reader"]
pub struct R(crate::R<SUBSYS_DBG_MON_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SUBSYS_DBG_MON_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SUBSYS_DBG_MON_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SUBSYS_DBG_MON_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SUBSYS_DBG_MON_SEL` writer"]
pub struct W(crate::W<SUBSYS_DBG_MON_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SUBSYS_DBG_MON_SEL_SPEC>;
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
impl From<crate::W<SUBSYS_DBG_MON_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SUBSYS_DBG_MON_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Select which subsystem the debug monitors are routed from\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SUBSYS_DEBUG_MON_SEL_A {
    #[doc = "0: Monitor A0 subsystem"]
    MONITOR_A0 = 0,
    #[doc = "1: Monitor the CPU subsystem"]
    MONITOR_CPU = 1,
    #[doc = "3: Monitor the FPGA Fabric subsystem"]
    MONITOR_FFE = 3,
    #[doc = "4: Monitor the audio subsystem"]
    MONITOR_AUDIO = 4,
}
impl From<SUBSYS_DEBUG_MON_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SUBSYS_DEBUG_MON_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SUBSYS_DEBUG_MON_SEL` reader - Select which subsystem the debug monitors are routed from"]
pub struct SUBSYS_DEBUG_MON_SEL_R(crate::FieldReader<u8, SUBSYS_DEBUG_MON_SEL_A>);
impl SUBSYS_DEBUG_MON_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SUBSYS_DEBUG_MON_SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SUBSYS_DEBUG_MON_SEL_A> {
        match self.bits {
            0 => Some(SUBSYS_DEBUG_MON_SEL_A::MONITOR_A0),
            1 => Some(SUBSYS_DEBUG_MON_SEL_A::MONITOR_CPU),
            3 => Some(SUBSYS_DEBUG_MON_SEL_A::MONITOR_FFE),
            4 => Some(SUBSYS_DEBUG_MON_SEL_A::MONITOR_AUDIO),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MONITOR_A0`"]
    #[inline(always)]
    pub fn is_monitor_a0(&self) -> bool {
        **self == SUBSYS_DEBUG_MON_SEL_A::MONITOR_A0
    }
    #[doc = "Checks if the value of the field is `MONITOR_CPU`"]
    #[inline(always)]
    pub fn is_monitor_cpu(&self) -> bool {
        **self == SUBSYS_DEBUG_MON_SEL_A::MONITOR_CPU
    }
    #[doc = "Checks if the value of the field is `MONITOR_FFE`"]
    #[inline(always)]
    pub fn is_monitor_ffe(&self) -> bool {
        **self == SUBSYS_DEBUG_MON_SEL_A::MONITOR_FFE
    }
    #[doc = "Checks if the value of the field is `MONITOR_AUDIO`"]
    #[inline(always)]
    pub fn is_monitor_audio(&self) -> bool {
        **self == SUBSYS_DEBUG_MON_SEL_A::MONITOR_AUDIO
    }
}
impl core::ops::Deref for SUBSYS_DEBUG_MON_SEL_R {
    type Target = crate::FieldReader<u8, SUBSYS_DEBUG_MON_SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SUBSYS_DEBUG_MON_SEL` writer - Select which subsystem the debug monitors are routed from"]
pub struct SUBSYS_DEBUG_MON_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SUBSYS_DEBUG_MON_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SUBSYS_DEBUG_MON_SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Monitor A0 subsystem"]
    #[inline(always)]
    pub fn monitor_a0(self) -> &'a mut W {
        self.variant(SUBSYS_DEBUG_MON_SEL_A::MONITOR_A0)
    }
    #[doc = "Monitor the CPU subsystem"]
    #[inline(always)]
    pub fn monitor_cpu(self) -> &'a mut W {
        self.variant(SUBSYS_DEBUG_MON_SEL_A::MONITOR_CPU)
    }
    #[doc = "Monitor the FPGA Fabric subsystem"]
    #[inline(always)]
    pub fn monitor_ffe(self) -> &'a mut W {
        self.variant(SUBSYS_DEBUG_MON_SEL_A::MONITOR_FFE)
    }
    #[doc = "Monitor the audio subsystem"]
    #[inline(always)]
    pub fn monitor_audio(self) -> &'a mut W {
        self.variant(SUBSYS_DEBUG_MON_SEL_A::MONITOR_AUDIO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Select which subsystem the debug monitors are routed from"]
    #[inline(always)]
    pub fn subsys_debug_mon_sel(&self) -> SUBSYS_DEBUG_MON_SEL_R {
        SUBSYS_DEBUG_MON_SEL_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Select which subsystem the debug monitors are routed from"]
    #[inline(always)]
    pub fn subsys_debug_mon_sel(&mut self) -> SUBSYS_DEBUG_MON_SEL_W {
        SUBSYS_DEBUG_MON_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Register for selecting the subsystem routed to the debug monitor\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [subsys_dbg_mon_sel](index.html) module"]
pub struct SUBSYS_DBG_MON_SEL_SPEC;
impl crate::RegisterSpec for SUBSYS_DBG_MON_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [subsys_dbg_mon_sel::R](R) reader structure"]
impl crate::Readable for SUBSYS_DBG_MON_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [subsys_dbg_mon_sel::W](W) writer structure"]
impl crate::Writable for SUBSYS_DBG_MON_SEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SUBSYS_DBG_MON_SEL to value 0"]
impl crate::Resettable for SUBSYS_DBG_MON_SEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
