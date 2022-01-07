#[doc = "Register `FB_PWR_MODE_CFG` reader"]
pub struct R(crate::R<FB_PWR_MODE_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FB_PWR_MODE_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FB_PWR_MODE_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FB_PWR_MODE_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FB_PWR_MODE_CFG` writer"]
pub struct W(crate::W<FB_PWR_MODE_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FB_PWR_MODE_CFG_SPEC>;
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
impl From<crate::W<FB_PWR_MODE_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FB_PWR_MODE_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "FW need to configure the registers before kicking off the power down event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FB_POWER_MODE_CFG_A {
    #[doc = "1: Configure power down event for deep sleep mode"]
    DEEP_SLEEP_MODE = 1,
    #[doc = "2: Configure power down event for shut down mode"]
    SHUT_DOWN_MODE = 2,
}
impl From<FB_POWER_MODE_CFG_A> for u8 {
    #[inline(always)]
    fn from(variant: FB_POWER_MODE_CFG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FB_Power_Mode_Cfg` reader - FW need to configure the registers before kicking off the power down event."]
pub struct FB_POWER_MODE_CFG_R(crate::FieldReader<u8, FB_POWER_MODE_CFG_A>);
impl FB_POWER_MODE_CFG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FB_POWER_MODE_CFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FB_POWER_MODE_CFG_A> {
        match self.bits {
            1 => Some(FB_POWER_MODE_CFG_A::DEEP_SLEEP_MODE),
            2 => Some(FB_POWER_MODE_CFG_A::SHUT_DOWN_MODE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DEEP_SLEEP_MODE`"]
    #[inline(always)]
    pub fn is_deep_sleep_mode(&self) -> bool {
        **self == FB_POWER_MODE_CFG_A::DEEP_SLEEP_MODE
    }
    #[doc = "Checks if the value of the field is `SHUT_DOWN_MODE`"]
    #[inline(always)]
    pub fn is_shut_down_mode(&self) -> bool {
        **self == FB_POWER_MODE_CFG_A::SHUT_DOWN_MODE
    }
}
impl core::ops::Deref for FB_POWER_MODE_CFG_R {
    type Target = crate::FieldReader<u8, FB_POWER_MODE_CFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FB_Power_Mode_Cfg` writer - FW need to configure the registers before kicking off the power down event."]
pub struct FB_POWER_MODE_CFG_W<'a> {
    w: &'a mut W,
}
impl<'a> FB_POWER_MODE_CFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FB_POWER_MODE_CFG_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Configure power down event for deep sleep mode"]
    #[inline(always)]
    pub fn deep_sleep_mode(self) -> &'a mut W {
        self.variant(FB_POWER_MODE_CFG_A::DEEP_SLEEP_MODE)
    }
    #[doc = "Configure power down event for shut down mode"]
    #[inline(always)]
    pub fn shut_down_mode(self) -> &'a mut W {
        self.variant(FB_POWER_MODE_CFG_A::SHUT_DOWN_MODE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - FW need to configure the registers before kicking off the power down event."]
    #[inline(always)]
    pub fn fb_power_mode_cfg(&self) -> FB_POWER_MODE_CFG_R {
        FB_POWER_MODE_CFG_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - FW need to configure the registers before kicking off the power down event."]
    #[inline(always)]
    pub fn fb_power_mode_cfg(&mut self) -> FB_POWER_MODE_CFG_W {
        FB_POWER_MODE_CFG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power mode configuration for the FPGA Fabric Power domain\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fb_pwr_mode_cfg](index.html) module"]
pub struct FB_PWR_MODE_CFG_SPEC;
impl crate::RegisterSpec for FB_PWR_MODE_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fb_pwr_mode_cfg::R](R) reader structure"]
impl crate::Readable for FB_PWR_MODE_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fb_pwr_mode_cfg::W](W) writer structure"]
impl crate::Writable for FB_PWR_MODE_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FB_PWR_MODE_CFG to value 0"]
impl crate::Resettable for FB_PWR_MODE_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
