#[doc = "Register `PF_MEM_CFG` reader"]
pub struct R(crate::R<PF_MEM_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PF_MEM_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PF_MEM_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PF_MEM_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Reserved\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pf_mem_cfg](index.html) module"]
pub struct PF_MEM_CFG_SPEC;
impl crate::RegisterSpec for PF_MEM_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pf_mem_cfg::R](R) reader structure"]
impl crate::Readable for PF_MEM_CFG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PF_MEM_CFG to value 0"]
impl crate::Resettable for PF_MEM_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
