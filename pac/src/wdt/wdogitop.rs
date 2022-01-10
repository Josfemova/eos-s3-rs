#[doc = "Register `WDOGITOP` writer"]
pub struct W(crate::W<WDOGITOP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDOGITOP_SPEC>;
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
impl From<crate::W<WDOGITOP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDOGITOP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Watchdog Integration Test Output Set Register When the WDOGITOP Register is in integration test mode, the values in this register directly drive the enabled interrupt output and reset output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WDOGITOP_AW {
    #[doc = "0: Select integration Test with WDOCRES value"]
    WDOCRES = 0,
    #[doc = "1: Select integration Test with WDOGINT value"]
    WDOGINT = 1,
}
impl From<WDOGITOP_AW> for u8 {
    #[inline(always)]
    fn from(variant: WDOGITOP_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `WDOGITOP` writer - Watchdog Integration Test Output Set Register When the WDOGITOP Register is in integration test mode, the values in this register directly drive the enabled interrupt output and reset output."]
pub struct WDOGITOP_W<'a> {
    w: &'a mut W,
}
impl<'a> WDOGITOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDOGITOP_AW) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select integration Test with WDOCRES value"]
    #[inline(always)]
    pub fn wdocres(self) -> &'a mut W {
        self.variant(WDOGITOP_AW::WDOCRES)
    }
    #[doc = "Select integration Test with WDOGINT value"]
    #[inline(always)]
    pub fn wdogint(self) -> &'a mut W {
        self.variant(WDOGITOP_AW::WDOGINT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:1 - Watchdog Integration Test Output Set Register When the WDOGITOP Register is in integration test mode, the values in this register directly drive the enabled interrupt output and reset output."]
    #[inline(always)]
    pub fn wdogitop(&mut self) -> WDOGITOP_W {
        WDOGITOP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Watchdog Integration Test Output Set Register When the WDOGITOP Register is in integration test mode, the values in this register directly drive the enabled interrupt output and reset output.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdogitop](index.html) module"]
pub struct WDOGITOP_SPEC;
impl crate::RegisterSpec for WDOGITOP_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [wdogitop::W](W) writer structure"]
impl crate::Writable for WDOGITOP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WDOGITOP to value 0"]
impl crate::Resettable for WDOGITOP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
