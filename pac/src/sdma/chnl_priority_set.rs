#[doc = "Register `CHNL_PRIORITY_SET` writer"]
pub struct W(crate::W<CHNL_PRIORITY_SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHNL_PRIORITY_SET_SPEC>;
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
impl From<crate::W<CHNL_PRIORITY_SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHNL_PRIORITY_SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `chnl_priority_set` writer - Returns the channel priority mask status, or sets the channel priority to high. Read as: \n \n Bit \\[C\\]
= 0 DMA channel C is using the default priority level. \n \n Bit \\[C\\]
= 1 DMA channel C is using a high priority level. \n \n Write as: \n \n Bit \\[C\\]
= 0 No effect. Use the chnl_priority_clr Register to set channel C to the default priority level. \n \n Bit \\[C\\]
= 1 Channel C uses the high priority level. \n \n Writing to a bit where a DMA channel is not implemented has no effect."]
pub struct CHNL_PRIORITY_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> CHNL_PRIORITY_SET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:15 - Returns the channel priority mask status, or sets the channel priority to high. Read as: \n \n Bit \\[C\\]
= 0 DMA channel C is using the default priority level. \n \n Bit \\[C\\]
= 1 DMA channel C is using a high priority level. \n \n Write as: \n \n Bit \\[C\\]
= 0 No effect. Use the chnl_priority_clr Register to set channel C to the default priority level. \n \n Bit \\[C\\]
= 1 Channel C uses the high priority level. \n \n Writing to a bit where a DMA channel is not implemented has no effect."]
    #[inline(always)]
    pub fn chnl_priority_set(&mut self) -> CHNL_PRIORITY_SET_W {
        CHNL_PRIORITY_SET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Returns the channel priority mask status, or sets the channel priority to high.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chnl_priority_set](index.html) module"]
pub struct CHNL_PRIORITY_SET_SPEC;
impl crate::RegisterSpec for CHNL_PRIORITY_SET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [chnl_priority_set::W](W) writer structure"]
impl crate::Writable for CHNL_PRIORITY_SET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHNL_PRIORITY_SET to value 0"]
impl crate::Resettable for CHNL_PRIORITY_SET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
