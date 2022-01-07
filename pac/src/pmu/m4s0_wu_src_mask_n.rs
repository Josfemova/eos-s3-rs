#[doc = "Register `M4S0_WU_SRC_MASK_N` reader"]
pub struct R(crate::R<M4S0_WU_SRC_MASK_N_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<M4S0_WU_SRC_MASK_N_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<M4S0_WU_SRC_MASK_N_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<M4S0_WU_SRC_MASK_N_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `M4S0_WU_SRC_MASK_N` writer"]
pub struct W(crate::W<M4S0_WU_SRC_MASK_N_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<M4S0_WU_SRC_MASK_N_SPEC>;
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
impl From<crate::W<M4S0_WU_SRC_MASK_N_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<M4S0_WU_SRC_MASK_N_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `M4S0_PD_Event_Mask` reader - If not mask, The M4S0 power domain will wake up once M4 is waking up from Power Saving and M4S0 power domain is NOT in active mode."]
pub struct M4S0_PD_EVENT_MASK_R(crate::FieldReader<bool, bool>);
impl M4S0_PD_EVENT_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        M4S0_PD_EVENT_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M4S0_PD_EVENT_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M4S0_PD_Event_Mask` writer - If not mask, The M4S0 power domain will wake up once M4 is waking up from Power Saving and M4S0 power domain is NOT in active mode."]
pub struct M4S0_PD_EVENT_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> M4S0_PD_EVENT_MASK_W<'a> {
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
    #[doc = "Bit 0 - If not mask, The M4S0 power domain will wake up once M4 is waking up from Power Saving and M4S0 power domain is NOT in active mode."]
    #[inline(always)]
    pub fn m4s0_pd_event_mask(&self) -> M4S0_PD_EVENT_MASK_R {
        M4S0_PD_EVENT_MASK_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - If not mask, The M4S0 power domain will wake up once M4 is waking up from Power Saving and M4S0 power domain is NOT in active mode."]
    #[inline(always)]
    pub fn m4s0_pd_event_mask(&mut self) -> M4S0_PD_EVENT_MASK_W {
        M4S0_PD_EVENT_MASK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control masking of wake-up event triggers for the M4S0 SRAM domain\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m4s0_wu_src_mask_n](index.html) module"]
pub struct M4S0_WU_SRC_MASK_N_SPEC;
impl crate::RegisterSpec for M4S0_WU_SRC_MASK_N_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [m4s0_wu_src_mask_n::R](R) reader structure"]
impl crate::Readable for M4S0_WU_SRC_MASK_N_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [m4s0_wu_src_mask_n::W](W) writer structure"]
impl crate::Writable for M4S0_WU_SRC_MASK_N_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets M4S0_WU_SRC_MASK_N to value 0"]
impl crate::Resettable for M4S0_WU_SRC_MASK_N_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
