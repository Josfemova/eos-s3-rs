#[doc = "Register `CHNL_ENABLE_CLR` writer"]
pub struct W(crate::W<CHNL_ENABLE_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHNL_ENABLE_CLR_SPEC>;
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
impl From<crate::W<CHNL_ENABLE_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHNL_ENABLE_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `chnl_enable_clr` writer - Set the appropriate bit to disable the corresponding DMA channel. Write as: \n \n Bit \\[C\\]
= 0 No effect. Use the chnl_enable_set Register to enable DMA channels. \n \n Bit \\[C\\]
= 1 Disables channel C. \n \n Writing to a bit where a DMA channel is not implemented has no effect."]
pub struct CHNL_ENABLE_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CHNL_ENABLE_CLR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:15 - Set the appropriate bit to disable the corresponding DMA channel. Write as: \n \n Bit \\[C\\]
= 0 No effect. Use the chnl_enable_set Register to enable DMA channels. \n \n Bit \\[C\\]
= 1 Disables channel C. \n \n Writing to a bit where a DMA channel is not implemented has no effect."]
    #[inline(always)]
    pub fn chnl_enable_clr(&mut self) -> CHNL_ENABLE_CLR_W {
        CHNL_ENABLE_CLR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Set the appropriate bit to disable the corresponding DMA channel.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chnl_enable_clr](index.html) module"]
pub struct CHNL_ENABLE_CLR_SPEC;
impl crate::RegisterSpec for CHNL_ENABLE_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [chnl_enable_clr::W](W) writer structure"]
impl crate::Writable for CHNL_ENABLE_CLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHNL_ENABLE_CLR to value 0"]
impl crate::Resettable for CHNL_ENABLE_CLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
