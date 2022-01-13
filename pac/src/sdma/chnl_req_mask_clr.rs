#[doc = "Register `CHNL_REQ_MASK_CLR` writer"]
pub struct W(crate::W<CHNL_REQ_MASK_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHNL_REQ_MASK_CLR_SPEC>;
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
impl From<crate::W<CHNL_REQ_MASK_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHNL_REQ_MASK_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `chnl_req_mask_cler` writer - Set the appropriate bit to enable DMA requests for the channel corresponding to dma_req\\[C\\]
and dma_sreq\\[C\\]. Write as: \n \n Bit \\[C\\]
= 0 No effect. Use the chnl_req_mask_set Register to disable dma_req\\[C\\]
and \n \n dma_sreq\\[C\\]
from generating requests. \n \n Bit \\[C\\]
= 1 Enables dma_req\\[C\\]
or dma_sreq\\[C\\]
to generate DMA requests. \n \n Writing to a bit where a DMA channel is not implemented has no effect."]
pub struct CHNL_REQ_MASK_CLER_W<'a> {
    w: &'a mut W,
}
impl<'a> CHNL_REQ_MASK_CLER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:15 - Set the appropriate bit to enable DMA requests for the channel corresponding to dma_req\\[C\\]
and dma_sreq\\[C\\]. Write as: \n \n Bit \\[C\\]
= 0 No effect. Use the chnl_req_mask_set Register to disable dma_req\\[C\\]
and \n \n dma_sreq\\[C\\]
from generating requests. \n \n Bit \\[C\\]
= 1 Enables dma_req\\[C\\]
or dma_sreq\\[C\\]
to generate DMA requests. \n \n Writing to a bit where a DMA channel is not implemented has no effect."]
    #[inline(always)]
    pub fn chnl_req_mask_cler(&mut self) -> CHNL_REQ_MASK_CLER_W {
        CHNL_REQ_MASK_CLER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Set the appropriate bit to enable DMA requests for the channel corresponding to dma_req\\[C\\]
and dma_sreq\\[C\\].\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chnl_req_mask_clr](index.html) module"]
pub struct CHNL_REQ_MASK_CLR_SPEC;
impl crate::RegisterSpec for CHNL_REQ_MASK_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [chnl_req_mask_clr::W](W) writer structure"]
impl crate::Writable for CHNL_REQ_MASK_CLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHNL_REQ_MASK_CLR to value 0"]
impl crate::Resettable for CHNL_REQ_MASK_CLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
