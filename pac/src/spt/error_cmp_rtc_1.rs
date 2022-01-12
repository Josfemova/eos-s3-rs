#[doc = "Register `ERROR_CMP_RTC_1` reader"]
pub struct R(crate::R<ERROR_CMP_RTC_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ERROR_CMP_RTC_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ERROR_CMP_RTC_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ERROR_CMP_RTC_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ERROR_CMP_RTC_1` writer"]
pub struct W(crate::W<ERROR_CMP_RTC_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ERROR_CMP_RTC_1_SPEC>;
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
impl From<crate::W<ERROR_CMP_RTC_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ERROR_CMP_RTC_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ERROT_CMP_RTC_0_256` reader - Every 256s there are 4 sub-events. Each bit pair in this field corresponds to those sub-events. Values of 0 and 2 do nothing. A value of 3 means the counter should decrement 1ms, and a value of 1 means the counter must add 1ms. \\[bits 1:0 128002ms event, bits 3:2 128450ms event, bits 5:4 192002ms event, bits 7:6 192450ms event\\]"]
pub struct ERROT_CMP_RTC_0_256_R(crate::FieldReader<u8, u8>);
impl ERROT_CMP_RTC_0_256_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ERROT_CMP_RTC_0_256_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERROT_CMP_RTC_0_256_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERROT_CMP_RTC_0_256` writer - Every 256s there are 4 sub-events. Each bit pair in this field corresponds to those sub-events. Values of 0 and 2 do nothing. A value of 3 means the counter should decrement 1ms, and a value of 1 means the counter must add 1ms. \\[bits 1:0 128002ms event, bits 3:2 128450ms event, bits 5:4 192002ms event, bits 7:6 192450ms event\\]"]
pub struct ERROT_CMP_RTC_0_256_W<'a> {
    w: &'a mut W,
}
impl<'a> ERROT_CMP_RTC_0_256_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `ERROT_CMP_RTC_0_128` reader - Every 128s there are 4 sub-events. Each bit pair in this field corresponds to those sub-events. Values of 0 and 2 do nothing. A value of 3 means the counter should decrement 1ms, and a value of 1 means the counter must add 1ms. \\[bits 9:8 64002ms event, bits 11:10 64450ms event, bits 13:12 96002ms event, bits 15:14 96450ms event\\]"]
pub struct ERROT_CMP_RTC_0_128_R(crate::FieldReader<u8, u8>);
impl ERROT_CMP_RTC_0_128_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ERROT_CMP_RTC_0_128_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERROT_CMP_RTC_0_128_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERROT_CMP_RTC_0_128` writer - Every 128s there are 4 sub-events. Each bit pair in this field corresponds to those sub-events. Values of 0 and 2 do nothing. A value of 3 means the counter should decrement 1ms, and a value of 1 means the counter must add 1ms. \\[bits 9:8 64002ms event, bits 11:10 64450ms event, bits 13:12 96002ms event, bits 15:14 96450ms event\\]"]
pub struct ERROT_CMP_RTC_0_128_W<'a> {
    w: &'a mut W,
}
impl<'a> ERROT_CMP_RTC_0_128_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `ERROT_CMP_RTC_0_64` reader - Every 64s there are 4 sub-events. Each bit pair in this field corresponds to those sub-events. Values of 0 and 2 do nothing. A value of 3 means the counter should decrement 1ms, and a value of 1 means the counter must add 1ms. \\[bits 17:16 32002ms event, bits 19:18 32450ms event, bits 21:20 48002ms event, bits 23:22 48450ms event\\]"]
pub struct ERROT_CMP_RTC_0_64_R(crate::FieldReader<u8, u8>);
impl ERROT_CMP_RTC_0_64_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ERROT_CMP_RTC_0_64_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERROT_CMP_RTC_0_64_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERROT_CMP_RTC_0_64` writer - Every 64s there are 4 sub-events. Each bit pair in this field corresponds to those sub-events. Values of 0 and 2 do nothing. A value of 3 means the counter should decrement 1ms, and a value of 1 means the counter must add 1ms. \\[bits 17:16 32002ms event, bits 19:18 32450ms event, bits 21:20 48002ms event, bits 23:22 48450ms event\\]"]
pub struct ERROT_CMP_RTC_0_64_W<'a> {
    w: &'a mut W,
}
impl<'a> ERROT_CMP_RTC_0_64_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `ERROT_CMP_RTC_0_32` reader - Every 32s there are 4 sub-events. Each bit pair in this field corresponds to those sub-events. Values of 0 and 2 do nothing. A value of 3 means the counter should decrement 1ms, and a value of 1 means the counter must add 1ms. \\[bits 25:24 16002ms event, bits 27:26 16450ms event, bits 29:28 24450 event, bits 31:30 24450 event\\]"]
pub struct ERROT_CMP_RTC_0_32_R(crate::FieldReader<u8, u8>);
impl ERROT_CMP_RTC_0_32_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ERROT_CMP_RTC_0_32_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERROT_CMP_RTC_0_32_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERROT_CMP_RTC_0_32` writer - Every 32s there are 4 sub-events. Each bit pair in this field corresponds to those sub-events. Values of 0 and 2 do nothing. A value of 3 means the counter should decrement 1ms, and a value of 1 means the counter must add 1ms. \\[bits 25:24 16002ms event, bits 27:26 16450ms event, bits 29:28 24450 event, bits 31:30 24450 event\\]"]
pub struct ERROT_CMP_RTC_0_32_W<'a> {
    w: &'a mut W,
}
impl<'a> ERROT_CMP_RTC_0_32_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Every 256s there are 4 sub-events. Each bit pair in this field corresponds to those sub-events. Values of 0 and 2 do nothing. A value of 3 means the counter should decrement 1ms, and a value of 1 means the counter must add 1ms. \\[bits 1:0 128002ms event, bits 3:2 128450ms event, bits 5:4 192002ms event, bits 7:6 192450ms event\\]"]
    #[inline(always)]
    pub fn errot_cmp_rtc_0_256(&self) -> ERROT_CMP_RTC_0_256_R {
        ERROT_CMP_RTC_0_256_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Every 128s there are 4 sub-events. Each bit pair in this field corresponds to those sub-events. Values of 0 and 2 do nothing. A value of 3 means the counter should decrement 1ms, and a value of 1 means the counter must add 1ms. \\[bits 9:8 64002ms event, bits 11:10 64450ms event, bits 13:12 96002ms event, bits 15:14 96450ms event\\]"]
    #[inline(always)]
    pub fn errot_cmp_rtc_0_128(&self) -> ERROT_CMP_RTC_0_128_R {
        ERROT_CMP_RTC_0_128_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Every 64s there are 4 sub-events. Each bit pair in this field corresponds to those sub-events. Values of 0 and 2 do nothing. A value of 3 means the counter should decrement 1ms, and a value of 1 means the counter must add 1ms. \\[bits 17:16 32002ms event, bits 19:18 32450ms event, bits 21:20 48002ms event, bits 23:22 48450ms event\\]"]
    #[inline(always)]
    pub fn errot_cmp_rtc_0_64(&self) -> ERROT_CMP_RTC_0_64_R {
        ERROT_CMP_RTC_0_64_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Every 32s there are 4 sub-events. Each bit pair in this field corresponds to those sub-events. Values of 0 and 2 do nothing. A value of 3 means the counter should decrement 1ms, and a value of 1 means the counter must add 1ms. \\[bits 25:24 16002ms event, bits 27:26 16450ms event, bits 29:28 24450 event, bits 31:30 24450 event\\]"]
    #[inline(always)]
    pub fn errot_cmp_rtc_0_32(&self) -> ERROT_CMP_RTC_0_32_R {
        ERROT_CMP_RTC_0_32_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Every 256s there are 4 sub-events. Each bit pair in this field corresponds to those sub-events. Values of 0 and 2 do nothing. A value of 3 means the counter should decrement 1ms, and a value of 1 means the counter must add 1ms. \\[bits 1:0 128002ms event, bits 3:2 128450ms event, bits 5:4 192002ms event, bits 7:6 192450ms event\\]"]
    #[inline(always)]
    pub fn errot_cmp_rtc_0_256(&mut self) -> ERROT_CMP_RTC_0_256_W {
        ERROT_CMP_RTC_0_256_W { w: self }
    }
    #[doc = "Bits 8:15 - Every 128s there are 4 sub-events. Each bit pair in this field corresponds to those sub-events. Values of 0 and 2 do nothing. A value of 3 means the counter should decrement 1ms, and a value of 1 means the counter must add 1ms. \\[bits 9:8 64002ms event, bits 11:10 64450ms event, bits 13:12 96002ms event, bits 15:14 96450ms event\\]"]
    #[inline(always)]
    pub fn errot_cmp_rtc_0_128(&mut self) -> ERROT_CMP_RTC_0_128_W {
        ERROT_CMP_RTC_0_128_W { w: self }
    }
    #[doc = "Bits 16:23 - Every 64s there are 4 sub-events. Each bit pair in this field corresponds to those sub-events. Values of 0 and 2 do nothing. A value of 3 means the counter should decrement 1ms, and a value of 1 means the counter must add 1ms. \\[bits 17:16 32002ms event, bits 19:18 32450ms event, bits 21:20 48002ms event, bits 23:22 48450ms event\\]"]
    #[inline(always)]
    pub fn errot_cmp_rtc_0_64(&mut self) -> ERROT_CMP_RTC_0_64_W {
        ERROT_CMP_RTC_0_64_W { w: self }
    }
    #[doc = "Bits 24:31 - Every 32s there are 4 sub-events. Each bit pair in this field corresponds to those sub-events. Values of 0 and 2 do nothing. A value of 3 means the counter should decrement 1ms, and a value of 1 means the counter must add 1ms. \\[bits 25:24 16002ms event, bits 27:26 16450ms event, bits 29:28 24450 event, bits 31:30 24450 event\\]"]
    #[inline(always)]
    pub fn errot_cmp_rtc_0_32(&mut self) -> ERROT_CMP_RTC_0_32_W {
        ERROT_CMP_RTC_0_32_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "32, 64, 128, 256 sec Increment Error Compensation Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [error_cmp_rtc_1](index.html) module"]
pub struct ERROR_CMP_RTC_1_SPEC;
impl crate::RegisterSpec for ERROR_CMP_RTC_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [error_cmp_rtc_1::R](R) reader structure"]
impl crate::Readable for ERROR_CMP_RTC_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [error_cmp_rtc_1::W](W) writer structure"]
impl crate::Writable for ERROR_CMP_RTC_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ERROR_CMP_RTC_1 to value 0"]
impl crate::Resettable for ERROR_CMP_RTC_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
