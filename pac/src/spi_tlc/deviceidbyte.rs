#[doc = "Register `DEVICEIDBYTE` reader"]
pub struct R(crate::R<DEVICEIDBYTE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEVICEIDBYTE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEVICEIDBYTE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEVICEIDBYTE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DeviceIDByte` reader - Device ID, Read Only. 0x21, the Protocol of accessing this SFR is different, See Device ID read Page for detail"]
pub struct DEVICEIDBYTE_R(crate::FieldReader<u8, u8>);
impl DEVICEIDBYTE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DEVICEIDBYTE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEVICEIDBYTE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Device ID, Read Only. 0x21, the Protocol of accessing this SFR is different, See Device ID read Page for detail"]
    #[inline(always)]
    pub fn device_idbyte(&self) -> DEVICEIDBYTE_R {
        DEVICEIDBYTE_R::new(self.bits as u8)
    }
}
#[doc = "Device ID, Read Only. 0x21, the Protocol of accessing this SFR is different, See Device ID read Page for detail\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [deviceidbyte](index.html) module"]
pub struct DEVICEIDBYTE_SPEC;
impl crate::RegisterSpec for DEVICEIDBYTE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [deviceidbyte::R](R) reader structure"]
impl crate::Readable for DEVICEIDBYTE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DEVICEIDBYTE to value 0x21"]
impl crate::Resettable for DEVICEIDBYTE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x21
    }
}
