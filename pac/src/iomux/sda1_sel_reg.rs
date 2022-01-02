#[doc = "Register `SDA1_SEL_REG` reader"]
pub struct R(crate::R<SDA1_SEL_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDA1_SEL_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDA1_SEL_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDA1_SEL_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDA1_SEL_REG` writer"]
pub struct W(crate::W<SDA1_SEL_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDA1_SEL_REG_SPEC>;
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
impl From<crate::W<SDA1_SEL_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDA1_SEL_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Sel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SEL_A {
    #[doc = "0: Documentation lacking, guessed meaning: Select pad #15 for the SDA function in I2C1"]
    DEFAULT_UNDEFINED = 0,
    #[doc = "1: Select pad #15 for the SDA function in I2C1"]
    PAD_15 = 1,
    #[doc = "2: Select pad #32 for the SDA function in I2C1"]
    PAD_32 = 2,
    #[doc = "3: Select pad #44 for the SDA function in I2C1"]
    PAD_44 = 3,
}
impl From<SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SEL` reader - Sel"]
pub struct SEL_R(crate::FieldReader<u8, SEL_A>);
impl SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEL_A {
        match self.bits {
            0 => SEL_A::DEFAULT_UNDEFINED,
            1 => SEL_A::PAD_15,
            2 => SEL_A::PAD_32,
            3 => SEL_A::PAD_44,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT_UNDEFINED`"]
    #[inline(always)]
    pub fn is_default_undefined(&self) -> bool {
        **self == SEL_A::DEFAULT_UNDEFINED
    }
    #[doc = "Checks if the value of the field is `PAD_15`"]
    #[inline(always)]
    pub fn is_pad_15(&self) -> bool {
        **self == SEL_A::PAD_15
    }
    #[doc = "Checks if the value of the field is `PAD_32`"]
    #[inline(always)]
    pub fn is_pad_32(&self) -> bool {
        **self == SEL_A::PAD_32
    }
    #[doc = "Checks if the value of the field is `PAD_44`"]
    #[inline(always)]
    pub fn is_pad_44(&self) -> bool {
        **self == SEL_A::PAD_44
    }
}
impl core::ops::Deref for SEL_R {
    type Target = crate::FieldReader<u8, SEL_A>;
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
        self.bits(variant.into())
    }
    #[doc = "Documentation lacking, guessed meaning: Select pad #15 for the SDA function in I2C1"]
    #[inline(always)]
    pub fn default_undefined(self) -> &'a mut W {
        self.variant(SEL_A::DEFAULT_UNDEFINED)
    }
    #[doc = "Select pad #15 for the SDA function in I2C1"]
    #[inline(always)]
    pub fn pad_15(self) -> &'a mut W {
        self.variant(SEL_A::PAD_15)
    }
    #[doc = "Select pad #32 for the SDA function in I2C1"]
    #[inline(always)]
    pub fn pad_32(self) -> &'a mut W {
        self.variant(SEL_A::PAD_32)
    }
    #[doc = "Select pad #44 for the SDA function in I2C1"]
    #[inline(always)]
    pub fn pad_44(self) -> &'a mut W {
        self.variant(SEL_A::PAD_44)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Sel"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Sel"]
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
#[doc = "Select pad for SDA function in I2C1 (3 pads available)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sda1_sel_reg](index.html) module"]
pub struct SDA1_SEL_REG_SPEC;
impl crate::RegisterSpec for SDA1_SEL_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sda1_sel_reg::R](R) reader structure"]
impl crate::Readable for SDA1_SEL_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sda1_sel_reg::W](W) writer structure"]
impl crate::Writable for SDA1_SEL_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SDA1_SEL_REG to value 0"]
impl crate::Resettable for SDA1_SEL_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
