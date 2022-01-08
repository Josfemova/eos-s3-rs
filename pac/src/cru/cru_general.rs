#[doc = "Register `CRU_GENERAL` reader"]
pub struct R(crate::R<CRU_GENERAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRU_GENERAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRU_GENERAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRU_GENERAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRU_GENERAL` writer"]
pub struct W(crate::W<CRU_GENERAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRU_GENERAL_SPEC>;
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
impl From<crate::W<CRU_GENERAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRU_GENERAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Controls wether the SPI clock is always on or not\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPICLK_ALWAYS_ON_A {
    #[doc = "0: Internal SPI Clock (C00) will be gated off if SPI CS is de-asserted even SPI Clock on the PAD is still running."]
    ALWAYS_OFF = 0,
    #[doc = "1: Internal SPIC Clock (C00) is running if SPI Clock on the PAD is toggling regardless of SPI CS value."]
    ALWAYS_ON = 1,
}
impl From<SPICLK_ALWAYS_ON_A> for bool {
    #[inline(always)]
    fn from(variant: SPICLK_ALWAYS_ON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPICLK_ALWAYS_ON` reader - Controls wether the SPI clock is always on or not"]
pub struct SPICLK_ALWAYS_ON_R(crate::FieldReader<bool, SPICLK_ALWAYS_ON_A>);
impl SPICLK_ALWAYS_ON_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPICLK_ALWAYS_ON_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPICLK_ALWAYS_ON_A {
        match self.bits {
            false => SPICLK_ALWAYS_ON_A::ALWAYS_OFF,
            true => SPICLK_ALWAYS_ON_A::ALWAYS_ON,
        }
    }
    #[doc = "Checks if the value of the field is `ALWAYS_OFF`"]
    #[inline(always)]
    pub fn is_always_off(&self) -> bool {
        **self == SPICLK_ALWAYS_ON_A::ALWAYS_OFF
    }
    #[doc = "Checks if the value of the field is `ALWAYS_ON`"]
    #[inline(always)]
    pub fn is_always_on(&self) -> bool {
        **self == SPICLK_ALWAYS_ON_A::ALWAYS_ON
    }
}
impl core::ops::Deref for SPICLK_ALWAYS_ON_R {
    type Target = crate::FieldReader<bool, SPICLK_ALWAYS_ON_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPICLK_ALWAYS_ON` writer - Controls wether the SPI clock is always on or not"]
pub struct SPICLK_ALWAYS_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> SPICLK_ALWAYS_ON_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPICLK_ALWAYS_ON_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Internal SPI Clock (C00) will be gated off if SPI CS is de-asserted even SPI Clock on the PAD is still running."]
    #[inline(always)]
    pub fn always_off(self) -> &'a mut W {
        self.variant(SPICLK_ALWAYS_ON_A::ALWAYS_OFF)
    }
    #[doc = "Internal SPIC Clock (C00) is running if SPI Clock on the PAD is toggling regardless of SPI CS value."]
    #[inline(always)]
    pub fn always_on(self) -> &'a mut W {
        self.variant(SPICLK_ALWAYS_ON_A::ALWAYS_ON)
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
#[doc = "Field `General` reader - General purpose register"]
pub struct GENERAL_R(crate::FieldReader<u8, u8>);
impl GENERAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        GENERAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GENERAL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `General` writer - General purpose register"]
pub struct GENERAL_W<'a> {
    w: &'a mut W,
}
impl<'a> GENERAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x7f << 1)) | ((value as u32 & 0x7f) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Controls wether the SPI clock is always on or not"]
    #[inline(always)]
    pub fn spiclk_always_on(&self) -> SPICLK_ALWAYS_ON_R {
        SPICLK_ALWAYS_ON_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:7 - General purpose register"]
    #[inline(always)]
    pub fn general(&self) -> GENERAL_R {
        GENERAL_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Controls wether the SPI clock is always on or not"]
    #[inline(always)]
    pub fn spiclk_always_on(&mut self) -> SPICLK_ALWAYS_ON_W {
        SPICLK_ALWAYS_ON_W { w: self }
    }
    #[doc = "Bits 1:7 - General purpose register"]
    #[inline(always)]
    pub fn general(&mut self) -> GENERAL_W {
        GENERAL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General reg and SPI ALWAYS ON control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cru_general](index.html) module"]
pub struct CRU_GENERAL_SPEC;
impl crate::RegisterSpec for CRU_GENERAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cru_general::R](R) reader structure"]
impl crate::Readable for CRU_GENERAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cru_general::W](W) writer structure"]
impl crate::Writable for CRU_GENERAL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CRU_GENERAL to value 0"]
impl crate::Resettable for CRU_GENERAL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
