#[doc = "Register `CHNL_USEBURST_SET` writer"]
pub struct W(crate::W<CHNL_USEBURST_SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHNL_USEBURST_SET_SPEC>;
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
impl From<crate::W<CHNL_USEBURST_SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHNL_USEBURST_SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `chnl_useburst_clr` writer - Set the appropriate bit to enable dma_sreq\\[\\]
to generate requests. Write as: \n \n Bit \\[C\\]
= 0 No effect. Use the chnl_useburst_set Register to disable dma_sreq\\[\\]
from generating requests. \n \n Bit \\[C\\]
= 1 Enables dma_sreq\\[C\\]
to generate DMA requests. \n\n Writing to a bit where a DMA channel is not implemented has no effect"]
pub struct CHNL_USEBURST_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CHNL_USEBURST_CLR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:15 - Set the appropriate bit to enable dma_sreq\\[\\]
to generate requests. Write as: \n \n Bit \\[C\\]
= 0 No effect. Use the chnl_useburst_set Register to disable dma_sreq\\[\\]
from generating requests. \n \n Bit \\[C\\]
= 1 Enables dma_sreq\\[C\\]
to generate DMA requests. \n\n Writing to a bit where a DMA channel is not implemented has no effect"]
    #[inline(always)]
    pub fn chnl_useburst_clr(&mut self) -> CHNL_USEBURST_CLR_W {
        CHNL_USEBURST_CLR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Set the appropriate bit to enable dma_sreq\\[Channel\\]
to generate requests.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chnl_useburst_set](index.html) module"]
pub struct CHNL_USEBURST_SET_SPEC;
impl crate::RegisterSpec for CHNL_USEBURST_SET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [chnl_useburst_set::W](W) writer structure"]
impl crate::Writable for CHNL_USEBURST_SET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHNL_USEBURST_SET to value 0"]
impl crate::Resettable for CHNL_USEBURST_SET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
