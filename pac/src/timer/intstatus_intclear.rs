#[doc = "Register `INTSTATUS_INTCLEAR` reader"]
pub struct R(crate::R<INTSTATUS_INTCLEAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTSTATUS_INTCLEAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTSTATUS_INTCLEAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTSTATUS_INTCLEAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTSTATUS_INTCLEAR` writer"]
pub struct W(crate::W<INTSTATUS_INTCLEAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTSTATUS_INTCLEAR_SPEC>;
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
impl From<crate::W<INTSTATUS_INTCLEAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTSTATUS_INTCLEAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTSTATUS_INTCLEAR` reader - Timer interrupt, Write one to clear"]
pub struct INTSTATUS_INTCLEAR_R(crate::FieldReader<bool, bool>);
impl INTSTATUS_INTCLEAR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INTSTATUS_INTCLEAR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTSTATUS_INTCLEAR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTSTATUS_INTCLEAR` writer - Timer interrupt, Write one to clear"]
pub struct INTSTATUS_INTCLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> INTSTATUS_INTCLEAR_W<'a> {
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
    #[doc = "Bit 0 - Timer interrupt, Write one to clear"]
    #[inline(always)]
    pub fn intstatus_intclear(&self) -> INTSTATUS_INTCLEAR_R {
        INTSTATUS_INTCLEAR_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer interrupt, Write one to clear"]
    #[inline(always)]
    pub fn intstatus_intclear(&mut self) -> INTSTATUS_INTCLEAR_W {
        INTSTATUS_INTCLEAR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer interrupt. Write one to clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intstatus_intclear](index.html) module"]
pub struct INTSTATUS_INTCLEAR_SPEC;
impl crate::RegisterSpec for INTSTATUS_INTCLEAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intstatus_intclear::R](R) reader structure"]
impl crate::Readable for INTSTATUS_INTCLEAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intstatus_intclear::W](W) writer structure"]
impl crate::Writable for INTSTATUS_INTCLEAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTSTATUS_INTCLEAR to value 0"]
impl crate::Resettable for INTSTATUS_INTCLEAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
