#[doc = "Register `SM1_DEBUG_SEL` reader"]
pub struct R(crate::R<SM1_DEBUG_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SM1_DEBUG_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SM1_DEBUG_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SM1_DEBUG_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SM1_DEBUG_SEL` writer"]
pub struct W(crate::W<SM1_DEBUG_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SM1_DEBUG_SEL_SPEC>;
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
impl From<crate::W<SM1_DEBUG_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SM1_DEBUG_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SM1_DEBUG_SEL` reader - SM Debug selection"]
pub struct SM1_DEBUG_SEL_R(crate::FieldReader<u8, u8>);
impl SM1_DEBUG_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SM1_DEBUG_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SM1_DEBUG_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SM1_DEBUG_SEL` writer - SM Debug selection"]
pub struct SM1_DEBUG_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SM1_DEBUG_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - SM Debug selection"]
    #[inline(always)]
    pub fn sm1_debug_sel(&self) -> SM1_DEBUG_SEL_R {
        SM1_DEBUG_SEL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SM Debug selection"]
    #[inline(always)]
    pub fn sm1_debug_sel(&mut self) -> SM1_DEBUG_SEL_W {
        SM1_DEBUG_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SM Debug selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sm1_debug_sel](index.html) module"]
pub struct SM1_DEBUG_SEL_SPEC;
impl crate::RegisterSpec for SM1_DEBUG_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sm1_debug_sel::R](R) reader structure"]
impl crate::Readable for SM1_DEBUG_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sm1_debug_sel::W](W) writer structure"]
impl crate::Writable for SM1_DEBUG_SEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SM1_DEBUG_SEL to value 0"]
impl crate::Resettable for SM1_DEBUG_SEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
