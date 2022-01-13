#[doc = "Register `VOICE_DMAC_DST_ADDR1` reader"]
pub struct R(crate::R<VOICE_DMAC_DST_ADDR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VOICE_DMAC_DST_ADDR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VOICE_DMAC_DST_ADDR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VOICE_DMAC_DST_ADDR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VOICE_DMAC_DST_ADDR1` writer"]
pub struct W(crate::W<VOICE_DMAC_DST_ADDR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VOICE_DMAC_DST_ADDR1_SPEC>;
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
impl From<crate::W<VOICE_DMAC_DST_ADDR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VOICE_DMAC_DST_ADDR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VOICE_DMAC_DST_ADDR1` reader - DMA1 dest address for the first buffer"]
pub struct VOICE_DMAC_DST_ADDR1_R(crate::FieldReader<u32, u32>);
impl VOICE_DMAC_DST_ADDR1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        VOICE_DMAC_DST_ADDR1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VOICE_DMAC_DST_ADDR1_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VOICE_DMAC_DST_ADDR1` writer - DMA1 dest address for the first buffer"]
pub struct VOICE_DMAC_DST_ADDR1_W<'a> {
    w: &'a mut W,
}
impl<'a> VOICE_DMAC_DST_ADDR1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - DMA1 dest address for the first buffer"]
    #[inline(always)]
    pub fn voice_dmac_dst_addr1(&self) -> VOICE_DMAC_DST_ADDR1_R {
        VOICE_DMAC_DST_ADDR1_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - DMA1 dest address for the first buffer"]
    #[inline(always)]
    pub fn voice_dmac_dst_addr1(&mut self) -> VOICE_DMAC_DST_ADDR1_W {
        VOICE_DMAC_DST_ADDR1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA1 dest address for the first buffer\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [voice_dmac_dst_addr1](index.html) module"]
pub struct VOICE_DMAC_DST_ADDR1_SPEC;
impl crate::RegisterSpec for VOICE_DMAC_DST_ADDR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [voice_dmac_dst_addr1::R](R) reader structure"]
impl crate::Readable for VOICE_DMAC_DST_ADDR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [voice_dmac_dst_addr1::W](W) writer structure"]
impl crate::Writable for VOICE_DMAC_DST_ADDR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets VOICE_DMAC_DST_ADDR1 to value 0"]
impl crate::Resettable for VOICE_DMAC_DST_ADDR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
