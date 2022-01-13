#[doc = "Register `DMAADDR2` reader"]
pub struct R(crate::R<DMAADDR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAADDR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAADDR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAADDR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMAADDR2` writer"]
pub struct W(crate::W<DMAADDR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMAADDR2_SPEC>;
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
impl From<crate::W<DMAADDR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMAADDR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMAAddr2` reader - DMA Starting Address, It is representing AHB Byte Address bit \\[23:16\\]"]
pub struct DMAADDR2_R(crate::FieldReader<u8, u8>);
impl DMAADDR2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DMAADDR2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMAADDR2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAAddr2` writer - DMA Starting Address, It is representing AHB Byte Address bit \\[23:16\\]"]
pub struct DMAADDR2_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAADDR2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = value as u8;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - DMA Starting Address, It is representing AHB Byte Address bit \\[23:16\\]"]
    #[inline(always)]
    pub fn dmaaddr2(&self) -> DMAADDR2_R {
        DMAADDR2_R::new(self.bits as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DMA Starting Address, It is representing AHB Byte Address bit \\[23:16\\]"]
    #[inline(always)]
    pub fn dmaaddr2(&mut self) -> DMAADDR2_W {
        DMAADDR2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Starting Address, It is representing AHB Byte Address bit \\[23:16\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmaaddr2](index.html) module"]
pub struct DMAADDR2_SPEC;
impl crate::RegisterSpec for DMAADDR2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [dmaaddr2::R](R) reader structure"]
impl crate::Readable for DMAADDR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmaaddr2::W](W) writer structure"]
impl crate::Writable for DMAADDR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMAADDR2 to value 0"]
impl crate::Resettable for DMAADDR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
