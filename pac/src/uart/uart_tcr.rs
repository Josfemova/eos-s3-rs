#[doc = "Register `UART_TCR` writer"]
pub struct W(crate::W<UART_TCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_TCR_SPEC>;
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
impl From<crate::W<UART_TCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_TCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ITEN` writer - Integration test enable."]
pub struct ITEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ITEN_W<'a> {
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
#[doc = "Field `TESTFINFO` writer - Test FIFO enable."]
pub struct TESTFINFO_W<'a> {
    w: &'a mut W,
}
impl<'a> TESTFINFO_W<'a> {
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
        self.w.bits =
            (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `SIRTEST` writer - Siren test enable."]
pub struct SIRTEST_W<'a> {
    w: &'a mut W,
}
impl<'a> SIRTEST_W<'a> {
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
        self.w.bits =
            (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Integration test enable."]
    #[inline(always)]
    pub fn iten(&mut self) -> ITEN_W {
        ITEN_W { w: self }
    }
    #[doc = "Bit 1 - Test FIFO enable."]
    #[inline(always)]
    pub fn testfinfo(&mut self) -> TESTFINFO_W {
        TESTFINFO_W { w: self }
    }
    #[doc = "Bit 2 - Siren test enable."]
    #[inline(always)]
    pub fn sirtest(&mut self) -> SIRTEST_W {
        SIRTEST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Test Control Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_tcr](index.html) module"]
pub struct UART_TCR_SPEC;
impl crate::RegisterSpec for UART_TCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [uart_tcr::W](W) writer structure"]
impl crate::Writable for UART_TCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART_TCR to value 0"]
impl crate::Resettable for UART_TCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
