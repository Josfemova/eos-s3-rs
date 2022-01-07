#[doc = "Register `CHIP_STA_0` reader"]
pub struct R(crate::R<CHIP_STA_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHIP_STA_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHIP_STA_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHIP_STA_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Reserved\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chip_sta_0](index.html) module"]
pub struct CHIP_STA_0_SPEC;
impl crate::RegisterSpec for CHIP_STA_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chip_sta_0::R](R) reader structure"]
impl crate::Readable for CHIP_STA_0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CHIP_STA_0 to value 0"]
impl crate::Resettable for CHIP_STA_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
