#[doc = "Register `DMABURSTSIZE0` reader"]
pub struct R(crate::R<DMABURSTSIZE0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMABURSTSIZE0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMABURSTSIZE0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMABURSTSIZE0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMABURSTSIZE0` writer"]
pub struct W(crate::W<DMABURSTSIZE0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMABURSTSIZE0_SPEC>;
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
impl From<crate::W<DMABURSTSIZE0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMABURSTSIZE0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMABurstSize0` reader - LSB Byte of DMA transfer size. It is representing BurstSize\\[7:0\\]"]
pub struct DMABURSTSIZE0_R(crate::FieldReader<u8, u8>);
impl DMABURSTSIZE0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DMABURSTSIZE0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMABURSTSIZE0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMABurstSize0` writer - LSB Byte of DMA transfer size. It is representing BurstSize\\[7:0\\]"]
pub struct DMABURSTSIZE0_W<'a> {
    w: &'a mut W,
}
impl<'a> DMABURSTSIZE0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = value as u8;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - LSB Byte of DMA transfer size. It is representing BurstSize\\[7:0\\]"]
    #[inline(always)]
    pub fn dmaburst_size0(&self) -> DMABURSTSIZE0_R {
        DMABURSTSIZE0_R::new(self.bits as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - LSB Byte of DMA transfer size. It is representing BurstSize\\[7:0\\]"]
    #[inline(always)]
    pub fn dmaburst_size0(&mut self) -> DMABURSTSIZE0_W {
        DMABURSTSIZE0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA transfer size indicates the number of bytes to be read out ( X ). The minimum transfer size is 4 bytes. \n Program the value for number of bytes to read minus 4 bytes ( X -4 ), into the 2 registers. \n DmaBurstSize0 register represents the BurstSize\\[7:0\\]. \n DmaBurstSize1 register represents the BurstSize\\[15:8\\]. \n Lower 2 bits are ignored by hardware, since it only supports Word Access. This means only multiples of 4 are supported. \n For example: \n To read 4 bytes, you would write: \n DmaBurstSize0 = 0, DmaBurstSize1 = 0 \n To read 8 bytes, you would write: \n DmaBurstSize0 = 4, DmaBurstSize1 = 0 \n … \n To read 256 bytes, you would write: \n DmaBurstSize0 = FC, DmaBurstSize1 = 0 \n To read 260 bytes, you would write: \n DmaBurstSize0 = 0, DmaBurstSize1 = 1 \n ... and so on, etc.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmaburstsize0](index.html) module"]
pub struct DMABURSTSIZE0_SPEC;
impl crate::RegisterSpec for DMABURSTSIZE0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [dmaburstsize0::R](R) reader structure"]
impl crate::Readable for DMABURSTSIZE0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmaburstsize0::W](W) writer structure"]
impl crate::Writable for DMABURSTSIZE0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMABURSTSIZE0 to value 0"]
impl crate::Resettable for DMABURSTSIZE0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
