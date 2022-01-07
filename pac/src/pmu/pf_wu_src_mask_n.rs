#[doc = "Register `PF_WU_SRC_MASK_N` reader"]
pub struct R(crate::R<PF_WU_SRC_MASK_N_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PF_WU_SRC_MASK_N_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PF_WU_SRC_MASK_N_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PF_WU_SRC_MASK_N_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Reserved\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pf_wu_src_mask_n](index.html) module"]
pub struct PF_WU_SRC_MASK_N_SPEC;
impl crate::RegisterSpec for PF_WU_SRC_MASK_N_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pf_wu_src_mask_n::R](R) reader structure"]
impl crate::Readable for PF_WU_SRC_MASK_N_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PF_WU_SRC_MASK_N to value 0"]
impl crate::Resettable for PF_WU_SRC_MASK_N_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
