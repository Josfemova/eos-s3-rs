#[doc = "Register `MS_CNT_VALUE` reader"]
pub struct R(crate::R<MS_CNT_VALUE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MS_CNT_VALUE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MS_CNT_VALUE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MS_CNT_VALUE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MS_CNT_VALUE` reader - Return the Value of the 1mS counter which is generating the 1mS event. It is an downcount counter. Default is 0x40 (65-1). For 32KHz input, the resoultion is ~15uS. For 16KHz input, the resoultion is ~30uS"]
pub struct MS_CNT_VALUE_R(crate::FieldReader<u8, u8>);
impl MS_CNT_VALUE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MS_CNT_VALUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MS_CNT_VALUE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Return the Value of the 1mS counter which is generating the 1mS event. It is an downcount counter. Default is 0x40 (65-1). For 32KHz input, the resoultion is ~15uS. For 16KHz input, the resoultion is ~30uS"]
    #[inline(always)]
    pub fn ms_cnt_value(&self) -> MS_CNT_VALUE_R {
        MS_CNT_VALUE_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Return the Value of the 1mS counter which is generating the 1mS event. It is an downcount counter. Default is 0x40 (65-1). For 32KHz input, the resoultion is ~15uS. For 16KHz input, the resoultion is ~30uS\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ms_cnt_value](index.html) module"]
pub struct MS_CNT_VALUE_SPEC;
impl crate::RegisterSpec for MS_CNT_VALUE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ms_cnt_value::R](R) reader structure"]
impl crate::Readable for MS_CNT_VALUE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MS_CNT_VALUE to value 0x40"]
impl crate::Resettable for MS_CNT_VALUE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x40
    }
}
