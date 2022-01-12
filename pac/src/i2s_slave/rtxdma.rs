#[doc = "Register `RTXDMA` writer"]
pub struct W(crate::W<RTXDMA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTXDMA_SPEC>;
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
impl From<crate::W<RTXDMA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTXDMA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTXDMA` writer - Reset Transmitter Block DMA Register. Writing a 1 to this self-clearing register resets the TXDMA register mid-cycle to point to the lowest enabled Transmit channel. NOTE: This register has no effect in the middle of a stereo pair write (such as,when left stereo data has been written but not right stereo data)"]
pub struct RTXDMA_W<'a> {
    w: &'a mut W,
}
impl<'a> RTXDMA_W<'a> {
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
    #[doc = "Bit 0 - Reset Transmitter Block DMA Register. Writing a 1 to this self-clearing register resets the TXDMA register mid-cycle to point to the lowest enabled Transmit channel. NOTE: This register has no effect in the middle of a stereo pair write (such as,when left stereo data has been written but not right stereo data)"]
    #[inline(always)]
    pub fn rtxdma(&mut self) -> RTXDMA_W {
        RTXDMA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reset Transmitter Block DMA Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtxdma](index.html) module"]
pub struct RTXDMA_SPEC;
impl crate::RegisterSpec for RTXDMA_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [rtxdma::W](W) writer structure"]
impl crate::Writable for RTXDMA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTXDMA to value 0"]
impl crate::Resettable for RTXDMA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
