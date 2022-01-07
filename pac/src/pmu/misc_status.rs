#[doc = "Register `MISC_STATUS` reader"]
pub struct R(crate::R<MISC_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MISC_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MISC_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MISC_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `I2S` reader - Bit will be set if the I2S is powered on"]
pub struct I2S_R(crate::FieldReader<bool, bool>);
impl I2S_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2S_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2S_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 4 - Bit will be set if the I2S is powered on"]
    #[inline(always)]
    pub fn i2s(&self) -> I2S_R {
        I2S_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
#[doc = "I2S Power info\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [misc_status](index.html) module"]
pub struct MISC_STATUS_SPEC;
impl crate::RegisterSpec for MISC_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [misc_status::R](R) reader structure"]
impl crate::Readable for MISC_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MISC_STATUS to value 0"]
impl crate::Resettable for MISC_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
