#[doc = "Register `AUDIO_MEM_CFG` reader"]
pub struct R(crate::R<AUDIO_MEM_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AUDIO_MEM_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AUDIO_MEM_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AUDIO_MEM_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Reserved\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [audio_mem_cfg](index.html) module"]
pub struct AUDIO_MEM_CFG_SPEC;
impl crate::RegisterSpec for AUDIO_MEM_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [audio_mem_cfg::R](R) reader structure"]
impl crate::Readable for AUDIO_MEM_CFG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets AUDIO_MEM_CFG to value 0"]
impl crate::Resettable for AUDIO_MEM_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
