#[doc = "Register `SPARE_BITS` reader"]
pub struct R(crate::R<SPARE_BITS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPARE_BITS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPARE_BITS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPARE_BITS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPARE_BITS` writer"]
pub struct W(crate::W<SPARE_BITS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPARE_BITS_SPEC>;
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
impl From<crate::W<SPARE_BITS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPARE_BITS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPARE_BITS` reader - "]
pub struct SPARE_BITS_R(crate::FieldReader<u8, u8>);
impl SPARE_BITS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SPARE_BITS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPARE_BITS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPARE_BITS` writer - "]
pub struct SPARE_BITS_W<'a> {
    w: &'a mut W,
}
impl<'a> SPARE_BITS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn spare_bits(&self) -> SPARE_BITS_R {
        SPARE_BITS_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn spare_bits(&mut self) -> SPARE_BITS_W {
        SPARE_BITS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spare_bits](index.html) module"]
pub struct SPARE_BITS_SPEC;
impl crate::RegisterSpec for SPARE_BITS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spare_bits::R](R) reader structure"]
impl crate::Readable for SPARE_BITS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spare_bits::W](W) writer structure"]
impl crate::Writable for SPARE_BITS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPARE_BITS to value 0"]
impl crate::Resettable for SPARE_BITS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
