#[doc = "Register `PCELL_ID_1` reader"]
pub struct R(crate::R<PCELL_ID_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCELL_ID_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCELL_ID_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCELL_ID_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `pcell_id_1` reader - PrimeCell identification 1"]
pub struct PCELL_ID_1_R(crate::FieldReader<u32, u32>);
impl PCELL_ID_1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        PCELL_ID_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCELL_ID_1_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - PrimeCell identification 1"]
    #[inline(always)]
    pub fn pcell_id_1(&self) -> PCELL_ID_1_R {
        PCELL_ID_1_R::new(self.bits as u32)
    }
}
#[doc = "PrimeCell identification 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcell_id_1](index.html) module"]
pub struct PCELL_ID_1_SPEC;
impl crate::RegisterSpec for PCELL_ID_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcell_id_1::R](R) reader structure"]
impl crate::Readable for PCELL_ID_1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PCELL_ID_1 to value 0xf0"]
impl crate::Resettable for PCELL_ID_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xf0
    }
}
