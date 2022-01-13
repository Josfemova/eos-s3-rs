#[doc = "Register `DMABURSTSIZE1` reader"]
pub struct R(crate::R<DMABURSTSIZE1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMABURSTSIZE1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMABURSTSIZE1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMABURSTSIZE1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMABURSTSIZE1` writer"]
pub struct W(crate::W<DMABURSTSIZE1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMABURSTSIZE1_SPEC>;
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
impl From<crate::W<DMABURSTSIZE1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMABURSTSIZE1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMABurstSize1` reader - MSB Byte of DMA transfer size. It is representing BurstSize\\[15:8\\]"]
pub struct DMABURSTSIZE1_R(crate::FieldReader<u8, u8>);
impl DMABURSTSIZE1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DMABURSTSIZE1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMABURSTSIZE1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMABurstSize1` writer - MSB Byte of DMA transfer size. It is representing BurstSize\\[15:8\\]"]
pub struct DMABURSTSIZE1_W<'a> {
    w: &'a mut W,
}
impl<'a> DMABURSTSIZE1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = value as u8;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - MSB Byte of DMA transfer size. It is representing BurstSize\\[15:8\\]"]
    #[inline(always)]
    pub fn dmaburst_size1(&self) -> DMABURSTSIZE1_R {
        DMABURSTSIZE1_R::new(self.bits as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - MSB Byte of DMA transfer size. It is representing BurstSize\\[15:8\\]"]
    #[inline(always)]
    pub fn dmaburst_size1(&mut self) -> DMABURSTSIZE1_W {
        DMABURSTSIZE1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MSB Byte of DMA transfer size. it is representing BurstSize\\[15:8\\]. Max transfer size is 64KB, Once it is written, DMA will be kickoff unless DmaBurstSize0\\[1:0\\]
= 2'b10. Min. Transfer Size will be 4 Bytes once DMA is Kick Off ({DmaBurstSize0, DmaBurstSize1} == 0x0}\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmaburstsize1](index.html) module"]
pub struct DMABURSTSIZE1_SPEC;
impl crate::RegisterSpec for DMABURSTSIZE1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [dmaburstsize1::R](R) reader structure"]
impl crate::Readable for DMABURSTSIZE1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmaburstsize1::W](W) writer structure"]
impl crate::Writable for DMABURSTSIZE1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMABURSTSIZE1 to value 0"]
impl crate::Resettable for DMABURSTSIZE1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
