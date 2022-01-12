#[doc = "Register `SSIENR` reader"]
pub struct R(crate::R<SSIENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSIENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSIENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSIENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SSIENR` writer"]
pub struct W(crate::W<SSIENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SSIENR_SPEC>;
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
impl From<crate::W<SSIENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SSIENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SSI_EN` reader - SSI Enable. Enables and disables all SPI Master operations. When disabled, all serial transfers are halted immediately. Transmit and receive FIFO buffers are cleared when the device is disabled. It is impossible to program some of the SPI Master control registers when enabled. When disabled, the ssi_sleep output is set (after delay) to inform the system that it is safe to remove the ssi_clk, thus saving power consumption in the system."]
pub struct SSI_EN_R(crate::FieldReader<bool, bool>);
impl SSI_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SSI_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SSI_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SSI_EN` writer - SSI Enable. Enables and disables all SPI Master operations. When disabled, all serial transfers are halted immediately. Transmit and receive FIFO buffers are cleared when the device is disabled. It is impossible to program some of the SPI Master control registers when enabled. When disabled, the ssi_sleep output is set (after delay) to inform the system that it is safe to remove the ssi_clk, thus saving power consumption in the system."]
pub struct SSI_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SSI_EN_W<'a> {
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
        self.w.bits = value as u8;
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - SSI Enable. Enables and disables all SPI Master operations. When disabled, all serial transfers are halted immediately. Transmit and receive FIFO buffers are cleared when the device is disabled. It is impossible to program some of the SPI Master control registers when enabled. When disabled, the ssi_sleep output is set (after delay) to inform the system that it is safe to remove the ssi_clk, thus saving power consumption in the system."]
    #[inline(always)]
    pub fn ssi_en(&self) -> SSI_EN_R {
        SSI_EN_R::new(self.bits != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SSI Enable. Enables and disables all SPI Master operations. When disabled, all serial transfers are halted immediately. Transmit and receive FIFO buffers are cleared when the device is disabled. It is impossible to program some of the SPI Master control registers when enabled. When disabled, the ssi_sleep output is set (after delay) to inform the system that it is safe to remove the ssi_clk, thus saving power consumption in the system."]
    #[inline(always)]
    pub fn ssi_en(&mut self) -> SSI_EN_W {
        SSI_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SSI Enable Register: This register enables and disables the SPI Master. The following SPI Master registers are NOT writeable when SPI Master is enabled =1: CTRLR0, CTRLR1, BAUDR. You must set SSIENR = 0, before writing these 3 registers. The following SPI Master registers are writeable ONLY when the SPI Master is enabled = 1: DR0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssienr](index.html) module"]
pub struct SSIENR_SPEC;
impl crate::RegisterSpec for SSIENR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ssienr::R](R) reader structure"]
impl crate::Readable for SSIENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ssienr::W](W) writer structure"]
impl crate::Writable for SSIENR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SSIENR to value 0"]
impl crate::Resettable for SSIENR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
