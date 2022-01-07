#[doc = "Register `MISC_POR_2` reader"]
pub struct R(crate::R<MISC_POR_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MISC_POR_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MISC_POR_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MISC_POR_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MISC_POR_2` writer"]
pub struct W(crate::W<MISC_POR_2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MISC_POR_2_SPEC>;
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
impl From<crate::W<MISC_POR_2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MISC_POR_2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "If 1'b0, Need cfgSM to reload the code when M4 waking up from SD (Mode1). FW needs to clear this bit if needs code reloading from SPI flash once waking up from SD mdoe (MODE1). CfgSM will be kick off if code reloading from SPI flash needed. This is for code reloading ONLY.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI_REBOOT_ENABLE_N_A {
    #[doc = "0: Need cfgSM to reload the code when M4 waking up from SD (Mode1)"]
    SPI_FLASH_CODE_RELOAD = 0,
}
impl From<SPI_REBOOT_ENABLE_N_A> for bool {
    #[inline(always)]
    fn from(variant: SPI_REBOOT_ENABLE_N_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPI_Reboot_enable_N` reader - If 1'b0, Need cfgSM to reload the code when M4 waking up from SD (Mode1). FW needs to clear this bit if needs code reloading from SPI flash once waking up from SD mdoe (MODE1). CfgSM will be kick off if code reloading from SPI flash needed. This is for code reloading ONLY."]
pub struct SPI_REBOOT_ENABLE_N_R(
    crate::FieldReader<bool, SPI_REBOOT_ENABLE_N_A>,
);
impl SPI_REBOOT_ENABLE_N_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPI_REBOOT_ENABLE_N_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SPI_REBOOT_ENABLE_N_A> {
        match self.bits {
            false => Some(SPI_REBOOT_ENABLE_N_A::SPI_FLASH_CODE_RELOAD),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SPI_FLASH_CODE_RELOAD`"]
    #[inline(always)]
    pub fn is_spi_flash_code_reload(&self) -> bool {
        **self == SPI_REBOOT_ENABLE_N_A::SPI_FLASH_CODE_RELOAD
    }
}
impl core::ops::Deref for SPI_REBOOT_ENABLE_N_R {
    type Target = crate::FieldReader<bool, SPI_REBOOT_ENABLE_N_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI_Reboot_enable_N` writer - If 1'b0, Need cfgSM to reload the code when M4 waking up from SD (Mode1). FW needs to clear this bit if needs code reloading from SPI flash once waking up from SD mdoe (MODE1). CfgSM will be kick off if code reloading from SPI flash needed. This is for code reloading ONLY."]
pub struct SPI_REBOOT_ENABLE_N_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_REBOOT_ENABLE_N_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI_REBOOT_ENABLE_N_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Need cfgSM to reload the code when M4 waking up from SD (Mode1)"]
    #[inline(always)]
    pub fn spi_flash_code_reload(self) -> &'a mut W {
        self.variant(SPI_REBOOT_ENABLE_N_A::SPI_FLASH_CODE_RELOAD)
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
#[doc = "If 1'b0, Need cfgSM to reload the code when M4 waking up from SD (Mode1) FW needs to clear this bit if needs code reloading from SPI flash once waking up from SD mdoe (MODE1). CfgSM will be kick off if code reloading from SPI flash needed. This is for code reloading ONLY.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AP_REBOOT_ENABLE_N_A {
    #[doc = "0: Need AP to reload the code when M4 waking up from SD (Mode1)"]
    AP_CODE_RELOAD = 0,
}
impl From<AP_REBOOT_ENABLE_N_A> for bool {
    #[inline(always)]
    fn from(variant: AP_REBOOT_ENABLE_N_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AP_Reboot_enable_N` reader - If 1'b0, Need cfgSM to reload the code when M4 waking up from SD (Mode1) FW needs to clear this bit if needs code reloading from SPI flash once waking up from SD mdoe (MODE1). CfgSM will be kick off if code reloading from SPI flash needed. This is for code reloading ONLY."]
pub struct AP_REBOOT_ENABLE_N_R(crate::FieldReader<bool, AP_REBOOT_ENABLE_N_A>);
impl AP_REBOOT_ENABLE_N_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AP_REBOOT_ENABLE_N_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<AP_REBOOT_ENABLE_N_A> {
        match self.bits {
            false => Some(AP_REBOOT_ENABLE_N_A::AP_CODE_RELOAD),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `AP_CODE_RELOAD`"]
    #[inline(always)]
    pub fn is_ap_code_reload(&self) -> bool {
        **self == AP_REBOOT_ENABLE_N_A::AP_CODE_RELOAD
    }
}
impl core::ops::Deref for AP_REBOOT_ENABLE_N_R {
    type Target = crate::FieldReader<bool, AP_REBOOT_ENABLE_N_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AP_Reboot_enable_N` writer - If 1'b0, Need cfgSM to reload the code when M4 waking up from SD (Mode1) FW needs to clear this bit if needs code reloading from SPI flash once waking up from SD mdoe (MODE1). CfgSM will be kick off if code reloading from SPI flash needed. This is for code reloading ONLY."]
pub struct AP_REBOOT_ENABLE_N_W<'a> {
    w: &'a mut W,
}
impl<'a> AP_REBOOT_ENABLE_N_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AP_REBOOT_ENABLE_N_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Need AP to reload the code when M4 waking up from SD (Mode1)"]
    #[inline(always)]
    pub fn ap_code_reload(self) -> &'a mut W {
        self.variant(AP_REBOOT_ENABLE_N_A::AP_CODE_RELOAD)
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
impl R {
    #[doc = "Bit 0 - If 1'b0, Need cfgSM to reload the code when M4 waking up from SD (Mode1). FW needs to clear this bit if needs code reloading from SPI flash once waking up from SD mdoe (MODE1). CfgSM will be kick off if code reloading from SPI flash needed. This is for code reloading ONLY."]
    #[inline(always)]
    pub fn spi_reboot_enable_n(&self) -> SPI_REBOOT_ENABLE_N_R {
        SPI_REBOOT_ENABLE_N_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - If 1'b0, Need cfgSM to reload the code when M4 waking up from SD (Mode1) FW needs to clear this bit if needs code reloading from SPI flash once waking up from SD mdoe (MODE1). CfgSM will be kick off if code reloading from SPI flash needed. This is for code reloading ONLY."]
    #[inline(always)]
    pub fn ap_reboot_enable_n(&self) -> AP_REBOOT_ENABLE_N_R {
        AP_REBOOT_ENABLE_N_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - If 1'b0, Need cfgSM to reload the code when M4 waking up from SD (Mode1). FW needs to clear this bit if needs code reloading from SPI flash once waking up from SD mdoe (MODE1). CfgSM will be kick off if code reloading from SPI flash needed. This is for code reloading ONLY."]
    #[inline(always)]
    pub fn spi_reboot_enable_n(&mut self) -> SPI_REBOOT_ENABLE_N_W {
        SPI_REBOOT_ENABLE_N_W { w: self }
    }
    #[doc = "Bit 1 - If 1'b0, Need cfgSM to reload the code when M4 waking up from SD (Mode1) FW needs to clear this bit if needs code reloading from SPI flash once waking up from SD mdoe (MODE1). CfgSM will be kick off if code reloading from SPI flash needed. This is for code reloading ONLY."]
    #[inline(always)]
    pub fn ap_reboot_enable_n(&mut self) -> AP_REBOOT_ENABLE_N_W {
        AP_REBOOT_ENABLE_N_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "On POR Reset Domain\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [misc_por_2](index.html) module"]
pub struct MISC_POR_2_SPEC;
impl crate::RegisterSpec for MISC_POR_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [misc_por_2::R](R) reader structure"]
impl crate::Readable for MISC_POR_2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [misc_por_2::W](W) writer structure"]
impl crate::Writable for MISC_POR_2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MISC_POR_2 to value 0x03"]
impl crate::Resettable for MISC_POR_2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x03
    }
}
