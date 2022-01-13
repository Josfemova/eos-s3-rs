#[doc = "Register `ALT_CTRL_BASE_PTR` reader"]
pub struct R(crate::R<ALT_CTRL_BASE_PTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ALT_CTRL_BASE_PTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ALT_CTRL_BASE_PTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ALT_CTRL_BASE_PTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `alt_ctrl_base_ptr` reader - Base address of the alternate data structure"]
pub struct ALT_CTRL_BASE_PTR_R(crate::FieldReader<u32, u32>);
impl ALT_CTRL_BASE_PTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        ALT_CTRL_BASE_PTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALT_CTRL_BASE_PTR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Base address of the alternate data structure"]
    #[inline(always)]
    pub fn alt_ctrl_base_ptr(&self) -> ALT_CTRL_BASE_PTR_R {
        ALT_CTRL_BASE_PTR_R::new(self.bits as u32)
    }
}
#[doc = "Base address of the alternate data structure.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alt_ctrl_base_ptr](index.html) module"]
pub struct ALT_CTRL_BASE_PTR_SPEC;
impl crate::RegisterSpec for ALT_CTRL_BASE_PTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [alt_ctrl_base_ptr::R](R) reader structure"]
impl crate::Readable for ALT_CTRL_BASE_PTR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ALT_CTRL_BASE_PTR to value 0x0100"]
impl crate::Resettable for ALT_CTRL_BASE_PTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0100
    }
}
