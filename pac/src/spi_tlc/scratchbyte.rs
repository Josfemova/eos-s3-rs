#[doc = "Register `SCRATCHBYTE` reader"]
pub struct R(crate::R<SCRATCHBYTE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCRATCHBYTE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCRATCHBYTE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCRATCHBYTE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCRATCHBYTE` writer"]
pub struct W(crate::W<SCRATCHBYTE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCRATCHBYTE_SPEC>;
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
impl From<crate::W<SCRATCHBYTE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCRATCHBYTE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ScratchByte` reader - General Purpose Registers, R/W, Default 0"]
pub struct SCRATCHBYTE_R(crate::FieldReader<u8, u8>);
impl SCRATCHBYTE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SCRATCHBYTE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCRATCHBYTE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ScratchByte` writer - General Purpose Registers, R/W, Default 0"]
pub struct SCRATCHBYTE_W<'a> {
    w: &'a mut W,
}
impl<'a> SCRATCHBYTE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = value as u8;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - General Purpose Registers, R/W, Default 0"]
    #[inline(always)]
    pub fn scratch_byte(&self) -> SCRATCHBYTE_R {
        SCRATCHBYTE_R::new(self.bits as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - General Purpose Registers, R/W, Default 0"]
    #[inline(always)]
    pub fn scratch_byte(&mut self) -> SCRATCHBYTE_W {
        SCRATCHBYTE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General Purpose Registers, R/W, Default 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scratchbyte](index.html) module"]
pub struct SCRATCHBYTE_SPEC;
impl crate::RegisterSpec for SCRATCHBYTE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [scratchbyte::R](R) reader structure"]
impl crate::Readable for SCRATCHBYTE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scratchbyte::W](W) writer structure"]
impl crate::Writable for SCRATCHBYTE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCRATCHBYTE to value 0"]
impl crate::Resettable for SCRATCHBYTE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
