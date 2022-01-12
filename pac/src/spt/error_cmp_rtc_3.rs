#[doc = "Register `ERROR_CMP_RTC_3` reader"]
pub struct R(crate::R<ERROR_CMP_RTC_3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ERROR_CMP_RTC_3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ERROR_CMP_RTC_3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ERROR_CMP_RTC_3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ERROR_CMP_RTC_3` writer"]
pub struct W(crate::W<ERROR_CMP_RTC_3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ERROR_CMP_RTC_3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<ERROR_CMP_RTC_3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ERROR_CMP_RTC_3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ERROT_CMP_RTC_0_16384` reader - Every 16384s there are 4 sub-events. Each bit pair in this field corresponds to those sub-events. Values of 0 and 2 do nothing. A value of 3 means the counter should decrement 1ms, and a value of 1 means the counter must add 1ms. \\[bits 1:0 8192002ms event, bits 3:2 8192450ms event, bits 5:4 12288002ms event, bits 7:6 12288450ms event\\]"]
pub struct ERROT_CMP_RTC_0_16384_R(crate::FieldReader<u8, u8>);
impl ERROT_CMP_RTC_0_16384_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ERROT_CMP_RTC_0_16384_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERROT_CMP_RTC_0_16384_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERROT_CMP_RTC_0_16384` writer - Every 16384s there are 4 sub-events. Each bit pair in this field corresponds to those sub-events. Values of 0 and 2 do nothing. A value of 3 means the counter should decrement 1ms, and a value of 1 means the counter must add 1ms. \\[bits 1:0 8192002ms event, bits 3:2 8192450ms event, bits 5:4 12288002ms event, bits 7:6 12288450ms event\\]"]
pub struct ERROT_CMP_RTC_0_16384_W<'a> {
    w: &'a mut W,
}
impl<'a> ERROT_CMP_RTC_0_16384_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `ERROT_CMP_RTC_2_8192` reader - Every 8192s there are 4 sub-events. Each bit pair in this field corresponds to those sub-events. Values of 0 and 2 do nothing. A value of 3 means the counter should decrement 1ms, and a value of 1 means the counter must add 1ms. \\[bits 9:8 4096002ms event, bits 11:10 4096450ms event, bits 13:12 6144002ms event, bits 15:14 6144450ms event\\]"]
pub struct ERROT_CMP_RTC_2_8192_R(crate::FieldReader<u8, u8>);
impl ERROT_CMP_RTC_2_8192_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ERROT_CMP_RTC_2_8192_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERROT_CMP_RTC_2_8192_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERROT_CMP_RTC_2_8192` writer - Every 8192s there are 4 sub-events. Each bit pair in this field corresponds to those sub-events. Values of 0 and 2 do nothing. A value of 3 means the counter should decrement 1ms, and a value of 1 means the counter must add 1ms. \\[bits 9:8 4096002ms event, bits 11:10 4096450ms event, bits 13:12 6144002ms event, bits 15:14 6144450ms event\\]"]
pub struct ERROT_CMP_RTC_2_8192_W<'a> {
    w: &'a mut W,
}
impl<'a> ERROT_CMP_RTC_2_8192_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Every 16384s there are 4 sub-events. Each bit pair in this field corresponds to those sub-events. Values of 0 and 2 do nothing. A value of 3 means the counter should decrement 1ms, and a value of 1 means the counter must add 1ms. \\[bits 1:0 8192002ms event, bits 3:2 8192450ms event, bits 5:4 12288002ms event, bits 7:6 12288450ms event\\]"]
    #[inline(always)]
    pub fn errot_cmp_rtc_0_16384(&self) -> ERROT_CMP_RTC_0_16384_R {
        ERROT_CMP_RTC_0_16384_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Every 8192s there are 4 sub-events. Each bit pair in this field corresponds to those sub-events. Values of 0 and 2 do nothing. A value of 3 means the counter should decrement 1ms, and a value of 1 means the counter must add 1ms. \\[bits 9:8 4096002ms event, bits 11:10 4096450ms event, bits 13:12 6144002ms event, bits 15:14 6144450ms event\\]"]
    #[inline(always)]
    pub fn errot_cmp_rtc_2_8192(&self) -> ERROT_CMP_RTC_2_8192_R {
        ERROT_CMP_RTC_2_8192_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Every 16384s there are 4 sub-events. Each bit pair in this field corresponds to those sub-events. Values of 0 and 2 do nothing. A value of 3 means the counter should decrement 1ms, and a value of 1 means the counter must add 1ms. \\[bits 1:0 8192002ms event, bits 3:2 8192450ms event, bits 5:4 12288002ms event, bits 7:6 12288450ms event\\]"]
    #[inline(always)]
    pub fn errot_cmp_rtc_0_16384(&mut self) -> ERROT_CMP_RTC_0_16384_W {
        ERROT_CMP_RTC_0_16384_W { w: self }
    }
    #[doc = "Bits 8:15 - Every 8192s there are 4 sub-events. Each bit pair in this field corresponds to those sub-events. Values of 0 and 2 do nothing. A value of 3 means the counter should decrement 1ms, and a value of 1 means the counter must add 1ms. \\[bits 9:8 4096002ms event, bits 11:10 4096450ms event, bits 13:12 6144002ms event, bits 15:14 6144450ms event\\]"]
    #[inline(always)]
    pub fn errot_cmp_rtc_2_8192(&mut self) -> ERROT_CMP_RTC_2_8192_W {
        ERROT_CMP_RTC_2_8192_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "8192, 16384 sec Increment Error Compensation Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [error_cmp_rtc_3](index.html) module"]
pub struct ERROR_CMP_RTC_3_SPEC;
impl crate::RegisterSpec for ERROR_CMP_RTC_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [error_cmp_rtc_3::R](R) reader structure"]
impl crate::Readable for ERROR_CMP_RTC_3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [error_cmp_rtc_3::W](W) writer structure"]
impl crate::Writable for ERROR_CMP_RTC_3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ERROR_CMP_RTC_3 to value 0"]
impl crate::Resettable for ERROR_CMP_RTC_3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
