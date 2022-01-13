#[doc = "Register `PERIPH_ID_4` reader"]
pub struct R(crate::R<PERIPH_ID_4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERIPH_ID_4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERIPH_ID_4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERIPH_ID_4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `periph_id_4` reader - Peripheral identification 4"]
pub struct PERIPH_ID_4_R(crate::FieldReader<u32, u32>);
impl PERIPH_ID_4_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        PERIPH_ID_4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PERIPH_ID_4_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Peripheral identification 4"]
    #[inline(always)]
    pub fn periph_id_4(&self) -> PERIPH_ID_4_R {
        PERIPH_ID_4_R::new(self.bits as u32)
    }
}
#[doc = "Peripheral identification 4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [periph_id_4](index.html) module"]
pub struct PERIPH_ID_4_SPEC;
impl crate::RegisterSpec for PERIPH_ID_4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [periph_id_4::R](R) reader structure"]
impl crate::Readable for PERIPH_ID_4_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PERIPH_ID_4 to value 0x04"]
impl crate::Resettable for PERIPH_ID_4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x04
    }
}
