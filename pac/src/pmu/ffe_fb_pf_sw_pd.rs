#[doc = "Register `FFE_FB_PF_SW_PD` reader"]
pub struct R(crate::R<FFE_FB_PF_SW_PD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FFE_FB_PF_SW_PD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FFE_FB_PF_SW_PD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FFE_FB_PF_SW_PD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FFE_FB_PF_SW_PD` writer"]
pub struct W(crate::W<FFE_FB_PF_SW_PD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FFE_FB_PF_SW_PD_SPEC>;
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
impl From<crate::W<FFE_FB_PF_SW_PD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FFE_FB_PF_SW_PD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Set to trigger a power down event. Bit is cleared after power down sequence finishes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FFE_SOFTWARE_PD_A {
    #[doc = "1: Put the power domain to power saving mode according to configuration setting, HW will clear it once power down sequence is finished."]
    POWER_DOWN = 1,
}
impl From<FFE_SOFTWARE_PD_A> for bool {
    #[inline(always)]
    fn from(variant: FFE_SOFTWARE_PD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FFE_Software_PD` reader - Set to trigger a power down event. Bit is cleared after power down sequence finishes"]
pub struct FFE_SOFTWARE_PD_R(crate::FieldReader<bool, FFE_SOFTWARE_PD_A>);
impl FFE_SOFTWARE_PD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FFE_SOFTWARE_PD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FFE_SOFTWARE_PD_A> {
        match self.bits {
            true => Some(FFE_SOFTWARE_PD_A::POWER_DOWN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `POWER_DOWN`"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        **self == FFE_SOFTWARE_PD_A::POWER_DOWN
    }
}
impl core::ops::Deref for FFE_SOFTWARE_PD_R {
    type Target = crate::FieldReader<bool, FFE_SOFTWARE_PD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FFE_Software_PD` writer - Set to trigger a power down event. Bit is cleared after power down sequence finishes"]
pub struct FFE_SOFTWARE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> FFE_SOFTWARE_PD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FFE_SOFTWARE_PD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Put the power domain to power saving mode according to configuration setting, HW will clear it once power down sequence is finished."]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut W {
        self.variant(FFE_SOFTWARE_PD_A::POWER_DOWN)
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
#[doc = "Set to trigger a power down event. Bit is cleared after power down sequence finishes"]
pub type FB_SOFTWARE_PD_A = FFE_SOFTWARE_PD_A;
#[doc = "Field `FB_Software_PD` reader - Set to trigger a power down event. Bit is cleared after power down sequence finishes"]
pub type FB_SOFTWARE_PD_R = FFE_SOFTWARE_PD_R;
#[doc = "Field `FB_Software_PD` writer - Set to trigger a power down event. Bit is cleared after power down sequence finishes"]
pub struct FB_SOFTWARE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> FB_SOFTWARE_PD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FB_SOFTWARE_PD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Put the power domain to power saving mode according to configuration setting, HW will clear it once power down sequence is finished."]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut W {
        self.variant(FB_SOFTWARE_PD_A::POWER_DOWN)
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
#[doc = "Set to trigger a power down event. Bit is cleared after power down sequence finishes"]
pub type PF_SOFTWARE_PD_A = FFE_SOFTWARE_PD_A;
#[doc = "Field `PF_Software_PD` reader - Set to trigger a power down event. Bit is cleared after power down sequence finishes"]
pub type PF_SOFTWARE_PD_R = FFE_SOFTWARE_PD_R;
#[doc = "Field `PF_Software_PD` writer - Set to trigger a power down event. Bit is cleared after power down sequence finishes"]
pub struct PF_SOFTWARE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> PF_SOFTWARE_PD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PF_SOFTWARE_PD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Put the power domain to power saving mode according to configuration setting, HW will clear it once power down sequence is finished."]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut W {
        self.variant(PF_SOFTWARE_PD_A::POWER_DOWN)
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
impl R {
    #[doc = "Bit 0 - Set to trigger a power down event. Bit is cleared after power down sequence finishes"]
    #[inline(always)]
    pub fn ffe_software_pd(&self) -> FFE_SOFTWARE_PD_R {
        FFE_SOFTWARE_PD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Set to trigger a power down event. Bit is cleared after power down sequence finishes"]
    #[inline(always)]
    pub fn fb_software_pd(&self) -> FB_SOFTWARE_PD_R {
        FB_SOFTWARE_PD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Set to trigger a power down event. Bit is cleared after power down sequence finishes"]
    #[inline(always)]
    pub fn pf_software_pd(&self) -> PF_SOFTWARE_PD_R {
        PF_SOFTWARE_PD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set to trigger a power down event. Bit is cleared after power down sequence finishes"]
    #[inline(always)]
    pub fn ffe_software_pd(&mut self) -> FFE_SOFTWARE_PD_W {
        FFE_SOFTWARE_PD_W { w: self }
    }
    #[doc = "Bit 1 - Set to trigger a power down event. Bit is cleared after power down sequence finishes"]
    #[inline(always)]
    pub fn fb_software_pd(&mut self) -> FB_SOFTWARE_PD_W {
        FB_SOFTWARE_PD_W { w: self }
    }
    #[doc = "Bit 2 - Set to trigger a power down event. Bit is cleared after power down sequence finishes"]
    #[inline(always)]
    pub fn pf_software_pd(&mut self) -> PF_SOFTWARE_PD_W {
        PF_SOFTWARE_PD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Registers for triggering power-down events in the FFE, FB and PF power domains.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ffe_fb_pf_sw_pd](index.html) module"]
pub struct FFE_FB_PF_SW_PD_SPEC;
impl crate::RegisterSpec for FFE_FB_PF_SW_PD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ffe_fb_pf_sw_pd::R](R) reader structure"]
impl crate::Readable for FFE_FB_PF_SW_PD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ffe_fb_pf_sw_pd::W](W) writer structure"]
impl crate::Writable for FFE_FB_PF_SW_PD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FFE_FB_PF_SW_PD to value 0"]
impl crate::Resettable for FFE_FB_PF_SW_PD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
