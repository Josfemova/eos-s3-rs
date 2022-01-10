#[doc = "Register `WDOGINTCLR` writer"]
pub struct W(crate::W<WDOGINTCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDOGINTCLR_SPEC>;
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
impl From<crate::W<WDOGINTCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDOGINTCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDOGINTCLR` writer - A write of any value to the WDOGINTCLR Register clears the watchdog interrupt, and reloads the counter from the value in WDOGLOAD."]
pub struct WDOGINTCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> WDOGINTCLR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - A write of any value to the WDOGINTCLR Register clears the watchdog interrupt, and reloads the counter from the value in WDOGLOAD."]
    #[inline(always)]
    pub fn wdogintclr(&mut self) -> WDOGINTCLR_W {
        WDOGINTCLR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A write of any value to the WDOGINTCLR Register clears the watchdog interrupt, and reloads the counter from the value in WDOGLOAD.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdogintclr](index.html) module"]
pub struct WDOGINTCLR_SPEC;
impl crate::RegisterSpec for WDOGINTCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [wdogintclr::W](W) writer structure"]
impl crate::Writable for WDOGINTCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WDOGINTCLR to value 0"]
impl crate::Resettable for WDOGINTCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
