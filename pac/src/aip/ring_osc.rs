#[doc = "Register `RING_OSC` reader"]
pub struct R(crate::R<RING_OSC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RING_OSC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RING_OSC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RING_OSC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RING_OSC` writer"]
pub struct W(crate::W<RING_OSC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RING_OSC_SPEC>;
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
impl From<crate::W<RING_OSC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RING_OSC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "1'b1 : Turn on the RING OSC, Ring OSC will be always on despite the HW control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RING_OSC_EN_A {
    #[doc = "0: Disable the ring oscillator"]
    OSCILLATOR_OFF = 0,
    #[doc = "1: Enable the ring oscillator"]
    OSCILLATOR_ON = 1,
}
impl From<RING_OSC_EN_A> for bool {
    #[inline(always)]
    fn from(variant: RING_OSC_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RING_OSC_EN` reader - 1'b1 : Turn on the RING OSC, Ring OSC will be always on despite the HW control."]
pub struct RING_OSC_EN_R(crate::FieldReader<bool, RING_OSC_EN_A>);
impl RING_OSC_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RING_OSC_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RING_OSC_EN_A {
        match self.bits {
            false => RING_OSC_EN_A::OSCILLATOR_OFF,
            true => RING_OSC_EN_A::OSCILLATOR_ON,
        }
    }
    #[doc = "Checks if the value of the field is `OSCILLATOR_OFF`"]
    #[inline(always)]
    pub fn is_oscillator_off(&self) -> bool {
        **self == RING_OSC_EN_A::OSCILLATOR_OFF
    }
    #[doc = "Checks if the value of the field is `OSCILLATOR_ON`"]
    #[inline(always)]
    pub fn is_oscillator_on(&self) -> bool {
        **self == RING_OSC_EN_A::OSCILLATOR_ON
    }
}
impl core::ops::Deref for RING_OSC_EN_R {
    type Target = crate::FieldReader<bool, RING_OSC_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RING_OSC_EN` writer - 1'b1 : Turn on the RING OSC, Ring OSC will be always on despite the HW control."]
pub struct RING_OSC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RING_OSC_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RING_OSC_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the ring oscillator"]
    #[inline(always)]
    pub fn oscillator_off(self) -> &'a mut W {
        self.variant(RING_OSC_EN_A::OSCILLATOR_OFF)
    }
    #[doc = "Enable the ring oscillator"]
    #[inline(always)]
    pub fn oscillator_on(self) -> &'a mut W {
        self.variant(RING_OSC_EN_A::OSCILLATOR_ON)
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
#[doc = "Field `General_Purpos_SFR` reader - 1'b1 : Turn on the RING OSC, Ring OSC will be always on despite the HW control."]
pub struct GENERAL_PURPOS_SFR_R(crate::FieldReader<u8, u8>);
impl GENERAL_PURPOS_SFR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        GENERAL_PURPOS_SFR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GENERAL_PURPOS_SFR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `General_Purpos_SFR` writer - 1'b1 : Turn on the RING OSC, Ring OSC will be always on despite the HW control."]
pub struct GENERAL_PURPOS_SFR_W<'a> {
    w: &'a mut W,
}
impl<'a> GENERAL_PURPOS_SFR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 1)) | ((value as u32 & 0x7f) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - 1'b1 : Turn on the RING OSC, Ring OSC will be always on despite the HW control."]
    #[inline(always)]
    pub fn ring_osc_en(&self) -> RING_OSC_EN_R {
        RING_OSC_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:7 - 1'b1 : Turn on the RING OSC, Ring OSC will be always on despite the HW control."]
    #[inline(always)]
    pub fn general_purpos_sfr(&self) -> GENERAL_PURPOS_SFR_R {
        GENERAL_PURPOS_SFR_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 1'b1 : Turn on the RING OSC, Ring OSC will be always on despite the HW control."]
    #[inline(always)]
    pub fn ring_osc_en(&mut self) -> RING_OSC_EN_W {
        RING_OSC_EN_W { w: self }
    }
    #[doc = "Bits 1:7 - 1'b1 : Turn on the RING OSC, Ring OSC will be always on despite the HW control."]
    #[inline(always)]
    pub fn general_purpos_sfr(&mut self) -> GENERAL_PURPOS_SFR_W {
        GENERAL_PURPOS_SFR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ring oscilator control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ring_osc](index.html) module"]
pub struct RING_OSC_SPEC;
impl crate::RegisterSpec for RING_OSC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ring_osc::R](R) reader structure"]
impl crate::Readable for RING_OSC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ring_osc::W](W) writer structure"]
impl crate::Writable for RING_OSC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RING_OSC to value 0"]
impl crate::Resettable for RING_OSC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
