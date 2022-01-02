#[doc = "Register `ADC_Status` reader"]
pub struct R(crate::R<ADC_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "End of Conversion. Rises when data is valid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOC_A {
    #[doc = "0: data in OUT field is not valid"]
    NON_VALID = 0,
    #[doc = "1: data in OUT field is valid"]
    VALID = 1,
}
impl From<EOC_A> for bool {
    #[inline(always)]
    fn from(variant: EOC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOC` reader - End of Conversion. Rises when data is valid"]
pub struct EOC_R(crate::FieldReader<bool, EOC_A>);
impl EOC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EOC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOC_A {
        match self.bits {
            false => EOC_A::NON_VALID,
            true => EOC_A::VALID,
        }
    }
    #[doc = "Checks if the value of the field is `NON_VALID`"]
    #[inline(always)]
    pub fn is_non_valid(&self) -> bool {
        **self == EOC_A::NON_VALID
    }
    #[doc = "Checks if the value of the field is `VALID`"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        **self == EOC_A::VALID
    }
}
impl core::ops::Deref for EOC_R {
    type Target = crate::FieldReader<bool, EOC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - End of Conversion. Rises when data is valid"]
    #[inline(always)]
    pub fn eoc(&self) -> EOC_R {
        EOC_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_status](index.html) module"]
pub struct ADC_STATUS_SPEC;
impl crate::RegisterSpec for ADC_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_status::R](R) reader structure"]
impl crate::Readable for ADC_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ADC_Status to value 0"]
impl crate::Resettable for ADC_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
