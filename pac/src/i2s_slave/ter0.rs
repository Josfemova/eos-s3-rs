#[doc = "Register `TER0` reader"]
pub struct R(crate::R<TER0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TER0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TER0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TER0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TER0` writer"]
pub struct W(crate::W<TER0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TER0_SPEC>;
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
impl From<crate::W<TER0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TER0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Transmit channel enable. This bit enables/disables a transmit channel, independently of all other channels. On enable, the channel begins transmitting on the next left stereo cycle. A global disable of DW_apb_i2s (IER\\[0\\]
= 0) or Transmitter block (ITER\\[0\\]
=0) overrides this value.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXCHEN0_A {
    #[doc = "0: Disable the transmit channel."]
    DISABLE = 0,
    #[doc = "1: Enable the transmit channel."]
    ENABLE = 1,
}
impl From<TXCHEN0_A> for bool {
    #[inline(always)]
    fn from(variant: TXCHEN0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXCHEN0` reader - Transmit channel enable. This bit enables/disables a transmit channel, independently of all other channels. On enable, the channel begins transmitting on the next left stereo cycle. A global disable of DW_apb_i2s (IER\\[0\\]
= 0) or Transmitter block (ITER\\[0\\]
=0) overrides this value."]
pub struct TXCHEN0_R(crate::FieldReader<bool, TXCHEN0_A>);
impl TXCHEN0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXCHEN0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXCHEN0_A {
        match self.bits {
            false => TXCHEN0_A::DISABLE,
            true => TXCHEN0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == TXCHEN0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == TXCHEN0_A::ENABLE
    }
}
impl core::ops::Deref for TXCHEN0_R {
    type Target = crate::FieldReader<bool, TXCHEN0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXCHEN0` writer - Transmit channel enable. This bit enables/disables a transmit channel, independently of all other channels. On enable, the channel begins transmitting on the next left stereo cycle. A global disable of DW_apb_i2s (IER\\[0\\]
= 0) or Transmitter block (ITER\\[0\\]
=0) overrides this value."]
pub struct TXCHEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> TXCHEN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXCHEN0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the transmit channel."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TXCHEN0_A::DISABLE)
    }
    #[doc = "Enable the transmit channel."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TXCHEN0_A::ENABLE)
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
impl R {
    #[doc = "Bit 0 - Transmit channel enable. This bit enables/disables a transmit channel, independently of all other channels. On enable, the channel begins transmitting on the next left stereo cycle. A global disable of DW_apb_i2s (IER\\[0\\]
= 0) or Transmitter block (ITER\\[0\\]
=0) overrides this value."]
    #[inline(always)]
    pub fn txchen0(&self) -> TXCHEN0_R {
        TXCHEN0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit channel enable. This bit enables/disables a transmit channel, independently of all other channels. On enable, the channel begins transmitting on the next left stereo cycle. A global disable of DW_apb_i2s (IER\\[0\\]
= 0) or Transmitter block (ITER\\[0\\]
=0) overrides this value."]
    #[inline(always)]
    pub fn txchen0(&mut self) -> TXCHEN0_W {
        TXCHEN0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ter0](index.html) module"]
pub struct TER0_SPEC;
impl crate::RegisterSpec for TER0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ter0::R](R) reader structure"]
impl crate::Readable for TER0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ter0::W](W) writer structure"]
impl crate::Writable for TER0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TER0 to value 0x01"]
impl crate::Resettable for TER0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
