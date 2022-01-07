#[doc = "Register `M4_PD_SRC_MASKk_N` reader"]
pub struct R(crate::R<M4_PD_SRC_MASKK_N_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<M4_PD_SRC_MASKK_N_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<M4_PD_SRC_MASKK_N_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<M4_PD_SRC_MASKK_N_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Reserved\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m4_pd_src_maskk_n](index.html) module"]
pub struct M4_PD_SRC_MASKK_N_SPEC;
impl crate::RegisterSpec for M4_PD_SRC_MASKK_N_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [m4_pd_src_maskk_n::R](R) reader structure"]
impl crate::Readable for M4_PD_SRC_MASKK_N_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets M4_PD_SRC_MASKk_N to value 0"]
impl crate::Resettable for M4_PD_SRC_MASKK_N_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
