#[doc = "Register `PWR_OFF_OSC` reader"]
pub struct R(crate::R<PWR_OFF_OSC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWR_OFF_OSC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWR_OFF_OSC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWR_OFF_OSC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWR_OFF_OSC` writer"]
pub struct W(crate::W<PWR_OFF_OSC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWR_OFF_OSC_SPEC>;
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
impl From<crate::W<PWR_OFF_OSC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWR_OFF_OSC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `Power_Off_OSC` reader - 1'b1 : Turn Off OSC once the M4 is in Power Saving Mode"]
pub struct POWER_OFF_OSC_R(crate::FieldReader<bool, bool>);
impl POWER_OFF_OSC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        POWER_OFF_OSC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POWER_OFF_OSC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Power_Off_OSC` writer - 1'b1 : Turn Off OSC once the M4 is in Power Saving Mode"]
pub struct POWER_OFF_OSC_W<'a> {
    w: &'a mut W,
}
impl<'a> POWER_OFF_OSC_W<'a> {
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
    #[doc = "Bit 0 - 1'b1 : Turn Off OSC once the M4 is in Power Saving Mode"]
    #[inline(always)]
    pub fn power_off_osc(&self) -> POWER_OFF_OSC_R {
        POWER_OFF_OSC_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1'b1 : Turn Off OSC once the M4 is in Power Saving Mode"]
    #[inline(always)]
    pub fn power_off_osc(&mut self) -> POWER_OFF_OSC_W {
        POWER_OFF_OSC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control the power state of Oscillator once the M4 is in Power Saving Mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_off_osc](index.html) module"]
pub struct PWR_OFF_OSC_SPEC;
impl crate::RegisterSpec for PWR_OFF_OSC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwr_off_osc::R](R) reader structure"]
impl crate::Readable for PWR_OFF_OSC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwr_off_osc::W](W) writer structure"]
impl crate::Writable for PWR_OFF_OSC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWR_OFF_OSC to value 0"]
impl crate::Resettable for PWR_OFF_OSC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
