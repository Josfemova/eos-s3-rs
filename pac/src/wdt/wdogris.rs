#[doc = "Register `WDOGRIS` reader"]
pub struct R(crate::R<WDOGRIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDOGRIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDOGRIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDOGRIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WDOGRIS` writer"]
pub struct W(crate::W<WDOGRIS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDOGRIS_SPEC>;
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
impl From<crate::W<WDOGRIS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDOGRIS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDOGRIS` reader - The WDOGRIS Register indicates the raw interrupt status from the counter. This value is ANDed with the interrupt enable bit from the control register to create the masked interrupt, that is passed to the interrupt output pin."]
pub struct WDOGRIS_R(crate::FieldReader<bool, bool>);
impl WDOGRIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WDOGRIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDOGRIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDOGRIS` writer - The WDOGRIS Register indicates the raw interrupt status from the counter. This value is ANDed with the interrupt enable bit from the control register to create the masked interrupt, that is passed to the interrupt output pin."]
pub struct WDOGRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> WDOGRIS_W<'a> {
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
    #[doc = "Bit 0 - The WDOGRIS Register indicates the raw interrupt status from the counter. This value is ANDed with the interrupt enable bit from the control register to create the masked interrupt, that is passed to the interrupt output pin."]
    #[inline(always)]
    pub fn wdogris(&self) -> WDOGRIS_R {
        WDOGRIS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The WDOGRIS Register indicates the raw interrupt status from the counter. This value is ANDed with the interrupt enable bit from the control register to create the masked interrupt, that is passed to the interrupt output pin."]
    #[inline(always)]
    pub fn wdogris(&mut self) -> WDOGRIS_W {
        WDOGRIS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The WDOGRIS Register indicates the raw interrupt status from the counter. This value is ANDed with the interrupt enable bit from the control register to create the masked interrupt, that is passed to the interrupt output pin.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdogris](index.html) module"]
pub struct WDOGRIS_SPEC;
impl crate::RegisterSpec for WDOGRIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdogris::R](R) reader structure"]
impl crate::Readable for WDOGRIS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdogris::W](W) writer structure"]
impl crate::Writable for WDOGRIS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WDOGRIS to value 0"]
impl crate::Resettable for WDOGRIS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
