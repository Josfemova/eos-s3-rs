#[doc = "Register `FFE_MEM_CFG` reader"]
pub struct R(crate::R<FFE_MEM_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FFE_MEM_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FFE_MEM_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FFE_MEM_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FFE_MEM_CFG` writer"]
pub struct W(crate::W<FFE_MEM_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FFE_MEM_CFG_SPEC>;
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
impl From<crate::W<FFE_MEM_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FFE_MEM_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Control LS pin of CM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFG_FFE_SRAM_LS_0_A {
    #[doc = "0: Disable, never put SRAM into Light Sleep mode."]
    DISABLE_LIGHT_SLEEP = 0,
    #[doc = "1: Enable, Allow the Hardware control Light Speed pin of SRAM macro directly."]
    ENABLE_LIGHT_SLEEP = 1,
}
impl From<CFG_FFE_SRAM_LS_0_A> for bool {
    #[inline(always)]
    fn from(variant: CFG_FFE_SRAM_LS_0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CFG_FFE_SRAM_LS_0` reader - Control LS pin of CM"]
pub struct CFG_FFE_SRAM_LS_0_R(crate::FieldReader<bool, CFG_FFE_SRAM_LS_0_A>);
impl CFG_FFE_SRAM_LS_0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CFG_FFE_SRAM_LS_0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFG_FFE_SRAM_LS_0_A {
        match self.bits {
            false => CFG_FFE_SRAM_LS_0_A::DISABLE_LIGHT_SLEEP,
            true => CFG_FFE_SRAM_LS_0_A::ENABLE_LIGHT_SLEEP,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE_LIGHT_SLEEP`"]
    #[inline(always)]
    pub fn is_disable_light_sleep(&self) -> bool {
        **self == CFG_FFE_SRAM_LS_0_A::DISABLE_LIGHT_SLEEP
    }
    #[doc = "Checks if the value of the field is `ENABLE_LIGHT_SLEEP`"]
    #[inline(always)]
    pub fn is_enable_light_sleep(&self) -> bool {
        **self == CFG_FFE_SRAM_LS_0_A::ENABLE_LIGHT_SLEEP
    }
}
impl core::ops::Deref for CFG_FFE_SRAM_LS_0_R {
    type Target = crate::FieldReader<bool, CFG_FFE_SRAM_LS_0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFG_FFE_SRAM_LS_0` writer - Control LS pin of CM"]
pub struct CFG_FFE_SRAM_LS_0_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG_FFE_SRAM_LS_0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFG_FFE_SRAM_LS_0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable, never put SRAM into Light Sleep mode."]
    #[inline(always)]
    pub fn disable_light_sleep(self) -> &'a mut W {
        self.variant(CFG_FFE_SRAM_LS_0_A::DISABLE_LIGHT_SLEEP)
    }
    #[doc = "Enable, Allow the Hardware control Light Speed pin of SRAM macro directly."]
    #[inline(always)]
    pub fn enable_light_sleep(self) -> &'a mut W {
        self.variant(CFG_FFE_SRAM_LS_0_A::ENABLE_LIGHT_SLEEP)
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
#[doc = "Control LS pin of DM"]
pub type CFG_FFE_SRAM_LS_1_A = CFG_FFE_SRAM_LS_0_A;
#[doc = "Field `CFG_FFE_SRAM_LS_1` reader - Control LS pin of DM"]
pub type CFG_FFE_SRAM_LS_1_R = CFG_FFE_SRAM_LS_0_R;
#[doc = "Field `CFG_FFE_SRAM_LS_1` writer - Control LS pin of DM"]
pub struct CFG_FFE_SRAM_LS_1_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG_FFE_SRAM_LS_1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFG_FFE_SRAM_LS_1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable, never put SRAM into Light Sleep mode."]
    #[inline(always)]
    pub fn disable_light_sleep(self) -> &'a mut W {
        self.variant(CFG_FFE_SRAM_LS_1_A::DISABLE_LIGHT_SLEEP)
    }
    #[doc = "Enable, Allow the Hardware control Light Speed pin of SRAM macro directly."]
    #[inline(always)]
    pub fn enable_light_sleep(self) -> &'a mut W {
        self.variant(CFG_FFE_SRAM_LS_1_A::ENABLE_LIGHT_SLEEP)
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
#[doc = "Control LS pin of SM0"]
pub type CFG_FFE_SRAM_LS_2_A = CFG_FFE_SRAM_LS_0_A;
#[doc = "Field `CFG_FFE_SRAM_LS_2` reader - Control LS pin of SM0"]
pub type CFG_FFE_SRAM_LS_2_R = CFG_FFE_SRAM_LS_0_R;
#[doc = "Field `CFG_FFE_SRAM_LS_2` writer - Control LS pin of SM0"]
pub struct CFG_FFE_SRAM_LS_2_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG_FFE_SRAM_LS_2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFG_FFE_SRAM_LS_2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable, never put SRAM into Light Sleep mode."]
    #[inline(always)]
    pub fn disable_light_sleep(self) -> &'a mut W {
        self.variant(CFG_FFE_SRAM_LS_2_A::DISABLE_LIGHT_SLEEP)
    }
    #[doc = "Enable, Allow the Hardware control Light Speed pin of SRAM macro directly."]
    #[inline(always)]
    pub fn enable_light_sleep(self) -> &'a mut W {
        self.variant(CFG_FFE_SRAM_LS_2_A::ENABLE_LIGHT_SLEEP)
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
#[doc = "Control LS pin of SM1"]
pub type CFG_FFE_SRAM_LS_3_A = CFG_FFE_SRAM_LS_0_A;
#[doc = "Field `CFG_FFE_SRAM_LS_3` reader - Control LS pin of SM1"]
pub type CFG_FFE_SRAM_LS_3_R = CFG_FFE_SRAM_LS_0_R;
#[doc = "Field `CFG_FFE_SRAM_LS_3` writer - Control LS pin of SM1"]
pub struct CFG_FFE_SRAM_LS_3_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG_FFE_SRAM_LS_3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFG_FFE_SRAM_LS_3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable, never put SRAM into Light Sleep mode."]
    #[inline(always)]
    pub fn disable_light_sleep(self) -> &'a mut W {
        self.variant(CFG_FFE_SRAM_LS_3_A::DISABLE_LIGHT_SLEEP)
    }
    #[doc = "Enable, Allow the Hardware control Light Speed pin of SRAM macro directly."]
    #[inline(always)]
    pub fn enable_light_sleep(self) -> &'a mut W {
        self.variant(CFG_FFE_SRAM_LS_3_A::ENABLE_LIGHT_SLEEP)
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
#[doc = "Field `General_Purpose_SFR` reader - Reseverd."]
pub struct GENERAL_PURPOSE_SFR_R(crate::FieldReader<u8, u8>);
impl GENERAL_PURPOSE_SFR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        GENERAL_PURPOSE_SFR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GENERAL_PURPOSE_SFR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `General_Purpose_SFR` writer - Reseverd."]
pub struct GENERAL_PURPOSE_SFR_W<'a> {
    w: &'a mut W,
}
impl<'a> GENERAL_PURPOSE_SFR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Control LS pin of CM"]
    #[inline(always)]
    pub fn cfg_ffe_sram_ls_0(&self) -> CFG_FFE_SRAM_LS_0_R {
        CFG_FFE_SRAM_LS_0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Control LS pin of DM"]
    #[inline(always)]
    pub fn cfg_ffe_sram_ls_1(&self) -> CFG_FFE_SRAM_LS_1_R {
        CFG_FFE_SRAM_LS_1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Control LS pin of SM0"]
    #[inline(always)]
    pub fn cfg_ffe_sram_ls_2(&self) -> CFG_FFE_SRAM_LS_2_R {
        CFG_FFE_SRAM_LS_2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Control LS pin of SM1"]
    #[inline(always)]
    pub fn cfg_ffe_sram_ls_3(&self) -> CFG_FFE_SRAM_LS_3_R {
        CFG_FFE_SRAM_LS_3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:7 - Reseverd."]
    #[inline(always)]
    pub fn general_purpose_sfr(&self) -> GENERAL_PURPOSE_SFR_R {
        GENERAL_PURPOSE_SFR_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Control LS pin of CM"]
    #[inline(always)]
    pub fn cfg_ffe_sram_ls_0(&mut self) -> CFG_FFE_SRAM_LS_0_W {
        CFG_FFE_SRAM_LS_0_W { w: self }
    }
    #[doc = "Bit 1 - Control LS pin of DM"]
    #[inline(always)]
    pub fn cfg_ffe_sram_ls_1(&mut self) -> CFG_FFE_SRAM_LS_1_W {
        CFG_FFE_SRAM_LS_1_W { w: self }
    }
    #[doc = "Bit 2 - Control LS pin of SM0"]
    #[inline(always)]
    pub fn cfg_ffe_sram_ls_2(&mut self) -> CFG_FFE_SRAM_LS_2_W {
        CFG_FFE_SRAM_LS_2_W { w: self }
    }
    #[doc = "Bit 3 - Control LS pin of SM1"]
    #[inline(always)]
    pub fn cfg_ffe_sram_ls_3(&mut self) -> CFG_FFE_SRAM_LS_3_W {
        CFG_FFE_SRAM_LS_3_W { w: self }
    }
    #[doc = "Bits 4:7 - Reseverd."]
    #[inline(always)]
    pub fn general_purpose_sfr(&mut self) -> GENERAL_PURPOSE_SFR_W {
        GENERAL_PURPOSE_SFR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Light Sleep pin of different FFE SRAM power domains\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ffe_mem_cfg](index.html) module"]
pub struct FFE_MEM_CFG_SPEC;
impl crate::RegisterSpec for FFE_MEM_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ffe_mem_cfg::R](R) reader structure"]
impl crate::Readable for FFE_MEM_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ffe_mem_cfg::W](W) writer structure"]
impl crate::Writable for FFE_MEM_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FFE_MEM_CFG to value 0"]
impl crate::Resettable for FFE_MEM_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
