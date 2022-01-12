#[doc = "Register `EVENT_CNT_VALUE` reader"]
pub struct R(crate::R<EVENT_CNT_VALUE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVENT_CNT_VALUE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVENT_CNT_VALUE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVENT_CNT_VALUE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EVENT_CNT_VALUE` reader - Return the Value of the Event counter generating FFE Time out event It is an upcount counter, in 1mS resoultion."]
pub struct EVENT_CNT_VALUE_R(crate::FieldReader<u8, u8>);
impl EVENT_CNT_VALUE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        EVENT_CNT_VALUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EVENT_CNT_VALUE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Return the Value of the Event counter generating FFE Time out event It is an upcount counter, in 1mS resoultion."]
    #[inline(always)]
    pub fn event_cnt_value(&self) -> EVENT_CNT_VALUE_R {
        EVENT_CNT_VALUE_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Return the Value of the Event counter generating FFE Time out event It is an upcount counter, in 1mS resoultion.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [event_cnt_value](index.html) module"]
pub struct EVENT_CNT_VALUE_SPEC;
impl crate::RegisterSpec for EVENT_CNT_VALUE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [event_cnt_value::R](R) reader structure"]
impl crate::Readable for EVENT_CNT_VALUE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets EVENT_CNT_VALUE to value 0"]
impl crate::Resettable for EVENT_CNT_VALUE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
