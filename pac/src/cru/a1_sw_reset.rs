#[doc = "Register `A1_SW_RESET` reader"]
pub struct R(crate::R<A1_SW_RESET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<A1_SW_RESET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<A1_SW_RESET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<A1_SW_RESET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `A1_SW_RESET` writer"]
pub struct W(crate::W<A1_SW_RESET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<A1_SW_RESET_SPEC>;
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
impl From<crate::W<A1_SW_RESET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<A1_SW_RESET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPT_SW_RESET` reader - 1'b1 : Enable the Software Reset. FW need to disable it manually. -> This is used to Reset the SPT"]
pub struct SPT_SW_RESET_R(crate::FieldReader<bool, bool>);
impl SPT_SW_RESET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPT_SW_RESET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPT_SW_RESET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPT_SW_RESET` writer - 1'b1 : Enable the Software Reset. FW need to disable it manually. -> This is used to Reset the SPT"]
pub struct SPT_SW_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> SPT_SW_RESET_W<'a> {
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
#[doc = "Field `CfgSM_SW_RESET` reader - 1'b1 : Enable the Software Reset. FW need to disable it manually. -> This is used to Reset the CfgSM/SPI Master and Related FIFO, DMA and AHB Master"]
pub struct CFGSM_SW_RESET_R(crate::FieldReader<bool, bool>);
impl CFGSM_SW_RESET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CFGSM_SW_RESET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFGSM_SW_RESET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CfgSM_SW_RESET` writer - 1'b1 : Enable the Software Reset. FW need to disable it manually. -> This is used to Reset the CfgSM/SPI Master and Related FIFO, DMA and AHB Master"]
pub struct CFGSM_SW_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> CFGSM_SW_RESET_W<'a> {
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
            (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - 1'b1 : Enable the Software Reset. FW need to disable it manually. -> This is used to Reset the SPT"]
    #[inline(always)]
    pub fn spt_sw_reset(&self) -> SPT_SW_RESET_R {
        SPT_SW_RESET_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - 1'b1 : Enable the Software Reset. FW need to disable it manually. -> This is used to Reset the CfgSM/SPI Master and Related FIFO, DMA and AHB Master"]
    #[inline(always)]
    pub fn cfg_sm_sw_reset(&self) -> CFGSM_SW_RESET_R {
        CFGSM_SW_RESET_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1'b1 : Enable the Software Reset. FW need to disable it manually. -> This is used to Reset the SPT"]
    #[inline(always)]
    pub fn spt_sw_reset(&mut self) -> SPT_SW_RESET_W {
        SPT_SW_RESET_W { w: self }
    }
    #[doc = "Bit 2 - 1'b1 : Enable the Software Reset. FW need to disable it manually. -> This is used to Reset the CfgSM/SPI Master and Related FIFO, DMA and AHB Master"]
    #[inline(always)]
    pub fn cfg_sm_sw_reset(&mut self) -> CFGSM_SW_RESET_W {
        CFGSM_SW_RESET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Once Program the SW Reset Bit to 1, the corresponding reset will be asserted immediately. Once Program the SW Reset Bit to 0, the corresponding reset will be de-asserted synchronous even the corresponding clock is not running. (Turn off by Clock gating cell)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [a1_sw_reset](index.html) module"]
pub struct A1_SW_RESET_SPEC;
impl crate::RegisterSpec for A1_SW_RESET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [a1_sw_reset::R](R) reader structure"]
impl crate::Readable for A1_SW_RESET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [a1_sw_reset::W](W) writer structure"]
impl crate::Writable for A1_SW_RESET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets A1_SW_RESET to value 0"]
impl crate::Resettable for A1_SW_RESET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
