#[doc = "Register `SLEEP_MODE` reader"]
pub struct R(crate::R<SLEEP_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLEEP_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLEEP_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLEEP_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLEEP_MODE` writer"]
pub struct W(crate::W<SLEEP_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLEEP_MODE_SPEC>;
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
impl From<crate::W<SLEEP_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLEEP_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLEEP_MODE` reader - If set, the PMU and FFE kick off signal will be blocked. This bit will be cleared by HW if any of non-Mask INT triggered"]
pub struct SLEEP_MODE_R(crate::FieldReader<bool, bool>);
impl SLEEP_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLEEP_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLEEP_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLEEP_MODE` writer - If set, the PMU and FFE kick off signal will be blocked. This bit will be cleared by HW if any of non-Mask INT triggered"]
pub struct SLEEP_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLEEP_MODE_W<'a> {
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
    #[doc = "Bit 0 - If set, the PMU and FFE kick off signal will be blocked. This bit will be cleared by HW if any of non-Mask INT triggered"]
    #[inline(always)]
    pub fn sleep_mode(&self) -> SLEEP_MODE_R {
        SLEEP_MODE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - If set, the PMU and FFE kick off signal will be blocked. This bit will be cleared by HW if any of non-Mask INT triggered"]
    #[inline(always)]
    pub fn sleep_mode(&mut self) -> SLEEP_MODE_W {
        SLEEP_MODE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Allows blocking the PMU and FFE kickoff signal\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sleep_mode](index.html) module"]
pub struct SLEEP_MODE_SPEC;
impl crate::RegisterSpec for SLEEP_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sleep_mode::R](R) reader structure"]
impl crate::Readable for SLEEP_MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sleep_mode::W](W) writer structure"]
impl crate::Writable for SLEEP_MODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SLEEP_MODE to value 0"]
impl crate::Resettable for SLEEP_MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
