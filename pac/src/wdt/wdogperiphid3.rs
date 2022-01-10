#[doc = "Register `WDOGPERIPHID3` reader"]
pub struct R(crate::R<WDOGPERIPHID3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDOGPERIPHID3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDOGPERIPHID3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDOGPERIPHID3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WDOGPERIPHID3` reader - Peripherial ID Register 3. \\[7:4\\]
ECO revision number. \\[3:0\\]
Customer modification number."]
pub struct WDOGPERIPHID3_R(crate::FieldReader<u8, u8>);
impl WDOGPERIPHID3_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WDOGPERIPHID3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDOGPERIPHID3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Peripherial ID Register 3. \\[7:4\\]
ECO revision number. \\[3:0\\]
Customer modification number."]
    #[inline(always)]
    pub fn wdogperiphid3(&self) -> WDOGPERIPHID3_R {
        WDOGPERIPHID3_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Peripherial ID Register 3. \\[7:4\\]
ECO revision number. \\[3:0\\]
Customer modification number.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdogperiphid3](index.html) module"]
pub struct WDOGPERIPHID3_SPEC;
impl crate::RegisterSpec for WDOGPERIPHID3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdogperiphid3::R](R) reader structure"]
impl crate::Readable for WDOGPERIPHID3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets WDOGPERIPHID3 to value 0"]
impl crate::Resettable for WDOGPERIPHID3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
