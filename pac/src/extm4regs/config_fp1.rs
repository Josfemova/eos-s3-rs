#[doc = "Register `CONFIG_FP1` reader"]
pub struct R(crate::R<CONFIG_FP1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONFIG_FP1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONFIG_FP1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONFIG_FP1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FPIDC` reader - Floating-point input denormal exception"]
pub struct FPIDC_R(crate::FieldReader<bool, bool>);
impl FPIDC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FPIDC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPIDC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPIZC` reader - Floating-point divide-by-zero exception"]
pub struct FPIZC_R(crate::FieldReader<bool, bool>);
impl FPIZC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FPIZC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPIZC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPIOC` reader - Floating-point invalid exception"]
pub struct FPIOC_R(crate::FieldReader<bool, bool>);
impl FPIOC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FPIOC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPIOC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPIFC` reader - Floating-point underflow exception"]
pub struct FPIFC_R(crate::FieldReader<bool, bool>);
impl FPIFC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FPIFC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPIFC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPOFC` reader - Floating-point overflow exception"]
pub struct FPOFC_R(crate::FieldReader<bool, bool>);
impl FPOFC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FPOFC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPOFC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPIXC` reader - Floating-point inexact exception"]
pub struct FPIXC_R(crate::FieldReader<bool, bool>);
impl FPIXC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FPIXC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPIXC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Floating-point input denormal exception"]
    #[inline(always)]
    pub fn fpidc(&self) -> FPIDC_R {
        FPIDC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Floating-point divide-by-zero exception"]
    #[inline(always)]
    pub fn fpizc(&self) -> FPIZC_R {
        FPIZC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Floating-point invalid exception"]
    #[inline(always)]
    pub fn fpioc(&self) -> FPIOC_R {
        FPIOC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Floating-point underflow exception"]
    #[inline(always)]
    pub fn fpifc(&self) -> FPIFC_R {
        FPIFC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Floating-point overflow exception"]
    #[inline(always)]
    pub fn fpofc(&self) -> FPOFC_R {
        FPOFC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Floating-point inexact exception"]
    #[inline(always)]
    pub fn fpixc(&self) -> FPIXC_R {
        FPIXC_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
#[doc = "FPU configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config_fp1](index.html) module"]
pub struct CONFIG_FP1_SPEC;
impl crate::RegisterSpec for CONFIG_FP1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [config_fp1::R](R) reader structure"]
impl crate::Readable for CONFIG_FP1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CONFIG_FP1 to value 0"]
impl crate::Resettable for CONFIG_FP1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
