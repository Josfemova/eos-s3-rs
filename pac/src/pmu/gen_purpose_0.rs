#[doc = "Register `GEN_PURPOSE_0` reader"]
pub struct R(crate::R<GEN_PURPOSE_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GEN_PURPOSE_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GEN_PURPOSE_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GEN_PURPOSE_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GEN_PURPOSE_0` writer"]
pub struct W(crate::W<GEN_PURPOSE_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GEN_PURPOSE_0_SPEC>;
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
impl From<crate::W<GEN_PURPOSE_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GEN_PURPOSE_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `General_Purpose` reader - General purpose SFR"]
pub struct GENERAL_PURPOSE_R(crate::FieldReader<u8, u8>);
impl GENERAL_PURPOSE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        GENERAL_PURPOSE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GENERAL_PURPOSE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `General_Purpose` writer - General purpose SFR"]
pub struct GENERAL_PURPOSE_W<'a> {
    w: &'a mut W,
}
impl<'a> GENERAL_PURPOSE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `Audio_SRAM_HW_DS_Cfg` reader - Set to allow audio HW to put the Audio SRAM into Deep Sleep mode. Please see Audio Spec for detail"]
pub struct AUDIO_SRAM_HW_DS_CFG_R(crate::FieldReader<bool, bool>);
impl AUDIO_SRAM_HW_DS_CFG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AUDIO_SRAM_HW_DS_CFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AUDIO_SRAM_HW_DS_CFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Audio_SRAM_HW_DS_Cfg` writer - Set to allow audio HW to put the Audio SRAM into Deep Sleep mode. Please see Audio Spec for detail"]
pub struct AUDIO_SRAM_HW_DS_CFG_W<'a> {
    w: &'a mut W,
}
impl<'a> AUDIO_SRAM_HW_DS_CFG_W<'a> {
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
            (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `FB_Cfg_Enable` reader - Set to enable FB configuration. Hardware will set once entering FB shutdown mode"]
pub struct FB_CFG_ENABLE_R(crate::FieldReader<bool, bool>);
impl FB_CFG_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FB_CFG_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FB_CFG_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FB_Cfg_Enable` writer - Set to enable FB configuration. Hardware will set once entering FB shutdown mode"]
pub struct FB_CFG_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> FB_CFG_ENABLE_W<'a> {
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
            (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - General purpose SFR"]
    #[inline(always)]
    pub fn general_purpose(&self) -> GENERAL_PURPOSE_R {
        GENERAL_PURPOSE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Set to allow audio HW to put the Audio SRAM into Deep Sleep mode. Please see Audio Spec for detail"]
    #[inline(always)]
    pub fn audio_sram_hw_ds_cfg(&self) -> AUDIO_SRAM_HW_DS_CFG_R {
        AUDIO_SRAM_HW_DS_CFG_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Set to enable FB configuration. Hardware will set once entering FB shutdown mode"]
    #[inline(always)]
    pub fn fb_cfg_enable(&self) -> FB_CFG_ENABLE_R {
        FB_CFG_ENABLE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - General purpose SFR"]
    #[inline(always)]
    pub fn general_purpose(&mut self) -> GENERAL_PURPOSE_W {
        GENERAL_PURPOSE_W { w: self }
    }
    #[doc = "Bit 8 - Set to allow audio HW to put the Audio SRAM into Deep Sleep mode. Please see Audio Spec for detail"]
    #[inline(always)]
    pub fn audio_sram_hw_ds_cfg(&mut self) -> AUDIO_SRAM_HW_DS_CFG_W {
        AUDIO_SRAM_HW_DS_CFG_W { w: self }
    }
    #[doc = "Bit 9 - Set to enable FB configuration. Hardware will set once entering FB shutdown mode"]
    #[inline(always)]
    pub fn fb_cfg_enable(&mut self) -> FB_CFG_ENABLE_W {
        FB_CFG_ENABLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configure FB config enable and wether Audio SRAM can be put into Deep Sleep by the Audio hardware\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gen_purpose_0](index.html) module"]
pub struct GEN_PURPOSE_0_SPEC;
impl crate::RegisterSpec for GEN_PURPOSE_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gen_purpose_0::R](R) reader structure"]
impl crate::Readable for GEN_PURPOSE_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gen_purpose_0::W](W) writer structure"]
impl crate::Writable for GEN_PURPOSE_0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GEN_PURPOSE_0 to value 0x0200"]
impl crate::Resettable for GEN_PURPOSE_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0200
    }
}
