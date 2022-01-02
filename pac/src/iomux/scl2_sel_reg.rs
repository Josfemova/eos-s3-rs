#[doc = "Register `SCL2_SEL_REG` reader"]
pub struct R(crate::R<SCL2_SEL_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCL2_SEL_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCL2_SEL_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCL2_SEL_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCL2_SEL_REG` writer"]
pub struct W(crate::W<SCL2_SEL_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCL2_SEL_REG_SPEC>;
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
impl From<crate::W<SCL2_SEL_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCL2_SEL_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Sel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEL_A {
    #[doc = "0: Documentation lacking, guessed meaning: Selects pad #40 for the SCL function in I2C2"]
    DEFAULT_UNDEFINED = 0,
    #[doc = "1: Selects pad #40 for the SCL function in I2C2"]
    PAD_40 = 1,
}
impl From<SEL_A> for bool {
    #[inline(always)]
    fn from(variant: SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEL` reader - Sel"]
pub struct SEL_R(crate::FieldReader<bool, SEL_A>);
impl SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEL_A {
        match self.bits {
            false => SEL_A::DEFAULT_UNDEFINED,
            true => SEL_A::PAD_40,
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT_UNDEFINED`"]
    #[inline(always)]
    pub fn is_default_undefined(&self) -> bool {
        **self == SEL_A::DEFAULT_UNDEFINED
    }
    #[doc = "Checks if the value of the field is `PAD_40`"]
    #[inline(always)]
    pub fn is_pad_40(&self) -> bool {
        **self == SEL_A::PAD_40
    }
}
impl core::ops::Deref for SEL_R {
    type Target = crate::FieldReader<bool, SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEL` writer - Sel"]
pub struct SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Documentation lacking, guessed meaning: Selects pad #40 for the SCL function in I2C2"]
    #[inline(always)]
    pub fn default_undefined(self) -> &'a mut W {
        self.variant(SEL_A::DEFAULT_UNDEFINED)
    }
    #[doc = "Selects pad #40 for the SCL function in I2C2"]
    #[inline(always)]
    pub fn pad_40(self) -> &'a mut W {
        self.variant(SEL_A::PAD_40)
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
    #[doc = "Bit 0 - Sel"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Sel"]
    #[inline(always)]
    pub fn sel(&mut self) -> SEL_W {
        SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Select pad for SCL function in I2C2 (only pad 40 is selectable)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scl2_sel_reg](index.html) module"]
pub struct SCL2_SEL_REG_SPEC;
impl crate::RegisterSpec for SCL2_SEL_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scl2_sel_reg::R](R) reader structure"]
impl crate::Readable for SCL2_SEL_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scl2_sel_reg::W](W) writer structure"]
impl crate::Writable for SCL2_SEL_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCL2_SEL_REG to value 0"]
impl crate::Resettable for SCL2_SEL_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
