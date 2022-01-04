#[doc = "Register `AUDIO_MISC_SW_RESET` reader"]
pub struct R(crate::R<AUDIO_MISC_SW_RESET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AUDIO_MISC_SW_RESET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AUDIO_MISC_SW_RESET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AUDIO_MISC_SW_RESET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AUDIO_MISC_SW_RESET` writer"]
pub struct W(crate::W<AUDIO_MISC_SW_RESET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AUDIO_MISC_SW_RESET_SPEC>;
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
impl From<crate::W<AUDIO_MISC_SW_RESET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AUDIO_MISC_SW_RESET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "1'b1 : Enable the Software Reset. FW need to disable it manually. ==> For DMA Power Domain.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AD0_SW_RESET_A {
    #[doc = "1: Enable the software reset. FW need to disable it manually"]
    ENABLE = 1,
}
impl From<AD0_SW_RESET_A> for bool {
    #[inline(always)]
    fn from(variant: AD0_SW_RESET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AD0_SW_RESET` reader - 1'b1 : Enable the Software Reset. FW need to disable it manually. ==> For DMA Power Domain."]
pub struct AD0_SW_RESET_R(crate::FieldReader<bool, AD0_SW_RESET_A>);
impl AD0_SW_RESET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AD0_SW_RESET_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<AD0_SW_RESET_A> {
        match self.bits {
            true => Some(AD0_SW_RESET_A::ENABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == AD0_SW_RESET_A::ENABLE
    }
}
impl core::ops::Deref for AD0_SW_RESET_R {
    type Target = crate::FieldReader<bool, AD0_SW_RESET_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AD0_SW_RESET` writer - 1'b1 : Enable the Software Reset. FW need to disable it manually. ==> For DMA Power Domain."]
pub struct AD0_SW_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> AD0_SW_RESET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AD0_SW_RESET_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable the software reset. FW need to disable it manually"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(AD0_SW_RESET_A::ENABLE)
    }
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
#[doc = "1'b1 : Enable the Software Reset. FW need to disable it manually. ==> For DMA Power Domain."]
pub type AD1_SW_RESET_A = AD0_SW_RESET_A;
#[doc = "Field `AD1_SW_RESET` reader - 1'b1 : Enable the Software Reset. FW need to disable it manually. ==> For DMA Power Domain."]
pub type AD1_SW_RESET_R = AD0_SW_RESET_R;
#[doc = "Field `AD1_SW_RESET` writer - 1'b1 : Enable the Software Reset. FW need to disable it manually. ==> For DMA Power Domain."]
pub struct AD1_SW_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> AD1_SW_RESET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AD1_SW_RESET_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable the software reset. FW need to disable it manually"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(AD1_SW_RESET_A::ENABLE)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "1'b1 : Enable the Software Reset. FW need to disable it manually. ==> For DMA Power Domain."]
pub type AD2_SW_RESET_A = AD0_SW_RESET_A;
#[doc = "Field `AD2_SW_RESET` reader - 1'b1 : Enable the Software Reset. FW need to disable it manually. ==> For DMA Power Domain."]
pub type AD2_SW_RESET_R = AD0_SW_RESET_R;
#[doc = "Field `AD2_SW_RESET` writer - 1'b1 : Enable the Software Reset. FW need to disable it manually. ==> For DMA Power Domain."]
pub struct AD2_SW_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> AD2_SW_RESET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AD2_SW_RESET_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable the software reset. FW need to disable it manually"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(AD2_SW_RESET_A::ENABLE)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "1'b1 : Enable the Software Reset. FW need to disable it manually. ==> For DMA Power Domain."]
pub type AD3_SW_RESET_A = AD0_SW_RESET_A;
#[doc = "Field `AD3_SW_RESET` reader - 1'b1 : Enable the Software Reset. FW need to disable it manually. ==> For DMA Power Domain."]
pub type AD3_SW_RESET_R = AD0_SW_RESET_R;
#[doc = "Field `AD3_SW_RESET` writer - 1'b1 : Enable the Software Reset. FW need to disable it manually. ==> For DMA Power Domain."]
pub struct AD3_SW_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> AD3_SW_RESET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AD3_SW_RESET_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable the software reset. FW need to disable it manually"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(AD3_SW_RESET_A::ENABLE)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "1'b1 : Enable the Software Reset. FW need to disable it manually. ==> For DMA Power Domain."]
pub type AD4_SW_RESET_A = AD0_SW_RESET_A;
#[doc = "Field `AD4_SW_RESET` reader - 1'b1 : Enable the Software Reset. FW need to disable it manually. ==> For DMA Power Domain."]
pub type AD4_SW_RESET_R = AD0_SW_RESET_R;
#[doc = "Field `AD4_SW_RESET` writer - 1'b1 : Enable the Software Reset. FW need to disable it manually. ==> For DMA Power Domain."]
pub struct AD4_SW_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> AD4_SW_RESET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AD4_SW_RESET_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable the software reset. FW need to disable it manually"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(AD4_SW_RESET_A::ENABLE)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "1'b1 : Enable the Software Reset. FW need to disable it manually. ==> For DMA Power Domain."]
pub type AD5_SW_RESET_A = AD0_SW_RESET_A;
#[doc = "Field `AD5_SW_RESET` reader - 1'b1 : Enable the Software Reset. FW need to disable it manually. ==> For DMA Power Domain."]
pub type AD5_SW_RESET_R = AD0_SW_RESET_R;
#[doc = "Field `AD5_SW_RESET` writer - 1'b1 : Enable the Software Reset. FW need to disable it manually. ==> For DMA Power Domain."]
pub struct AD5_SW_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> AD5_SW_RESET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AD5_SW_RESET_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable the software reset. FW need to disable it manually"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(AD5_SW_RESET_A::ENABLE)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "1'b1 : Enable the Software Reset. FW need to disable it manually. ==> For DMA Power Domain."]
pub type DMA_SW_RESET_A = AD0_SW_RESET_A;
#[doc = "Field `DMA_SW_RESET` reader - 1'b1 : Enable the Software Reset. FW need to disable it manually. ==> For DMA Power Domain."]
pub type DMA_SW_RESET_R = AD0_SW_RESET_R;
#[doc = "Field `DMA_SW_RESET` writer - 1'b1 : Enable the Software Reset. FW need to disable it manually. ==> For DMA Power Domain."]
pub struct DMA_SW_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_SW_RESET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA_SW_RESET_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable the software reset. FW need to disable it manually"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DMA_SW_RESET_A::ENABLE)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "1'b1 : Enable the Software Reset. FW need to disable it manually. ==> For I2S Power Domain. Note: It will only reset the AHB interface R01, but it will not reset R32 path. Suggest to power down, then power on I2S if Software Reset is needed"]
pub type I2S_SW_RESET_A = AD0_SW_RESET_A;
#[doc = "Field `I2S_SW_RESET` reader - 1'b1 : Enable the Software Reset. FW need to disable it manually. ==> For I2S Power Domain. Note: It will only reset the AHB interface R01, but it will not reset R32 path. Suggest to power down, then power on I2S if Software Reset is needed"]
pub type I2S_SW_RESET_R = AD0_SW_RESET_R;
#[doc = "Field `I2S_SW_RESET` writer - 1'b1 : Enable the Software Reset. FW need to disable it manually. ==> For I2S Power Domain. Note: It will only reset the AHB interface R01, but it will not reset R32 path. Suggest to power down, then power on I2S if Software Reset is needed"]
pub struct I2S_SW_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_SW_RESET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2S_SW_RESET_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable the software reset. FW need to disable it manually"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(I2S_SW_RESET_A::ENABLE)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - 1'b1 : Enable the Software Reset. FW need to disable it manually. ==> For DMA Power Domain."]
    #[inline(always)]
    pub fn ad0_sw_reset(&self) -> AD0_SW_RESET_R {
        AD0_SW_RESET_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1'b1 : Enable the Software Reset. FW need to disable it manually. ==> For DMA Power Domain."]
    #[inline(always)]
    pub fn ad1_sw_reset(&self) -> AD1_SW_RESET_R {
        AD1_SW_RESET_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 1'b1 : Enable the Software Reset. FW need to disable it manually. ==> For DMA Power Domain."]
    #[inline(always)]
    pub fn ad2_sw_reset(&self) -> AD2_SW_RESET_R {
        AD2_SW_RESET_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 1'b1 : Enable the Software Reset. FW need to disable it manually. ==> For DMA Power Domain."]
    #[inline(always)]
    pub fn ad3_sw_reset(&self) -> AD3_SW_RESET_R {
        AD3_SW_RESET_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 1'b1 : Enable the Software Reset. FW need to disable it manually. ==> For DMA Power Domain."]
    #[inline(always)]
    pub fn ad4_sw_reset(&self) -> AD4_SW_RESET_R {
        AD4_SW_RESET_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 1'b1 : Enable the Software Reset. FW need to disable it manually. ==> For DMA Power Domain."]
    #[inline(always)]
    pub fn ad5_sw_reset(&self) -> AD5_SW_RESET_R {
        AD5_SW_RESET_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 1'b1 : Enable the Software Reset. FW need to disable it manually. ==> For DMA Power Domain."]
    #[inline(always)]
    pub fn dma_sw_reset(&self) -> DMA_SW_RESET_R {
        DMA_SW_RESET_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 1'b1 : Enable the Software Reset. FW need to disable it manually. ==> For I2S Power Domain. Note: It will only reset the AHB interface R01, but it will not reset R32 path. Suggest to power down, then power on I2S if Software Reset is needed"]
    #[inline(always)]
    pub fn i2s_sw_reset(&self) -> I2S_SW_RESET_R {
        I2S_SW_RESET_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1'b1 : Enable the Software Reset. FW need to disable it manually. ==> For DMA Power Domain."]
    #[inline(always)]
    pub fn ad0_sw_reset(&mut self) -> AD0_SW_RESET_W {
        AD0_SW_RESET_W { w: self }
    }
    #[doc = "Bit 1 - 1'b1 : Enable the Software Reset. FW need to disable it manually. ==> For DMA Power Domain."]
    #[inline(always)]
    pub fn ad1_sw_reset(&mut self) -> AD1_SW_RESET_W {
        AD1_SW_RESET_W { w: self }
    }
    #[doc = "Bit 2 - 1'b1 : Enable the Software Reset. FW need to disable it manually. ==> For DMA Power Domain."]
    #[inline(always)]
    pub fn ad2_sw_reset(&mut self) -> AD2_SW_RESET_W {
        AD2_SW_RESET_W { w: self }
    }
    #[doc = "Bit 3 - 1'b1 : Enable the Software Reset. FW need to disable it manually. ==> For DMA Power Domain."]
    #[inline(always)]
    pub fn ad3_sw_reset(&mut self) -> AD3_SW_RESET_W {
        AD3_SW_RESET_W { w: self }
    }
    #[doc = "Bit 4 - 1'b1 : Enable the Software Reset. FW need to disable it manually. ==> For DMA Power Domain."]
    #[inline(always)]
    pub fn ad4_sw_reset(&mut self) -> AD4_SW_RESET_W {
        AD4_SW_RESET_W { w: self }
    }
    #[doc = "Bit 5 - 1'b1 : Enable the Software Reset. FW need to disable it manually. ==> For DMA Power Domain."]
    #[inline(always)]
    pub fn ad5_sw_reset(&mut self) -> AD5_SW_RESET_W {
        AD5_SW_RESET_W { w: self }
    }
    #[doc = "Bit 6 - 1'b1 : Enable the Software Reset. FW need to disable it manually. ==> For DMA Power Domain."]
    #[inline(always)]
    pub fn dma_sw_reset(&mut self) -> DMA_SW_RESET_W {
        DMA_SW_RESET_W { w: self }
    }
    #[doc = "Bit 7 - 1'b1 : Enable the Software Reset. FW need to disable it manually. ==> For I2S Power Domain. Note: It will only reset the AHB interface R01, but it will not reset R32 path. Suggest to power down, then power on I2S if Software Reset is needed"]
    #[inline(always)]
    pub fn i2s_sw_reset(&mut self) -> I2S_SW_RESET_W {
        I2S_SW_RESET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Once Program the SW Reset Bit to 1, the corresponding reset will be asserted immediately. Once Program the SW Reset Bit to 0, the corresponding reset will be de-asserted synchronous even the corresponding clock is not running. (Turn off by Clock gating cell)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [audio_misc_sw_reset](index.html) module"]
pub struct AUDIO_MISC_SW_RESET_SPEC;
impl crate::RegisterSpec for AUDIO_MISC_SW_RESET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [audio_misc_sw_reset::R](R) reader structure"]
impl crate::Readable for AUDIO_MISC_SW_RESET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [audio_misc_sw_reset::W](W) writer structure"]
impl crate::Writable for AUDIO_MISC_SW_RESET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AUDIO_MISC_SW_RESET to value 0"]
impl crate::Resettable for AUDIO_MISC_SW_RESET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
