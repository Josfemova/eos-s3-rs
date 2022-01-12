#[doc = "Register `ITER` reader"]
pub struct R(crate::R<ITER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ITER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ITER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ITER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ITER` writer"]
pub struct W(crate::W<ITER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ITER_SPEC>;
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
impl From<crate::W<ITER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ITER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Transmitter block enable. A disable on this bit overrides any individual transmit channel enables.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXEN_A {
    #[doc = "0: Disable the transmitter."]
    DISABLE = 0,
    #[doc = "1: Enable the transmitter."]
    ENABLE = 1,
}
impl From<TXEN_A> for bool {
    #[inline(always)]
    fn from(variant: TXEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXEN` reader - Transmitter block enable. A disable on this bit overrides any individual transmit channel enables."]
pub struct TXEN_R(crate::FieldReader<bool, TXEN_A>);
impl TXEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXEN_A {
        match self.bits {
            false => TXEN_A::DISABLE,
            true => TXEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == TXEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == TXEN_A::ENABLE
    }
}
impl core::ops::Deref for TXEN_R {
    type Target = crate::FieldReader<bool, TXEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXEN` writer - Transmitter block enable. A disable on this bit overrides any individual transmit channel enables."]
pub struct TXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the transmitter."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TXEN_A::DISABLE)
    }
    #[doc = "Enable the transmitter."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TXEN_A::ENABLE)
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
    #[doc = "Bit 0 - Transmitter block enable. A disable on this bit overrides any individual transmit channel enables."]
    #[inline(always)]
    pub fn txen(&self) -> TXEN_R {
        TXEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmitter block enable. A disable on this bit overrides any individual transmit channel enables."]
    #[inline(always)]
    pub fn txen(&mut self) -> TXEN_W {
        TXEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2S Transmitter Block Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iter](index.html) module"]
pub struct ITER_SPEC;
impl crate::RegisterSpec for ITER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iter::R](R) reader structure"]
impl crate::Readable for ITER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iter::W](W) writer structure"]
impl crate::Writable for ITER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ITER to value 0"]
impl crate::Resettable for ITER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
