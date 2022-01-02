#[doc = "Register `FB_DEVICE_ID` reader"]
pub struct R(crate::R<FB_DEVICE_ID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FB_DEVICE_ID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FB_DEVICE_ID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FB_DEVICE_ID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ID` reader - Fabric device ID"]
pub struct ID_R(crate::FieldReader<u16, u16>);
impl ID_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        ID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ID_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Fabric device ID"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Fabric device ID\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fb_device_id](index.html) module"]
pub struct FB_DEVICE_ID_SPEC;
impl crate::RegisterSpec for FB_DEVICE_ID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fb_device_id::R](R) reader structure"]
impl crate::Readable for FB_DEVICE_ID_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FB_DEVICE_ID to value 0"]
impl crate::Resettable for FB_DEVICE_ID_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
