#[doc = "Register `AUDIO_SRAM_SW_WU` reader"]
pub struct R(crate::R<AUDIO_SRAM_SW_WU_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AUDIO_SRAM_SW_WU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AUDIO_SRAM_SW_WU_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AUDIO_SRAM_SW_WU_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AUDIO_SRAM_SW_WU` writer"]
pub struct W(crate::W<AUDIO_SRAM_SW_WU_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AUDIO_SRAM_SW_WU_SPEC>;
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
impl From<crate::W<AUDIO_SRAM_SW_WU_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AUDIO_SRAM_SW_WU_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Set to trigger a wake up event. Bit will be cleared once wake up sequence is finished\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUDIO_AD0_WU_A {
    #[doc = "1: wake up power domain, HW will clear it once power up sequence is finished."]
    WAKE_UP = 1,
}
impl From<AUDIO_AD0_WU_A> for bool {
    #[inline(always)]
    fn from(variant: AUDIO_AD0_WU_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Audio_AD0_WU` reader - Set to trigger a wake up event. Bit will be cleared once wake up sequence is finished"]
pub struct AUDIO_AD0_WU_R(crate::FieldReader<bool, AUDIO_AD0_WU_A>);
impl AUDIO_AD0_WU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AUDIO_AD0_WU_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<AUDIO_AD0_WU_A> {
        match self.bits {
            true => Some(AUDIO_AD0_WU_A::WAKE_UP),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `WAKE_UP`"]
    #[inline(always)]
    pub fn is_wake_up(&self) -> bool {
        **self == AUDIO_AD0_WU_A::WAKE_UP
    }
}
impl core::ops::Deref for AUDIO_AD0_WU_R {
    type Target = crate::FieldReader<bool, AUDIO_AD0_WU_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Audio_AD0_WU` writer - Set to trigger a wake up event. Bit will be cleared once wake up sequence is finished"]
pub struct AUDIO_AD0_WU_W<'a> {
    w: &'a mut W,
}
impl<'a> AUDIO_AD0_WU_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AUDIO_AD0_WU_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "wake up power domain, HW will clear it once power up sequence is finished."]
    #[inline(always)]
    pub fn wake_up(self) -> &'a mut W {
        self.variant(AUDIO_AD0_WU_A::WAKE_UP)
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
#[doc = "Set to trigger a wake up event. Bit will be cleared once wake up sequence is finished"]
pub type AUDIO_AD1_WU_A = AUDIO_AD0_WU_A;
#[doc = "Field `Audio_AD1_WU` reader - Set to trigger a wake up event. Bit will be cleared once wake up sequence is finished"]
pub type AUDIO_AD1_WU_R = AUDIO_AD0_WU_R;
#[doc = "Field `Audio_AD1_WU` writer - Set to trigger a wake up event. Bit will be cleared once wake up sequence is finished"]
pub struct AUDIO_AD1_WU_W<'a> {
    w: &'a mut W,
}
impl<'a> AUDIO_AD1_WU_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AUDIO_AD1_WU_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "wake up power domain, HW will clear it once power up sequence is finished."]
    #[inline(always)]
    pub fn wake_up(self) -> &'a mut W {
        self.variant(AUDIO_AD1_WU_A::WAKE_UP)
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
#[doc = "Set to trigger a wake up event. Bit will be cleared once wake up sequence is finished"]
pub type AUDIO_AD2_WU_A = AUDIO_AD0_WU_A;
#[doc = "Field `Audio_AD2_WU` reader - Set to trigger a wake up event. Bit will be cleared once wake up sequence is finished"]
pub type AUDIO_AD2_WU_R = AUDIO_AD0_WU_R;
#[doc = "Field `Audio_AD2_WU` writer - Set to trigger a wake up event. Bit will be cleared once wake up sequence is finished"]
pub struct AUDIO_AD2_WU_W<'a> {
    w: &'a mut W,
}
impl<'a> AUDIO_AD2_WU_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AUDIO_AD2_WU_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "wake up power domain, HW will clear it once power up sequence is finished."]
    #[inline(always)]
    pub fn wake_up(self) -> &'a mut W {
        self.variant(AUDIO_AD2_WU_A::WAKE_UP)
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
#[doc = "Set to trigger a wake up event. Bit will be cleared once wake up sequence is finished"]
pub type AUDIO_AD3_WU_A = AUDIO_AD0_WU_A;
#[doc = "Field `Audio_AD3_WU` reader - Set to trigger a wake up event. Bit will be cleared once wake up sequence is finished"]
pub type AUDIO_AD3_WU_R = AUDIO_AD0_WU_R;
#[doc = "Field `Audio_AD3_WU` writer - Set to trigger a wake up event. Bit will be cleared once wake up sequence is finished"]
pub struct AUDIO_AD3_WU_W<'a> {
    w: &'a mut W,
}
impl<'a> AUDIO_AD3_WU_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AUDIO_AD3_WU_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "wake up power domain, HW will clear it once power up sequence is finished."]
    #[inline(always)]
    pub fn wake_up(self) -> &'a mut W {
        self.variant(AUDIO_AD3_WU_A::WAKE_UP)
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
#[doc = "Set to trigger a wake up event. Bit will be cleared once wake up sequence is finished"]
pub type AUDIO_AD4_WU_A = AUDIO_AD0_WU_A;
#[doc = "Field `Audio_AD4_WU` reader - Set to trigger a wake up event. Bit will be cleared once wake up sequence is finished"]
pub type AUDIO_AD4_WU_R = AUDIO_AD0_WU_R;
#[doc = "Field `Audio_AD4_WU` writer - Set to trigger a wake up event. Bit will be cleared once wake up sequence is finished"]
pub struct AUDIO_AD4_WU_W<'a> {
    w: &'a mut W,
}
impl<'a> AUDIO_AD4_WU_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AUDIO_AD4_WU_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "wake up power domain, HW will clear it once power up sequence is finished."]
    #[inline(always)]
    pub fn wake_up(self) -> &'a mut W {
        self.variant(AUDIO_AD4_WU_A::WAKE_UP)
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
#[doc = "Set to trigger a wake up event. Bit will be cleared once wake up sequence is finished"]
pub type AUDIO_AD5_WU_A = AUDIO_AD0_WU_A;
#[doc = "Field `Audio_AD5_WU` reader - Set to trigger a wake up event. Bit will be cleared once wake up sequence is finished"]
pub type AUDIO_AD5_WU_R = AUDIO_AD0_WU_R;
#[doc = "Field `Audio_AD5_WU` writer - Set to trigger a wake up event. Bit will be cleared once wake up sequence is finished"]
pub struct AUDIO_AD5_WU_W<'a> {
    w: &'a mut W,
}
impl<'a> AUDIO_AD5_WU_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AUDIO_AD5_WU_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "wake up power domain, HW will clear it once power up sequence is finished."]
    #[inline(always)]
    pub fn wake_up(self) -> &'a mut W {
        self.variant(AUDIO_AD5_WU_A::WAKE_UP)
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
    #[doc = "Bit 0 - Set to trigger a wake up event. Bit will be cleared once wake up sequence is finished"]
    #[inline(always)]
    pub fn audio_ad0_wu(&self) -> AUDIO_AD0_WU_R {
        AUDIO_AD0_WU_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Set to trigger a wake up event. Bit will be cleared once wake up sequence is finished"]
    #[inline(always)]
    pub fn audio_ad1_wu(&self) -> AUDIO_AD1_WU_R {
        AUDIO_AD1_WU_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Set to trigger a wake up event. Bit will be cleared once wake up sequence is finished"]
    #[inline(always)]
    pub fn audio_ad2_wu(&self) -> AUDIO_AD2_WU_R {
        AUDIO_AD2_WU_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Set to trigger a wake up event. Bit will be cleared once wake up sequence is finished"]
    #[inline(always)]
    pub fn audio_ad3_wu(&self) -> AUDIO_AD3_WU_R {
        AUDIO_AD3_WU_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Set to trigger a wake up event. Bit will be cleared once wake up sequence is finished"]
    #[inline(always)]
    pub fn audio_ad4_wu(&self) -> AUDIO_AD4_WU_R {
        AUDIO_AD4_WU_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Set to trigger a wake up event. Bit will be cleared once wake up sequence is finished"]
    #[inline(always)]
    pub fn audio_ad5_wu(&self) -> AUDIO_AD5_WU_R {
        AUDIO_AD5_WU_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set to trigger a wake up event. Bit will be cleared once wake up sequence is finished"]
    #[inline(always)]
    pub fn audio_ad0_wu(&mut self) -> AUDIO_AD0_WU_W {
        AUDIO_AD0_WU_W { w: self }
    }
    #[doc = "Bit 1 - Set to trigger a wake up event. Bit will be cleared once wake up sequence is finished"]
    #[inline(always)]
    pub fn audio_ad1_wu(&mut self) -> AUDIO_AD1_WU_W {
        AUDIO_AD1_WU_W { w: self }
    }
    #[doc = "Bit 2 - Set to trigger a wake up event. Bit will be cleared once wake up sequence is finished"]
    #[inline(always)]
    pub fn audio_ad2_wu(&mut self) -> AUDIO_AD2_WU_W {
        AUDIO_AD2_WU_W { w: self }
    }
    #[doc = "Bit 3 - Set to trigger a wake up event. Bit will be cleared once wake up sequence is finished"]
    #[inline(always)]
    pub fn audio_ad3_wu(&mut self) -> AUDIO_AD3_WU_W {
        AUDIO_AD3_WU_W { w: self }
    }
    #[doc = "Bit 4 - Set to trigger a wake up event. Bit will be cleared once wake up sequence is finished"]
    #[inline(always)]
    pub fn audio_ad4_wu(&mut self) -> AUDIO_AD4_WU_W {
        AUDIO_AD4_WU_W { w: self }
    }
    #[doc = "Bit 5 - Set to trigger a wake up event. Bit will be cleared once wake up sequence is finished"]
    #[inline(always)]
    pub fn audio_ad5_wu(&mut self) -> AUDIO_AD5_WU_W {
        AUDIO_AD5_WU_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Register for triggering wake-up events in Audio power domains. (RWHC)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [audio_sram_sw_wu](index.html) module"]
pub struct AUDIO_SRAM_SW_WU_SPEC;
impl crate::RegisterSpec for AUDIO_SRAM_SW_WU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [audio_sram_sw_wu::R](R) reader structure"]
impl crate::Readable for AUDIO_SRAM_SW_WU_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [audio_sram_sw_wu::W](W) writer structure"]
impl crate::Writable for AUDIO_SRAM_SW_WU_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AUDIO_SRAM_SW_WU to value 0"]
impl crate::Resettable for AUDIO_SRAM_SW_WU_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
