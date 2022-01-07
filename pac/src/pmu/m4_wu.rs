#[doc = "Register `M4_WU` reader"]
pub struct R(crate::R<M4_WU_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<M4_WU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<M4_WU_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<M4_WU_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Reserved\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m4_wu](index.html) module"]
pub struct M4_WU_SPEC;
impl crate::RegisterSpec for M4_WU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [m4_wu::R](R) reader structure"]
impl crate::Readable for M4_WU_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets M4_WU to value 0"]
impl crate::Resettable for M4_WU_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
