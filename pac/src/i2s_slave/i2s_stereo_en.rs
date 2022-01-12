#[doc = "Register `I2S_STEREO_EN` reader"]
pub struct R(crate::R<I2S_STEREO_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2S_STEREO_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2S_STEREO_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2S_STEREO_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2S_STEREO_EN` writer"]
pub struct W(crate::W<I2S_STEREO_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2S_STEREO_EN_SPEC>;
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
impl From<crate::W<I2S_STEREO_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2S_STEREO_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Select between mono or stereo modes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2S_STEREO_EN_A {
    #[doc = "0: Configure I2S for monoaural sound reproduction"]
    MONO = 0,
    #[doc = "1: Configure I2S for stereo sound reproduction"]
    STEREO = 1,
}
impl From<I2S_STEREO_EN_A> for bool {
    #[inline(always)]
    fn from(variant: I2S_STEREO_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2S_STEREO_EN` reader - Select between mono or stereo modes"]
pub struct I2S_STEREO_EN_R(crate::FieldReader<bool, I2S_STEREO_EN_A>);
impl I2S_STEREO_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2S_STEREO_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2S_STEREO_EN_A {
        match self.bits {
            false => I2S_STEREO_EN_A::MONO,
            true => I2S_STEREO_EN_A::STEREO,
        }
    }
    #[doc = "Checks if the value of the field is `MONO`"]
    #[inline(always)]
    pub fn is_mono(&self) -> bool {
        **self == I2S_STEREO_EN_A::MONO
    }
    #[doc = "Checks if the value of the field is `STEREO`"]
    #[inline(always)]
    pub fn is_stereo(&self) -> bool {
        **self == I2S_STEREO_EN_A::STEREO
    }
}
impl core::ops::Deref for I2S_STEREO_EN_R {
    type Target = crate::FieldReader<bool, I2S_STEREO_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2S_STEREO_EN` writer - Select between mono or stereo modes"]
pub struct I2S_STEREO_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_STEREO_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2S_STEREO_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Configure I2S for monoaural sound reproduction"]
    #[inline(always)]
    pub fn mono(self) -> &'a mut W {
        self.variant(I2S_STEREO_EN_A::MONO)
    }
    #[doc = "Configure I2S for stereo sound reproduction"]
    #[inline(always)]
    pub fn stereo(self) -> &'a mut W {
        self.variant(I2S_STEREO_EN_A::STEREO)
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
    #[doc = "Bit 0 - Select between mono or stereo modes"]
    #[inline(always)]
    pub fn i2s_stereo_en(&self) -> I2S_STEREO_EN_R {
        I2S_STEREO_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Select between mono or stereo modes"]
    #[inline(always)]
    pub fn i2s_stereo_en(&mut self) -> I2S_STEREO_EN_W {
        I2S_STEREO_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sound channel mode (mono or stereo)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2s_stereo_en](index.html) module"]
pub struct I2S_STEREO_EN_SPEC;
impl crate::RegisterSpec for I2S_STEREO_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2s_stereo_en::R](R) reader structure"]
impl crate::Readable for I2S_STEREO_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2s_stereo_en::W](W) writer structure"]
impl crate::Writable for I2S_STEREO_EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2S_STEREO_EN to value 0"]
impl crate::Resettable for I2S_STEREO_EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
