#[doc = "Register `CID0` reader"]
pub struct R(crate::R<CID0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CID0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CID0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CID0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PID` reader - Component ID"]
pub struct PID_R(crate::FieldReader<u8, u8>);
impl PID_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PID_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Component ID"]
    #[inline(always)]
    pub fn pid(&self) -> PID_R {
        PID_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Component ID register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cid0](index.html) module"]
pub struct CID0_SPEC;
impl crate::RegisterSpec for CID0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cid0::R](R) reader structure"]
impl crate::Readable for CID0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CID0 to value 0"]
impl crate::Resettable for CID0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
