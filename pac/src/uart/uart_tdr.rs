#[doc = "Register `UART_TDR` reader"]
pub struct R(crate::R<UART_TDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_TDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_TDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_TDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART_TDR` writer"]
pub struct W(crate::W<UART_TDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_TDR_SPEC>;
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
impl From<crate::W<UART_TDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_TDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UART_TDR` reader - Test data"]
pub struct UART_TDR_R(crate::FieldReader<u16, u16>);
impl UART_TDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        UART_TDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART_TDR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART_TDR` writer - Test data"]
pub struct UART_TDR_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_TDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | (value as u32 & 0x07ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:10 - Test data"]
    #[inline(always)]
    pub fn uart_tdr(&self) -> UART_TDR_R {
        UART_TDR_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Test data"]
    #[inline(always)]
    pub fn uart_tdr(&mut self) -> UART_TDR_W {
        UART_TDR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Test data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_tdr](index.html) module"]
pub struct UART_TDR_SPEC;
impl crate::RegisterSpec for UART_TDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_tdr::R](R) reader structure"]
impl crate::Readable for UART_TDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_tdr::W](W) writer structure"]
impl crate::Writable for UART_TDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART_TDR to value 0"]
impl crate::Resettable for UART_TDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
