#[doc = "Register `PERIPH_ID_0` reader"]
pub struct R(crate::R<PERIPH_ID_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERIPH_ID_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERIPH_ID_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERIPH_ID_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `periph_id_0` reader - Peripheral identification 0"]
pub struct PERIPH_ID_0_R(crate::FieldReader<u32, u32>);
impl PERIPH_ID_0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        PERIPH_ID_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PERIPH_ID_0_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Peripheral identification 0"]
    #[inline(always)]
    pub fn periph_id_0(&self) -> PERIPH_ID_0_R {
        PERIPH_ID_0_R::new(self.bits as u32)
    }
}
#[doc = "Peripheral identification 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [periph_id_0](index.html) module"]
pub struct PERIPH_ID_0_SPEC;
impl crate::RegisterSpec for PERIPH_ID_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [periph_id_0::R](R) reader structure"]
impl crate::Readable for PERIPH_ID_0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PERIPH_ID_0 to value 0x30"]
impl crate::Resettable for PERIPH_ID_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x30
    }
}
