#[doc = "Register `PDM_CLKIN_SEL` reader"]
pub struct R(crate::R<PDM_CLKIN_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDM_CLKIN_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDM_CLKIN_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDM_CLKIN_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDM_CLKIN_SEL` writer"]
pub struct W(crate::W<PDM_CLKIN_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDM_CLKIN_SEL_SPEC>;
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
impl From<crate::W<PDM_CLKIN_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDM_CLKIN_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL` reader - Sel"]
pub struct SEL_R(crate::FieldReader<bool, bool>);
impl SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEL` writer - Sel"]
pub struct SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL_W<'a> {
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
    #[doc = "Bit 0 - Sel"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Sel"]
    #[inline(always)]
    pub fn sel(&mut self) -> SEL_W {
        SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Selects pin for PDM CLKIN source (AP PDM CKO IN)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdm_clkin_sel](index.html) module"]
pub struct PDM_CLKIN_SEL_SPEC;
impl crate::RegisterSpec for PDM_CLKIN_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdm_clkin_sel::R](R) reader structure"]
impl crate::Readable for PDM_CLKIN_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdm_clkin_sel::W](W) writer structure"]
impl crate::Writable for PDM_CLKIN_SEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDM_CLKIN_SEL to value 0"]
impl crate::Resettable for PDM_CLKIN_SEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
