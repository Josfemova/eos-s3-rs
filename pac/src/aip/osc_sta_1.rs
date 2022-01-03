#[doc = "Register `OSC_STA_1` reader"]
pub struct R(crate::R<OSC_STA_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OSC_STA_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OSC_STA_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OSC_STA_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `sdo` reader - Please refer to the Technical Reference Manual for detail, NO SYNC, FW need to read it twice to ensure the value."]
pub struct SDO_R(crate::FieldReader<bool, bool>);
impl SDO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SDO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Please refer to the Technical Reference Manual for detail, NO SYNC, FW need to read it twice to ensure the value."]
    #[inline(always)]
    pub fn sdo(&self) -> SDO_R {
        SDO_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "Contains information about oscilator status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osc_sta_1](index.html) module"]
pub struct OSC_STA_1_SPEC;
impl crate::RegisterSpec for OSC_STA_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [osc_sta_1::R](R) reader structure"]
impl crate::Readable for OSC_STA_1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets OSC_STA_1 to value 0"]
impl crate::Resettable for OSC_STA_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
