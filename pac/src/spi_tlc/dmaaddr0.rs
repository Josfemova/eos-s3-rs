#[doc = "Register `DMAADDR0` reader"]
pub struct R(crate::R<DMAADDR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAADDR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAADDR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAADDR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMAADDR0` writer"]
pub struct W(crate::W<DMAADDR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMAADDR0_SPEC>;
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
impl From<crate::W<DMAADDR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMAADDR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMAAddr0` reader - DMA Starting Address, It is representing AHB Byte Address bit \\[7:0\\]. Bit \\[1:0\\]
will be ignored since only Support Word Access"]
pub struct DMAADDR0_R(crate::FieldReader<u8, u8>);
impl DMAADDR0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DMAADDR0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMAADDR0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAAddr0` writer - DMA Starting Address, It is representing AHB Byte Address bit \\[7:0\\]. Bit \\[1:0\\]
will be ignored since only Support Word Access"]
pub struct DMAADDR0_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAADDR0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = value as u8;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - DMA Starting Address, It is representing AHB Byte Address bit \\[7:0\\]. Bit \\[1:0\\]
will be ignored since only Support Word Access"]
    #[inline(always)]
    pub fn dmaaddr0(&self) -> DMAADDR0_R {
        DMAADDR0_R::new(self.bits as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DMA Starting Address, It is representing AHB Byte Address bit \\[7:0\\]. Bit \\[1:0\\]
will be ignored since only Support Word Access"]
    #[inline(always)]
    pub fn dmaaddr0(&mut self) -> DMAADDR0_W {
        DMAADDR0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Starting Address, It is representing AHB Byte Address bit \\[7:0\\]. Bit \\[1:0\\]
will be ignored since only Support Word Access\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmaaddr0](index.html) module"]
pub struct DMAADDR0_SPEC;
impl crate::RegisterSpec for DMAADDR0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [dmaaddr0::R](R) reader structure"]
impl crate::Readable for DMAADDR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmaaddr0::W](W) writer structure"]
impl crate::Writable for DMAADDR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMAADDR0 to value 0"]
impl crate::Resettable for DMAADDR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
