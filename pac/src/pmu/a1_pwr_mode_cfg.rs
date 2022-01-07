#[doc = "Register `A1_PWR_MODE_CFG` reader"]
pub struct R(crate::R<A1_PWR_MODE_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<A1_PWR_MODE_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<A1_PWR_MODE_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<A1_PWR_MODE_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `A1_PWR_MODE_CFG` writer"]
pub struct W(crate::W<A1_PWR_MODE_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<A1_PWR_MODE_CFG_SPEC>;
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
impl From<crate::W<A1_PWR_MODE_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<A1_PWR_MODE_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "FW need to configure the registers before kicking off the power down event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum A1_POWER_MODE_CFG_A {
    #[doc = "1: Retention Mode"]
    RETENTION_MODE = 1,
    #[doc = "2: Shut Down Mode"]
    SHUT_DOWN_MODE = 2,
}
impl From<A1_POWER_MODE_CFG_A> for u8 {
    #[inline(always)]
    fn from(variant: A1_POWER_MODE_CFG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `A1_Power_Mode_Cfg` reader - FW need to configure the registers before kicking off the power down event"]
pub struct A1_POWER_MODE_CFG_R(crate::FieldReader<u8, A1_POWER_MODE_CFG_A>);
impl A1_POWER_MODE_CFG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        A1_POWER_MODE_CFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<A1_POWER_MODE_CFG_A> {
        match self.bits {
            1 => Some(A1_POWER_MODE_CFG_A::RETENTION_MODE),
            2 => Some(A1_POWER_MODE_CFG_A::SHUT_DOWN_MODE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RETENTION_MODE`"]
    #[inline(always)]
    pub fn is_retention_mode(&self) -> bool {
        **self == A1_POWER_MODE_CFG_A::RETENTION_MODE
    }
    #[doc = "Checks if the value of the field is `SHUT_DOWN_MODE`"]
    #[inline(always)]
    pub fn is_shut_down_mode(&self) -> bool {
        **self == A1_POWER_MODE_CFG_A::SHUT_DOWN_MODE
    }
}
impl core::ops::Deref for A1_POWER_MODE_CFG_R {
    type Target = crate::FieldReader<u8, A1_POWER_MODE_CFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `A1_Power_Mode_Cfg` writer - FW need to configure the registers before kicking off the power down event"]
pub struct A1_POWER_MODE_CFG_W<'a> {
    w: &'a mut W,
}
impl<'a> A1_POWER_MODE_CFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: A1_POWER_MODE_CFG_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Retention Mode"]
    #[inline(always)]
    pub fn retention_mode(self) -> &'a mut W {
        self.variant(A1_POWER_MODE_CFG_A::RETENTION_MODE)
    }
    #[doc = "Shut Down Mode"]
    #[inline(always)]
    pub fn shut_down_mode(self) -> &'a mut W {
        self.variant(A1_POWER_MODE_CFG_A::SHUT_DOWN_MODE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - FW need to configure the registers before kicking off the power down event"]
    #[inline(always)]
    pub fn a1_power_mode_cfg(&self) -> A1_POWER_MODE_CFG_R {
        A1_POWER_MODE_CFG_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - FW need to configure the registers before kicking off the power down event"]
    #[inline(always)]
    pub fn a1_power_mode_cfg(&mut self) -> A1_POWER_MODE_CFG_W {
        A1_POWER_MODE_CFG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power mode configuration for the A1 power domain\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [a1_pwr_mode_cfg](index.html) module"]
pub struct A1_PWR_MODE_CFG_SPEC;
impl crate::RegisterSpec for A1_PWR_MODE_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [a1_pwr_mode_cfg::R](R) reader structure"]
impl crate::Readable for A1_PWR_MODE_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [a1_pwr_mode_cfg::W](W) writer structure"]
impl crate::Writable for A1_PWR_MODE_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets A1_PWR_MODE_CFG to value 0"]
impl crate::Resettable for A1_PWR_MODE_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
