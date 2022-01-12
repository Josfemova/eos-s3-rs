#[doc = "Register `DMA_REQ` writer"]
pub struct W(crate::W<DMA_REQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_REQ_SPEC>;
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
impl From<crate::W<DMA_REQ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_REQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `dma_req` writer - Burst DMA request send to System DMA"]
pub struct DMA_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_REQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | (value as u32 & 0x07ff);
        self.w
    }
}
#[doc = "Field `dma_sreq` writer - Single DMA request send to System DMA"]
pub struct DMA_SREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_SREQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x07ff << 16)) | ((value as u32 & 0x07ff) << 16);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:10 - Burst DMA request send to System DMA"]
    #[inline(always)]
    pub fn dma_req(&mut self) -> DMA_REQ_W {
        DMA_REQ_W { w: self }
    }
    #[doc = "Bits 16:26 - Single DMA request send to System DMA"]
    #[inline(always)]
    pub fn dma_sreq(&mut self) -> DMA_SREQ_W {
        DMA_SREQ_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA request\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_req](index.html) module"]
pub struct DMA_REQ_SPEC;
impl crate::RegisterSpec for DMA_REQ_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [dma_req::W](W) writer structure"]
impl crate::Writable for DMA_REQ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_REQ to value 0"]
impl crate::Resettable for DMA_REQ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
