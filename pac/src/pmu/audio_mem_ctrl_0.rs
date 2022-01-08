#[doc = "Register `AUDIO_MEM_CTRL_0` reader"]
pub struct R(crate::R<AUDIO_MEM_CTRL_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AUDIO_MEM_CTRL_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AUDIO_MEM_CTRL_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AUDIO_MEM_CTRL_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AUDIO_MEM_CTRL_0` writer"]
pub struct W(crate::W<AUDIO_MEM_CTRL_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AUDIO_MEM_CTRL_0_SPEC>;
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
impl From<crate::W<AUDIO_MEM_CTRL_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AUDIO_MEM_CTRL_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Used to control DS pin of the Left Channel R0 2Kx32\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUDIO_SRAM_LC_DS_0_A {
    #[doc = "0: Disable the Deep Sleep function of SRAM Macro, Memory content will be kept."]
    DISABLE_DEEP_SLEEP = 0,
    #[doc = "1: Enable the Deep Sleep function of SRAM Macro, Memory content will be kept."]
    ENABLE_DEEP_SLEEP = 1,
}
impl From<AUDIO_SRAM_LC_DS_0_A> for bool {
    #[inline(always)]
    fn from(variant: AUDIO_SRAM_LC_DS_0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUDIO_SRAM_LC_DS_0` reader - Used to control DS pin of the Left Channel R0 2Kx32"]
pub struct AUDIO_SRAM_LC_DS_0_R(crate::FieldReader<bool, AUDIO_SRAM_LC_DS_0_A>);
impl AUDIO_SRAM_LC_DS_0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AUDIO_SRAM_LC_DS_0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUDIO_SRAM_LC_DS_0_A {
        match self.bits {
            false => AUDIO_SRAM_LC_DS_0_A::DISABLE_DEEP_SLEEP,
            true => AUDIO_SRAM_LC_DS_0_A::ENABLE_DEEP_SLEEP,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE_DEEP_SLEEP`"]
    #[inline(always)]
    pub fn is_disable_deep_sleep(&self) -> bool {
        **self == AUDIO_SRAM_LC_DS_0_A::DISABLE_DEEP_SLEEP
    }
    #[doc = "Checks if the value of the field is `ENABLE_DEEP_SLEEP`"]
    #[inline(always)]
    pub fn is_enable_deep_sleep(&self) -> bool {
        **self == AUDIO_SRAM_LC_DS_0_A::ENABLE_DEEP_SLEEP
    }
}
impl core::ops::Deref for AUDIO_SRAM_LC_DS_0_R {
    type Target = crate::FieldReader<bool, AUDIO_SRAM_LC_DS_0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AUDIO_SRAM_LC_DS_0` writer - Used to control DS pin of the Left Channel R0 2Kx32"]
pub struct AUDIO_SRAM_LC_DS_0_W<'a> {
    w: &'a mut W,
}
impl<'a> AUDIO_SRAM_LC_DS_0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AUDIO_SRAM_LC_DS_0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the Deep Sleep function of SRAM Macro, Memory content will be kept."]
    #[inline(always)]
    pub fn disable_deep_sleep(self) -> &'a mut W {
        self.variant(AUDIO_SRAM_LC_DS_0_A::DISABLE_DEEP_SLEEP)
    }
    #[doc = "Enable the Deep Sleep function of SRAM Macro, Memory content will be kept."]
    #[inline(always)]
    pub fn enable_deep_sleep(self) -> &'a mut W {
        self.variant(AUDIO_SRAM_LC_DS_0_A::ENABLE_DEEP_SLEEP)
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
#[doc = "Used to control DS pin of the Left Channel R1 128Kx32"]
pub type AUDIO_SRAM_LC_DS_1_A = AUDIO_SRAM_LC_DS_0_A;
#[doc = "Field `AUDIO_SRAM_LC_DS_1` reader - Used to control DS pin of the Left Channel R1 128Kx32"]
pub type AUDIO_SRAM_LC_DS_1_R = AUDIO_SRAM_LC_DS_0_R;
#[doc = "Field `AUDIO_SRAM_LC_DS_1` writer - Used to control DS pin of the Left Channel R1 128Kx32"]
pub struct AUDIO_SRAM_LC_DS_1_W<'a> {
    w: &'a mut W,
}
impl<'a> AUDIO_SRAM_LC_DS_1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AUDIO_SRAM_LC_DS_1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the Deep Sleep function of SRAM Macro, Memory content will be kept."]
    #[inline(always)]
    pub fn disable_deep_sleep(self) -> &'a mut W {
        self.variant(AUDIO_SRAM_LC_DS_1_A::DISABLE_DEEP_SLEEP)
    }
    #[doc = "Enable the Deep Sleep function of SRAM Macro, Memory content will be kept."]
    #[inline(always)]
    pub fn enable_deep_sleep(self) -> &'a mut W {
        self.variant(AUDIO_SRAM_LC_DS_1_A::ENABLE_DEEP_SLEEP)
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
#[doc = "Used to control DS pin of the Left Channel R2 256Kx15"]
pub type AUDIO_SRAM_LC_DS_2_A = AUDIO_SRAM_LC_DS_0_A;
#[doc = "Field `AUDIO_SRAM_LC_DS_2` reader - Used to control DS pin of the Left Channel R2 256Kx15"]
pub type AUDIO_SRAM_LC_DS_2_R = AUDIO_SRAM_LC_DS_0_R;
#[doc = "Field `AUDIO_SRAM_LC_DS_2` writer - Used to control DS pin of the Left Channel R2 256Kx15"]
pub struct AUDIO_SRAM_LC_DS_2_W<'a> {
    w: &'a mut W,
}
impl<'a> AUDIO_SRAM_LC_DS_2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AUDIO_SRAM_LC_DS_2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the Deep Sleep function of SRAM Macro, Memory content will be kept."]
    #[inline(always)]
    pub fn disable_deep_sleep(self) -> &'a mut W {
        self.variant(AUDIO_SRAM_LC_DS_2_A::DISABLE_DEEP_SLEEP)
    }
    #[doc = "Enable the Deep Sleep function of SRAM Macro, Memory content will be kept."]
    #[inline(always)]
    pub fn enable_deep_sleep(self) -> &'a mut W {
        self.variant(AUDIO_SRAM_LC_DS_2_A::ENABLE_DEEP_SLEEP)
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
#[doc = "Used to control DS pin of the Right Channel R0 2Kx32"]
pub type AUDIO_SRAM_RC_DS_0_A = AUDIO_SRAM_LC_DS_0_A;
#[doc = "Field `AUDIO_SRAM_RC_DS_0` reader - Used to control DS pin of the Right Channel R0 2Kx32"]
pub type AUDIO_SRAM_RC_DS_0_R = AUDIO_SRAM_LC_DS_0_R;
#[doc = "Field `AUDIO_SRAM_RC_DS_0` writer - Used to control DS pin of the Right Channel R0 2Kx32"]
pub struct AUDIO_SRAM_RC_DS_0_W<'a> {
    w: &'a mut W,
}
impl<'a> AUDIO_SRAM_RC_DS_0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AUDIO_SRAM_RC_DS_0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the Deep Sleep function of SRAM Macro, Memory content will be kept."]
    #[inline(always)]
    pub fn disable_deep_sleep(self) -> &'a mut W {
        self.variant(AUDIO_SRAM_RC_DS_0_A::DISABLE_DEEP_SLEEP)
    }
    #[doc = "Enable the Deep Sleep function of SRAM Macro, Memory content will be kept."]
    #[inline(always)]
    pub fn enable_deep_sleep(self) -> &'a mut W {
        self.variant(AUDIO_SRAM_RC_DS_0_A::ENABLE_DEEP_SLEEP)
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
#[doc = "Used to control DS pin of the Right Channel R1 128Kx32"]
pub type AUDIO_SRAM_RC_DS_1_A = AUDIO_SRAM_LC_DS_0_A;
#[doc = "Field `AUDIO_SRAM_RC_DS_1` reader - Used to control DS pin of the Right Channel R1 128Kx32"]
pub type AUDIO_SRAM_RC_DS_1_R = AUDIO_SRAM_LC_DS_0_R;
#[doc = "Field `AUDIO_SRAM_RC_DS_1` writer - Used to control DS pin of the Right Channel R1 128Kx32"]
pub struct AUDIO_SRAM_RC_DS_1_W<'a> {
    w: &'a mut W,
}
impl<'a> AUDIO_SRAM_RC_DS_1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AUDIO_SRAM_RC_DS_1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the Deep Sleep function of SRAM Macro, Memory content will be kept."]
    #[inline(always)]
    pub fn disable_deep_sleep(self) -> &'a mut W {
        self.variant(AUDIO_SRAM_RC_DS_1_A::DISABLE_DEEP_SLEEP)
    }
    #[doc = "Enable the Deep Sleep function of SRAM Macro, Memory content will be kept."]
    #[inline(always)]
    pub fn enable_deep_sleep(self) -> &'a mut W {
        self.variant(AUDIO_SRAM_RC_DS_1_A::ENABLE_DEEP_SLEEP)
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
#[doc = "Used to control DS pin of the Right Channel R2 256Kx15"]
pub type AUDIO_SRAM_RC_DS_2_A = AUDIO_SRAM_LC_DS_0_A;
#[doc = "Field `AUDIO_SRAM_RC_DS_2` reader - Used to control DS pin of the Right Channel R2 256Kx15"]
pub type AUDIO_SRAM_RC_DS_2_R = AUDIO_SRAM_LC_DS_0_R;
#[doc = "Field `AUDIO_SRAM_RC_DS_2` writer - Used to control DS pin of the Right Channel R2 256Kx15"]
pub struct AUDIO_SRAM_RC_DS_2_W<'a> {
    w: &'a mut W,
}
impl<'a> AUDIO_SRAM_RC_DS_2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AUDIO_SRAM_RC_DS_2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the Deep Sleep function of SRAM Macro, Memory content will be kept."]
    #[inline(always)]
    pub fn disable_deep_sleep(self) -> &'a mut W {
        self.variant(AUDIO_SRAM_RC_DS_2_A::DISABLE_DEEP_SLEEP)
    }
    #[doc = "Enable the Deep Sleep function of SRAM Macro, Memory content will be kept."]
    #[inline(always)]
    pub fn enable_deep_sleep(self) -> &'a mut W {
        self.variant(AUDIO_SRAM_RC_DS_2_A::ENABLE_DEEP_SLEEP)
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
    #[doc = "Bit 0 - Used to control DS pin of the Left Channel R0 2Kx32"]
    #[inline(always)]
    pub fn audio_sram_lc_ds_0(&self) -> AUDIO_SRAM_LC_DS_0_R {
        AUDIO_SRAM_LC_DS_0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Used to control DS pin of the Left Channel R1 128Kx32"]
    #[inline(always)]
    pub fn audio_sram_lc_ds_1(&self) -> AUDIO_SRAM_LC_DS_1_R {
        AUDIO_SRAM_LC_DS_1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Used to control DS pin of the Left Channel R2 256Kx15"]
    #[inline(always)]
    pub fn audio_sram_lc_ds_2(&self) -> AUDIO_SRAM_LC_DS_2_R {
        AUDIO_SRAM_LC_DS_2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Used to control DS pin of the Right Channel R0 2Kx32"]
    #[inline(always)]
    pub fn audio_sram_rc_ds_0(&self) -> AUDIO_SRAM_RC_DS_0_R {
        AUDIO_SRAM_RC_DS_0_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Used to control DS pin of the Right Channel R1 128Kx32"]
    #[inline(always)]
    pub fn audio_sram_rc_ds_1(&self) -> AUDIO_SRAM_RC_DS_1_R {
        AUDIO_SRAM_RC_DS_1_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Used to control DS pin of the Right Channel R2 256Kx15"]
    #[inline(always)]
    pub fn audio_sram_rc_ds_2(&self) -> AUDIO_SRAM_RC_DS_2_R {
        AUDIO_SRAM_RC_DS_2_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Used to control DS pin of the Left Channel R0 2Kx32"]
    #[inline(always)]
    pub fn audio_sram_lc_ds_0(&mut self) -> AUDIO_SRAM_LC_DS_0_W {
        AUDIO_SRAM_LC_DS_0_W { w: self }
    }
    #[doc = "Bit 1 - Used to control DS pin of the Left Channel R1 128Kx32"]
    #[inline(always)]
    pub fn audio_sram_lc_ds_1(&mut self) -> AUDIO_SRAM_LC_DS_1_W {
        AUDIO_SRAM_LC_DS_1_W { w: self }
    }
    #[doc = "Bit 2 - Used to control DS pin of the Left Channel R2 256Kx15"]
    #[inline(always)]
    pub fn audio_sram_lc_ds_2(&mut self) -> AUDIO_SRAM_LC_DS_2_W {
        AUDIO_SRAM_LC_DS_2_W { w: self }
    }
    #[doc = "Bit 3 - Used to control DS pin of the Right Channel R0 2Kx32"]
    #[inline(always)]
    pub fn audio_sram_rc_ds_0(&mut self) -> AUDIO_SRAM_RC_DS_0_W {
        AUDIO_SRAM_RC_DS_0_W { w: self }
    }
    #[doc = "Bit 4 - Used to control DS pin of the Right Channel R1 128Kx32"]
    #[inline(always)]
    pub fn audio_sram_rc_ds_1(&mut self) -> AUDIO_SRAM_RC_DS_1_W {
        AUDIO_SRAM_RC_DS_1_W { w: self }
    }
    #[doc = "Bit 5 - Used to control DS pin of the Right Channel R2 256Kx15"]
    #[inline(always)]
    pub fn audio_sram_rc_ds_2(&mut self) -> AUDIO_SRAM_RC_DS_2_W {
        AUDIO_SRAM_RC_DS_2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control the Deep Sleep pin of Audio channels. For each: 1'b1 : Enable the Deep Sleep function of SRAM Macro, Memory content will be kept.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [audio_mem_ctrl_0](index.html) module"]
pub struct AUDIO_MEM_CTRL_0_SPEC;
impl crate::RegisterSpec for AUDIO_MEM_CTRL_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [audio_mem_ctrl_0::R](R) reader structure"]
impl crate::Readable for AUDIO_MEM_CTRL_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [audio_mem_ctrl_0::W](W) writer structure"]
impl crate::Writable for AUDIO_MEM_CTRL_0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AUDIO_MEM_CTRL_0 to value 0"]
impl crate::Resettable for AUDIO_MEM_CTRL_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
