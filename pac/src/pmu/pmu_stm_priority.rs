#[doc = "Register `PMU_STM_PRIORITY` reader"]
pub struct R(crate::R<PMU_STM_PRIORITY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMU_STM_PRIORITY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMU_STM_PRIORITY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMU_STM_PRIORITY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMU_STM_PRIORITY` writer"]
pub struct W(crate::W<PMU_STM_PRIORITY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMU_STM_PRIORITY_SPEC>;
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
impl From<crate::W<PMU_STM_PRIORITY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMU_STM_PRIORITY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Controls how PMU STM and M4/FFE STM will be arbitrated.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PMU_STM_PRIORITY_A {
    #[doc = "0: Audio/SRAM PMU STM (State Machine) and M4/FFE PMU STM could run at same time."]
    SIMULTANEOUS = 0,
    #[doc = "1: Audio/SRAM PMU STM and M4/FFE PMU STM will be arbitrate in Round Robin Scheme"]
    ROUND_ROBIN = 1,
}
impl From<PMU_STM_PRIORITY_A> for bool {
    #[inline(always)]
    fn from(variant: PMU_STM_PRIORITY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PMU_STM_PRIORITY` reader - Controls how PMU STM and M4/FFE STM will be arbitrated."]
pub struct PMU_STM_PRIORITY_R(crate::FieldReader<bool, PMU_STM_PRIORITY_A>);
impl PMU_STM_PRIORITY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PMU_STM_PRIORITY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PMU_STM_PRIORITY_A {
        match self.bits {
            false => PMU_STM_PRIORITY_A::SIMULTANEOUS,
            true => PMU_STM_PRIORITY_A::ROUND_ROBIN,
        }
    }
    #[doc = "Checks if the value of the field is `SIMULTANEOUS`"]
    #[inline(always)]
    pub fn is_simultaneous(&self) -> bool {
        **self == PMU_STM_PRIORITY_A::SIMULTANEOUS
    }
    #[doc = "Checks if the value of the field is `ROUND_ROBIN`"]
    #[inline(always)]
    pub fn is_round_robin(&self) -> bool {
        **self == PMU_STM_PRIORITY_A::ROUND_ROBIN
    }
}
impl core::ops::Deref for PMU_STM_PRIORITY_R {
    type Target = crate::FieldReader<bool, PMU_STM_PRIORITY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMU_STM_PRIORITY` writer - Controls how PMU STM and M4/FFE STM will be arbitrated."]
pub struct PMU_STM_PRIORITY_W<'a> {
    w: &'a mut W,
}
impl<'a> PMU_STM_PRIORITY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PMU_STM_PRIORITY_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Audio/SRAM PMU STM (State Machine) and M4/FFE PMU STM could run at same time."]
    #[inline(always)]
    pub fn simultaneous(self) -> &'a mut W {
        self.variant(PMU_STM_PRIORITY_A::SIMULTANEOUS)
    }
    #[doc = "Audio/SRAM PMU STM and M4/FFE PMU STM will be arbitrate in Round Robin Scheme"]
    #[inline(always)]
    pub fn round_robin(self) -> &'a mut W {
        self.variant(PMU_STM_PRIORITY_A::ROUND_ROBIN)
    }
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
    #[doc = "Bit 0 - Controls how PMU STM and M4/FFE STM will be arbitrated."]
    #[inline(always)]
    pub fn pmu_stm_priority(&self) -> PMU_STM_PRIORITY_R {
        PMU_STM_PRIORITY_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Controls how PMU STM and M4/FFE STM will be arbitrated."]
    #[inline(always)]
    pub fn pmu_stm_priority(&mut self) -> PMU_STM_PRIORITY_W {
        PMU_STM_PRIORITY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power Management Unit Software Test Mode priority control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmu_stm_priority](index.html) module"]
pub struct PMU_STM_PRIORITY_SPEC;
impl crate::RegisterSpec for PMU_STM_PRIORITY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pmu_stm_priority::R](R) reader structure"]
impl crate::Readable for PMU_STM_PRIORITY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmu_stm_priority::W](W) writer structure"]
impl crate::Writable for PMU_STM_PRIORITY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PMU_STM_PRIORITY to value 0"]
impl crate::Resettable for PMU_STM_PRIORITY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
