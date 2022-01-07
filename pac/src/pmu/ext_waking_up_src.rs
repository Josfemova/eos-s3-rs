#[doc = "Register `EXT_WAKING_UP_SRC` reader"]
pub struct R(crate::R<EXT_WAKING_UP_SRC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXT_WAKING_UP_SRC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXT_WAKING_UP_SRC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXT_WAKING_UP_SRC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXT_WAKING_UP_SRC` writer"]
pub struct W(crate::W<EXT_WAKING_UP_SRC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXT_WAKING_UP_SRC_SPEC>;
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
impl From<crate::W<EXT_WAKING_UP_SRC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXT_WAKING_UP_SRC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `Ext_WU_Source_Mask_N_0_GPIO_INT` reader - Bit 7:0 GPIO/Sensor INT, level only, (Raw information from PAD direclty) (Turn on OSC)"]
pub struct EXT_WU_SOURCE_MASK_N_0_GPIO_INT_R(crate::FieldReader<u8, u8>);
impl EXT_WU_SOURCE_MASK_N_0_GPIO_INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        EXT_WU_SOURCE_MASK_N_0_GPIO_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXT_WU_SOURCE_MASK_N_0_GPIO_INT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Ext_WU_Source_Mask_N_0_GPIO_INT` writer - Bit 7:0 GPIO/Sensor INT, level only, (Raw information from PAD direclty) (Turn on OSC)"]
pub struct EXT_WU_SOURCE_MASK_N_0_GPIO_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> EXT_WU_SOURCE_MASK_N_0_GPIO_INT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `Ext_WU_Source_Mask_N_0_PMU_Timer` reader - Bit 8 : PMU Timer"]
pub struct EXT_WU_SOURCE_MASK_N_0_PMU_TIMER_R(crate::FieldReader<bool, bool>);
impl EXT_WU_SOURCE_MASK_N_0_PMU_TIMER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EXT_WU_SOURCE_MASK_N_0_PMU_TIMER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXT_WU_SOURCE_MASK_N_0_PMU_TIMER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Ext_WU_Source_Mask_N_0_PMU_Timer` writer - Bit 8 : PMU Timer"]
pub struct EXT_WU_SOURCE_MASK_N_0_PMU_TIMER_W<'a> {
    w: &'a mut W,
}
impl<'a> EXT_WU_SOURCE_MASK_N_0_PMU_TIMER_W<'a> {
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
        self.w.bits =
            (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `Ext_WU_Source_Mask_N_0_Reset_Interrupt` reader - Bit 9 : Reset Interrupt INT (Always Mask)."]
pub struct EXT_WU_SOURCE_MASK_N_0_RESET_INTERRUPT_R(
    crate::FieldReader<bool, bool>,
);
impl EXT_WU_SOURCE_MASK_N_0_RESET_INTERRUPT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EXT_WU_SOURCE_MASK_N_0_RESET_INTERRUPT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXT_WU_SOURCE_MASK_N_0_RESET_INTERRUPT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Ext_WU_Source_Mask_N_0_Reset_Interrupt` writer - Bit 9 : Reset Interrupt INT (Always Mask)."]
pub struct EXT_WU_SOURCE_MASK_N_0_RESET_INTERRUPT_W<'a> {
    w: &'a mut W,
}
impl<'a> EXT_WU_SOURCE_MASK_N_0_RESET_INTERRUPT_W<'a> {
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
        self.w.bits =
            (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Bit 7:0 GPIO/Sensor INT, level only, (Raw information from PAD direclty) (Turn on OSC)"]
    #[inline(always)]
    pub fn ext_wu_source_mask_n_0_gpio_int(
        &self,
    ) -> EXT_WU_SOURCE_MASK_N_0_GPIO_INT_R {
        EXT_WU_SOURCE_MASK_N_0_GPIO_INT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Bit 8 : PMU Timer"]
    #[inline(always)]
    pub fn ext_wu_source_mask_n_0_pmu_timer(
        &self,
    ) -> EXT_WU_SOURCE_MASK_N_0_PMU_TIMER_R {
        EXT_WU_SOURCE_MASK_N_0_PMU_TIMER_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Bit 9 : Reset Interrupt INT (Always Mask)."]
    #[inline(always)]
    pub fn ext_wu_source_mask_n_0_reset_interrupt(
        &self,
    ) -> EXT_WU_SOURCE_MASK_N_0_RESET_INTERRUPT_R {
        EXT_WU_SOURCE_MASK_N_0_RESET_INTERRUPT_R::new(
            ((self.bits >> 9) & 0x01) != 0,
        )
    }
}
impl W {
    #[doc = "Bits 0:7 - Bit 7:0 GPIO/Sensor INT, level only, (Raw information from PAD direclty) (Turn on OSC)"]
    #[inline(always)]
    pub fn ext_wu_source_mask_n_0_gpio_int(
        &mut self,
    ) -> EXT_WU_SOURCE_MASK_N_0_GPIO_INT_W {
        EXT_WU_SOURCE_MASK_N_0_GPIO_INT_W { w: self }
    }
    #[doc = "Bit 8 - Bit 8 : PMU Timer"]
    #[inline(always)]
    pub fn ext_wu_source_mask_n_0_pmu_timer(
        &mut self,
    ) -> EXT_WU_SOURCE_MASK_N_0_PMU_TIMER_W {
        EXT_WU_SOURCE_MASK_N_0_PMU_TIMER_W { w: self }
    }
    #[doc = "Bit 9 - Bit 9 : Reset Interrupt INT (Always Mask)."]
    #[inline(always)]
    pub fn ext_wu_source_mask_n_0_reset_interrupt(
        &mut self,
    ) -> EXT_WU_SOURCE_MASK_N_0_RESET_INTERRUPT_W {
        EXT_WU_SOURCE_MASK_N_0_RESET_INTERRUPT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configure the external wakeup event source. Turn on the OSC once PMUT is time out or GPIO INT is triggering.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ext_waking_up_src](index.html) module"]
pub struct EXT_WAKING_UP_SRC_SPEC;
impl crate::RegisterSpec for EXT_WAKING_UP_SRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ext_waking_up_src::R](R) reader structure"]
impl crate::Readable for EXT_WAKING_UP_SRC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ext_waking_up_src::W](W) writer structure"]
impl crate::Writable for EXT_WAKING_UP_SRC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EXT_WAKING_UP_SRC to value 0"]
impl crate::Resettable for EXT_WAKING_UP_SRC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
