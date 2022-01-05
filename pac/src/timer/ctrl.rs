#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - Enable"]
pub struct ENABLE_R(crate::FieldReader<bool, bool>);
impl ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENABLE` writer - Enable"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
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
#[doc = "Field `SEL_EXTINT_AS_ENABLE` reader - Select external input as enable"]
pub struct SEL_EXTINT_AS_ENABLE_R(crate::FieldReader<bool, bool>);
impl SEL_EXTINT_AS_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SEL_EXTINT_AS_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEL_EXTINT_AS_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEL_EXTINT_AS_ENABLE` writer - Select external input as enable"]
pub struct SEL_EXTINT_AS_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL_EXTINT_AS_ENABLE_W<'a> {
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
        self.w.bits =
            (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `SEL_EXINT_AS_CLOCK` reader - Select external input as clock"]
pub struct SEL_EXINT_AS_CLOCK_R(crate::FieldReader<bool, bool>);
impl SEL_EXINT_AS_CLOCK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SEL_EXINT_AS_CLOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEL_EXINT_AS_CLOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEL_EXINT_AS_CLOCK` writer - Select external input as clock"]
pub struct SEL_EXINT_AS_CLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL_EXINT_AS_CLOCK_W<'a> {
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
        self.w.bits =
            (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `TIMER_INT_ENABLE` reader - Timer interrupt enable"]
pub struct TIMER_INT_ENABLE_R(crate::FieldReader<bool, bool>);
impl TIMER_INT_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMER_INT_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER_INT_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER_INT_ENABLE` writer - Timer interrupt enable"]
pub struct TIMER_INT_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_INT_ENABLE_W<'a> {
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
        self.w.bits =
            (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Select external input as enable"]
    #[inline(always)]
    pub fn sel_extint_as_enable(&self) -> SEL_EXTINT_AS_ENABLE_R {
        SEL_EXTINT_AS_ENABLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Select external input as clock"]
    #[inline(always)]
    pub fn sel_exint_as_clock(&self) -> SEL_EXINT_AS_CLOCK_R {
        SEL_EXINT_AS_CLOCK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Timer interrupt enable"]
    #[inline(always)]
    pub fn timer_int_enable(&self) -> TIMER_INT_ENABLE_R {
        TIMER_INT_ENABLE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Bit 1 - Select external input as enable"]
    #[inline(always)]
    pub fn sel_extint_as_enable(&mut self) -> SEL_EXTINT_AS_ENABLE_W {
        SEL_EXTINT_AS_ENABLE_W { w: self }
    }
    #[doc = "Bit 2 - Select external input as clock"]
    #[inline(always)]
    pub fn sel_exint_as_clock(&mut self) -> SEL_EXINT_AS_CLOCK_W {
        SEL_EXINT_AS_CLOCK_W { w: self }
    }
    #[doc = "Bit 3 - Timer interrupt enable"]
    #[inline(always)]
    pub fn timer_int_enable(&mut self) -> TIMER_INT_ENABLE_W {
        TIMER_INT_ENABLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
