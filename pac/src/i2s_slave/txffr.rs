#[doc = "Register `TXFFR` writer"]
pub struct W(crate::W<TXFFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXFFR_SPEC>;
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
impl From<crate::W<TXFFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXFFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXFFR` writer - Transmitter FIFO Reset. Writing a 1 to this register flushes all the TX FIFOs (this is a self clearing bit). The Transmitter Block must be disabled prior to writing this bit."]
pub struct TXFFR_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFFR_W<'a> {
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
impl W {
    #[doc = "Bit 0 - Transmitter FIFO Reset. Writing a 1 to this register flushes all the TX FIFOs (this is a self clearing bit). The Transmitter Block must be disabled prior to writing this bit."]
    #[inline(always)]
    pub fn txffr(&mut self) -> TXFFR_W {
        TXFFR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmitter Block FIFO Reset Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txffr](index.html) module"]
pub struct TXFFR_SPEC;
impl crate::RegisterSpec for TXFFR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [txffr::W](W) writer structure"]
impl crate::Writable for TXFFR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TXFFR to value 0"]
impl crate::Resettable for TXFFR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
