#[doc = "Register `SDMA_SRAM_CTL` reader"]
pub struct R(crate::R<SDMA_SRAM_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDMA_SRAM_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDMA_SRAM_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDMA_SRAM_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDMA_SRAM_CTL` writer"]
pub struct W(crate::W<SDMA_SRAM_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDMA_SRAM_CTL_SPEC>;
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
impl From<crate::W<SDMA_SRAM_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDMA_SRAM_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `sdma_sram_test1` reader - Set this bit to enable test mode"]
pub struct SDMA_SRAM_TEST1_R(crate::FieldReader<bool, bool>);
impl SDMA_SRAM_TEST1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SDMA_SRAM_TEST1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDMA_SRAM_TEST1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sdma_sram_test1` writer - Set this bit to enable test mode"]
pub struct SDMA_SRAM_TEST1_W<'a> {
    w: &'a mut W,
}
impl<'a> SDMA_SRAM_TEST1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `sdma_sram_rme` reader - Set to enable SRAM timing adjust enable"]
pub struct SDMA_SRAM_RME_R(crate::FieldReader<bool, bool>);
impl SDMA_SRAM_RME_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SDMA_SRAM_RME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDMA_SRAM_RME_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sdma_sram_rme` writer - Set to enable SRAM timing adjust enable"]
pub struct SDMA_SRAM_RME_W<'a> {
    w: &'a mut W,
}
impl<'a> SDMA_SRAM_RME_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `sdma_sram_rm` reader - SRAM adjust timing value"]
pub struct SDMA_SRAM_RM_R(crate::FieldReader<u8, u8>);
impl SDMA_SRAM_RM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SDMA_SRAM_RM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDMA_SRAM_RM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sdma_sram_rm` writer - SRAM adjust timing value"]
pub struct SDMA_SRAM_RM_W<'a> {
    w: &'a mut W,
}
impl<'a> SDMA_SRAM_RM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x0f << 2)) | ((value as u32 & 0x0f) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Set this bit to enable test mode"]
    #[inline(always)]
    pub fn sdma_sram_test1(&self) -> SDMA_SRAM_TEST1_R {
        SDMA_SRAM_TEST1_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Set to enable SRAM timing adjust enable"]
    #[inline(always)]
    pub fn sdma_sram_rme(&self) -> SDMA_SRAM_RME_R {
        SDMA_SRAM_RME_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:5 - SRAM adjust timing value"]
    #[inline(always)]
    pub fn sdma_sram_rm(&self) -> SDMA_SRAM_RM_R {
        SDMA_SRAM_RM_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to enable test mode"]
    #[inline(always)]
    pub fn sdma_sram_test1(&mut self) -> SDMA_SRAM_TEST1_W {
        SDMA_SRAM_TEST1_W { w: self }
    }
    #[doc = "Bit 1 - Set to enable SRAM timing adjust enable"]
    #[inline(always)]
    pub fn sdma_sram_rme(&mut self) -> SDMA_SRAM_RME_W {
        SDMA_SRAM_RME_W { w: self }
    }
    #[doc = "Bits 2:5 - SRAM adjust timing value"]
    #[inline(always)]
    pub fn sdma_sram_rm(&mut self) -> SDMA_SRAM_RM_W {
        SDMA_SRAM_RM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control register for System DMA SRAM\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdma_sram_ctl](index.html) module"]
pub struct SDMA_SRAM_CTL_SPEC;
impl crate::RegisterSpec for SDMA_SRAM_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdma_sram_ctl::R](R) reader structure"]
impl crate::Readable for SDMA_SRAM_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdma_sram_ctl::W](W) writer structure"]
impl crate::Writable for SDMA_SRAM_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SDMA_SRAM_CTL to value 0"]
impl crate::Resettable for SDMA_SRAM_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
