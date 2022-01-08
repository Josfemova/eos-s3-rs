#[doc = "Register `FFE_FB_PF_SW_WU` reader"]
pub struct R(crate::R<FFE_FB_PF_SW_WU_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FFE_FB_PF_SW_WU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FFE_FB_PF_SW_WU_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FFE_FB_PF_SW_WU_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FFE_FB_PF_SW_WU` writer"]
pub struct W(crate::W<FFE_FB_PF_SW_WU_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FFE_FB_PF_SW_WU_SPEC>;
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
impl From<crate::W<FFE_FB_PF_SW_WU_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FFE_FB_PF_SW_WU_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Set to trigger a wake up event. Bit is cleared after wake up sequence finishes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FFE_SOFTWARE_WU_A {
    #[doc = "1: wake up power domain, HW will clear it once power up sequence is finished."]
    WAKE_UP = 1,
}
impl From<FFE_SOFTWARE_WU_A> for bool {
    #[inline(always)]
    fn from(variant: FFE_SOFTWARE_WU_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FFE_Software_WU` reader - Set to trigger a wake up event. Bit is cleared after wake up sequence finishes"]
pub struct FFE_SOFTWARE_WU_R(crate::FieldReader<bool, FFE_SOFTWARE_WU_A>);
impl FFE_SOFTWARE_WU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FFE_SOFTWARE_WU_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FFE_SOFTWARE_WU_A> {
        match self.bits {
            true => Some(FFE_SOFTWARE_WU_A::WAKE_UP),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `WAKE_UP`"]
    #[inline(always)]
    pub fn is_wake_up(&self) -> bool {
        **self == FFE_SOFTWARE_WU_A::WAKE_UP
    }
}
impl core::ops::Deref for FFE_SOFTWARE_WU_R {
    type Target = crate::FieldReader<bool, FFE_SOFTWARE_WU_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FFE_Software_WU` writer - Set to trigger a wake up event. Bit is cleared after wake up sequence finishes"]
pub struct FFE_SOFTWARE_WU_W<'a> {
    w: &'a mut W,
}
impl<'a> FFE_SOFTWARE_WU_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FFE_SOFTWARE_WU_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "wake up power domain, HW will clear it once power up sequence is finished."]
    #[inline(always)]
    pub fn wake_up(self) -> &'a mut W {
        self.variant(FFE_SOFTWARE_WU_A::WAKE_UP)
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
#[doc = "Set to trigger a wake up event. Bit is cleared after wake up sequence finishes"]
pub type FB_SOFTWARE_WU_A = FFE_SOFTWARE_WU_A;
#[doc = "Field `FB_Software_WU` reader - Set to trigger a wake up event. Bit is cleared after wake up sequence finishes"]
pub type FB_SOFTWARE_WU_R = FFE_SOFTWARE_WU_R;
#[doc = "Field `FB_Software_WU` writer - Set to trigger a wake up event. Bit is cleared after wake up sequence finishes"]
pub struct FB_SOFTWARE_WU_W<'a> {
    w: &'a mut W,
}
impl<'a> FB_SOFTWARE_WU_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FB_SOFTWARE_WU_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "wake up power domain, HW will clear it once power up sequence is finished."]
    #[inline(always)]
    pub fn wake_up(self) -> &'a mut W {
        self.variant(FB_SOFTWARE_WU_A::WAKE_UP)
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
#[doc = "Set to trigger a wake up event. Bit is cleared after wake up sequence finishes"]
pub type PF_SOFTWARE_WU_A = FFE_SOFTWARE_WU_A;
#[doc = "Field `PF_Software_WU` reader - Set to trigger a wake up event. Bit is cleared after wake up sequence finishes"]
pub type PF_SOFTWARE_WU_R = FFE_SOFTWARE_WU_R;
#[doc = "Field `PF_Software_WU` writer - Set to trigger a wake up event. Bit is cleared after wake up sequence finishes"]
pub struct PF_SOFTWARE_WU_W<'a> {
    w: &'a mut W,
}
impl<'a> PF_SOFTWARE_WU_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PF_SOFTWARE_WU_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "wake up power domain, HW will clear it once power up sequence is finished."]
    #[inline(always)]
    pub fn wake_up(self) -> &'a mut W {
        self.variant(PF_SOFTWARE_WU_A::WAKE_UP)
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
    #[doc = "Bit 0 - Set to trigger a wake up event. Bit is cleared after wake up sequence finishes"]
    #[inline(always)]
    pub fn ffe_software_wu(&self) -> FFE_SOFTWARE_WU_R {
        FFE_SOFTWARE_WU_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Set to trigger a wake up event. Bit is cleared after wake up sequence finishes"]
    #[inline(always)]
    pub fn fb_software_wu(&self) -> FB_SOFTWARE_WU_R {
        FB_SOFTWARE_WU_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Set to trigger a wake up event. Bit is cleared after wake up sequence finishes"]
    #[inline(always)]
    pub fn pf_software_wu(&self) -> PF_SOFTWARE_WU_R {
        PF_SOFTWARE_WU_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set to trigger a wake up event. Bit is cleared after wake up sequence finishes"]
    #[inline(always)]
    pub fn ffe_software_wu(&mut self) -> FFE_SOFTWARE_WU_W {
        FFE_SOFTWARE_WU_W { w: self }
    }
    #[doc = "Bit 1 - Set to trigger a wake up event. Bit is cleared after wake up sequence finishes"]
    #[inline(always)]
    pub fn fb_software_wu(&mut self) -> FB_SOFTWARE_WU_W {
        FB_SOFTWARE_WU_W { w: self }
    }
    #[doc = "Bit 2 - Set to trigger a wake up event. Bit is cleared after wake up sequence finishes"]
    #[inline(always)]
    pub fn pf_software_wu(&mut self) -> PF_SOFTWARE_WU_W {
        PF_SOFTWARE_WU_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Registers for triggering wake-up events in the FFE, FB and PF power domains.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ffe_fb_pf_sw_wu](index.html) module"]
pub struct FFE_FB_PF_SW_WU_SPEC;
impl crate::RegisterSpec for FFE_FB_PF_SW_WU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ffe_fb_pf_sw_wu::R](R) reader structure"]
impl crate::Readable for FFE_FB_PF_SW_WU_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ffe_fb_pf_sw_wu::W](W) writer structure"]
impl crate::Writable for FFE_FB_PF_SW_WU_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FFE_FB_PF_SW_WU to value 0"]
impl crate::Resettable for FFE_FB_PF_SW_WU_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
