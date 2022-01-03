#[doc = "Register `APC_CTRL_3` reader"]
pub struct R(crate::R<APC_CTRL_3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APC_CTRL_3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APC_CTRL_3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APC_CTRL_3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "APC control register 3 // Reserved\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apc_ctrl_3](index.html) module"]
pub struct APC_CTRL_3_SPEC;
impl crate::RegisterSpec for APC_CTRL_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apc_ctrl_3::R](R) reader structure"]
impl crate::Readable for APC_CTRL_3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets APC_CTRL_3 to value 0"]
impl crate::Resettable for APC_CTRL_3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
