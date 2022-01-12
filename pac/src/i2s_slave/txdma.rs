#[doc = "Register `TXDMA` writer"]
pub struct W(crate::W<TXDMA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXDMA_SPEC>;
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
impl From<crate::W<TXDMA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXDMA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXDMA` writer - Transmitter Block DMA Register. This register can be used to cycle repeatedly through the enabled Transmit channels (from lowest numbered to highest) to allow writing of stereo data pairs"]
pub struct TXDMA_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDMA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Transmitter Block DMA Register. This register can be used to cycle repeatedly through the enabled Transmit channels (from lowest numbered to highest) to allow writing of stereo data pairs"]
    #[inline(always)]
    pub fn txdma(&mut self) -> TXDMA_W {
        TXDMA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmitter Block DMA Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txdma](index.html) module"]
pub struct TXDMA_SPEC;
impl crate::RegisterSpec for TXDMA_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [txdma::W](W) writer structure"]
impl crate::Writable for TXDMA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TXDMA to value 0"]
impl crate::Resettable for TXDMA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
