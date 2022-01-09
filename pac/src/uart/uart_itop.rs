#[doc = "Register `UART_ITOP` reader"]
pub struct R(crate::R<UART_ITOP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_ITOP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_ITOP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_ITOP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART_ITOP` writer"]
pub struct W(crate::W<UART_ITOP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_ITOP_SPEC>;
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
impl From<crate::W<UART_ITOP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_ITOP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UART_ITOP` reader - Integration test output."]
pub struct UART_ITOP_R(crate::FieldReader<u16, u16>);
impl UART_ITOP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        UART_ITOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART_ITOP_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART_ITOP` writer - Integration test output."]
pub struct UART_ITOP_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_ITOP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Integration test output."]
    #[inline(always)]
    pub fn uart_itop(&self) -> UART_ITOP_R {
        UART_ITOP_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Integration test output."]
    #[inline(always)]
    pub fn uart_itop(&mut self) -> UART_ITOP_W {
        UART_ITOP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Integration test output register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_itop](index.html) module"]
pub struct UART_ITOP_SPEC;
impl crate::RegisterSpec for UART_ITOP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_itop::R](R) reader structure"]
impl crate::Readable for UART_ITOP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_itop::W](W) writer structure"]
impl crate::Writable for UART_ITOP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART_ITOP to value 0"]
impl crate::Resettable for UART_ITOP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
