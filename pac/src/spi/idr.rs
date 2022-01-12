#[doc = "Register `IDR` reader"]
pub struct R(crate::R<IDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IDCODE` reader - Identification Code. This register contains the peripherals identification code."]
pub struct IDCODE_R(crate::FieldReader<u32, u32>);
impl IDCODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        IDCODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IDCODE_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Identification Code. This register contains the peripherals identification code."]
    #[inline(always)]
    pub fn idcode(&self) -> IDCODE_R {
        IDCODE_R::new(self.bits as u32)
    }
}
#[doc = "Identification Code. This register contains the peripherals identification code.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idr](index.html) module"]
pub struct IDR_SPEC;
impl crate::RegisterSpec for IDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [idr::R](R) reader structure"]
impl crate::Readable for IDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IDR to value 0xffff_ffff"]
impl crate::Resettable for IDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
