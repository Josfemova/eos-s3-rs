#[doc = "Register `VOICE_CONFIG` reader"]
pub struct R(crate::R<VOICE_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VOICE_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VOICE_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VOICE_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VOICE_CONFIG` writer"]
pub struct W(crate::W<VOICE_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VOICE_CONFIG_SPEC>;
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
impl From<crate::W<VOICE_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VOICE_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Select the source for digital mic signal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMIC_SEL_A {
    #[doc = "0: DMIC source is PDM"]
    PDM = 0,
    #[doc = "1: DMIC source is I2S"]
    I2S = 1,
}
impl From<DMIC_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: DMIC_SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMIC_SEL` reader - Select the source for digital mic signal"]
pub struct DMIC_SEL_R(crate::FieldReader<bool, DMIC_SEL_A>);
impl DMIC_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMIC_SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMIC_SEL_A {
        match self.bits {
            false => DMIC_SEL_A::PDM,
            true => DMIC_SEL_A::I2S,
        }
    }
    #[doc = "Checks if the value of the field is `PDM`"]
    #[inline(always)]
    pub fn is_pdm(&self) -> bool {
        **self == DMIC_SEL_A::PDM
    }
    #[doc = "Checks if the value of the field is `I2S`"]
    #[inline(always)]
    pub fn is_i2s(&self) -> bool {
        **self == DMIC_SEL_A::I2S
    }
}
impl core::ops::Deref for DMIC_SEL_R {
    type Target = crate::FieldReader<bool, DMIC_SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMIC_SEL` writer - Select the source for digital mic signal"]
pub struct DMIC_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DMIC_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMIC_SEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "DMIC source is PDM"]
    #[inline(always)]
    pub fn pdm(self) -> &'a mut W {
        self.variant(DMIC_SEL_A::PDM)
    }
    #[doc = "DMIC source is I2S"]
    #[inline(always)]
    pub fn i2s(self) -> &'a mut W {
        self.variant(DMIC_SEL_A::I2S)
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
#[doc = "Select between external or internal sensory LSPD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPSD_SEL_A {
    #[doc = "0: Use internal sensory LPSD"]
    INTERNAL = 0,
    #[doc = "1: Use external sensory LPSD"]
    EXTERNAL = 1,
}
impl From<LPSD_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: LPSD_SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPSD_SEL` reader - Select between external or internal sensory LSPD"]
pub struct LPSD_SEL_R(crate::FieldReader<bool, LPSD_SEL_A>);
impl LPSD_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LPSD_SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPSD_SEL_A {
        match self.bits {
            false => LPSD_SEL_A::INTERNAL,
            true => LPSD_SEL_A::EXTERNAL,
        }
    }
    #[doc = "Checks if the value of the field is `INTERNAL`"]
    #[inline(always)]
    pub fn is_internal(&self) -> bool {
        **self == LPSD_SEL_A::INTERNAL
    }
    #[doc = "Checks if the value of the field is `EXTERNAL`"]
    #[inline(always)]
    pub fn is_external(&self) -> bool {
        **self == LPSD_SEL_A::EXTERNAL
    }
}
impl core::ops::Deref for LPSD_SEL_R {
    type Target = crate::FieldReader<bool, LPSD_SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPSD_SEL` writer - Select between external or internal sensory LSPD"]
pub struct LPSD_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LPSD_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPSD_SEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Use internal sensory LPSD"]
    #[inline(always)]
    pub fn internal(self) -> &'a mut W {
        self.variant(LPSD_SEL_A::INTERNAL)
    }
    #[doc = "Use external sensory LPSD"]
    #[inline(always)]
    pub fn external(self) -> &'a mut W {
        self.variant(LPSD_SEL_A::EXTERNAL)
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
        self.w.bits =
            (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Select either monoaural or stereo mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE_SEL_A {
    #[doc = "0: Audio mode selected as Mono"]
    MONO = 0,
    #[doc = "1: Audio mode selected as Stereo"]
    STEREO = 1,
}
impl From<MODE_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: MODE_SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODE_SEL` reader - Select either monoaural or stereo mode"]
pub struct MODE_SEL_R(crate::FieldReader<bool, MODE_SEL_A>);
impl MODE_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MODE_SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_SEL_A {
        match self.bits {
            false => MODE_SEL_A::MONO,
            true => MODE_SEL_A::STEREO,
        }
    }
    #[doc = "Checks if the value of the field is `MONO`"]
    #[inline(always)]
    pub fn is_mono(&self) -> bool {
        **self == MODE_SEL_A::MONO
    }
    #[doc = "Checks if the value of the field is `STEREO`"]
    #[inline(always)]
    pub fn is_stereo(&self) -> bool {
        **self == MODE_SEL_A::STEREO
    }
}
impl core::ops::Deref for MODE_SEL_R {
    type Target = crate::FieldReader<bool, MODE_SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE_SEL` writer - Select either monoaural or stereo mode"]
pub struct MODE_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_SEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Audio mode selected as Mono"]
    #[inline(always)]
    pub fn mono(self) -> &'a mut W {
        self.variant(MODE_SEL_A::MONO)
    }
    #[doc = "Audio mode selected as Stereo"]
    #[inline(always)]
    pub fn stereo(self) -> &'a mut W {
        self.variant(MODE_SEL_A::STEREO)
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
        self.w.bits =
            (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Select either right channel in mono mode or left channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MONO_CHN_SEL_A {
    #[doc = "0: Left channel in mono mode"]
    LEFT_CHANNEL = 0,
    #[doc = "1: Right channel in mono mode"]
    RIGHT_CHANNEL = 1,
}
impl From<MONO_CHN_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: MONO_CHN_SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MONO_CHN_SEL` reader - Select either right channel in mono mode or left channel"]
pub struct MONO_CHN_SEL_R(crate::FieldReader<bool, MONO_CHN_SEL_A>);
impl MONO_CHN_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MONO_CHN_SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MONO_CHN_SEL_A {
        match self.bits {
            false => MONO_CHN_SEL_A::LEFT_CHANNEL,
            true => MONO_CHN_SEL_A::RIGHT_CHANNEL,
        }
    }
    #[doc = "Checks if the value of the field is `LEFT_CHANNEL`"]
    #[inline(always)]
    pub fn is_left_channel(&self) -> bool {
        **self == MONO_CHN_SEL_A::LEFT_CHANNEL
    }
    #[doc = "Checks if the value of the field is `RIGHT_CHANNEL`"]
    #[inline(always)]
    pub fn is_right_channel(&self) -> bool {
        **self == MONO_CHN_SEL_A::RIGHT_CHANNEL
    }
}
impl core::ops::Deref for MONO_CHN_SEL_R {
    type Target = crate::FieldReader<bool, MONO_CHN_SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MONO_CHN_SEL` writer - Select either right channel in mono mode or left channel"]
pub struct MONO_CHN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MONO_CHN_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MONO_CHN_SEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Left channel in mono mode"]
    #[inline(always)]
    pub fn left_channel(self) -> &'a mut W {
        self.variant(MONO_CHN_SEL_A::LEFT_CHANNEL)
    }
    #[doc = "Right channel in mono mode"]
    #[inline(always)]
    pub fn right_channel(self) -> &'a mut W {
        self.variant(MONO_CHN_SEL_A::RIGHT_CHANNEL)
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
        self.w.bits =
            (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `I2S_DS_EN` reader - Set to enable the I2S Deep Sleep mode"]
pub struct I2S_DS_EN_R(crate::FieldReader<bool, bool>);
impl I2S_DS_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2S_DS_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2S_DS_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2S_DS_EN` writer - Set to enable the I2S Deep Sleep mode"]
pub struct I2S_DS_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_DS_EN_W<'a> {
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
            (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Choose the PDM voice scenario\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PDM_VOICE_SCENARIO_A {
    #[doc = "0: Choose voice scenario 1"]
    SCENARIO1 = 0,
    #[doc = "1: Choose voice scenario 2"]
    SCENARIO2 = 1,
    #[doc = "2: Choose voice scenario 3 mode 1"]
    SCENARIO3_MODE1 = 2,
    #[doc = "3: Choose voice scenario 3 mode 2"]
    SCENARIO3_MODE2 = 3,
    #[doc = "4: Choose voice scenario 3 mode 3"]
    SCENARIO3_MODE3 = 4,
}
impl From<PDM_VOICE_SCENARIO_A> for u8 {
    #[inline(always)]
    fn from(variant: PDM_VOICE_SCENARIO_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PDM_VOICE_SCENARIO` reader - Choose the PDM voice scenario"]
pub struct PDM_VOICE_SCENARIO_R(crate::FieldReader<u8, PDM_VOICE_SCENARIO_A>);
impl PDM_VOICE_SCENARIO_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PDM_VOICE_SCENARIO_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PDM_VOICE_SCENARIO_A> {
        match self.bits {
            0 => Some(PDM_VOICE_SCENARIO_A::SCENARIO1),
            1 => Some(PDM_VOICE_SCENARIO_A::SCENARIO2),
            2 => Some(PDM_VOICE_SCENARIO_A::SCENARIO3_MODE1),
            3 => Some(PDM_VOICE_SCENARIO_A::SCENARIO3_MODE2),
            4 => Some(PDM_VOICE_SCENARIO_A::SCENARIO3_MODE3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SCENARIO1`"]
    #[inline(always)]
    pub fn is_scenario1(&self) -> bool {
        **self == PDM_VOICE_SCENARIO_A::SCENARIO1
    }
    #[doc = "Checks if the value of the field is `SCENARIO2`"]
    #[inline(always)]
    pub fn is_scenario2(&self) -> bool {
        **self == PDM_VOICE_SCENARIO_A::SCENARIO2
    }
    #[doc = "Checks if the value of the field is `SCENARIO3_MODE1`"]
    #[inline(always)]
    pub fn is_scenario3_mode1(&self) -> bool {
        **self == PDM_VOICE_SCENARIO_A::SCENARIO3_MODE1
    }
    #[doc = "Checks if the value of the field is `SCENARIO3_MODE2`"]
    #[inline(always)]
    pub fn is_scenario3_mode2(&self) -> bool {
        **self == PDM_VOICE_SCENARIO_A::SCENARIO3_MODE2
    }
    #[doc = "Checks if the value of the field is `SCENARIO3_MODE3`"]
    #[inline(always)]
    pub fn is_scenario3_mode3(&self) -> bool {
        **self == PDM_VOICE_SCENARIO_A::SCENARIO3_MODE3
    }
}
impl core::ops::Deref for PDM_VOICE_SCENARIO_R {
    type Target = crate::FieldReader<u8, PDM_VOICE_SCENARIO_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDM_VOICE_SCENARIO` writer - Choose the PDM voice scenario"]
pub struct PDM_VOICE_SCENARIO_W<'a> {
    w: &'a mut W,
}
impl<'a> PDM_VOICE_SCENARIO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDM_VOICE_SCENARIO_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Choose voice scenario 1"]
    #[inline(always)]
    pub fn scenario1(self) -> &'a mut W {
        self.variant(PDM_VOICE_SCENARIO_A::SCENARIO1)
    }
    #[doc = "Choose voice scenario 2"]
    #[inline(always)]
    pub fn scenario2(self) -> &'a mut W {
        self.variant(PDM_VOICE_SCENARIO_A::SCENARIO2)
    }
    #[doc = "Choose voice scenario 3 mode 1"]
    #[inline(always)]
    pub fn scenario3_mode1(self) -> &'a mut W {
        self.variant(PDM_VOICE_SCENARIO_A::SCENARIO3_MODE1)
    }
    #[doc = "Choose voice scenario 3 mode 2"]
    #[inline(always)]
    pub fn scenario3_mode2(self) -> &'a mut W {
        self.variant(PDM_VOICE_SCENARIO_A::SCENARIO3_MODE2)
    }
    #[doc = "Choose voice scenario 3 mode 3"]
    #[inline(always)]
    pub fn scenario3_mode3(self) -> &'a mut W {
        self.variant(PDM_VOICE_SCENARIO_A::SCENARIO3_MODE3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x07 << 5)) | ((value as u32 & 0x07) << 5);
        self.w
    }
}
#[doc = "Set to switch mic to AP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDM_MIC_SWITCH_TO_AP_A {
    #[doc = "0: Don't switch to AP"]
    NO_SWITCH = 0,
    #[doc = "1: Switch to AP"]
    SWITCH = 1,
}
impl From<PDM_MIC_SWITCH_TO_AP_A> for bool {
    #[inline(always)]
    fn from(variant: PDM_MIC_SWITCH_TO_AP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDM_MIC_SWITCH_TO_AP` reader - Set to switch mic to AP"]
pub struct PDM_MIC_SWITCH_TO_AP_R(
    crate::FieldReader<bool, PDM_MIC_SWITCH_TO_AP_A>,
);
impl PDM_MIC_SWITCH_TO_AP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PDM_MIC_SWITCH_TO_AP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDM_MIC_SWITCH_TO_AP_A {
        match self.bits {
            false => PDM_MIC_SWITCH_TO_AP_A::NO_SWITCH,
            true => PDM_MIC_SWITCH_TO_AP_A::SWITCH,
        }
    }
    #[doc = "Checks if the value of the field is `NO_SWITCH`"]
    #[inline(always)]
    pub fn is_no_switch(&self) -> bool {
        **self == PDM_MIC_SWITCH_TO_AP_A::NO_SWITCH
    }
    #[doc = "Checks if the value of the field is `SWITCH`"]
    #[inline(always)]
    pub fn is_switch(&self) -> bool {
        **self == PDM_MIC_SWITCH_TO_AP_A::SWITCH
    }
}
impl core::ops::Deref for PDM_MIC_SWITCH_TO_AP_R {
    type Target = crate::FieldReader<bool, PDM_MIC_SWITCH_TO_AP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDM_MIC_SWITCH_TO_AP` writer - Set to switch mic to AP"]
pub struct PDM_MIC_SWITCH_TO_AP_W<'a> {
    w: &'a mut W,
}
impl<'a> PDM_MIC_SWITCH_TO_AP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDM_MIC_SWITCH_TO_AP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Don't switch to AP"]
    #[inline(always)]
    pub fn no_switch(self) -> &'a mut W {
        self.variant(PDM_MIC_SWITCH_TO_AP_A::NO_SWITCH)
    }
    #[doc = "Switch to AP"]
    #[inline(always)]
    pub fn switch(self) -> &'a mut W {
        self.variant(PDM_MIC_SWITCH_TO_AP_A::SWITCH)
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
        self.w.bits =
            (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `LPSD_USE_DC_BLOCK` reader - Set to use LPSD DC Block"]
pub struct LPSD_USE_DC_BLOCK_R(crate::FieldReader<bool, bool>);
impl LPSD_USE_DC_BLOCK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LPSD_USE_DC_BLOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPSD_USE_DC_BLOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPSD_USE_DC_BLOCK` writer - Set to use LPSD DC Block"]
pub struct LPSD_USE_DC_BLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> LPSD_USE_DC_BLOCK_W<'a> {
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
#[doc = "Choose the channel pcm data for LPSD in stereo mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPSD_MUX_A {
    #[doc = "0: Left channel pcm data to LPSD in stereo mode"]
    LEFT_CHANNEL = 0,
    #[doc = "1: Right channel pcm data to LPSD in stereo mode"]
    RIGHT_CHANNEL = 1,
}
impl From<LPSD_MUX_A> for bool {
    #[inline(always)]
    fn from(variant: LPSD_MUX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPSD_MUX` reader - Choose the channel pcm data for LPSD in stereo mode"]
pub struct LPSD_MUX_R(crate::FieldReader<bool, LPSD_MUX_A>);
impl LPSD_MUX_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LPSD_MUX_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPSD_MUX_A {
        match self.bits {
            false => LPSD_MUX_A::LEFT_CHANNEL,
            true => LPSD_MUX_A::RIGHT_CHANNEL,
        }
    }
    #[doc = "Checks if the value of the field is `LEFT_CHANNEL`"]
    #[inline(always)]
    pub fn is_left_channel(&self) -> bool {
        **self == LPSD_MUX_A::LEFT_CHANNEL
    }
    #[doc = "Checks if the value of the field is `RIGHT_CHANNEL`"]
    #[inline(always)]
    pub fn is_right_channel(&self) -> bool {
        **self == LPSD_MUX_A::RIGHT_CHANNEL
    }
}
impl core::ops::Deref for LPSD_MUX_R {
    type Target = crate::FieldReader<bool, LPSD_MUX_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPSD_MUX` writer - Choose the channel pcm data for LPSD in stereo mode"]
pub struct LPSD_MUX_W<'a> {
    w: &'a mut W,
}
impl<'a> LPSD_MUX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPSD_MUX_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Left channel pcm data to LPSD in stereo mode"]
    #[inline(always)]
    pub fn left_channel(self) -> &'a mut W {
        self.variant(LPSD_MUX_A::LEFT_CHANNEL)
    }
    #[doc = "Right channel pcm data to LPSD in stereo mode"]
    #[inline(always)]
    pub fn right_channel(self) -> &'a mut W {
        self.variant(LPSD_MUX_A::RIGHT_CHANNEL)
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
        self.w.bits =
            (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Set to disable HW LPSD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPSD_NO_A {
    #[doc = "0: Enable hardware LPSD"]
    ENABLE_HW = 0,
    #[doc = "1: Disable HW LPSD"]
    DISABLE_HW = 1,
}
impl From<LPSD_NO_A> for bool {
    #[inline(always)]
    fn from(variant: LPSD_NO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPSD_NO` reader - Set to disable HW LPSD"]
pub struct LPSD_NO_R(crate::FieldReader<bool, LPSD_NO_A>);
impl LPSD_NO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LPSD_NO_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPSD_NO_A {
        match self.bits {
            false => LPSD_NO_A::ENABLE_HW,
            true => LPSD_NO_A::DISABLE_HW,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE_HW`"]
    #[inline(always)]
    pub fn is_enable_hw(&self) -> bool {
        **self == LPSD_NO_A::ENABLE_HW
    }
    #[doc = "Checks if the value of the field is `DISABLE_HW`"]
    #[inline(always)]
    pub fn is_disable_hw(&self) -> bool {
        **self == LPSD_NO_A::DISABLE_HW
    }
}
impl core::ops::Deref for LPSD_NO_R {
    type Target = crate::FieldReader<bool, LPSD_NO_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPSD_NO` writer - Set to disable HW LPSD"]
pub struct LPSD_NO_W<'a> {
    w: &'a mut W,
}
impl<'a> LPSD_NO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPSD_NO_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable hardware LPSD"]
    #[inline(always)]
    pub fn enable_hw(self) -> &'a mut W {
        self.variant(LPSD_NO_A::ENABLE_HW)
    }
    #[doc = "Disable HW LPSD"]
    #[inline(always)]
    pub fn disable_hw(self) -> &'a mut W {
        self.variant(LPSD_NO_A::DISABLE_HW)
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
        self.w.bits =
            (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `I2S_FPGA_EN` reader - Set to enable the FPGA I2S"]
pub struct I2S_FPGA_EN_R(crate::FieldReader<bool, bool>);
impl I2S_FPGA_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2S_FPGA_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2S_FPGA_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2S_FPGA_EN` writer - Set to enable the FPGA I2S"]
pub struct I2S_FPGA_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_FPGA_EN_W<'a> {
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
            (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `DIV_AP` reader - AP_PDM_CKO_IN frequency divide-down ratio for AP clock detection"]
pub struct DIV_AP_R(crate::FieldReader<u8, u8>);
impl DIV_AP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DIV_AP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIV_AP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIV_AP` writer - AP_PDM_CKO_IN frequency divide-down ratio for AP clock detection"]
pub struct DIV_AP_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_AP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x07 << 15)) | ((value as u32 & 0x07) << 15);
        self.w
    }
}
#[doc = "Field `DIV_WD` reader - AP_PDM_CKO_IN clock detection window range"]
pub struct DIV_WD_R(crate::FieldReader<u8, u8>);
impl DIV_WD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DIV_WD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIV_WD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIV_WD` writer - AP_PDM_CKO_IN clock detection window range"]
pub struct DIV_WD_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_WD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x3f << 18)) | ((value as u32 & 0x3f) << 18);
        self.w
    }
}
#[doc = "Field `FIFO_0_CLEAR` reader - Set to flush FIFO"]
pub struct FIFO_0_CLEAR_R(crate::FieldReader<bool, bool>);
impl FIFO_0_CLEAR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FIFO_0_CLEAR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFO_0_CLEAR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFO_0_CLEAR` writer - Set to flush FIFO"]
pub struct FIFO_0_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFO_0_CLEAR_W<'a> {
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
            (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `FIFO_1_CLEAR` reader - Set to Flush FIFO"]
pub struct FIFO_1_CLEAR_R(crate::FieldReader<bool, bool>);
impl FIFO_1_CLEAR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FIFO_1_CLEAR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFO_1_CLEAR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFO_1_CLEAR` writer - Set to Flush FIFO"]
pub struct FIFO_1_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFO_1_CLEAR_W<'a> {
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
            (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Control the masking of the interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPSD_VOICE_DETECTED_MASK_A {
    #[doc = "0: Unmask the interrupt"]
    UNMASK = 0,
    #[doc = "1: Mask the interrupt"]
    MASK = 1,
}
impl From<LPSD_VOICE_DETECTED_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: LPSD_VOICE_DETECTED_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPSD_VOICE_DETECTED_MASK` reader - Control the masking of the interrupt"]
pub struct LPSD_VOICE_DETECTED_MASK_R(
    crate::FieldReader<bool, LPSD_VOICE_DETECTED_MASK_A>,
);
impl LPSD_VOICE_DETECTED_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LPSD_VOICE_DETECTED_MASK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPSD_VOICE_DETECTED_MASK_A {
        match self.bits {
            false => LPSD_VOICE_DETECTED_MASK_A::UNMASK,
            true => LPSD_VOICE_DETECTED_MASK_A::MASK,
        }
    }
    #[doc = "Checks if the value of the field is `UNMASK`"]
    #[inline(always)]
    pub fn is_unmask(&self) -> bool {
        **self == LPSD_VOICE_DETECTED_MASK_A::UNMASK
    }
    #[doc = "Checks if the value of the field is `MASK`"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        **self == LPSD_VOICE_DETECTED_MASK_A::MASK
    }
}
impl core::ops::Deref for LPSD_VOICE_DETECTED_MASK_R {
    type Target = crate::FieldReader<bool, LPSD_VOICE_DETECTED_MASK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPSD_VOICE_DETECTED_MASK` writer - Control the masking of the interrupt"]
pub struct LPSD_VOICE_DETECTED_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> LPSD_VOICE_DETECTED_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPSD_VOICE_DETECTED_MASK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Unmask the interrupt"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut W {
        self.variant(LPSD_VOICE_DETECTED_MASK_A::UNMASK)
    }
    #[doc = "Mask the interrupt"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(LPSD_VOICE_DETECTED_MASK_A::MASK)
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
        self.w.bits =
            (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Control the masking of the interrupt"]
pub type DMIC_VOICE_DETECTED_MASK_A = LPSD_VOICE_DETECTED_MASK_A;
#[doc = "Field `DMIC_VOICE_DETECTED_MASK` reader - Control the masking of the interrupt"]
pub type DMIC_VOICE_DETECTED_MASK_R = LPSD_VOICE_DETECTED_MASK_R;
#[doc = "Field `DMIC_VOICE_DETECTED_MASK` writer - Control the masking of the interrupt"]
pub struct DMIC_VOICE_DETECTED_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> DMIC_VOICE_DETECTED_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMIC_VOICE_DETECTED_MASK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Unmask the interrupt"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut W {
        self.variant(DMIC_VOICE_DETECTED_MASK_A::UNMASK)
    }
    #[doc = "Mask the interrupt"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(DMIC_VOICE_DETECTED_MASK_A::MASK)
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
        self.w.bits =
            (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Control the masking of the interrupt"]
pub type DMAC_BLK_DONE_MASK_A = LPSD_VOICE_DETECTED_MASK_A;
#[doc = "Field `DMAC_BLK_DONE_MASK` reader - Control the masking of the interrupt"]
pub type DMAC_BLK_DONE_MASK_R = LPSD_VOICE_DETECTED_MASK_R;
#[doc = "Field `DMAC_BLK_DONE_MASK` writer - Control the masking of the interrupt"]
pub struct DMAC_BLK_DONE_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAC_BLK_DONE_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMAC_BLK_DONE_MASK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Unmask the interrupt"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut W {
        self.variant(DMAC_BLK_DONE_MASK_A::UNMASK)
    }
    #[doc = "Mask the interrupt"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(DMAC_BLK_DONE_MASK_A::MASK)
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
        self.w.bits =
            (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Control the masking of the interrupt"]
pub type DMAC_BUF_DONE_MASK_A = LPSD_VOICE_DETECTED_MASK_A;
#[doc = "Field `DMAC_BUF_DONE_MASK` reader - Control the masking of the interrupt"]
pub type DMAC_BUF_DONE_MASK_R = LPSD_VOICE_DETECTED_MASK_R;
#[doc = "Field `DMAC_BUF_DONE_MASK` writer - Control the masking of the interrupt"]
pub struct DMAC_BUF_DONE_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAC_BUF_DONE_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMAC_BUF_DONE_MASK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Unmask the interrupt"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut W {
        self.variant(DMAC_BUF_DONE_MASK_A::UNMASK)
    }
    #[doc = "Mask the interrupt"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(DMAC_BUF_DONE_MASK_A::MASK)
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
        self.w.bits =
            (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Control the masking of the interrupt"]
pub type AP_PDM_CLK_IB_MASK_A = LPSD_VOICE_DETECTED_MASK_A;
#[doc = "Field `AP_PDM_CLK_IB_MASK` reader - Control the masking of the interrupt"]
pub type AP_PDM_CLK_IB_MASK_R = LPSD_VOICE_DETECTED_MASK_R;
#[doc = "Field `AP_PDM_CLK_IB_MASK` writer - Control the masking of the interrupt"]
pub struct AP_PDM_CLK_IB_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> AP_PDM_CLK_IB_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AP_PDM_CLK_IB_MASK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Unmask the interrupt"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut W {
        self.variant(AP_PDM_CLK_IB_MASK_A::UNMASK)
    }
    #[doc = "Mask the interrupt"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(AP_PDM_CLK_IB_MASK_A::MASK)
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
        self.w.bits =
            (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Control the masking of the interrupt"]
pub type AP_PDM_CLK_OFF_MASK_A = LPSD_VOICE_DETECTED_MASK_A;
#[doc = "Field `AP_PDM_CLK_OFF_MASK` reader - Control the masking of the interrupt"]
pub type AP_PDM_CLK_OFF_MASK_R = LPSD_VOICE_DETECTED_MASK_R;
#[doc = "Field `AP_PDM_CLK_OFF_MASK` writer - Control the masking of the interrupt"]
pub struct AP_PDM_CLK_OFF_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> AP_PDM_CLK_OFF_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AP_PDM_CLK_OFF_MASK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Unmask the interrupt"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut W {
        self.variant(AP_PDM_CLK_OFF_MASK_A::UNMASK)
    }
    #[doc = "Mask the interrupt"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(AP_PDM_CLK_OFF_MASK_A::MASK)
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
        self.w.bits =
            (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Select the source for digital mic signal"]
    #[inline(always)]
    pub fn dmic_sel(&self) -> DMIC_SEL_R {
        DMIC_SEL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Select between external or internal sensory LSPD"]
    #[inline(always)]
    pub fn lpsd_sel(&self) -> LPSD_SEL_R {
        LPSD_SEL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Select either monoaural or stereo mode"]
    #[inline(always)]
    pub fn mode_sel(&self) -> MODE_SEL_R {
        MODE_SEL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Select either right channel in mono mode or left channel"]
    #[inline(always)]
    pub fn mono_chn_sel(&self) -> MONO_CHN_SEL_R {
        MONO_CHN_SEL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Set to enable the I2S Deep Sleep mode"]
    #[inline(always)]
    pub fn i2s_ds_en(&self) -> I2S_DS_EN_R {
        I2S_DS_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:7 - Choose the PDM voice scenario"]
    #[inline(always)]
    pub fn pdm_voice_scenario(&self) -> PDM_VOICE_SCENARIO_R {
        PDM_VOICE_SCENARIO_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bit 8 - Set to switch mic to AP"]
    #[inline(always)]
    pub fn pdm_mic_switch_to_ap(&self) -> PDM_MIC_SWITCH_TO_AP_R {
        PDM_MIC_SWITCH_TO_AP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Set to use LPSD DC Block"]
    #[inline(always)]
    pub fn lpsd_use_dc_block(&self) -> LPSD_USE_DC_BLOCK_R {
        LPSD_USE_DC_BLOCK_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Choose the channel pcm data for LPSD in stereo mode"]
    #[inline(always)]
    pub fn lpsd_mux(&self) -> LPSD_MUX_R {
        LPSD_MUX_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Set to disable HW LPSD"]
    #[inline(always)]
    pub fn lpsd_no(&self) -> LPSD_NO_R {
        LPSD_NO_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Set to enable the FPGA I2S"]
    #[inline(always)]
    pub fn i2s_fpga_en(&self) -> I2S_FPGA_EN_R {
        I2S_FPGA_EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 15:17 - AP_PDM_CKO_IN frequency divide-down ratio for AP clock detection"]
    #[inline(always)]
    pub fn div_ap(&self) -> DIV_AP_R {
        DIV_AP_R::new(((self.bits >> 15) & 0x07) as u8)
    }
    #[doc = "Bits 18:23 - AP_PDM_CKO_IN clock detection window range"]
    #[inline(always)]
    pub fn div_wd(&self) -> DIV_WD_R {
        DIV_WD_R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bit 24 - Set to flush FIFO"]
    #[inline(always)]
    pub fn fifo_0_clear(&self) -> FIFO_0_CLEAR_R {
        FIFO_0_CLEAR_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Set to Flush FIFO"]
    #[inline(always)]
    pub fn fifo_1_clear(&self) -> FIFO_1_CLEAR_R {
        FIFO_1_CLEAR_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Control the masking of the interrupt"]
    #[inline(always)]
    pub fn lpsd_voice_detected_mask(&self) -> LPSD_VOICE_DETECTED_MASK_R {
        LPSD_VOICE_DETECTED_MASK_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Control the masking of the interrupt"]
    #[inline(always)]
    pub fn dmic_voice_detected_mask(&self) -> DMIC_VOICE_DETECTED_MASK_R {
        DMIC_VOICE_DETECTED_MASK_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Control the masking of the interrupt"]
    #[inline(always)]
    pub fn dmac_blk_done_mask(&self) -> DMAC_BLK_DONE_MASK_R {
        DMAC_BLK_DONE_MASK_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Control the masking of the interrupt"]
    #[inline(always)]
    pub fn dmac_buf_done_mask(&self) -> DMAC_BUF_DONE_MASK_R {
        DMAC_BUF_DONE_MASK_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Control the masking of the interrupt"]
    #[inline(always)]
    pub fn ap_pdm_clk_ib_mask(&self) -> AP_PDM_CLK_IB_MASK_R {
        AP_PDM_CLK_IB_MASK_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Control the masking of the interrupt"]
    #[inline(always)]
    pub fn ap_pdm_clk_off_mask(&self) -> AP_PDM_CLK_OFF_MASK_R {
        AP_PDM_CLK_OFF_MASK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Select the source for digital mic signal"]
    #[inline(always)]
    pub fn dmic_sel(&mut self) -> DMIC_SEL_W {
        DMIC_SEL_W { w: self }
    }
    #[doc = "Bit 1 - Select between external or internal sensory LSPD"]
    #[inline(always)]
    pub fn lpsd_sel(&mut self) -> LPSD_SEL_W {
        LPSD_SEL_W { w: self }
    }
    #[doc = "Bit 2 - Select either monoaural or stereo mode"]
    #[inline(always)]
    pub fn mode_sel(&mut self) -> MODE_SEL_W {
        MODE_SEL_W { w: self }
    }
    #[doc = "Bit 3 - Select either right channel in mono mode or left channel"]
    #[inline(always)]
    pub fn mono_chn_sel(&mut self) -> MONO_CHN_SEL_W {
        MONO_CHN_SEL_W { w: self }
    }
    #[doc = "Bit 4 - Set to enable the I2S Deep Sleep mode"]
    #[inline(always)]
    pub fn i2s_ds_en(&mut self) -> I2S_DS_EN_W {
        I2S_DS_EN_W { w: self }
    }
    #[doc = "Bits 5:7 - Choose the PDM voice scenario"]
    #[inline(always)]
    pub fn pdm_voice_scenario(&mut self) -> PDM_VOICE_SCENARIO_W {
        PDM_VOICE_SCENARIO_W { w: self }
    }
    #[doc = "Bit 8 - Set to switch mic to AP"]
    #[inline(always)]
    pub fn pdm_mic_switch_to_ap(&mut self) -> PDM_MIC_SWITCH_TO_AP_W {
        PDM_MIC_SWITCH_TO_AP_W { w: self }
    }
    #[doc = "Bit 9 - Set to use LPSD DC Block"]
    #[inline(always)]
    pub fn lpsd_use_dc_block(&mut self) -> LPSD_USE_DC_BLOCK_W {
        LPSD_USE_DC_BLOCK_W { w: self }
    }
    #[doc = "Bit 10 - Choose the channel pcm data for LPSD in stereo mode"]
    #[inline(always)]
    pub fn lpsd_mux(&mut self) -> LPSD_MUX_W {
        LPSD_MUX_W { w: self }
    }
    #[doc = "Bit 11 - Set to disable HW LPSD"]
    #[inline(always)]
    pub fn lpsd_no(&mut self) -> LPSD_NO_W {
        LPSD_NO_W { w: self }
    }
    #[doc = "Bit 12 - Set to enable the FPGA I2S"]
    #[inline(always)]
    pub fn i2s_fpga_en(&mut self) -> I2S_FPGA_EN_W {
        I2S_FPGA_EN_W { w: self }
    }
    #[doc = "Bits 15:17 - AP_PDM_CKO_IN frequency divide-down ratio for AP clock detection"]
    #[inline(always)]
    pub fn div_ap(&mut self) -> DIV_AP_W {
        DIV_AP_W { w: self }
    }
    #[doc = "Bits 18:23 - AP_PDM_CKO_IN clock detection window range"]
    #[inline(always)]
    pub fn div_wd(&mut self) -> DIV_WD_W {
        DIV_WD_W { w: self }
    }
    #[doc = "Bit 24 - Set to flush FIFO"]
    #[inline(always)]
    pub fn fifo_0_clear(&mut self) -> FIFO_0_CLEAR_W {
        FIFO_0_CLEAR_W { w: self }
    }
    #[doc = "Bit 25 - Set to Flush FIFO"]
    #[inline(always)]
    pub fn fifo_1_clear(&mut self) -> FIFO_1_CLEAR_W {
        FIFO_1_CLEAR_W { w: self }
    }
    #[doc = "Bit 26 - Control the masking of the interrupt"]
    #[inline(always)]
    pub fn lpsd_voice_detected_mask(&mut self) -> LPSD_VOICE_DETECTED_MASK_W {
        LPSD_VOICE_DETECTED_MASK_W { w: self }
    }
    #[doc = "Bit 27 - Control the masking of the interrupt"]
    #[inline(always)]
    pub fn dmic_voice_detected_mask(&mut self) -> DMIC_VOICE_DETECTED_MASK_W {
        DMIC_VOICE_DETECTED_MASK_W { w: self }
    }
    #[doc = "Bit 28 - Control the masking of the interrupt"]
    #[inline(always)]
    pub fn dmac_blk_done_mask(&mut self) -> DMAC_BLK_DONE_MASK_W {
        DMAC_BLK_DONE_MASK_W { w: self }
    }
    #[doc = "Bit 29 - Control the masking of the interrupt"]
    #[inline(always)]
    pub fn dmac_buf_done_mask(&mut self) -> DMAC_BUF_DONE_MASK_W {
        DMAC_BUF_DONE_MASK_W { w: self }
    }
    #[doc = "Bit 30 - Control the masking of the interrupt"]
    #[inline(always)]
    pub fn ap_pdm_clk_ib_mask(&mut self) -> AP_PDM_CLK_IB_MASK_W {
        AP_PDM_CLK_IB_MASK_W { w: self }
    }
    #[doc = "Bit 31 - Control the masking of the interrupt"]
    #[inline(always)]
    pub fn ap_pdm_clk_off_mask(&mut self) -> AP_PDM_CLK_OFF_MASK_W {
        AP_PDM_CLK_OFF_MASK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Audio system configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [voice_config](index.html) module"]
pub struct VOICE_CONFIG_SPEC;
impl crate::RegisterSpec for VOICE_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [voice_config::R](R) reader structure"]
impl crate::Readable for VOICE_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [voice_config::W](W) writer structure"]
impl crate::Writable for VOICE_CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets VOICE_CONFIG to value 0x0041_0000"]
impl crate::Resettable for VOICE_CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0041_0000
    }
}
