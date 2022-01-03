#[doc = "Register `RTC_CTRL_2` reader"]
pub struct R(crate::R<RTC_CTRL_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_CTRL_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_CTRL_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_CTRL_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_CTRL_2` writer"]
pub struct W(crate::W<RTC_CTRL_2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_CTRL_2_SPEC>;
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
impl From<crate::W<RTC_CTRL_2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_CTRL_2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `clke` reader - 1'b1 RTC Clock Output Enable (No SYNC Needed)"]
pub struct CLKE_R(crate::FieldReader<bool, bool>);
impl CLKE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLKE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clke` writer - 1'b1 RTC Clock Output Enable (No SYNC Needed)"]
pub struct CLKE_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKE_W<'a> {
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
#[doc = "Field `byp16k` reader - Changes internal clock division for 16384 Hz bypass compatibility -- 1'b0 : xtal is 32KHz - 1'b1 : xtal is 16KHz"]
pub struct BYP16K_R(crate::FieldReader<bool, bool>);
impl BYP16K_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BYP16K_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BYP16K_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `byp16k` writer - Changes internal clock division for 16384 Hz bypass compatibility -- 1'b0 : xtal is 32KHz - 1'b1 : xtal is 16KHz"]
pub struct BYP16K_W<'a> {
    w: &'a mut W,
}
impl<'a> BYP16K_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `test_ctrl` reader - RTC test\\[4:3\\]
control for rtc bypass mode -- 0: test\\[4:3\\]
will be forced to 2'b11 when pad8 is strapped to 1, and forced to 2'b00 when pad8 is strapped to 0 , 1: normal mode; test\\[4:0\\]
controlled from 0x1C"]
pub struct TEST_CTRL_R(crate::FieldReader<bool, bool>);
impl TEST_CTRL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TEST_CTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEST_CTRL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `test_ctrl` writer - RTC test\\[4:3\\]
control for rtc bypass mode -- 0: test\\[4:3\\]
will be forced to 2'b11 when pad8 is strapped to 1, and forced to 2'b00 when pad8 is strapped to 0 , 1: normal mode; test\\[4:0\\]
controlled from 0x1C"]
pub struct TEST_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> TEST_CTRL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - 1'b1 RTC Clock Output Enable (No SYNC Needed)"]
    #[inline(always)]
    pub fn clke(&self) -> CLKE_R {
        CLKE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Changes internal clock division for 16384 Hz bypass compatibility -- 1'b0 : xtal is 32KHz - 1'b1 : xtal is 16KHz"]
    #[inline(always)]
    pub fn byp16k(&self) -> BYP16K_R {
        BYP16K_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RTC test\\[4:3\\]
control for rtc bypass mode -- 0: test\\[4:3\\]
will be forced to 2'b11 when pad8 is strapped to 1, and forced to 2'b00 when pad8 is strapped to 0 , 1: normal mode; test\\[4:0\\]
controlled from 0x1C"]
    #[inline(always)]
    pub fn test_ctrl(&self) -> TEST_CTRL_R {
        TEST_CTRL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1'b1 RTC Clock Output Enable (No SYNC Needed)"]
    #[inline(always)]
    pub fn clke(&mut self) -> CLKE_W {
        CLKE_W { w: self }
    }
    #[doc = "Bit 1 - Changes internal clock division for 16384 Hz bypass compatibility -- 1'b0 : xtal is 32KHz - 1'b1 : xtal is 16KHz"]
    #[inline(always)]
    pub fn byp16k(&mut self) -> BYP16K_W {
        BYP16K_W { w: self }
    }
    #[doc = "Bit 2 - RTC test\\[4:3\\]
control for rtc bypass mode -- 0: test\\[4:3\\]
will be forced to 2'b11 when pad8 is strapped to 1, and forced to 2'b00 when pad8 is strapped to 0 , 1: normal mode; test\\[4:0\\]
controlled from 0x1C"]
    #[inline(always)]
    pub fn test_ctrl(&mut self) -> TEST_CTRL_W {
        TEST_CTRL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_ctrl_2](index.html) module"]
pub struct RTC_CTRL_2_SPEC;
impl crate::RegisterSpec for RTC_CTRL_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_ctrl_2::R](R) reader structure"]
impl crate::Readable for RTC_CTRL_2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_ctrl_2::W](W) writer structure"]
impl crate::Writable for RTC_CTRL_2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_CTRL_2 to value 0x01"]
impl crate::Resettable for RTC_CTRL_2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
