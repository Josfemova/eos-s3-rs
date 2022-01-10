#[doc = "Register `WDOGPERIPHID0` reader"]
pub struct R(crate::R<WDOGPERIPHID0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDOGPERIPHID0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDOGPERIPHID0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDOGPERIPHID0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WDOGPERIPHID0` reader - Periperhal ID Register 0. \\[7:0\\]
Part number\\[7:0\\]."]
pub struct WDOGPERIPHID0_R(crate::FieldReader<u8, u8>);
impl WDOGPERIPHID0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WDOGPERIPHID0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDOGPERIPHID0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Periperhal ID Register 0. \\[7:0\\]
Part number\\[7:0\\]."]
    #[inline(always)]
    pub fn wdogperiphid0(&self) -> WDOGPERIPHID0_R {
        WDOGPERIPHID0_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Periperhal ID Register 0. \\[7:0\\]
Part number\\[7:0\\].\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdogperiphid0](index.html) module"]
pub struct WDOGPERIPHID0_SPEC;
impl crate::RegisterSpec for WDOGPERIPHID0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdogperiphid0::R](R) reader structure"]
impl crate::Readable for WDOGPERIPHID0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets WDOGPERIPHID0 to value 0x24"]
impl crate::Resettable for WDOGPERIPHID0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x24
    }
}
