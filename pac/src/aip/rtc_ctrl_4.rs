#[doc = "Register `RTC_CTRL_4` reader"]
pub struct R(crate::R<RTC_CTRL_4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_CTRL_4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_CTRL_4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_CTRL_4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_CTRL_4` writer"]
pub struct W(crate::W<RTC_CTRL_4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_CTRL_4_SPEC>;
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
impl From<crate::W<RTC_CTRL_4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_CTRL_4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `wr` reader - Write Pulse to program the RTC internal Register. Please refer to the Technical Reference Manual for detail"]
pub struct WR_R(crate::FieldReader<bool, bool>);
impl WR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `wr` writer - Write Pulse to program the RTC internal Register. Please refer to the Technical Reference Manual for detail"]
pub struct WR_W<'a> {
    w: &'a mut W,
}
impl<'a> WR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Write Pulse to program the RTC internal Register. Please refer to the Technical Reference Manual for detail"]
    #[inline(always)]
    pub fn wr(&self) -> WR_R {
        WR_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write Pulse to program the RTC internal Register. Please refer to the Technical Reference Manual for detail"]
    #[inline(always)]
    pub fn wr(&mut self) -> WR_W {
        WR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Control register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_ctrl_4](index.html) module"]
pub struct RTC_CTRL_4_SPEC;
impl crate::RegisterSpec for RTC_CTRL_4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_ctrl_4::R](R) reader structure"]
impl crate::Readable for RTC_CTRL_4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_ctrl_4::W](W) writer structure"]
impl crate::Writable for RTC_CTRL_4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_CTRL_4 to value 0"]
impl crate::Resettable for RTC_CTRL_4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
