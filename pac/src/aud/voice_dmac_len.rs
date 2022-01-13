#[doc = "Register `VOICE_DMAC_LEN` reader"]
pub struct R(crate::R<VOICE_DMAC_LEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VOICE_DMAC_LEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VOICE_DMAC_LEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VOICE_DMAC_LEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VOICE_DMAC_LEN` writer"]
pub struct W(crate::W<VOICE_DMAC_LEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VOICE_DMAC_LEN_SPEC>;
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
impl From<crate::W<VOICE_DMAC_LEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VOICE_DMAC_LEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMAC_BLK_LEN` reader - DMA block transfer length in words"]
pub struct DMAC_BLK_LEN_R(crate::FieldReader<u16, u16>);
impl DMAC_BLK_LEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        DMAC_BLK_LEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMAC_BLK_LEN_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAC_BLK_LEN` writer - DMA block transfer length in words"]
pub struct DMAC_BLK_LEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAC_BLK_LEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `DMAC_BUF_LEN` reader - DMA buffer transfer length in words"]
pub struct DMAC_BUF_LEN_R(crate::FieldReader<u16, u16>);
impl DMAC_BUF_LEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        DMAC_BUF_LEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMAC_BUF_LEN_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAC_BUF_LEN` writer - DMA buffer transfer length in words"]
pub struct DMAC_BUF_LEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAC_BUF_LEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - DMA block transfer length in words"]
    #[inline(always)]
    pub fn dmac_blk_len(&self) -> DMAC_BLK_LEN_R {
        DMAC_BLK_LEN_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - DMA buffer transfer length in words"]
    #[inline(always)]
    pub fn dmac_buf_len(&self) -> DMAC_BUF_LEN_R {
        DMAC_BUF_LEN_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - DMA block transfer length in words"]
    #[inline(always)]
    pub fn dmac_blk_len(&mut self) -> DMAC_BLK_LEN_W {
        DMAC_BLK_LEN_W { w: self }
    }
    #[doc = "Bits 16:31 - DMA buffer transfer length in words"]
    #[inline(always)]
    pub fn dmac_buf_len(&mut self) -> DMAC_BUF_LEN_W {
        DMAC_BUF_LEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Audio DMAC length register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [voice_dmac_len](index.html) module"]
pub struct VOICE_DMAC_LEN_SPEC;
impl crate::RegisterSpec for VOICE_DMAC_LEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [voice_dmac_len::R](R) reader structure"]
impl crate::Readable for VOICE_DMAC_LEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [voice_dmac_len::W](W) writer structure"]
impl crate::Writable for VOICE_DMAC_LEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets VOICE_DMAC_LEN to value 0x04b0_0078"]
impl crate::Resettable for VOICE_DMAC_LEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x04b0_0078
    }
}
