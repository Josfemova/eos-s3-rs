#[doc = "Register `UART_ILPR` reader"]
pub struct R(crate::R<UART_ILPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_ILPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_ILPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_ILPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART_ILPR` writer"]
pub struct W(crate::W<UART_ILPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_ILPR_SPEC>;
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
impl From<crate::W<UART_ILPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_ILPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UART_ILPR` reader - 8-bit low-power divisor value."]
pub struct UART_ILPR_R(crate::FieldReader<u8, u8>);
impl UART_ILPR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        UART_ILPR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART_ILPR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART_ILPR` writer - 8-bit low-power divisor value."]
pub struct UART_ILPR_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_ILPR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - 8-bit low-power divisor value."]
    #[inline(always)]
    pub fn uart_ilpr(&self) -> UART_ILPR_R {
        UART_ILPR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8-bit low-power divisor value."]
    #[inline(always)]
    pub fn uart_ilpr(&mut self) -> UART_ILPR_W {
        UART_ILPR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "8-bit low-power divisor value.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_ilpr](index.html) module"]
pub struct UART_ILPR_SPEC;
impl crate::RegisterSpec for UART_ILPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_ilpr::R](R) reader structure"]
impl crate::Readable for UART_ILPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_ilpr::W](W) writer structure"]
impl crate::Writable for UART_ILPR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART_ILPR to value 0"]
impl crate::Resettable for UART_ILPR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
