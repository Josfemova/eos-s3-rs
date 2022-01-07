#[doc = "Register `WIC_STATUS` reader"]
pub struct R(crate::R<WIC_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WIC_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WIC_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WIC_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WIC_Ready` reader - This bit is indicates that M4 Power Saving is enabled. This bit is based on the WIC/M4 interface signal."]
pub struct WIC_READY_R(crate::FieldReader<bool, bool>);
impl WIC_READY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WIC_READY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WIC_READY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - This bit is indicates that M4 Power Saving is enabled. This bit is based on the WIC/M4 interface signal."]
    #[inline(always)]
    pub fn wic_ready(&self) -> WIC_READY_R {
        WIC_READY_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "Wake-up Interrupt Controller Status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wic_status](index.html) module"]
pub struct WIC_STATUS_SPEC;
impl crate::RegisterSpec for WIC_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wic_status::R](R) reader structure"]
impl crate::Readable for WIC_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets WIC_STATUS to value 0"]
impl crate::Resettable for WIC_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
