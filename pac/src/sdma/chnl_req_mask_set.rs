#[doc = "Register `CHNL_REQ_MASK_SET` reader"]
pub struct R(crate::R<CHNL_REQ_MASK_SET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHNL_REQ_MASK_SET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHNL_REQ_MASK_SET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHNL_REQ_MASK_SET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHNL_REQ_MASK_SET` writer"]
pub struct W(crate::W<CHNL_REQ_MASK_SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHNL_REQ_MASK_SET_SPEC>;
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
impl From<crate::W<CHNL_REQ_MASK_SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHNL_REQ_MASK_SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `chnl_req_mask_set` reader - Returns the request mask status of dma_req\\[C\\]
and dma_sreq\\[C\\], or disables the corresponding channel from generating DMA requests. Read as: \n \n Bit \\[C\\]
= 0 External requests are enabled for channel C. \n \n Bit \\[C\\]
= 1 External requests are disabled for channel C. \n \n Write as: \n \n Bit \\[C\\]
= 0 No effect. Use the chnl_req_mask_clr Register to enable DMA requests. \n Bit \\[C\\]
= 1 Disables dma_req\\[C\\]
and dma_sreq\\[C\\]
from generating DMA requests. \n Writing to a bit where a DMA channel is not implemented has no effect."]
pub struct CHNL_REQ_MASK_SET_R(crate::FieldReader<u16, u16>);
impl CHNL_REQ_MASK_SET_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        CHNL_REQ_MASK_SET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHNL_REQ_MASK_SET_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `chnl_req_mask_set` writer - Returns the request mask status of dma_req\\[C\\]
and dma_sreq\\[C\\], or disables the corresponding channel from generating DMA requests. Read as: \n \n Bit \\[C\\]
= 0 External requests are enabled for channel C. \n \n Bit \\[C\\]
= 1 External requests are disabled for channel C. \n \n Write as: \n \n Bit \\[C\\]
= 0 No effect. Use the chnl_req_mask_clr Register to enable DMA requests. \n Bit \\[C\\]
= 1 Disables dma_req\\[C\\]
and dma_sreq\\[C\\]
from generating DMA requests. \n Writing to a bit where a DMA channel is not implemented has no effect."]
pub struct CHNL_REQ_MASK_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> CHNL_REQ_MASK_SET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Returns the request mask status of dma_req\\[C\\]
and dma_sreq\\[C\\], or disables the corresponding channel from generating DMA requests. Read as: \n \n Bit \\[C\\]
= 0 External requests are enabled for channel C. \n \n Bit \\[C\\]
= 1 External requests are disabled for channel C. \n \n Write as: \n \n Bit \\[C\\]
= 0 No effect. Use the chnl_req_mask_clr Register to enable DMA requests. \n Bit \\[C\\]
= 1 Disables dma_req\\[C\\]
and dma_sreq\\[C\\]
from generating DMA requests. \n Writing to a bit where a DMA channel is not implemented has no effect."]
    #[inline(always)]
    pub fn chnl_req_mask_set(&self) -> CHNL_REQ_MASK_SET_R {
        CHNL_REQ_MASK_SET_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Returns the request mask status of dma_req\\[C\\]
and dma_sreq\\[C\\], or disables the corresponding channel from generating DMA requests. Read as: \n \n Bit \\[C\\]
= 0 External requests are enabled for channel C. \n \n Bit \\[C\\]
= 1 External requests are disabled for channel C. \n \n Write as: \n \n Bit \\[C\\]
= 0 No effect. Use the chnl_req_mask_clr Register to enable DMA requests. \n Bit \\[C\\]
= 1 Disables dma_req\\[C\\]
and dma_sreq\\[C\\]
from generating DMA requests. \n Writing to a bit where a DMA channel is not implemented has no effect."]
    #[inline(always)]
    pub fn chnl_req_mask_set(&mut self) -> CHNL_REQ_MASK_SET_W {
        CHNL_REQ_MASK_SET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Returns the request mask status of dma_req\\[\\]
and dma_sreq\\[\\], or disables the corresponding channel from generating DMA requests.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chnl_req_mask_set](index.html) module"]
pub struct CHNL_REQ_MASK_SET_SPEC;
impl crate::RegisterSpec for CHNL_REQ_MASK_SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chnl_req_mask_set::R](R) reader structure"]
impl crate::Readable for CHNL_REQ_MASK_SET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chnl_req_mask_set::W](W) writer structure"]
impl crate::Writable for CHNL_REQ_MASK_SET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHNL_REQ_MASK_SET to value 0"]
impl crate::Resettable for CHNL_REQ_MASK_SET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
