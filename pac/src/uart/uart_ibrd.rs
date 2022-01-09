#[doc = "Register `UART_IBRD` reader"]
pub struct R(crate::R<UART_IBRD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_IBRD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_IBRD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_IBRD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART_IBRD` writer"]
pub struct W(crate::W<UART_IBRD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_IBRD_SPEC>;
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
impl From<crate::W<UART_IBRD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_IBRD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UART_IBRD` reader - The integer baud rate divisor."]
pub struct UART_IBRD_R(crate::FieldReader<u16, u16>);
impl UART_IBRD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        UART_IBRD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART_IBRD_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART_IBRD` writer - The integer baud rate divisor."]
pub struct UART_IBRD_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_IBRD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - The integer baud rate divisor."]
    #[inline(always)]
    pub fn uart_ibrd(&self) -> UART_IBRD_R {
        UART_IBRD_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - The integer baud rate divisor."]
    #[inline(always)]
    pub fn uart_ibrd(&mut self) -> UART_IBRD_W {
        UART_IBRD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The integer baud rate divisor.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_ibrd](index.html) module"]
pub struct UART_IBRD_SPEC;
impl crate::RegisterSpec for UART_IBRD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_ibrd::R](R) reader structure"]
impl crate::Readable for UART_IBRD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_ibrd::W](W) writer structure"]
impl crate::Writable for UART_IBRD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART_IBRD to value 0"]
impl crate::Resettable for UART_IBRD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
