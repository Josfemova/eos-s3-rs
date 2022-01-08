#[doc = "Register `MISC_SW_PD` reader"]
pub struct R(crate::R<MISC_SW_PD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MISC_SW_PD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MISC_SW_PD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MISC_SW_PD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MISC_SW_PD` writer"]
pub struct W(crate::W<MISC_SW_PD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MISC_SW_PD_SPEC>;
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
impl From<crate::W<MISC_SW_PD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MISC_SW_PD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "System DMA Power Domain. Set to put the power domain to power saving mode according to configuration setting, HW will clear it once power down sequence is finished.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDMA_SOFTWARE_PD_A {
    #[doc = "1: Put the power domain to power saving mode according to configuration setting, HW will clear it once power down sequence is finished."]
    POWER_DOWN = 1,
}
impl From<SDMA_SOFTWARE_PD_A> for bool {
    #[inline(always)]
    fn from(variant: SDMA_SOFTWARE_PD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDMA_Software_PD` reader - System DMA Power Domain. Set to put the power domain to power saving mode according to configuration setting, HW will clear it once power down sequence is finished."]
pub struct SDMA_SOFTWARE_PD_R(crate::FieldReader<bool, SDMA_SOFTWARE_PD_A>);
impl SDMA_SOFTWARE_PD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SDMA_SOFTWARE_PD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SDMA_SOFTWARE_PD_A> {
        match self.bits {
            true => Some(SDMA_SOFTWARE_PD_A::POWER_DOWN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `POWER_DOWN`"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        **self == SDMA_SOFTWARE_PD_A::POWER_DOWN
    }
}
impl core::ops::Deref for SDMA_SOFTWARE_PD_R {
    type Target = crate::FieldReader<bool, SDMA_SOFTWARE_PD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDMA_Software_PD` writer - System DMA Power Domain. Set to put the power domain to power saving mode according to configuration setting, HW will clear it once power down sequence is finished."]
pub struct SDMA_SOFTWARE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> SDMA_SOFTWARE_PD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDMA_SOFTWARE_PD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Put the power domain to power saving mode according to configuration setting, HW will clear it once power down sequence is finished."]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut W {
        self.variant(SDMA_SOFTWARE_PD_A::POWER_DOWN)
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
#[doc = "Field `General_Purpose_1` reader - General purpose SFR"]
pub struct GENERAL_PURPOSE_1_R(crate::FieldReader<bool, bool>);
impl GENERAL_PURPOSE_1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GENERAL_PURPOSE_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GENERAL_PURPOSE_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `General_Purpose_1` writer - General purpose SFR"]
pub struct GENERAL_PURPOSE_1_W<'a> {
    w: &'a mut W,
}
impl<'a> GENERAL_PURPOSE_1_W<'a> {
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
#[doc = "Field `General_Purpose_2` reader - General purpose SFR"]
pub struct GENERAL_PURPOSE_2_R(crate::FieldReader<bool, bool>);
impl GENERAL_PURPOSE_2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GENERAL_PURPOSE_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GENERAL_PURPOSE_2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `General_Purpose_2` writer - General purpose SFR"]
pub struct GENERAL_PURPOSE_2_W<'a> {
    w: &'a mut W,
}
impl<'a> GENERAL_PURPOSE_2_W<'a> {
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
#[doc = "Field `General_Purpose_3` reader - General purpose SFR"]
pub struct GENERAL_PURPOSE_3_R(crate::FieldReader<bool, bool>);
impl GENERAL_PURPOSE_3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GENERAL_PURPOSE_3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GENERAL_PURPOSE_3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `General_Purpose_3` writer - General purpose SFR"]
pub struct GENERAL_PURPOSE_3_W<'a> {
    w: &'a mut W,
}
impl<'a> GENERAL_PURPOSE_3_W<'a> {
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
#[doc = "Reserved for Future Use -- Power Domain. Set to put the power domain to power saving mode according to configuration setting, HW will clear it once power down sequence is finished."]
pub type I2S_SOFTWARE_PD_A = SDMA_SOFTWARE_PD_A;
#[doc = "Field `I2S_Software_PD` reader - Reserved for Future Use -- Power Domain. Set to put the power domain to power saving mode according to configuration setting, HW will clear it once power down sequence is finished."]
pub type I2S_SOFTWARE_PD_R = SDMA_SOFTWARE_PD_R;
#[doc = "Field `I2S_Software_PD` writer - Reserved for Future Use -- Power Domain. Set to put the power domain to power saving mode according to configuration setting, HW will clear it once power down sequence is finished."]
pub struct I2S_SOFTWARE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_SOFTWARE_PD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2S_SOFTWARE_PD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Put the power domain to power saving mode according to configuration setting, HW will clear it once power down sequence is finished."]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut W {
        self.variant(I2S_SOFTWARE_PD_A::POWER_DOWN)
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
            (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Reserved for Future Use -- Power Domain. Set to put the power domain to power saving mode according to configuration setting, HW will clear it once power down sequence is finished."]
pub type A1_SOFTWARE_PD_A = SDMA_SOFTWARE_PD_A;
#[doc = "Field `A1_Software_PD` reader - Reserved for Future Use -- Power Domain. Set to put the power domain to power saving mode according to configuration setting, HW will clear it once power down sequence is finished."]
pub type A1_SOFTWARE_PD_R = SDMA_SOFTWARE_PD_R;
#[doc = "Field `A1_Software_PD` writer - Reserved for Future Use -- Power Domain. Set to put the power domain to power saving mode according to configuration setting, HW will clear it once power down sequence is finished."]
pub struct A1_SOFTWARE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> A1_SOFTWARE_PD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: A1_SOFTWARE_PD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Put the power domain to power saving mode according to configuration setting, HW will clear it once power down sequence is finished."]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut W {
        self.variant(A1_SOFTWARE_PD_A::POWER_DOWN)
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
            (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - System DMA Power Domain. Set to put the power domain to power saving mode according to configuration setting, HW will clear it once power down sequence is finished."]
    #[inline(always)]
    pub fn sdma_software_pd(&self) -> SDMA_SOFTWARE_PD_R {
        SDMA_SOFTWARE_PD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - General purpose SFR"]
    #[inline(always)]
    pub fn general_purpose_1(&self) -> GENERAL_PURPOSE_1_R {
        GENERAL_PURPOSE_1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - General purpose SFR"]
    #[inline(always)]
    pub fn general_purpose_2(&self) -> GENERAL_PURPOSE_2_R {
        GENERAL_PURPOSE_2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - General purpose SFR"]
    #[inline(always)]
    pub fn general_purpose_3(&self) -> GENERAL_PURPOSE_3_R {
        GENERAL_PURPOSE_3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Reserved for Future Use -- Power Domain. Set to put the power domain to power saving mode according to configuration setting, HW will clear it once power down sequence is finished."]
    #[inline(always)]
    pub fn i2s_software_pd(&self) -> I2S_SOFTWARE_PD_R {
        I2S_SOFTWARE_PD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Reserved for Future Use -- Power Domain. Set to put the power domain to power saving mode according to configuration setting, HW will clear it once power down sequence is finished."]
    #[inline(always)]
    pub fn a1_software_pd(&self) -> A1_SOFTWARE_PD_R {
        A1_SOFTWARE_PD_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - System DMA Power Domain. Set to put the power domain to power saving mode according to configuration setting, HW will clear it once power down sequence is finished."]
    #[inline(always)]
    pub fn sdma_software_pd(&mut self) -> SDMA_SOFTWARE_PD_W {
        SDMA_SOFTWARE_PD_W { w: self }
    }
    #[doc = "Bit 1 - General purpose SFR"]
    #[inline(always)]
    pub fn general_purpose_1(&mut self) -> GENERAL_PURPOSE_1_W {
        GENERAL_PURPOSE_1_W { w: self }
    }
    #[doc = "Bit 2 - General purpose SFR"]
    #[inline(always)]
    pub fn general_purpose_2(&mut self) -> GENERAL_PURPOSE_2_W {
        GENERAL_PURPOSE_2_W { w: self }
    }
    #[doc = "Bit 3 - General purpose SFR"]
    #[inline(always)]
    pub fn general_purpose_3(&mut self) -> GENERAL_PURPOSE_3_W {
        GENERAL_PURPOSE_3_W { w: self }
    }
    #[doc = "Bit 5 - Reserved for Future Use -- Power Domain. Set to put the power domain to power saving mode according to configuration setting, HW will clear it once power down sequence is finished."]
    #[inline(always)]
    pub fn i2s_software_pd(&mut self) -> I2S_SOFTWARE_PD_W {
        I2S_SOFTWARE_PD_W { w: self }
    }
    #[doc = "Bit 6 - Reserved for Future Use -- Power Domain. Set to put the power domain to power saving mode according to configuration setting, HW will clear it once power down sequence is finished."]
    #[inline(always)]
    pub fn a1_software_pd(&mut self) -> A1_SOFTWARE_PD_W {
        A1_SOFTWARE_PD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Register for triggering power down events in MISC power domains + some general purpose SFR's (RWHC)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [misc_sw_pd](index.html) module"]
pub struct MISC_SW_PD_SPEC;
impl crate::RegisterSpec for MISC_SW_PD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [misc_sw_pd::R](R) reader structure"]
impl crate::Readable for MISC_SW_PD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [misc_sw_pd::W](W) writer structure"]
impl crate::Writable for MISC_SW_PD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MISC_SW_PD to value 0"]
impl crate::Resettable for MISC_SW_PD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
