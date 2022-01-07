#[doc = "Register `MISC_POR_1` reader"]
pub struct R(crate::R<MISC_POR_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MISC_POR_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MISC_POR_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MISC_POR_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MISC_POR_1` writer"]
pub struct W(crate::W<MISC_POR_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MISC_POR_1_SPEC>;
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
impl From<crate::W<MISC_POR_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MISC_POR_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "The FW need to program this bit if FW put the SPI flash to Power Down Mode. The CfgSM will wake up the Flash before reloading the boot code if this bit is 1. FW need to ensure this bit is clear after boot code reloading.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFG_FPD_ON_A {
    #[doc = "1: The SPI is in Power Down Mode. If set, flash will be woken up"]
    WAKEUP_FLASH = 1,
}
impl From<CFG_FPD_ON_A> for bool {
    #[inline(always)]
    fn from(variant: CFG_FPD_ON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Cfg_fpd_on` reader - The FW need to program this bit if FW put the SPI flash to Power Down Mode. The CfgSM will wake up the Flash before reloading the boot code if this bit is 1. FW need to ensure this bit is clear after boot code reloading."]
pub struct CFG_FPD_ON_R(crate::FieldReader<bool, CFG_FPD_ON_A>);
impl CFG_FPD_ON_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CFG_FPD_ON_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CFG_FPD_ON_A> {
        match self.bits {
            true => Some(CFG_FPD_ON_A::WAKEUP_FLASH),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `WAKEUP_FLASH`"]
    #[inline(always)]
    pub fn is_wakeup_flash(&self) -> bool {
        **self == CFG_FPD_ON_A::WAKEUP_FLASH
    }
}
impl core::ops::Deref for CFG_FPD_ON_R {
    type Target = crate::FieldReader<bool, CFG_FPD_ON_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Cfg_fpd_on` writer - The FW need to program this bit if FW put the SPI flash to Power Down Mode. The CfgSM will wake up the Flash before reloading the boot code if this bit is 1. FW need to ensure this bit is clear after boot code reloading."]
pub struct CFG_FPD_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG_FPD_ON_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFG_FPD_ON_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The SPI is in Power Down Mode. If set, flash will be woken up"]
    #[inline(always)]
    pub fn wakeup_flash(self) -> &'a mut W {
        self.variant(CFG_FPD_ON_A::WAKEUP_FLASH)
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
#[doc = "RWHC: Interrupt pin. This bit will be clear if System Reset Trigger when this bit is one\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYS_RST_AS_INT_A {
    #[doc = "0: Treat SYSTEM Reset as Chip Reset"]
    SYS_RST_AS_CHIP_RST = 0,
    #[doc = "1: Reconfigure the SYSTEM Reset pin as the external interrupt pin"]
    SYS_RST_PIN_AS_EXTI = 1,
}
impl From<SYS_RST_AS_INT_A> for bool {
    #[inline(always)]
    fn from(variant: SYS_RST_AS_INT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Sys_rst_as_int` reader - RWHC: Interrupt pin. This bit will be clear if System Reset Trigger when this bit is one"]
pub struct SYS_RST_AS_INT_R(crate::FieldReader<bool, SYS_RST_AS_INT_A>);
impl SYS_RST_AS_INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SYS_RST_AS_INT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYS_RST_AS_INT_A {
        match self.bits {
            false => SYS_RST_AS_INT_A::SYS_RST_AS_CHIP_RST,
            true => SYS_RST_AS_INT_A::SYS_RST_PIN_AS_EXTI,
        }
    }
    #[doc = "Checks if the value of the field is `SYS_RST_AS_CHIP_RST`"]
    #[inline(always)]
    pub fn is_sys_rst_as_chip_rst(&self) -> bool {
        **self == SYS_RST_AS_INT_A::SYS_RST_AS_CHIP_RST
    }
    #[doc = "Checks if the value of the field is `SYS_RST_PIN_AS_EXTI`"]
    #[inline(always)]
    pub fn is_sys_rst_pin_as_exti(&self) -> bool {
        **self == SYS_RST_AS_INT_A::SYS_RST_PIN_AS_EXTI
    }
}
impl core::ops::Deref for SYS_RST_AS_INT_R {
    type Target = crate::FieldReader<bool, SYS_RST_AS_INT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Sys_rst_as_int` writer - RWHC: Interrupt pin. This bit will be clear if System Reset Trigger when this bit is one"]
pub struct SYS_RST_AS_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> SYS_RST_AS_INT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYS_RST_AS_INT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Treat SYSTEM Reset as Chip Reset"]
    #[inline(always)]
    pub fn sys_rst_as_chip_rst(self) -> &'a mut W {
        self.variant(SYS_RST_AS_INT_A::SYS_RST_AS_CHIP_RST)
    }
    #[doc = "Reconfigure the SYSTEM Reset pin as the external interrupt pin"]
    #[inline(always)]
    pub fn sys_rst_pin_as_exti(self) -> &'a mut W {
        self.variant(SYS_RST_AS_INT_A::SYS_RST_PIN_AS_EXTI)
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
#[doc = "If Sys_rst_as_int bit is programmed, then this system Reset INT will trigger if the reset pin is low and the pulse width greater than the configuration value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYS_RST_AS_INT_PW_A {
    #[doc = "0: PWM must be 1S ~ 2S for Reset INT"]
    PULSE_1S_2S = 0,
    #[doc = "1: PWM must be 2S ~ 3S for Reset INT"]
    PULSE_2S_3S = 1,
    #[doc = "2: PWM must be 3S ~ 4S for Reset INT"]
    PULSE_3S_4S = 2,
    #[doc = "3: PWM must be 4S ~ 5S for Reset INT"]
    PULSE_4S_5S = 3,
}
impl From<SYS_RST_AS_INT_PW_A> for u8 {
    #[inline(always)]
    fn from(variant: SYS_RST_AS_INT_PW_A) -> Self {
        variant as _
    }
}
#[doc = "Field `Sys_rst_as_int_pw` reader - If Sys_rst_as_int bit is programmed, then this system Reset INT will trigger if the reset pin is low and the pulse width greater than the configuration value."]
pub struct SYS_RST_AS_INT_PW_R(crate::FieldReader<u8, SYS_RST_AS_INT_PW_A>);
impl SYS_RST_AS_INT_PW_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SYS_RST_AS_INT_PW_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYS_RST_AS_INT_PW_A {
        match self.bits {
            0 => SYS_RST_AS_INT_PW_A::PULSE_1S_2S,
            1 => SYS_RST_AS_INT_PW_A::PULSE_2S_3S,
            2 => SYS_RST_AS_INT_PW_A::PULSE_3S_4S,
            3 => SYS_RST_AS_INT_PW_A::PULSE_4S_5S,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULSE_1S_2S`"]
    #[inline(always)]
    pub fn is_pulse_1s_2s(&self) -> bool {
        **self == SYS_RST_AS_INT_PW_A::PULSE_1S_2S
    }
    #[doc = "Checks if the value of the field is `PULSE_2S_3S`"]
    #[inline(always)]
    pub fn is_pulse_2s_3s(&self) -> bool {
        **self == SYS_RST_AS_INT_PW_A::PULSE_2S_3S
    }
    #[doc = "Checks if the value of the field is `PULSE_3S_4S`"]
    #[inline(always)]
    pub fn is_pulse_3s_4s(&self) -> bool {
        **self == SYS_RST_AS_INT_PW_A::PULSE_3S_4S
    }
    #[doc = "Checks if the value of the field is `PULSE_4S_5S`"]
    #[inline(always)]
    pub fn is_pulse_4s_5s(&self) -> bool {
        **self == SYS_RST_AS_INT_PW_A::PULSE_4S_5S
    }
}
impl core::ops::Deref for SYS_RST_AS_INT_PW_R {
    type Target = crate::FieldReader<u8, SYS_RST_AS_INT_PW_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Sys_rst_as_int_pw` writer - If Sys_rst_as_int bit is programmed, then this system Reset INT will trigger if the reset pin is low and the pulse width greater than the configuration value."]
pub struct SYS_RST_AS_INT_PW_W<'a> {
    w: &'a mut W,
}
impl<'a> SYS_RST_AS_INT_PW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYS_RST_AS_INT_PW_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "PWM must be 1S ~ 2S for Reset INT"]
    #[inline(always)]
    pub fn pulse_1s_2s(self) -> &'a mut W {
        self.variant(SYS_RST_AS_INT_PW_A::PULSE_1S_2S)
    }
    #[doc = "PWM must be 2S ~ 3S for Reset INT"]
    #[inline(always)]
    pub fn pulse_2s_3s(self) -> &'a mut W {
        self.variant(SYS_RST_AS_INT_PW_A::PULSE_2S_3S)
    }
    #[doc = "PWM must be 3S ~ 4S for Reset INT"]
    #[inline(always)]
    pub fn pulse_3s_4s(self) -> &'a mut W {
        self.variant(SYS_RST_AS_INT_PW_A::PULSE_3S_4S)
    }
    #[doc = "PWM must be 4S ~ 5S for Reset INT"]
    #[inline(always)]
    pub fn pulse_4s_5s(self) -> &'a mut W {
        self.variant(SYS_RST_AS_INT_PW_A::PULSE_4S_5S)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "If reset pin is still ketp low after the system reset INT is triggered, then the Chip will be reset if its pulse width is greater than configuration value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYS_RST_PULSE_EXT_A {
    #[doc = "0: 4S after INT trigger"]
    AFTER_4S = 0,
    #[doc = "1: 8S after INT trigger"]
    AFTER_8S = 1,
}
impl From<SYS_RST_PULSE_EXT_A> for bool {
    #[inline(always)]
    fn from(variant: SYS_RST_PULSE_EXT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Sys_rst_pulse_ext` reader - If reset pin is still ketp low after the system reset INT is triggered, then the Chip will be reset if its pulse width is greater than configuration value"]
pub struct SYS_RST_PULSE_EXT_R(crate::FieldReader<bool, SYS_RST_PULSE_EXT_A>);
impl SYS_RST_PULSE_EXT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SYS_RST_PULSE_EXT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYS_RST_PULSE_EXT_A {
        match self.bits {
            false => SYS_RST_PULSE_EXT_A::AFTER_4S,
            true => SYS_RST_PULSE_EXT_A::AFTER_8S,
        }
    }
    #[doc = "Checks if the value of the field is `AFTER_4S`"]
    #[inline(always)]
    pub fn is_after_4s(&self) -> bool {
        **self == SYS_RST_PULSE_EXT_A::AFTER_4S
    }
    #[doc = "Checks if the value of the field is `AFTER_8S`"]
    #[inline(always)]
    pub fn is_after_8s(&self) -> bool {
        **self == SYS_RST_PULSE_EXT_A::AFTER_8S
    }
}
impl core::ops::Deref for SYS_RST_PULSE_EXT_R {
    type Target = crate::FieldReader<bool, SYS_RST_PULSE_EXT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Sys_rst_pulse_ext` writer - If reset pin is still ketp low after the system reset INT is triggered, then the Chip will be reset if its pulse width is greater than configuration value"]
pub struct SYS_RST_PULSE_EXT_W<'a> {
    w: &'a mut W,
}
impl<'a> SYS_RST_PULSE_EXT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYS_RST_PULSE_EXT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "4S after INT trigger"]
    #[inline(always)]
    pub fn after_4s(self) -> &'a mut W {
        self.variant(SYS_RST_PULSE_EXT_A::AFTER_4S)
    }
    #[doc = "8S after INT trigger"]
    #[inline(always)]
    pub fn after_8s(self) -> &'a mut W {
        self.variant(SYS_RST_PULSE_EXT_A::AFTER_8S)
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
impl R {
    #[doc = "Bit 0 - The FW need to program this bit if FW put the SPI flash to Power Down Mode. The CfgSM will wake up the Flash before reloading the boot code if this bit is 1. FW need to ensure this bit is clear after boot code reloading."]
    #[inline(always)]
    pub fn cfg_fpd_on(&self) -> CFG_FPD_ON_R {
        CFG_FPD_ON_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - RWHC: Interrupt pin. This bit will be clear if System Reset Trigger when this bit is one"]
    #[inline(always)]
    pub fn sys_rst_as_int(&self) -> SYS_RST_AS_INT_R {
        SYS_RST_AS_INT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - If Sys_rst_as_int bit is programmed, then this system Reset INT will trigger if the reset pin is low and the pulse width greater than the configuration value."]
    #[inline(always)]
    pub fn sys_rst_as_int_pw(&self) -> SYS_RST_AS_INT_PW_R {
        SYS_RST_AS_INT_PW_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 4 - If reset pin is still ketp low after the system reset INT is triggered, then the Chip will be reset if its pulse width is greater than configuration value"]
    #[inline(always)]
    pub fn sys_rst_pulse_ext(&self) -> SYS_RST_PULSE_EXT_R {
        SYS_RST_PULSE_EXT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The FW need to program this bit if FW put the SPI flash to Power Down Mode. The CfgSM will wake up the Flash before reloading the boot code if this bit is 1. FW need to ensure this bit is clear after boot code reloading."]
    #[inline(always)]
    pub fn cfg_fpd_on(&mut self) -> CFG_FPD_ON_W {
        CFG_FPD_ON_W { w: self }
    }
    #[doc = "Bit 1 - RWHC: Interrupt pin. This bit will be clear if System Reset Trigger when this bit is one"]
    #[inline(always)]
    pub fn sys_rst_as_int(&mut self) -> SYS_RST_AS_INT_W {
        SYS_RST_AS_INT_W { w: self }
    }
    #[doc = "Bits 2:3 - If Sys_rst_as_int bit is programmed, then this system Reset INT will trigger if the reset pin is low and the pulse width greater than the configuration value."]
    #[inline(always)]
    pub fn sys_rst_as_int_pw(&mut self) -> SYS_RST_AS_INT_PW_W {
        SYS_RST_AS_INT_PW_W { w: self }
    }
    #[doc = "Bit 4 - If reset pin is still ketp low after the system reset INT is triggered, then the Chip will be reset if its pulse width is greater than configuration value"]
    #[inline(always)]
    pub fn sys_rst_pulse_ext(&mut self) -> SYS_RST_PULSE_EXT_W {
        SYS_RST_PULSE_EXT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "On POR Reset Domain\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [misc_por_1](index.html) module"]
pub struct MISC_POR_1_SPEC;
impl crate::RegisterSpec for MISC_POR_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [misc_por_1::R](R) reader structure"]
impl crate::Readable for MISC_POR_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [misc_por_1::W](W) writer structure"]
impl crate::Writable for MISC_POR_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MISC_POR_1 to value 0"]
impl crate::Resettable for MISC_POR_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
