#[doc = "Register `GEN_PURPOSE_1` reader"]
pub struct R(crate::R<GEN_PURPOSE_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GEN_PURPOSE_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GEN_PURPOSE_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GEN_PURPOSE_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GEN_PURPOSE_1` writer"]
pub struct W(crate::W<GEN_PURPOSE_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GEN_PURPOSE_1_SPEC>;
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
impl From<crate::W<GEN_PURPOSE_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GEN_PURPOSE_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOW_POWER_Mode` reader - Set to enable the low power mdoe"]
pub struct LOW_POWER_MODE_R(crate::FieldReader<bool, bool>);
impl LOW_POWER_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOW_POWER_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOW_POWER_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOW_POWER_Mode` writer - Set to enable the low power mdoe"]
pub struct LOW_POWER_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> LOW_POWER_MODE_W<'a> {
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
#[doc = "Field `LOW_POWER_MODE_M4` reader - Set to use M4 Power State to Turn on/off OSC and switching the Clock between OSC and RTC if Bit 0 is 1"]
pub struct LOW_POWER_MODE_M4_R(crate::FieldReader<bool, bool>);
impl LOW_POWER_MODE_M4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOW_POWER_MODE_M4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOW_POWER_MODE_M4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOW_POWER_MODE_M4` writer - Set to use M4 Power State to Turn on/off OSC and switching the Clock between OSC and RTC if Bit 0 is 1"]
pub struct LOW_POWER_MODE_M4_W<'a> {
    w: &'a mut W,
}
impl<'a> LOW_POWER_MODE_M4_W<'a> {
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
#[doc = "Field `LOW_POWER_MODE_FFE` reader - Set to use FFE Power State to Turn on/off OSC and switching the Clock between OSC and RTC if Bit 0 is 1."]
pub struct LOW_POWER_MODE_FFE_R(crate::FieldReader<bool, bool>);
impl LOW_POWER_MODE_FFE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOW_POWER_MODE_FFE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOW_POWER_MODE_FFE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOW_POWER_MODE_FFE` writer - Set to use FFE Power State to Turn on/off OSC and switching the Clock between OSC and RTC if Bit 0 is 1."]
pub struct LOW_POWER_MODE_FFE_W<'a> {
    w: &'a mut W,
}
impl<'a> LOW_POWER_MODE_FFE_W<'a> {
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
#[doc = "Field `Kickoff_FFE_use_INT` reader - Set to use external INT to wake up FFE."]
pub struct KICKOFF_FFE_USE_INT_R(crate::FieldReader<bool, bool>);
impl KICKOFF_FFE_USE_INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        KICKOFF_FFE_USE_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KICKOFF_FFE_USE_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Kickoff_FFE_use_INT` writer - Set to use external INT to wake up FFE."]
pub struct KICKOFF_FFE_USE_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> KICKOFF_FFE_USE_INT_W<'a> {
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
            (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
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
        self.w.bits =
            (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Set to enable the low power mdoe"]
    #[inline(always)]
    pub fn low_power_mode(&self) -> LOW_POWER_MODE_R {
        LOW_POWER_MODE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Set to use M4 Power State to Turn on/off OSC and switching the Clock between OSC and RTC if Bit 0 is 1"]
    #[inline(always)]
    pub fn low_power_mode_m4(&self) -> LOW_POWER_MODE_M4_R {
        LOW_POWER_MODE_M4_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Set to use FFE Power State to Turn on/off OSC and switching the Clock between OSC and RTC if Bit 0 is 1."]
    #[inline(always)]
    pub fn low_power_mode_ffe(&self) -> LOW_POWER_MODE_FFE_R {
        LOW_POWER_MODE_FFE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Set to use external INT to wake up FFE."]
    #[inline(always)]
    pub fn kickoff_ffe_use_int(&self) -> KICKOFF_FFE_USE_INT_R {
        KICKOFF_FFE_USE_INT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:7 - General purpose SFR"]
    #[inline(always)]
    pub fn general_purpose(&self) -> GENERAL_PURPOSE_R {
        GENERAL_PURPOSE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Set to enable the low power mdoe"]
    #[inline(always)]
    pub fn low_power_mode(&mut self) -> LOW_POWER_MODE_W {
        LOW_POWER_MODE_W { w: self }
    }
    #[doc = "Bit 1 - Set to use M4 Power State to Turn on/off OSC and switching the Clock between OSC and RTC if Bit 0 is 1"]
    #[inline(always)]
    pub fn low_power_mode_m4(&mut self) -> LOW_POWER_MODE_M4_W {
        LOW_POWER_MODE_M4_W { w: self }
    }
    #[doc = "Bit 2 - Set to use FFE Power State to Turn on/off OSC and switching the Clock between OSC and RTC if Bit 0 is 1."]
    #[inline(always)]
    pub fn low_power_mode_ffe(&mut self) -> LOW_POWER_MODE_FFE_W {
        LOW_POWER_MODE_FFE_W { w: self }
    }
    #[doc = "Bit 3 - Set to use external INT to wake up FFE."]
    #[inline(always)]
    pub fn kickoff_ffe_use_int(&mut self) -> KICKOFF_FFE_USE_INT_W {
        KICKOFF_FFE_USE_INT_W { w: self }
    }
    #[doc = "Bits 4:7 - General purpose SFR"]
    #[inline(always)]
    pub fn general_purpose(&mut self) -> GENERAL_PURPOSE_W {
        GENERAL_PURPOSE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control for: Wether ext-interrupt can be used to wake up FFE, and clock switching for FFE/M4 power domains\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gen_purpose_1](index.html) module"]
pub struct GEN_PURPOSE_1_SPEC;
impl crate::RegisterSpec for GEN_PURPOSE_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gen_purpose_1::R](R) reader structure"]
impl crate::Readable for GEN_PURPOSE_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gen_purpose_1::W](W) writer structure"]
impl crate::Writable for GEN_PURPOSE_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GEN_PURPOSE_1 to value 0"]
impl crate::Resettable for GEN_PURPOSE_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
