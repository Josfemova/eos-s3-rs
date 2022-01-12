#[doc = "Register `DMA_DEST_ADDR` reader"]
pub struct R(crate::R<DMA_DEST_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_DEST_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_DEST_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_DEST_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_DEST_ADDR` writer"]
pub struct W(crate::W<DMA_DEST_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_DEST_ADDR_SPEC>;
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
impl From<crate::W<DMA_DEST_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_DEST_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `dma_dest_addr` reader - DMA output data address : this register is only accessable when the dma or cfg_sm is not selecting the dmas_mux."]
pub struct DMA_DEST_ADDR_R(crate::FieldReader<u32, u32>);
impl DMA_DEST_ADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        DMA_DEST_ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_DEST_ADDR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dma_dest_addr` writer - DMA output data address : this register is only accessable when the dma or cfg_sm is not selecting the dmas_mux."]
pub struct DMA_DEST_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_DEST_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - DMA output data address : this register is only accessable when the dma or cfg_sm is not selecting the dmas_mux."]
    #[inline(always)]
    pub fn dma_dest_addr(&self) -> DMA_DEST_ADDR_R {
        DMA_DEST_ADDR_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - DMA output data address : this register is only accessable when the dma or cfg_sm is not selecting the dmas_mux."]
    #[inline(always)]
    pub fn dma_dest_addr(&mut self) -> DMA_DEST_ADDR_W {
        DMA_DEST_ADDR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA destination address : this register is only accessable when the dma or cfg_sm is not selecting the dmas_mux.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_dest_addr](index.html) module"]
pub struct DMA_DEST_ADDR_SPEC;
impl crate::RegisterSpec for DMA_DEST_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_dest_addr::R](R) reader structure"]
impl crate::Readable for DMA_DEST_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_dest_addr::W](W) writer structure"]
impl crate::Writable for DMA_DEST_ADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_DEST_ADDR to value 0"]
impl crate::Resettable for DMA_DEST_ADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
