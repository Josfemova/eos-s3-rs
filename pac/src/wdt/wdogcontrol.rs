#[doc = "Register `WDOGCONTROL` reader"]
pub struct R(crate::R<WDOGCONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDOGCONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDOGCONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDOGCONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WDOGCONTROL` writer"]
pub struct W(crate::W<WDOGCONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDOGCONTROL_SPEC>;
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
impl From<crate::W<WDOGCONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDOGCONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTEN` reader - Enable the interrupt event, WDOGINT. Set HIGH to enable the counter and the interrupt, or LOW to diable the counter and interrupt. Reloads the counter from the value in WDOGLAND when the interrupt is enabled, after previously being disabled."]
pub struct INTEN_R(crate::FieldReader<bool, bool>);
impl INTEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INTEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTEN` writer - Enable the interrupt event, WDOGINT. Set HIGH to enable the counter and the interrupt, or LOW to diable the counter and interrupt. Reloads the counter from the value in WDOGLAND when the interrupt is enabled, after previously being disabled."]
pub struct INTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> INTEN_W<'a> {
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
#[doc = "Field `RESEN` reader - Enable watchdog reset output, WDOGRES. Acts as a mask for the reset output. Set HIGH to enable the reset or LOW to disable the reset."]
pub struct RESEN_R(crate::FieldReader<bool, bool>);
impl RESEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RESEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESEN` writer - Enable watchdog reset output, WDOGRES. Acts as a mask for the reset output. Set HIGH to enable the reset or LOW to disable the reset."]
pub struct RESEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RESEN_W<'a> {
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
            (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable the interrupt event, WDOGINT. Set HIGH to enable the counter and the interrupt, or LOW to diable the counter and interrupt. Reloads the counter from the value in WDOGLAND when the interrupt is enabled, after previously being disabled."]
    #[inline(always)]
    pub fn inten(&self) -> INTEN_R {
        INTEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable watchdog reset output, WDOGRES. Acts as a mask for the reset output. Set HIGH to enable the reset or LOW to disable the reset."]
    #[inline(always)]
    pub fn resen(&self) -> RESEN_R {
        RESEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable the interrupt event, WDOGINT. Set HIGH to enable the counter and the interrupt, or LOW to diable the counter and interrupt. Reloads the counter from the value in WDOGLAND when the interrupt is enabled, after previously being disabled."]
    #[inline(always)]
    pub fn inten(&mut self) -> INTEN_W {
        INTEN_W { w: self }
    }
    #[doc = "Bit 1 - Enable watchdog reset output, WDOGRES. Acts as a mask for the reset output. Set HIGH to enable the reset or LOW to disable the reset."]
    #[inline(always)]
    pub fn resen(&mut self) -> RESEN_W {
        RESEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control register for the WatchDog timer\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdogcontrol](index.html) module"]
pub struct WDOGCONTROL_SPEC;
impl crate::RegisterSpec for WDOGCONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdogcontrol::R](R) reader structure"]
impl crate::Readable for WDOGCONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdogcontrol::W](W) writer structure"]
impl crate::Writable for WDOGCONTROL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WDOGCONTROL to value 0"]
impl crate::Resettable for WDOGCONTROL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
