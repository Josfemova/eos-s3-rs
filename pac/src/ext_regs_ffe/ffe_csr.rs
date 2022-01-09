#[doc = "Register `FFE_CSR` reader"]
pub struct R(crate::R<FFE_CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FFE_CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FFE_CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FFE_CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FFE_CSR` writer"]
pub struct W(crate::W<FFE_CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FFE_CSR_SPEC>;
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
impl From<crate::W<FFE_CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FFE_CSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Select wich i2c is taken as i2c\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C2_SEL_A {
    #[doc = "0: Select i2c1 as i2c2"]
    I2C1 = 0,
    #[doc = "1: Select i2c2 as i2c2"]
    I2C2 = 1,
}
impl From<I2C2_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: I2C2_SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `i2c2_sel` reader - Select wich i2c is taken as i2c"]
pub struct I2C2_SEL_R(crate::FieldReader<bool, I2C2_SEL_A>);
impl I2C2_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2C2_SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C2_SEL_A {
        match self.bits {
            false => I2C2_SEL_A::I2C1,
            true => I2C2_SEL_A::I2C2,
        }
    }
    #[doc = "Checks if the value of the field is `I2C1`"]
    #[inline(always)]
    pub fn is_i2c1(&self) -> bool {
        **self == I2C2_SEL_A::I2C1
    }
    #[doc = "Checks if the value of the field is `I2C2`"]
    #[inline(always)]
    pub fn is_i2c2(&self) -> bool {
        **self == I2C2_SEL_A::I2C2
    }
}
impl core::ops::Deref for I2C2_SEL_R {
    type Target = crate::FieldReader<bool, I2C2_SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `i2c2_sel` writer - Select wich i2c is taken as i2c"]
pub struct I2C2_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C2_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C2_SEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Select i2c1 as i2c2"]
    #[inline(always)]
    pub fn i2c1(self) -> &'a mut W {
        self.variant(I2C2_SEL_A::I2C1)
    }
    #[doc = "Select i2c2 as i2c2"]
    #[inline(always)]
    pub fn i2c2(self) -> &'a mut W {
        self.variant(I2C2_SEL_A::I2C2)
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
#[doc = "Field `i2c0_dyn_pullup_en` reader - i2c0 dynamic pull-up enable"]
pub struct I2C0_DYN_PULLUP_EN_R(crate::FieldReader<bool, bool>);
impl I2C0_DYN_PULLUP_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2C0_DYN_PULLUP_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C0_DYN_PULLUP_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `i2c0_dyn_pullup_en` writer - i2c0 dynamic pull-up enable"]
pub struct I2C0_DYN_PULLUP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C0_DYN_PULLUP_EN_W<'a> {
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
#[doc = "Field `i2c1_dyn_pullup_en` reader - i2c1 dynamic pull-up enable"]
pub struct I2C1_DYN_PULLUP_EN_R(crate::FieldReader<bool, bool>);
impl I2C1_DYN_PULLUP_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2C1_DYN_PULLUP_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C1_DYN_PULLUP_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `i2c1_dyn_pullup_en` writer - i2c1 dynamic pull-up enable"]
pub struct I2C1_DYN_PULLUP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1_DYN_PULLUP_EN_W<'a> {
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
#[doc = "Field `i2c2_dyn_pullup_en` reader - i2c2 dynamic pull-up enable"]
pub struct I2C2_DYN_PULLUP_EN_R(crate::FieldReader<bool, bool>);
impl I2C2_DYN_PULLUP_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2C2_DYN_PULLUP_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C2_DYN_PULLUP_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `i2c2_dyn_pullup_en` writer - i2c2 dynamic pull-up enable"]
pub struct I2C2_DYN_PULLUP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C2_DYN_PULLUP_EN_W<'a> {
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
            (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Select wich i2c is taken as i2c"]
    #[inline(always)]
    pub fn i2c2_sel(&self) -> I2C2_SEL_R {
        I2C2_SEL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - i2c0 dynamic pull-up enable"]
    #[inline(always)]
    pub fn i2c0_dyn_pullup_en(&self) -> I2C0_DYN_PULLUP_EN_R {
        I2C0_DYN_PULLUP_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - i2c1 dynamic pull-up enable"]
    #[inline(always)]
    pub fn i2c1_dyn_pullup_en(&self) -> I2C1_DYN_PULLUP_EN_R {
        I2C1_DYN_PULLUP_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - i2c2 dynamic pull-up enable"]
    #[inline(always)]
    pub fn i2c2_dyn_pullup_en(&self) -> I2C2_DYN_PULLUP_EN_R {
        I2C2_DYN_PULLUP_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Select wich i2c is taken as i2c"]
    #[inline(always)]
    pub fn i2c2_sel(&mut self) -> I2C2_SEL_W {
        I2C2_SEL_W { w: self }
    }
    #[doc = "Bit 1 - i2c0 dynamic pull-up enable"]
    #[inline(always)]
    pub fn i2c0_dyn_pullup_en(&mut self) -> I2C0_DYN_PULLUP_EN_W {
        I2C0_DYN_PULLUP_EN_W { w: self }
    }
    #[doc = "Bit 2 - i2c1 dynamic pull-up enable"]
    #[inline(always)]
    pub fn i2c1_dyn_pullup_en(&mut self) -> I2C1_DYN_PULLUP_EN_W {
        I2C1_DYN_PULLUP_EN_W { w: self }
    }
    #[doc = "Bit 3 - i2c2 dynamic pull-up enable"]
    #[inline(always)]
    pub fn i2c2_dyn_pullup_en(&mut self) -> I2C2_DYN_PULLUP_EN_W {
        I2C2_DYN_PULLUP_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flexible Fusion Engine status and control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ffe_csr](index.html) module"]
pub struct FFE_CSR_SPEC;
impl crate::RegisterSpec for FFE_CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ffe_csr::R](R) reader structure"]
impl crate::Readable for FFE_CSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ffe_csr::W](W) writer structure"]
impl crate::Writable for FFE_CSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FFE_CSR to value 0"]
impl crate::Resettable for FFE_CSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
