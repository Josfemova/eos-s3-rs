#[doc = "Register `RTC_CTRL_7` reader"]
pub struct R(crate::R<RTC_CTRL_7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_CTRL_7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_CTRL_7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_CTRL_7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_CTRL_7` writer"]
pub struct W(crate::W<RTC_CTRL_7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_CTRL_7_SPEC>;
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
impl From<crate::W<RTC_CTRL_7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_CTRL_7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `test` reader - Please refer to the Technical Reference Manual for detail"]
pub struct TEST_R(crate::FieldReader<u8, u8>);
impl TEST_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TEST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEST_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `test` writer - Please refer to the Technical Reference Manual for detail"]
pub struct TEST_W<'a> {
    w: &'a mut W,
}
impl<'a> TEST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Please refer to the Technical Reference Manual for detail"]
    #[inline(always)]
    pub fn test(&self) -> TEST_R {
        TEST_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Please refer to the Technical Reference Manual for detail"]
    #[inline(always)]
    pub fn test(&mut self) -> TEST_W {
        TEST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC control register 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_ctrl_7](index.html) module"]
pub struct RTC_CTRL_7_SPEC;
impl crate::RegisterSpec for RTC_CTRL_7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_ctrl_7::R](R) reader structure"]
impl crate::Readable for RTC_CTRL_7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_ctrl_7::W](W) writer structure"]
impl crate::Writable for RTC_CTRL_7_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_CTRL_7 to value 0"]
impl crate::Resettable for RTC_CTRL_7_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
