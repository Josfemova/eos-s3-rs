#[doc = "Register `PDM_STAT_IN_SEL` reader"]
pub struct R(crate::R<PDM_STAT_IN_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDM_STAT_IN_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDM_STAT_IN_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDM_STAT_IN_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDM_STAT_IN_SEL` writer"]
pub struct W(crate::W<PDM_STAT_IN_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDM_STAT_IN_SEL_SPEC>;
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
impl From<crate::W<PDM_STAT_IN_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDM_STAT_IN_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL` reader - Sel"]
pub struct SEL_R(crate::FieldReader<u8, u8>);
impl SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEL_R {
    type Target = crate::FieldReader<u8, u8>;
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
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Sel"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Sel"]
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
#[doc = "Selects pin for PDM STATUS_IN function\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdm_stat_in_sel](index.html) module"]
pub struct PDM_STAT_IN_SEL_SPEC;
impl crate::RegisterSpec for PDM_STAT_IN_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdm_stat_in_sel::R](R) reader structure"]
impl crate::Readable for PDM_STAT_IN_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdm_stat_in_sel::W](W) writer structure"]
impl crate::Writable for PDM_STAT_IN_SEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDM_STAT_IN_SEL to value 0"]
impl crate::Resettable for PDM_STAT_IN_SEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
