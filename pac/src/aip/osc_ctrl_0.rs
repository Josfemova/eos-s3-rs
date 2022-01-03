#[doc = "Register `OSC_CTRL_0` reader"]
pub struct R(crate::R<OSC_CTRL_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OSC_CTRL_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OSC_CTRL_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OSC_CTRL_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OSC_CTRL_0` writer"]
pub struct W(crate::W<OSC_CTRL_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OSC_CTRL_0_SPEC>;
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
impl From<crate::W<OSC_CTRL_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OSC_CTRL_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "1'b0 : OSC OFF -- 1'b1 : OSC ON , (NO SYNC needed, OSC guarantee there is no Glitch)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN_A {
    #[doc = "0: Disable the oscillator"]
    OSCILLATOR_OFF = 0,
    #[doc = "1: Enable the oscillator"]
    OSCILLATOR_ON = 1,
}
impl From<EN_A> for bool {
    #[inline(always)]
    fn from(variant: EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `en` reader - 1'b0 : OSC OFF -- 1'b1 : OSC ON , (NO SYNC needed, OSC guarantee there is no Glitch)"]
pub struct EN_R(crate::FieldReader<bool, EN_A>);
impl EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN_A {
        match self.bits {
            false => EN_A::OSCILLATOR_OFF,
            true => EN_A::OSCILLATOR_ON,
        }
    }
    #[doc = "Checks if the value of the field is `OSCILLATOR_OFF`"]
    #[inline(always)]
    pub fn is_oscillator_off(&self) -> bool {
        **self == EN_A::OSCILLATOR_OFF
    }
    #[doc = "Checks if the value of the field is `OSCILLATOR_ON`"]
    #[inline(always)]
    pub fn is_oscillator_on(&self) -> bool {
        **self == EN_A::OSCILLATOR_ON
    }
}
impl core::ops::Deref for EN_R {
    type Target = crate::FieldReader<bool, EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `en` writer - 1'b0 : OSC OFF -- 1'b1 : OSC ON , (NO SYNC needed, OSC guarantee there is no Glitch)"]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the oscillator"]
    #[inline(always)]
    pub fn oscillator_off(self) -> &'a mut W {
        self.variant(EN_A::OSCILLATOR_OFF)
    }
    #[doc = "Enable the oscillator"]
    #[inline(always)]
    pub fn oscillator_on(self) -> &'a mut W {
        self.variant(EN_A::OSCILLATOR_ON)
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
#[doc = "Reference clock selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FREF16K_SEL_A {
    #[doc = "0: Reference clock is 32KHz"]
    WITH_32K_OSC = 0,
    #[doc = "1: Reference clock is 16KHz"]
    WITH_16K_OSC = 1,
}
impl From<FREF16K_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: FREF16K_SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `fref16k_sel` reader - Reference clock selection"]
pub struct FREF16K_SEL_R(crate::FieldReader<bool, FREF16K_SEL_A>);
impl FREF16K_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FREF16K_SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FREF16K_SEL_A {
        match self.bits {
            false => FREF16K_SEL_A::WITH_32K_OSC,
            true => FREF16K_SEL_A::WITH_16K_OSC,
        }
    }
    #[doc = "Checks if the value of the field is `WITH_32K_OSC`"]
    #[inline(always)]
    pub fn is_with_32k_osc(&self) -> bool {
        **self == FREF16K_SEL_A::WITH_32K_OSC
    }
    #[doc = "Checks if the value of the field is `WITH_16K_OSC`"]
    #[inline(always)]
    pub fn is_with_16k_osc(&self) -> bool {
        **self == FREF16K_SEL_A::WITH_16K_OSC
    }
}
impl core::ops::Deref for FREF16K_SEL_R {
    type Target = crate::FieldReader<bool, FREF16K_SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `fref16k_sel` writer - Reference clock selection"]
pub struct FREF16K_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FREF16K_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FREF16K_SEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reference clock is 32KHz"]
    #[inline(always)]
    pub fn with_32k_osc(self) -> &'a mut W {
        self.variant(FREF16K_SEL_A::WITH_32K_OSC)
    }
    #[doc = "Reference clock is 16KHz"]
    #[inline(always)]
    pub fn with_16k_osc(self) -> &'a mut W {
        self.variant(FREF16K_SEL_A::WITH_16K_OSC)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - 1'b0 : OSC OFF -- 1'b1 : OSC ON , (NO SYNC needed, OSC guarantee there is no Glitch)"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Reference clock selection"]
    #[inline(always)]
    pub fn fref16k_sel(&self) -> FREF16K_SEL_R {
        FREF16K_SEL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1'b0 : OSC OFF -- 1'b1 : OSC ON , (NO SYNC needed, OSC guarantee there is no Glitch)"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    #[doc = "Bit 1 - Reference clock selection"]
    #[inline(always)]
    pub fn fref16k_sel(&mut self) -> FREF16K_SEL_W {
        FREF16K_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Oscilator control register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osc_ctrl_0](index.html) module"]
pub struct OSC_CTRL_0_SPEC;
impl crate::RegisterSpec for OSC_CTRL_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [osc_ctrl_0::R](R) reader structure"]
impl crate::Readable for OSC_CTRL_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [osc_ctrl_0::W](W) writer structure"]
impl crate::Writable for OSC_CTRL_0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OSC_CTRL_0 to value 0x01"]
impl crate::Resettable for OSC_CTRL_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
