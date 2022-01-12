#[doc = "Register `DMA_WAITONREQ_REG` reader"]
pub struct R(crate::R<DMA_WAITONREQ_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_WAITONREQ_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_WAITONREQ_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_WAITONREQ_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_WAITONREQ_REG` writer"]
pub struct W(crate::W<DMA_WAITONREQ_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_WAITONREQ_REG_SPEC>;
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
impl From<crate::W<DMA_WAITONREQ_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_WAITONREQ_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `dma_waitonreq` reader - waitonreq signal send to System DMA"]
pub struct DMA_WAITONREQ_R(crate::FieldReader<u16, u16>);
impl DMA_WAITONREQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        DMA_WAITONREQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_WAITONREQ_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dma_waitonreq` writer - waitonreq signal send to System DMA"]
pub struct DMA_WAITONREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_WAITONREQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | (value as u32 & 0x07ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:10 - waitonreq signal send to System DMA"]
    #[inline(always)]
    pub fn dma_waitonreq(&self) -> DMA_WAITONREQ_R {
        DMA_WAITONREQ_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - waitonreq signal send to System DMA"]
    #[inline(always)]
    pub fn dma_waitonreq(&mut self) -> DMA_WAITONREQ_W {
        DMA_WAITONREQ_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA wait on request register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_waitonreq_reg](index.html) module"]
pub struct DMA_WAITONREQ_REG_SPEC;
impl crate::RegisterSpec for DMA_WAITONREQ_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_waitonreq_reg::R](R) reader structure"]
impl crate::Readable for DMA_WAITONREQ_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_waitonreq_reg::W](W) writer structure"]
impl crate::Writable for DMA_WAITONREQ_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_WAITONREQ_REG to value 0"]
impl crate::Resettable for DMA_WAITONREQ_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
