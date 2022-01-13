#[doc = "Register `CHNL_ENABLE_SET` reader"]
pub struct R(crate::R<CHNL_ENABLE_SET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHNL_ENABLE_SET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHNL_ENABLE_SET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHNL_ENABLE_SET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHNL_ENABLE_SET` writer"]
pub struct W(crate::W<CHNL_ENABLE_SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHNL_ENABLE_SET_SPEC>;
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
impl From<crate::W<CHNL_ENABLE_SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHNL_ENABLE_SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `chnl_enable_set` reader - Returns the enable status of the channels, or enables the corresponding channels. Read as: \n \n Bit \\[C\\]
= 0 Channel C is disabled. \n \n Bit \\[C\\]
= 1 Channel C is enabled. \n \n Write as: \n \n Bit \\[C\\]
= 0 No effect. Use the chnl_enable_clr Register to disable a channel. \n \n Bit \\[C\\]
= 1 Enables channel C. \n \n Writing to a bit where a DMA channel is not implemented has no effect."]
pub struct CHNL_ENABLE_SET_R(crate::FieldReader<u16, u16>);
impl CHNL_ENABLE_SET_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        CHNL_ENABLE_SET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHNL_ENABLE_SET_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `chnl_enable_set` writer - Returns the enable status of the channels, or enables the corresponding channels. Read as: \n \n Bit \\[C\\]
= 0 Channel C is disabled. \n \n Bit \\[C\\]
= 1 Channel C is enabled. \n \n Write as: \n \n Bit \\[C\\]
= 0 No effect. Use the chnl_enable_clr Register to disable a channel. \n \n Bit \\[C\\]
= 1 Enables channel C. \n \n Writing to a bit where a DMA channel is not implemented has no effect."]
pub struct CHNL_ENABLE_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> CHNL_ENABLE_SET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Returns the enable status of the channels, or enables the corresponding channels. Read as: \n \n Bit \\[C\\]
= 0 Channel C is disabled. \n \n Bit \\[C\\]
= 1 Channel C is enabled. \n \n Write as: \n \n Bit \\[C\\]
= 0 No effect. Use the chnl_enable_clr Register to disable a channel. \n \n Bit \\[C\\]
= 1 Enables channel C. \n \n Writing to a bit where a DMA channel is not implemented has no effect."]
    #[inline(always)]
    pub fn chnl_enable_set(&self) -> CHNL_ENABLE_SET_R {
        CHNL_ENABLE_SET_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Returns the enable status of the channels, or enables the corresponding channels. Read as: \n \n Bit \\[C\\]
= 0 Channel C is disabled. \n \n Bit \\[C\\]
= 1 Channel C is enabled. \n \n Write as: \n \n Bit \\[C\\]
= 0 No effect. Use the chnl_enable_clr Register to disable a channel. \n \n Bit \\[C\\]
= 1 Enables channel C. \n \n Writing to a bit where a DMA channel is not implemented has no effect."]
    #[inline(always)]
    pub fn chnl_enable_set(&mut self) -> CHNL_ENABLE_SET_W {
        CHNL_ENABLE_SET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Returns the enable status of the channels, or enables the corresponding channels.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chnl_enable_set](index.html) module"]
pub struct CHNL_ENABLE_SET_SPEC;
impl crate::RegisterSpec for CHNL_ENABLE_SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chnl_enable_set::R](R) reader structure"]
impl crate::Readable for CHNL_ENABLE_SET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chnl_enable_set::W](W) writer structure"]
impl crate::Writable for CHNL_ENABLE_SET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHNL_ENABLE_SET to value 0"]
impl crate::Resettable for CHNL_ENABLE_SET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
