#[doc = "Register `FFE_MEM_CTRL_1` reader"]
pub struct R(crate::R<FFE_MEM_CTRL_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FFE_MEM_CTRL_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FFE_MEM_CTRL_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FFE_MEM_CTRL_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FFE_MEM_CTRL_1` writer"]
pub struct W(crate::W<FFE_MEM_CTRL_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FFE_MEM_CTRL_1_SPEC>;
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
impl From<crate::W<FFE_MEM_CTRL_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FFE_MEM_CTRL_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Control SD pin of CM0 8Kx40\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTRL_FFE_SRAM_SD_CM0_A {
    #[doc = "0: Disable the shut down function of SRAM Macro."]
    DISABLE_SHUT_DOWN = 0,
    #[doc = "1: Enable the shut down function of SRAM Macro, Memory content will be lost."]
    ENABLE_SHUT_DOWN = 1,
}
impl From<CTRL_FFE_SRAM_SD_CM0_A> for bool {
    #[inline(always)]
    fn from(variant: CTRL_FFE_SRAM_SD_CM0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTRL_FFE_SRAM_SD_CM0` reader - Control SD pin of CM0 8Kx40"]
pub struct CTRL_FFE_SRAM_SD_CM0_R(
    crate::FieldReader<bool, CTRL_FFE_SRAM_SD_CM0_A>,
);
impl CTRL_FFE_SRAM_SD_CM0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CTRL_FFE_SRAM_SD_CM0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTRL_FFE_SRAM_SD_CM0_A {
        match self.bits {
            false => CTRL_FFE_SRAM_SD_CM0_A::DISABLE_SHUT_DOWN,
            true => CTRL_FFE_SRAM_SD_CM0_A::ENABLE_SHUT_DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE_SHUT_DOWN`"]
    #[inline(always)]
    pub fn is_disable_shut_down(&self) -> bool {
        **self == CTRL_FFE_SRAM_SD_CM0_A::DISABLE_SHUT_DOWN
    }
    #[doc = "Checks if the value of the field is `ENABLE_SHUT_DOWN`"]
    #[inline(always)]
    pub fn is_enable_shut_down(&self) -> bool {
        **self == CTRL_FFE_SRAM_SD_CM0_A::ENABLE_SHUT_DOWN
    }
}
impl core::ops::Deref for CTRL_FFE_SRAM_SD_CM0_R {
    type Target = crate::FieldReader<bool, CTRL_FFE_SRAM_SD_CM0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTRL_FFE_SRAM_SD_CM0` writer - Control SD pin of CM0 8Kx40"]
pub struct CTRL_FFE_SRAM_SD_CM0_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRL_FFE_SRAM_SD_CM0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTRL_FFE_SRAM_SD_CM0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the shut down function of SRAM Macro."]
    #[inline(always)]
    pub fn disable_shut_down(self) -> &'a mut W {
        self.variant(CTRL_FFE_SRAM_SD_CM0_A::DISABLE_SHUT_DOWN)
    }
    #[doc = "Enable the shut down function of SRAM Macro, Memory content will be lost."]
    #[inline(always)]
    pub fn enable_shut_down(self) -> &'a mut W {
        self.variant(CTRL_FFE_SRAM_SD_CM0_A::ENABLE_SHUT_DOWN)
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
#[doc = "Control SD pin of DM0 1Kx32"]
pub type CTRL_FFE_SRAM_SD_DM0_A = CTRL_FFE_SRAM_SD_CM0_A;
#[doc = "Field `CTRL_FFE_SRAM_SD_DM0` reader - Control SD pin of DM0 1Kx32"]
pub type CTRL_FFE_SRAM_SD_DM0_R = CTRL_FFE_SRAM_SD_CM0_R;
#[doc = "Field `CTRL_FFE_SRAM_SD_DM0` writer - Control SD pin of DM0 1Kx32"]
pub struct CTRL_FFE_SRAM_SD_DM0_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRL_FFE_SRAM_SD_DM0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTRL_FFE_SRAM_SD_DM0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the shut down function of SRAM Macro."]
    #[inline(always)]
    pub fn disable_shut_down(self) -> &'a mut W {
        self.variant(CTRL_FFE_SRAM_SD_DM0_A::DISABLE_SHUT_DOWN)
    }
    #[doc = "Enable the shut down function of SRAM Macro, Memory content will be lost."]
    #[inline(always)]
    pub fn enable_shut_down(self) -> &'a mut W {
        self.variant(CTRL_FFE_SRAM_SD_DM0_A::ENABLE_SHUT_DOWN)
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
#[doc = "Control SD pin of DM1 1Kx32"]
pub type CTRL_FFE_SRAM_SD_DM1_A = CTRL_FFE_SRAM_SD_CM0_A;
#[doc = "Field `CTRL_FFE_SRAM_SD_DM1` reader - Control SD pin of DM1 1Kx32"]
pub type CTRL_FFE_SRAM_SD_DM1_R = CTRL_FFE_SRAM_SD_CM0_R;
#[doc = "Field `CTRL_FFE_SRAM_SD_DM1` writer - Control SD pin of DM1 1Kx32"]
pub struct CTRL_FFE_SRAM_SD_DM1_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRL_FFE_SRAM_SD_DM1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTRL_FFE_SRAM_SD_DM1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the shut down function of SRAM Macro."]
    #[inline(always)]
    pub fn disable_shut_down(self) -> &'a mut W {
        self.variant(CTRL_FFE_SRAM_SD_DM1_A::DISABLE_SHUT_DOWN)
    }
    #[doc = "Enable the shut down function of SRAM Macro, Memory content will be lost."]
    #[inline(always)]
    pub fn enable_shut_down(self) -> &'a mut W {
        self.variant(CTRL_FFE_SRAM_SD_DM1_A::ENABLE_SHUT_DOWN)
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
#[doc = "Control SD pin of SM0 1Kx18"]
pub type CTRL_FFE_SRAM_SD_SM0_A = CTRL_FFE_SRAM_SD_CM0_A;
#[doc = "Field `CTRL_FFE_SRAM_SD_SM0` reader - Control SD pin of SM0 1Kx18"]
pub type CTRL_FFE_SRAM_SD_SM0_R = CTRL_FFE_SRAM_SD_CM0_R;
#[doc = "Field `CTRL_FFE_SRAM_SD_SM0` writer - Control SD pin of SM0 1Kx18"]
pub struct CTRL_FFE_SRAM_SD_SM0_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRL_FFE_SRAM_SD_SM0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTRL_FFE_SRAM_SD_SM0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the shut down function of SRAM Macro."]
    #[inline(always)]
    pub fn disable_shut_down(self) -> &'a mut W {
        self.variant(CTRL_FFE_SRAM_SD_SM0_A::DISABLE_SHUT_DOWN)
    }
    #[doc = "Enable the shut down function of SRAM Macro, Memory content will be lost."]
    #[inline(always)]
    pub fn enable_shut_down(self) -> &'a mut W {
        self.variant(CTRL_FFE_SRAM_SD_SM0_A::ENABLE_SHUT_DOWN)
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
#[doc = "Control SD pin of SM1 0.5Kx18"]
pub type CTRL_FFE_SRAM_SD_SM1_A = CTRL_FFE_SRAM_SD_CM0_A;
#[doc = "Field `CTRL_FFE_SRAM_SD_SM1` reader - Control SD pin of SM1 0.5Kx18"]
pub type CTRL_FFE_SRAM_SD_SM1_R = CTRL_FFE_SRAM_SD_CM0_R;
#[doc = "Field `CTRL_FFE_SRAM_SD_SM1` writer - Control SD pin of SM1 0.5Kx18"]
pub struct CTRL_FFE_SRAM_SD_SM1_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRL_FFE_SRAM_SD_SM1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTRL_FFE_SRAM_SD_SM1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the shut down function of SRAM Macro."]
    #[inline(always)]
    pub fn disable_shut_down(self) -> &'a mut W {
        self.variant(CTRL_FFE_SRAM_SD_SM1_A::DISABLE_SHUT_DOWN)
    }
    #[doc = "Enable the shut down function of SRAM Macro, Memory content will be lost."]
    #[inline(always)]
    pub fn enable_shut_down(self) -> &'a mut W {
        self.variant(CTRL_FFE_SRAM_SD_SM1_A::ENABLE_SHUT_DOWN)
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
#[doc = "Control SD pin of CM1 2Kx40"]
pub type CTRL_FFE_SRAM_SD_CM1_A = CTRL_FFE_SRAM_SD_CM0_A;
#[doc = "Field `CTRL_FFE_SRAM_SD_CM1` reader - Control SD pin of CM1 2Kx40"]
pub type CTRL_FFE_SRAM_SD_CM1_R = CTRL_FFE_SRAM_SD_CM0_R;
#[doc = "Field `CTRL_FFE_SRAM_SD_CM1` writer - Control SD pin of CM1 2Kx40"]
pub struct CTRL_FFE_SRAM_SD_CM1_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRL_FFE_SRAM_SD_CM1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTRL_FFE_SRAM_SD_CM1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the shut down function of SRAM Macro."]
    #[inline(always)]
    pub fn disable_shut_down(self) -> &'a mut W {
        self.variant(CTRL_FFE_SRAM_SD_CM1_A::DISABLE_SHUT_DOWN)
    }
    #[doc = "Enable the shut down function of SRAM Macro, Memory content will be lost."]
    #[inline(always)]
    pub fn enable_shut_down(self) -> &'a mut W {
        self.variant(CTRL_FFE_SRAM_SD_CM1_A::ENABLE_SHUT_DOWN)
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
#[doc = "Control SD pin of CM0 1Kx32"]
pub type CTRL_FFE_SRAM_SD_DM2_A = CTRL_FFE_SRAM_SD_CM0_A;
#[doc = "Field `CTRL_FFE_SRAM_SD_DM2` reader - Control SD pin of CM0 1Kx32"]
pub type CTRL_FFE_SRAM_SD_DM2_R = CTRL_FFE_SRAM_SD_CM0_R;
#[doc = "Field `CTRL_FFE_SRAM_SD_DM2` writer - Control SD pin of CM0 1Kx32"]
pub struct CTRL_FFE_SRAM_SD_DM2_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRL_FFE_SRAM_SD_DM2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTRL_FFE_SRAM_SD_DM2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the shut down function of SRAM Macro."]
    #[inline(always)]
    pub fn disable_shut_down(self) -> &'a mut W {
        self.variant(CTRL_FFE_SRAM_SD_DM2_A::DISABLE_SHUT_DOWN)
    }
    #[doc = "Enable the shut down function of SRAM Macro, Memory content will be lost."]
    #[inline(always)]
    pub fn enable_shut_down(self) -> &'a mut W {
        self.variant(CTRL_FFE_SRAM_SD_DM2_A::ENABLE_SHUT_DOWN)
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
#[doc = "Control SD pin of CM0 1Kx32"]
pub type CTRL_FFE_SRAM_SD_DM3_A = CTRL_FFE_SRAM_SD_CM0_A;
#[doc = "Field `CTRL_FFE_SRAM_SD_DM3` reader - Control SD pin of CM0 1Kx32"]
pub type CTRL_FFE_SRAM_SD_DM3_R = CTRL_FFE_SRAM_SD_CM0_R;
#[doc = "Field `CTRL_FFE_SRAM_SD_DM3` writer - Control SD pin of CM0 1Kx32"]
pub struct CTRL_FFE_SRAM_SD_DM3_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRL_FFE_SRAM_SD_DM3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTRL_FFE_SRAM_SD_DM3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the shut down function of SRAM Macro."]
    #[inline(always)]
    pub fn disable_shut_down(self) -> &'a mut W {
        self.variant(CTRL_FFE_SRAM_SD_DM3_A::DISABLE_SHUT_DOWN)
    }
    #[doc = "Enable the shut down function of SRAM Macro, Memory content will be lost."]
    #[inline(always)]
    pub fn enable_shut_down(self) -> &'a mut W {
        self.variant(CTRL_FFE_SRAM_SD_DM3_A::ENABLE_SHUT_DOWN)
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
            (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Control SD pin of CM0 8Kx40"]
    #[inline(always)]
    pub fn ctrl_ffe_sram_sd_cm0(&self) -> CTRL_FFE_SRAM_SD_CM0_R {
        CTRL_FFE_SRAM_SD_CM0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Control SD pin of DM0 1Kx32"]
    #[inline(always)]
    pub fn ctrl_ffe_sram_sd_dm0(&self) -> CTRL_FFE_SRAM_SD_DM0_R {
        CTRL_FFE_SRAM_SD_DM0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Control SD pin of DM1 1Kx32"]
    #[inline(always)]
    pub fn ctrl_ffe_sram_sd_dm1(&self) -> CTRL_FFE_SRAM_SD_DM1_R {
        CTRL_FFE_SRAM_SD_DM1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Control SD pin of SM0 1Kx18"]
    #[inline(always)]
    pub fn ctrl_ffe_sram_sd_sm0(&self) -> CTRL_FFE_SRAM_SD_SM0_R {
        CTRL_FFE_SRAM_SD_SM0_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Control SD pin of SM1 0.5Kx18"]
    #[inline(always)]
    pub fn ctrl_ffe_sram_sd_sm1(&self) -> CTRL_FFE_SRAM_SD_SM1_R {
        CTRL_FFE_SRAM_SD_SM1_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Control SD pin of CM1 2Kx40"]
    #[inline(always)]
    pub fn ctrl_ffe_sram_sd_cm1(&self) -> CTRL_FFE_SRAM_SD_CM1_R {
        CTRL_FFE_SRAM_SD_CM1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Control SD pin of CM0 1Kx32"]
    #[inline(always)]
    pub fn ctrl_ffe_sram_sd_dm2(&self) -> CTRL_FFE_SRAM_SD_DM2_R {
        CTRL_FFE_SRAM_SD_DM2_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Control SD pin of CM0 1Kx32"]
    #[inline(always)]
    pub fn ctrl_ffe_sram_sd_dm3(&self) -> CTRL_FFE_SRAM_SD_DM3_R {
        CTRL_FFE_SRAM_SD_DM3_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Control SD pin of CM0 8Kx40"]
    #[inline(always)]
    pub fn ctrl_ffe_sram_sd_cm0(&mut self) -> CTRL_FFE_SRAM_SD_CM0_W {
        CTRL_FFE_SRAM_SD_CM0_W { w: self }
    }
    #[doc = "Bit 1 - Control SD pin of DM0 1Kx32"]
    #[inline(always)]
    pub fn ctrl_ffe_sram_sd_dm0(&mut self) -> CTRL_FFE_SRAM_SD_DM0_W {
        CTRL_FFE_SRAM_SD_DM0_W { w: self }
    }
    #[doc = "Bit 2 - Control SD pin of DM1 1Kx32"]
    #[inline(always)]
    pub fn ctrl_ffe_sram_sd_dm1(&mut self) -> CTRL_FFE_SRAM_SD_DM1_W {
        CTRL_FFE_SRAM_SD_DM1_W { w: self }
    }
    #[doc = "Bit 3 - Control SD pin of SM0 1Kx18"]
    #[inline(always)]
    pub fn ctrl_ffe_sram_sd_sm0(&mut self) -> CTRL_FFE_SRAM_SD_SM0_W {
        CTRL_FFE_SRAM_SD_SM0_W { w: self }
    }
    #[doc = "Bit 4 - Control SD pin of SM1 0.5Kx18"]
    #[inline(always)]
    pub fn ctrl_ffe_sram_sd_sm1(&mut self) -> CTRL_FFE_SRAM_SD_SM1_W {
        CTRL_FFE_SRAM_SD_SM1_W { w: self }
    }
    #[doc = "Bit 5 - Control SD pin of CM1 2Kx40"]
    #[inline(always)]
    pub fn ctrl_ffe_sram_sd_cm1(&mut self) -> CTRL_FFE_SRAM_SD_CM1_W {
        CTRL_FFE_SRAM_SD_CM1_W { w: self }
    }
    #[doc = "Bit 6 - Control SD pin of CM0 1Kx32"]
    #[inline(always)]
    pub fn ctrl_ffe_sram_sd_dm2(&mut self) -> CTRL_FFE_SRAM_SD_DM2_W {
        CTRL_FFE_SRAM_SD_DM2_W { w: self }
    }
    #[doc = "Bit 7 - Control SD pin of CM0 1Kx32"]
    #[inline(always)]
    pub fn ctrl_ffe_sram_sd_dm3(&mut self) -> CTRL_FFE_SRAM_SD_DM3_W {
        CTRL_FFE_SRAM_SD_DM3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control de Shut Down pin of various elements in the Flexible Fusion Engine power domain. For each: 1'b1 : Enable the Deep Sleep function of SRAM Macro, Memory content will be kept.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ffe_mem_ctrl_1](index.html) module"]
pub struct FFE_MEM_CTRL_1_SPEC;
impl crate::RegisterSpec for FFE_MEM_CTRL_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ffe_mem_ctrl_1::R](R) reader structure"]
impl crate::Readable for FFE_MEM_CTRL_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ffe_mem_ctrl_1::W](W) writer structure"]
impl crate::Writable for FFE_MEM_CTRL_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FFE_MEM_CTRL_1 to value 0"]
impl crate::Resettable for FFE_MEM_CTRL_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
