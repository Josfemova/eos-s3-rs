#[doc = "Register `AUDIO_SW_PD` reader"]
pub struct R(crate::R<AUDIO_SW_PD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AUDIO_SW_PD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AUDIO_SW_PD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AUDIO_SW_PD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AUDIO_SW_PD` writer"]
pub struct W(crate::W<AUDIO_SW_PD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AUDIO_SW_PD_SPEC>;
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
impl From<crate::W<AUDIO_SW_PD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AUDIO_SW_PD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Set to trigger a power down event. Bit will be cleared once power down sequence is finished\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUDIO_AD0_PD_A {
    #[doc = "1: Put the power domain to shut down, HW will clear it once power down sequence is finished."]
    POWER_DOWN = 1,
}
impl From<AUDIO_AD0_PD_A> for bool {
    #[inline(always)]
    fn from(variant: AUDIO_AD0_PD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Audio_AD0_PD` reader - Set to trigger a power down event. Bit will be cleared once power down sequence is finished"]
pub struct AUDIO_AD0_PD_R(crate::FieldReader<bool, AUDIO_AD0_PD_A>);
impl AUDIO_AD0_PD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AUDIO_AD0_PD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<AUDIO_AD0_PD_A> {
        match self.bits {
            true => Some(AUDIO_AD0_PD_A::POWER_DOWN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `POWER_DOWN`"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        **self == AUDIO_AD0_PD_A::POWER_DOWN
    }
}
impl core::ops::Deref for AUDIO_AD0_PD_R {
    type Target = crate::FieldReader<bool, AUDIO_AD0_PD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Audio_AD0_PD` writer - Set to trigger a power down event. Bit will be cleared once power down sequence is finished"]
pub struct AUDIO_AD0_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> AUDIO_AD0_PD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AUDIO_AD0_PD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Put the power domain to shut down, HW will clear it once power down sequence is finished."]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut W {
        self.variant(AUDIO_AD0_PD_A::POWER_DOWN)
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
#[doc = "Set to trigger a power down event. Bit will be cleared once power down sequence is finished"]
pub type AUDIO_AD1_PD_A = AUDIO_AD0_PD_A;
#[doc = "Field `Audio_AD1_PD` reader - Set to trigger a power down event. Bit will be cleared once power down sequence is finished"]
pub type AUDIO_AD1_PD_R = AUDIO_AD0_PD_R;
#[doc = "Field `Audio_AD1_PD` writer - Set to trigger a power down event. Bit will be cleared once power down sequence is finished"]
pub struct AUDIO_AD1_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> AUDIO_AD1_PD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AUDIO_AD1_PD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Put the power domain to shut down, HW will clear it once power down sequence is finished."]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut W {
        self.variant(AUDIO_AD1_PD_A::POWER_DOWN)
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
#[doc = "Set to trigger a power down event. Bit will be cleared once power down sequence is finished"]
pub type AUDIO_AD2_PD_A = AUDIO_AD0_PD_A;
#[doc = "Field `Audio_AD2_PD` reader - Set to trigger a power down event. Bit will be cleared once power down sequence is finished"]
pub type AUDIO_AD2_PD_R = AUDIO_AD0_PD_R;
#[doc = "Field `Audio_AD2_PD` writer - Set to trigger a power down event. Bit will be cleared once power down sequence is finished"]
pub struct AUDIO_AD2_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> AUDIO_AD2_PD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AUDIO_AD2_PD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Put the power domain to shut down, HW will clear it once power down sequence is finished."]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut W {
        self.variant(AUDIO_AD2_PD_A::POWER_DOWN)
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
#[doc = "Set to trigger a power down event. Bit will be cleared once power down sequence is finished"]
pub type AUDIO_AD3_PD_A = AUDIO_AD0_PD_A;
#[doc = "Field `Audio_AD3_PD` reader - Set to trigger a power down event. Bit will be cleared once power down sequence is finished"]
pub type AUDIO_AD3_PD_R = AUDIO_AD0_PD_R;
#[doc = "Field `Audio_AD3_PD` writer - Set to trigger a power down event. Bit will be cleared once power down sequence is finished"]
pub struct AUDIO_AD3_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> AUDIO_AD3_PD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AUDIO_AD3_PD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Put the power domain to shut down, HW will clear it once power down sequence is finished."]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut W {
        self.variant(AUDIO_AD3_PD_A::POWER_DOWN)
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
#[doc = "Set to trigger a power down event. Bit will be cleared once power down sequence is finished"]
pub type AUDIO_AD4_PD_A = AUDIO_AD0_PD_A;
#[doc = "Field `Audio_AD4_PD` reader - Set to trigger a power down event. Bit will be cleared once power down sequence is finished"]
pub type AUDIO_AD4_PD_R = AUDIO_AD0_PD_R;
#[doc = "Field `Audio_AD4_PD` writer - Set to trigger a power down event. Bit will be cleared once power down sequence is finished"]
pub struct AUDIO_AD4_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> AUDIO_AD4_PD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AUDIO_AD4_PD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Put the power domain to shut down, HW will clear it once power down sequence is finished."]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut W {
        self.variant(AUDIO_AD4_PD_A::POWER_DOWN)
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
            (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Set to trigger a power down event. Bit will be cleared once power down sequence is finished"]
pub type AUDIO_AD5_PD_A = AUDIO_AD0_PD_A;
#[doc = "Field `Audio_AD5_PD` reader - Set to trigger a power down event. Bit will be cleared once power down sequence is finished"]
pub type AUDIO_AD5_PD_R = AUDIO_AD0_PD_R;
#[doc = "Field `Audio_AD5_PD` writer - Set to trigger a power down event. Bit will be cleared once power down sequence is finished"]
pub struct AUDIO_AD5_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> AUDIO_AD5_PD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AUDIO_AD5_PD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Put the power domain to shut down, HW will clear it once power down sequence is finished."]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut W {
        self.variant(AUDIO_AD5_PD_A::POWER_DOWN)
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
impl R {
    #[doc = "Bit 0 - Set to trigger a power down event. Bit will be cleared once power down sequence is finished"]
    #[inline(always)]
    pub fn audio_ad0_pd(&self) -> AUDIO_AD0_PD_R {
        AUDIO_AD0_PD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Set to trigger a power down event. Bit will be cleared once power down sequence is finished"]
    #[inline(always)]
    pub fn audio_ad1_pd(&self) -> AUDIO_AD1_PD_R {
        AUDIO_AD1_PD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Set to trigger a power down event. Bit will be cleared once power down sequence is finished"]
    #[inline(always)]
    pub fn audio_ad2_pd(&self) -> AUDIO_AD2_PD_R {
        AUDIO_AD2_PD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Set to trigger a power down event. Bit will be cleared once power down sequence is finished"]
    #[inline(always)]
    pub fn audio_ad3_pd(&self) -> AUDIO_AD3_PD_R {
        AUDIO_AD3_PD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Set to trigger a power down event. Bit will be cleared once power down sequence is finished"]
    #[inline(always)]
    pub fn audio_ad4_pd(&self) -> AUDIO_AD4_PD_R {
        AUDIO_AD4_PD_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Set to trigger a power down event. Bit will be cleared once power down sequence is finished"]
    #[inline(always)]
    pub fn audio_ad5_pd(&self) -> AUDIO_AD5_PD_R {
        AUDIO_AD5_PD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set to trigger a power down event. Bit will be cleared once power down sequence is finished"]
    #[inline(always)]
    pub fn audio_ad0_pd(&mut self) -> AUDIO_AD0_PD_W {
        AUDIO_AD0_PD_W { w: self }
    }
    #[doc = "Bit 1 - Set to trigger a power down event. Bit will be cleared once power down sequence is finished"]
    #[inline(always)]
    pub fn audio_ad1_pd(&mut self) -> AUDIO_AD1_PD_W {
        AUDIO_AD1_PD_W { w: self }
    }
    #[doc = "Bit 2 - Set to trigger a power down event. Bit will be cleared once power down sequence is finished"]
    #[inline(always)]
    pub fn audio_ad2_pd(&mut self) -> AUDIO_AD2_PD_W {
        AUDIO_AD2_PD_W { w: self }
    }
    #[doc = "Bit 3 - Set to trigger a power down event. Bit will be cleared once power down sequence is finished"]
    #[inline(always)]
    pub fn audio_ad3_pd(&mut self) -> AUDIO_AD3_PD_W {
        AUDIO_AD3_PD_W { w: self }
    }
    #[doc = "Bit 4 - Set to trigger a power down event. Bit will be cleared once power down sequence is finished"]
    #[inline(always)]
    pub fn audio_ad4_pd(&mut self) -> AUDIO_AD4_PD_W {
        AUDIO_AD4_PD_W { w: self }
    }
    #[doc = "Bit 5 - Set to trigger a power down event. Bit will be cleared once power down sequence is finished"]
    #[inline(always)]
    pub fn audio_ad5_pd(&mut self) -> AUDIO_AD5_PD_W {
        AUDIO_AD5_PD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Register for triggering power-down events in Audio power domains. (RWHC)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [audio_sw_pd](index.html) module"]
pub struct AUDIO_SW_PD_SPEC;
impl crate::RegisterSpec for AUDIO_SW_PD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [audio_sw_pd::R](R) reader structure"]
impl crate::Readable for AUDIO_SW_PD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [audio_sw_pd::W](W) writer structure"]
impl crate::Writable for AUDIO_SW_PD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AUDIO_SW_PD to value 0"]
impl crate::Resettable for AUDIO_SW_PD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
