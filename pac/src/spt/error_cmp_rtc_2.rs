#[doc = "Register `ERROR_CMP_RTC_2` reader"]
pub struct R(crate::R<ERROR_CMP_RTC_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ERROR_CMP_RTC_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ERROR_CMP_RTC_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ERROR_CMP_RTC_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ERROR_CMP_RTC_2` writer"]
pub struct W(crate::W<ERROR_CMP_RTC_2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ERROR_CMP_RTC_2_SPEC>;
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
impl From<crate::W<ERROR_CMP_RTC_2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ERROR_CMP_RTC_2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ERROT_CMP_RTC_0_4096` reader - Every 4096s there are 4 sub-events. Each bit pair in this field corresponds to those sub-events. Values of 0 and 2 do nothing. A value of 3 means the counter should decrement 1ms, and a value of 1 means the counter must add 1ms. \\[bits 1:0 2048002ms event, bits 3:2 2048450ms event, bits 5:4 3072002ms event, bits 7:6 3072450ms event\\]"]
pub struct ERROT_CMP_RTC_0_4096_R(crate::FieldReader<u8, u8>);
impl ERROT_CMP_RTC_0_4096_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ERROT_CMP_RTC_0_4096_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERROT_CMP_RTC_0_4096_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERROT_CMP_RTC_0_4096` writer - Every 4096s there are 4 sub-events. Each bit pair in this field corresponds to those sub-events. Values of 0 and 2 do nothing. A value of 3 means the counter should decrement 1ms, and a value of 1 means the counter must add 1ms. \\[bits 1:0 2048002ms event, bits 3:2 2048450ms event, bits 5:4 3072002ms event, bits 7:6 3072450ms event\\]"]
pub struct ERROT_CMP_RTC_0_4096_W<'a> {
    w: &'a mut W,
}
impl<'a> ERROT_CMP_RTC_0_4096_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `ERROT_CMP_RTC_2_2048` reader - Every 2048s there are 4 sub-events. Each bit pair in this field corresponds to those sub-events. Values of 0 and 2 do nothing. A value of 3 means the counter should decrement 1ms, and a value of 1 means the counter must add 1ms. \\[bits 9:8 1024002ms event, bits 11:10 1024450ms event, bits 13:12 1536002ms event, bits 15:14 1536450ms event\\]"]
pub struct ERROT_CMP_RTC_2_2048_R(crate::FieldReader<u8, u8>);
impl ERROT_CMP_RTC_2_2048_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ERROT_CMP_RTC_2_2048_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERROT_CMP_RTC_2_2048_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERROT_CMP_RTC_2_2048` writer - Every 2048s there are 4 sub-events. Each bit pair in this field corresponds to those sub-events. Values of 0 and 2 do nothing. A value of 3 means the counter should decrement 1ms, and a value of 1 means the counter must add 1ms. \\[bits 9:8 1024002ms event, bits 11:10 1024450ms event, bits 13:12 1536002ms event, bits 15:14 1536450ms event\\]"]
pub struct ERROT_CMP_RTC_2_2048_W<'a> {
    w: &'a mut W,
}
impl<'a> ERROT_CMP_RTC_2_2048_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `ERROT_CMP_RTC_0_1024` reader - Every 1024s there are 4 sub-events. Each bit pair in this field corresponds to those sub-events. Values of 0 and 2 do nothing. A value of 3 means the counter should decrement 1ms, and a value of 1 means the counter must add 1ms. \\[bits 17:16 512002ms event, bits 19:18 512450ms event, bits 21:20 768002ms event, bits 23:22 768450ms event\\]"]
pub struct ERROT_CMP_RTC_0_1024_R(crate::FieldReader<u8, u8>);
impl ERROT_CMP_RTC_0_1024_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ERROT_CMP_RTC_0_1024_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERROT_CMP_RTC_0_1024_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERROT_CMP_RTC_0_1024` writer - Every 1024s there are 4 sub-events. Each bit pair in this field corresponds to those sub-events. Values of 0 and 2 do nothing. A value of 3 means the counter should decrement 1ms, and a value of 1 means the counter must add 1ms. \\[bits 17:16 512002ms event, bits 19:18 512450ms event, bits 21:20 768002ms event, bits 23:22 768450ms event\\]"]
pub struct ERROT_CMP_RTC_0_1024_W<'a> {
    w: &'a mut W,
}
impl<'a> ERROT_CMP_RTC_0_1024_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `ERROT_CMP_RTC_0_512` reader - Every 512s there are 4 sub-events. Each bit pair in this field corresponds to those sub-events. Values of 0 and 2 do nothing. A value of 3 means the counter should decrement 1ms, and a value of 1 means the counter must add 1ms. \\[bits 25:24 256002ms event, bits 27:26 256450ms event, bits 29:28 384002 event, bits 31:30 384450 event\\]"]
pub struct ERROT_CMP_RTC_0_512_R(crate::FieldReader<u8, u8>);
impl ERROT_CMP_RTC_0_512_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ERROT_CMP_RTC_0_512_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERROT_CMP_RTC_0_512_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERROT_CMP_RTC_0_512` writer - Every 512s there are 4 sub-events. Each bit pair in this field corresponds to those sub-events. Values of 0 and 2 do nothing. A value of 3 means the counter should decrement 1ms, and a value of 1 means the counter must add 1ms. \\[bits 25:24 256002ms event, bits 27:26 256450ms event, bits 29:28 384002 event, bits 31:30 384450 event\\]"]
pub struct ERROT_CMP_RTC_0_512_W<'a> {
    w: &'a mut W,
}
impl<'a> ERROT_CMP_RTC_0_512_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Every 4096s there are 4 sub-events. Each bit pair in this field corresponds to those sub-events. Values of 0 and 2 do nothing. A value of 3 means the counter should decrement 1ms, and a value of 1 means the counter must add 1ms. \\[bits 1:0 2048002ms event, bits 3:2 2048450ms event, bits 5:4 3072002ms event, bits 7:6 3072450ms event\\]"]
    #[inline(always)]
    pub fn errot_cmp_rtc_0_4096(&self) -> ERROT_CMP_RTC_0_4096_R {
        ERROT_CMP_RTC_0_4096_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Every 2048s there are 4 sub-events. Each bit pair in this field corresponds to those sub-events. Values of 0 and 2 do nothing. A value of 3 means the counter should decrement 1ms, and a value of 1 means the counter must add 1ms. \\[bits 9:8 1024002ms event, bits 11:10 1024450ms event, bits 13:12 1536002ms event, bits 15:14 1536450ms event\\]"]
    #[inline(always)]
    pub fn errot_cmp_rtc_2_2048(&self) -> ERROT_CMP_RTC_2_2048_R {
        ERROT_CMP_RTC_2_2048_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Every 1024s there are 4 sub-events. Each bit pair in this field corresponds to those sub-events. Values of 0 and 2 do nothing. A value of 3 means the counter should decrement 1ms, and a value of 1 means the counter must add 1ms. \\[bits 17:16 512002ms event, bits 19:18 512450ms event, bits 21:20 768002ms event, bits 23:22 768450ms event\\]"]
    #[inline(always)]
    pub fn errot_cmp_rtc_0_1024(&self) -> ERROT_CMP_RTC_0_1024_R {
        ERROT_CMP_RTC_0_1024_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Every 512s there are 4 sub-events. Each bit pair in this field corresponds to those sub-events. Values of 0 and 2 do nothing. A value of 3 means the counter should decrement 1ms, and a value of 1 means the counter must add 1ms. \\[bits 25:24 256002ms event, bits 27:26 256450ms event, bits 29:28 384002 event, bits 31:30 384450 event\\]"]
    #[inline(always)]
    pub fn errot_cmp_rtc_0_512(&self) -> ERROT_CMP_RTC_0_512_R {
        ERROT_CMP_RTC_0_512_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Every 4096s there are 4 sub-events. Each bit pair in this field corresponds to those sub-events. Values of 0 and 2 do nothing. A value of 3 means the counter should decrement 1ms, and a value of 1 means the counter must add 1ms. \\[bits 1:0 2048002ms event, bits 3:2 2048450ms event, bits 5:4 3072002ms event, bits 7:6 3072450ms event\\]"]
    #[inline(always)]
    pub fn errot_cmp_rtc_0_4096(&mut self) -> ERROT_CMP_RTC_0_4096_W {
        ERROT_CMP_RTC_0_4096_W { w: self }
    }
    #[doc = "Bits 8:15 - Every 2048s there are 4 sub-events. Each bit pair in this field corresponds to those sub-events. Values of 0 and 2 do nothing. A value of 3 means the counter should decrement 1ms, and a value of 1 means the counter must add 1ms. \\[bits 9:8 1024002ms event, bits 11:10 1024450ms event, bits 13:12 1536002ms event, bits 15:14 1536450ms event\\]"]
    #[inline(always)]
    pub fn errot_cmp_rtc_2_2048(&mut self) -> ERROT_CMP_RTC_2_2048_W {
        ERROT_CMP_RTC_2_2048_W { w: self }
    }
    #[doc = "Bits 16:23 - Every 1024s there are 4 sub-events. Each bit pair in this field corresponds to those sub-events. Values of 0 and 2 do nothing. A value of 3 means the counter should decrement 1ms, and a value of 1 means the counter must add 1ms. \\[bits 17:16 512002ms event, bits 19:18 512450ms event, bits 21:20 768002ms event, bits 23:22 768450ms event\\]"]
    #[inline(always)]
    pub fn errot_cmp_rtc_0_1024(&mut self) -> ERROT_CMP_RTC_0_1024_W {
        ERROT_CMP_RTC_0_1024_W { w: self }
    }
    #[doc = "Bits 24:31 - Every 512s there are 4 sub-events. Each bit pair in this field corresponds to those sub-events. Values of 0 and 2 do nothing. A value of 3 means the counter should decrement 1ms, and a value of 1 means the counter must add 1ms. \\[bits 25:24 256002ms event, bits 27:26 256450ms event, bits 29:28 384002 event, bits 31:30 384450 event\\]"]
    #[inline(always)]
    pub fn errot_cmp_rtc_0_512(&mut self) -> ERROT_CMP_RTC_0_512_W {
        ERROT_CMP_RTC_0_512_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "512, 1024, 2048, 4096 sec Increment Error Compensation Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [error_cmp_rtc_2](index.html) module"]
pub struct ERROR_CMP_RTC_2_SPEC;
impl crate::RegisterSpec for ERROR_CMP_RTC_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [error_cmp_rtc_2::R](R) reader structure"]
impl crate::Readable for ERROR_CMP_RTC_2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [error_cmp_rtc_2::W](W) writer structure"]
impl crate::Writable for ERROR_CMP_RTC_2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ERROR_CMP_RTC_2 to value 0"]
impl crate::Resettable for ERROR_CMP_RTC_2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
