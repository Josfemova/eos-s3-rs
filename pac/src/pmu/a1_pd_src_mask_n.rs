#[doc = "Register `A1_PD_SRC_MASK_N` reader"]
pub struct R(crate::R<A1_PD_SRC_MASK_N_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<A1_PD_SRC_MASK_N_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<A1_PD_SRC_MASK_N_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<A1_PD_SRC_MASK_N_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Reserved\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [a1_pd_src_mask_n](index.html) module"]
pub struct A1_PD_SRC_MASK_N_SPEC;
impl crate::RegisterSpec for A1_PD_SRC_MASK_N_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [a1_pd_src_mask_n::R](R) reader structure"]
impl crate::Readable for A1_PD_SRC_MASK_N_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets A1_PD_SRC_MASK_N to value 0"]
impl crate::Resettable for A1_PD_SRC_MASK_N_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
