#[doc = "Register `ERROR_CMP_RTC_0` reader"]
pub struct R(crate::R<ERROR_CMP_RTC_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ERROR_CMP_RTC_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ERROR_CMP_RTC_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ERROR_CMP_RTC_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ERROR_CMP_RTC_0` writer"]
pub struct W(crate::W<ERROR_CMP_RTC_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ERROR_CMP_RTC_0_SPEC>;
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
impl From<crate::W<ERROR_CMP_RTC_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ERROR_CMP_RTC_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ERROT_CMP_RTC_0_16` reader - Every 16s there are 4 sub-events. Each bit pair in this field corresponds to those sub-events. Values of 0 and 2 do nothing. A value of 3 means the counter should decrement 1ms, and a value of 1 means the counter must add 1ms. \\[bits 1:0 4002ms event, bits 3:2 4450ms event, bits 5:4 6002ms event, bits 7:6 6450ms event\\]"]
pub struct ERROT_CMP_RTC_0_16_R(crate::FieldReader<u8, u8>);
impl ERROT_CMP_RTC_0_16_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ERROT_CMP_RTC_0_16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERROT_CMP_RTC_0_16_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERROT_CMP_RTC_0_16` writer - Every 16s there are 4 sub-events. Each bit pair in this field corresponds to those sub-events. Values of 0 and 2 do nothing. A value of 3 means the counter should decrement 1ms, and a value of 1 means the counter must add 1ms. \\[bits 1:0 4002ms event, bits 3:2 4450ms event, bits 5:4 6002ms event, bits 7:6 6450ms event\\]"]
pub struct ERROT_CMP_RTC_0_16_W<'a> {
    w: &'a mut W,
}
impl<'a> ERROT_CMP_RTC_0_16_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `ERROT_CMP_RTC_0_8` reader - Every 8s there are 4 sub-events. Each bit pair in this field corresponds to those sub-events. Values of 0 and 2 do nothing. A value of 3 means the counter should decrement 1ms, and a value of 1 means the counter must add 1ms. \\[bits 9:8 4002ms event, bits 11:10 4450ms event, bits 13:12 6002ms event, bits 15:14 6450ms event\\]"]
pub struct ERROT_CMP_RTC_0_8_R(crate::FieldReader<u8, u8>);
impl ERROT_CMP_RTC_0_8_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ERROT_CMP_RTC_0_8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERROT_CMP_RTC_0_8_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERROT_CMP_RTC_0_8` writer - Every 8s there are 4 sub-events. Each bit pair in this field corresponds to those sub-events. Values of 0 and 2 do nothing. A value of 3 means the counter should decrement 1ms, and a value of 1 means the counter must add 1ms. \\[bits 9:8 4002ms event, bits 11:10 4450ms event, bits 13:12 6002ms event, bits 15:14 6450ms event\\]"]
pub struct ERROT_CMP_RTC_0_8_W<'a> {
    w: &'a mut W,
}
impl<'a> ERROT_CMP_RTC_0_8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `ERROT_CMP_RTC_0_4` reader - Every 4s there are 4 sub-events. Each bit pair in this field corresponds to those sub-events. Values of 0 and 2 do nothing. A value of 3 means the counter should decrement 1ms, and a value of 1 means the counter must add 1ms. \\[bits 17:16 2002ms event, bits 19:18 2450ms event, bits 21:20 3002ms event, bits 23:22 3450ms event\\]"]
pub struct ERROT_CMP_RTC_0_4_R(crate::FieldReader<u8, u8>);
impl ERROT_CMP_RTC_0_4_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ERROT_CMP_RTC_0_4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERROT_CMP_RTC_0_4_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERROT_CMP_RTC_0_4` writer - Every 4s there are 4 sub-events. Each bit pair in this field corresponds to those sub-events. Values of 0 and 2 do nothing. A value of 3 means the counter should decrement 1ms, and a value of 1 means the counter must add 1ms. \\[bits 17:16 2002ms event, bits 19:18 2450ms event, bits 21:20 3002ms event, bits 23:22 3450ms event\\]"]
pub struct ERROT_CMP_RTC_0_4_W<'a> {
    w: &'a mut W,
}
impl<'a> ERROT_CMP_RTC_0_4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `ERROT_CMP_RTC_0_2` reader - Every 2s there are 2 sub-events. Each bit pair in this field corresponds to those sub-events. Values of 0 and 2 do nothing. A value of 3 means the counter should decrement 1ms, and a value of 1 means the counter must add 1ms. \\[bits 25:24 1002ms event, bits 27:26 1450ms event\\]"]
pub struct ERROT_CMP_RTC_0_2_R(crate::FieldReader<u8, u8>);
impl ERROT_CMP_RTC_0_2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ERROT_CMP_RTC_0_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERROT_CMP_RTC_0_2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERROT_CMP_RTC_0_2` writer - Every 2s there are 2 sub-events. Each bit pair in this field corresponds to those sub-events. Values of 0 and 2 do nothing. A value of 3 means the counter should decrement 1ms, and a value of 1 means the counter must add 1ms. \\[bits 25:24 1002ms event, bits 27:26 1450ms event\\]"]
pub struct ERROT_CMP_RTC_0_2_W<'a> {
    w: &'a mut W,
}
impl<'a> ERROT_CMP_RTC_0_2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Every 16s there are 4 sub-events. Each bit pair in this field corresponds to those sub-events. Values of 0 and 2 do nothing. A value of 3 means the counter should decrement 1ms, and a value of 1 means the counter must add 1ms. \\[bits 1:0 4002ms event, bits 3:2 4450ms event, bits 5:4 6002ms event, bits 7:6 6450ms event\\]"]
    #[inline(always)]
    pub fn errot_cmp_rtc_0_16(&self) -> ERROT_CMP_RTC_0_16_R {
        ERROT_CMP_RTC_0_16_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Every 8s there are 4 sub-events. Each bit pair in this field corresponds to those sub-events. Values of 0 and 2 do nothing. A value of 3 means the counter should decrement 1ms, and a value of 1 means the counter must add 1ms. \\[bits 9:8 4002ms event, bits 11:10 4450ms event, bits 13:12 6002ms event, bits 15:14 6450ms event\\]"]
    #[inline(always)]
    pub fn errot_cmp_rtc_0_8(&self) -> ERROT_CMP_RTC_0_8_R {
        ERROT_CMP_RTC_0_8_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Every 4s there are 4 sub-events. Each bit pair in this field corresponds to those sub-events. Values of 0 and 2 do nothing. A value of 3 means the counter should decrement 1ms, and a value of 1 means the counter must add 1ms. \\[bits 17:16 2002ms event, bits 19:18 2450ms event, bits 21:20 3002ms event, bits 23:22 3450ms event\\]"]
    #[inline(always)]
    pub fn errot_cmp_rtc_0_4(&self) -> ERROT_CMP_RTC_0_4_R {
        ERROT_CMP_RTC_0_4_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:27 - Every 2s there are 2 sub-events. Each bit pair in this field corresponds to those sub-events. Values of 0 and 2 do nothing. A value of 3 means the counter should decrement 1ms, and a value of 1 means the counter must add 1ms. \\[bits 25:24 1002ms event, bits 27:26 1450ms event\\]"]
    #[inline(always)]
    pub fn errot_cmp_rtc_0_2(&self) -> ERROT_CMP_RTC_0_2_R {
        ERROT_CMP_RTC_0_2_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Every 16s there are 4 sub-events. Each bit pair in this field corresponds to those sub-events. Values of 0 and 2 do nothing. A value of 3 means the counter should decrement 1ms, and a value of 1 means the counter must add 1ms. \\[bits 1:0 4002ms event, bits 3:2 4450ms event, bits 5:4 6002ms event, bits 7:6 6450ms event\\]"]
    #[inline(always)]
    pub fn errot_cmp_rtc_0_16(&mut self) -> ERROT_CMP_RTC_0_16_W {
        ERROT_CMP_RTC_0_16_W { w: self }
    }
    #[doc = "Bits 8:15 - Every 8s there are 4 sub-events. Each bit pair in this field corresponds to those sub-events. Values of 0 and 2 do nothing. A value of 3 means the counter should decrement 1ms, and a value of 1 means the counter must add 1ms. \\[bits 9:8 4002ms event, bits 11:10 4450ms event, bits 13:12 6002ms event, bits 15:14 6450ms event\\]"]
    #[inline(always)]
    pub fn errot_cmp_rtc_0_8(&mut self) -> ERROT_CMP_RTC_0_8_W {
        ERROT_CMP_RTC_0_8_W { w: self }
    }
    #[doc = "Bits 16:23 - Every 4s there are 4 sub-events. Each bit pair in this field corresponds to those sub-events. Values of 0 and 2 do nothing. A value of 3 means the counter should decrement 1ms, and a value of 1 means the counter must add 1ms. \\[bits 17:16 2002ms event, bits 19:18 2450ms event, bits 21:20 3002ms event, bits 23:22 3450ms event\\]"]
    #[inline(always)]
    pub fn errot_cmp_rtc_0_4(&mut self) -> ERROT_CMP_RTC_0_4_W {
        ERROT_CMP_RTC_0_4_W { w: self }
    }
    #[doc = "Bits 24:27 - Every 2s there are 2 sub-events. Each bit pair in this field corresponds to those sub-events. Values of 0 and 2 do nothing. A value of 3 means the counter should decrement 1ms, and a value of 1 means the counter must add 1ms. \\[bits 25:24 1002ms event, bits 27:26 1450ms event\\]"]
    #[inline(always)]
    pub fn errot_cmp_rtc_0_2(&mut self) -> ERROT_CMP_RTC_0_2_W {
        ERROT_CMP_RTC_0_2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "2, 4, 6, 8, 16 sec Increment Error Compensation Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [error_cmp_rtc_0](index.html) module"]
pub struct ERROR_CMP_RTC_0_SPEC;
impl crate::RegisterSpec for ERROR_CMP_RTC_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [error_cmp_rtc_0::R](R) reader structure"]
impl crate::Readable for ERROR_CMP_RTC_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [error_cmp_rtc_0::W](W) writer structure"]
impl crate::Writable for ERROR_CMP_RTC_0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ERROR_CMP_RTC_0 to value 0"]
impl crate::Resettable for ERROR_CMP_RTC_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
