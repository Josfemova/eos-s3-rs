#[doc = "Register `PCELL_ID_2` reader"]
pub struct R(crate::R<PCELL_ID_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCELL_ID_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCELL_ID_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCELL_ID_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `pcell_id_2` reader - PrimeCell identification 2"]
pub struct PCELL_ID_2_R(crate::FieldReader<u32, u32>);
impl PCELL_ID_2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        PCELL_ID_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCELL_ID_2_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - PrimeCell identification 2"]
    #[inline(always)]
    pub fn pcell_id_2(&self) -> PCELL_ID_2_R {
        PCELL_ID_2_R::new(self.bits as u32)
    }
}
#[doc = "PrimeCell identification 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcell_id_2](index.html) module"]
pub struct PCELL_ID_2_SPEC;
impl crate::RegisterSpec for PCELL_ID_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcell_id_2::R](R) reader structure"]
impl crate::Readable for PCELL_ID_2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PCELL_ID_2 to value 0x05"]
impl crate::Resettable for PCELL_ID_2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x05
    }
}
