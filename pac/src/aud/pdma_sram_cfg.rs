#[doc = "Register `PDMA_SRAM_CFG` reader"]
pub struct R(crate::R<PDMA_SRAM_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDMA_SRAM_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDMA_SRAM_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDMA_SRAM_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDMA_SRAM_CFG` writer"]
pub struct W(crate::W<PDMA_SRAM_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDMA_SRAM_CFG_SPEC>;
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
impl From<crate::W<PDMA_SRAM_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDMA_SRAM_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PDM_SRAM_L_TEST1` reader - Test pin to bypass self-timed circuit"]
pub struct PDM_SRAM_L_TEST1_R(crate::FieldReader<bool, bool>);
impl PDM_SRAM_L_TEST1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PDM_SRAM_L_TEST1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDM_SRAM_L_TEST1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDM_SRAM_L_TEST1` writer - Test pin to bypass self-timed circuit"]
pub struct PDM_SRAM_L_TEST1_W<'a> {
    w: &'a mut W,
}
impl<'a> PDM_SRAM_L_TEST1_W<'a> {
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
#[doc = "Field `PDM_SRAM_L_RME` reader - Read-Write margin Enable Input"]
pub struct PDM_SRAM_L_RME_R(crate::FieldReader<bool, bool>);
impl PDM_SRAM_L_RME_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PDM_SRAM_L_RME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDM_SRAM_L_RME_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDM_SRAM_L_RME` writer - Read-Write margin Enable Input"]
pub struct PDM_SRAM_L_RME_W<'a> {
    w: &'a mut W,
}
impl<'a> PDM_SRAM_L_RME_W<'a> {
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
#[doc = "Field `PDM_SRAM_L_RM` reader - Read-Write margin Input for Right Channel PDM SRAM"]
pub struct PDM_SRAM_L_RM_R(crate::FieldReader<u8, u8>);
impl PDM_SRAM_L_RM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PDM_SRAM_L_RM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDM_SRAM_L_RM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDM_SRAM_L_RM` writer - Read-Write margin Input for Right Channel PDM SRAM"]
pub struct PDM_SRAM_L_RM_W<'a> {
    w: &'a mut W,
}
impl<'a> PDM_SRAM_L_RM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x0f << 2)) | ((value as u32 & 0x0f) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Test pin to bypass self-timed circuit"]
    #[inline(always)]
    pub fn pdm_sram_l_test1(&self) -> PDM_SRAM_L_TEST1_R {
        PDM_SRAM_L_TEST1_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Read-Write margin Enable Input"]
    #[inline(always)]
    pub fn pdm_sram_l_rme(&self) -> PDM_SRAM_L_RME_R {
        PDM_SRAM_L_RME_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:5 - Read-Write margin Input for Right Channel PDM SRAM"]
    #[inline(always)]
    pub fn pdm_sram_l_rm(&self) -> PDM_SRAM_L_RM_R {
        PDM_SRAM_L_RM_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Test pin to bypass self-timed circuit"]
    #[inline(always)]
    pub fn pdm_sram_l_test1(&mut self) -> PDM_SRAM_L_TEST1_W {
        PDM_SRAM_L_TEST1_W { w: self }
    }
    #[doc = "Bit 1 - Read-Write margin Enable Input"]
    #[inline(always)]
    pub fn pdm_sram_l_rme(&mut self) -> PDM_SRAM_L_RME_W {
        PDM_SRAM_L_RME_W { w: self }
    }
    #[doc = "Bits 2:5 - Read-Write margin Input for Right Channel PDM SRAM"]
    #[inline(always)]
    pub fn pdm_sram_l_rm(&mut self) -> PDM_SRAM_L_RM_W {
        PDM_SRAM_L_RM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDM core SRAM configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_sram_cfg](index.html) module"]
pub struct PDMA_SRAM_CFG_SPEC;
impl crate::RegisterSpec for PDMA_SRAM_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdma_sram_cfg::R](R) reader structure"]
impl crate::Readable for PDMA_SRAM_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdma_sram_cfg::W](W) writer structure"]
impl crate::Writable for PDMA_SRAM_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDMA_SRAM_CFG to value 0"]
impl crate::Resettable for PDMA_SRAM_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
