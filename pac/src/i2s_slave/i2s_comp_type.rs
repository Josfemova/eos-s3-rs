#[doc = "Register `I2S_COMP_TYPE` reader"]
pub struct R(crate::R<I2S_COMP_TYPE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2S_COMP_TYPE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2S_COMP_TYPE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2S_COMP_TYPE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `I2S_COMP_TYPE` reader - Component type of the I2S peripheral."]
pub struct I2S_COMP_TYPE_R(crate::FieldReader<u32, u32>);
impl I2S_COMP_TYPE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        I2S_COMP_TYPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2S_COMP_TYPE_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Component type of the I2S peripheral."]
    #[inline(always)]
    pub fn i2s_comp_type(&self) -> I2S_COMP_TYPE_R {
        I2S_COMP_TYPE_R::new(self.bits as u32)
    }
}
#[doc = "Component type of the I2S peripheral.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2s_comp_type](index.html) module"]
pub struct I2S_COMP_TYPE_SPEC;
impl crate::RegisterSpec for I2S_COMP_TYPE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2s_comp_type::R](R) reader structure"]
impl crate::Readable for I2S_COMP_TYPE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets I2S_COMP_TYPE to value 0x4457_01a0"]
impl crate::Resettable for I2S_COMP_TYPE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x4457_01a0
    }
}
