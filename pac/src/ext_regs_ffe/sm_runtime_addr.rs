#[doc = "Register `SM_RUNTIME_ADDR` reader"]
pub struct R(crate::R<SM_RUNTIME_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SM_RUNTIME_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SM_RUNTIME_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SM_RUNTIME_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SM_RUNTIME_ADDR` writer"]
pub struct W(crate::W<SM_RUNTIME_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SM_RUNTIME_ADDR_SPEC>;
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
impl From<crate::W<SM_RUNTIME_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SM_RUNTIME_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SM_RUNTIME_ADDR` reader - SM0/SM1 run time address"]
pub struct SM_RUNTIME_ADDR_R(crate::FieldReader<u16, u16>);
impl SM_RUNTIME_ADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        SM_RUNTIME_ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SM_RUNTIME_ADDR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SM_RUNTIME_ADDR` writer - SM0/SM1 run time address"]
pub struct SM_RUNTIME_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> SM_RUNTIME_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - SM0/SM1 run time address"]
    #[inline(always)]
    pub fn sm_runtime_addr(&self) -> SM_RUNTIME_ADDR_R {
        SM_RUNTIME_ADDR_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - SM0/SM1 run time address"]
    #[inline(always)]
    pub fn sm_runtime_addr(&mut self) -> SM_RUNTIME_ADDR_W {
        SM_RUNTIME_ADDR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SM0/SM1 run time address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sm_runtime_addr](index.html) module"]
pub struct SM_RUNTIME_ADDR_SPEC;
impl crate::RegisterSpec for SM_RUNTIME_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sm_runtime_addr::R](R) reader structure"]
impl crate::Readable for SM_RUNTIME_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sm_runtime_addr::W](W) writer structure"]
impl crate::Writable for SM_RUNTIME_ADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SM_RUNTIME_ADDR to value 0"]
impl crate::Resettable for SM_RUNTIME_ADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
