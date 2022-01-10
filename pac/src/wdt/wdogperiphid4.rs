#[doc = "Register `WDOGPERIPHID4` reader"]
pub struct R(crate::R<WDOGPERIPHID4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDOGPERIPHID4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDOGPERIPHID4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDOGPERIPHID4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WDOGPERIPHID4` reader - Peripheral ID Register 4: \\[7:4\\]
Block count. \\[3:0\\]
jep106_c_code."]
pub struct WDOGPERIPHID4_R(crate::FieldReader<u8, u8>);
impl WDOGPERIPHID4_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WDOGPERIPHID4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDOGPERIPHID4_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Peripheral ID Register 4: \\[7:4\\]
Block count. \\[3:0\\]
jep106_c_code."]
    #[inline(always)]
    pub fn wdogperiphid4(&self) -> WDOGPERIPHID4_R {
        WDOGPERIPHID4_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Peripheral ID Register 4: \\[7:4\\]
Block count. \\[3:0\\]
jep106_c_code.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdogperiphid4](index.html) module"]
pub struct WDOGPERIPHID4_SPEC;
impl crate::RegisterSpec for WDOGPERIPHID4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdogperiphid4::R](R) reader structure"]
impl crate::Readable for WDOGPERIPHID4_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets WDOGPERIPHID4 to value 0x04"]
impl crate::Resettable for WDOGPERIPHID4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x04
    }
}
