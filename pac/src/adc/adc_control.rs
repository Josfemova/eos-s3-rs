#[doc = "Register `ADC_Control` reader"]
pub struct R(crate::R<ADC_CONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_CONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_CONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_CONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_Control` writer"]
pub struct W(crate::W<ADC_CONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_CONTROL_SPEC>;
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
impl From<crate::W<ADC_CONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_CONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Asynchronous start-of-conversion. Needs to rise and be held high for each conversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOC_A {
    #[doc = "0: Stop conversion"]
    STOP = 0,
    #[doc = "1: Start conversion. Needs to be held high until EOC is valid"]
    START = 1,
}
impl From<SOC_A> for bool {
    #[inline(always)]
    fn from(variant: SOC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOC` reader - Asynchronous start-of-conversion. Needs to rise and be held high for each conversion"]
pub struct SOC_R(crate::FieldReader<bool, SOC_A>);
impl SOC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SOC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOC_A {
        match self.bits {
            false => SOC_A::STOP,
            true => SOC_A::START,
        }
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        **self == SOC_A::STOP
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        **self == SOC_A::START
    }
}
impl core::ops::Deref for SOC_R {
    type Target = crate::FieldReader<bool, SOC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOC` writer - Asynchronous start-of-conversion. Needs to rise and be held high for each conversion"]
pub struct SOC_W<'a> {
    w: &'a mut W,
}
impl<'a> SOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SOC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Stop conversion"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(SOC_A::STOP)
    }
    #[doc = "Start conversion. Needs to be held high until EOC is valid"]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(SOC_A::START)
    }
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
#[doc = "Channel Selection. 0 = CH0, 1 = CH1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEL_A {
    #[doc = "0: Select channel 0"]
    SELECT_CH0 = 0,
    #[doc = "1: Select channel 1"]
    SELECT_CH1 = 1,
}
impl From<SEL_A> for bool {
    #[inline(always)]
    fn from(variant: SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEL` reader - Channel Selection. 0 = CH0, 1 = CH1"]
pub struct SEL_R(crate::FieldReader<bool, SEL_A>);
impl SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEL_A {
        match self.bits {
            false => SEL_A::SELECT_CH0,
            true => SEL_A::SELECT_CH1,
        }
    }
    #[doc = "Checks if the value of the field is `SELECT_CH0`"]
    #[inline(always)]
    pub fn is_select_ch0(&self) -> bool {
        **self == SEL_A::SELECT_CH0
    }
    #[doc = "Checks if the value of the field is `SELECT_CH1`"]
    #[inline(always)]
    pub fn is_select_ch1(&self) -> bool {
        **self == SEL_A::SELECT_CH1
    }
}
impl core::ops::Deref for SEL_R {
    type Target = crate::FieldReader<bool, SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEL` writer - Channel Selection. 0 = CH0, 1 = CH1"]
pub struct SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Select channel 0"]
    #[inline(always)]
    pub fn select_ch0(self) -> &'a mut W {
        self.variant(SEL_A::SELECT_CH0)
    }
    #[doc = "Select channel 1"]
    #[inline(always)]
    pub fn select_ch1(self) -> &'a mut W {
        self.variant(SEL_A::SELECT_CH1)
    }
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
#[doc = "Battery measurement enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BAT_A {
    #[doc = "0: Disables battery measurement"]
    DISABLE = 0,
    #[doc = "1: Enables battery measurement"]
    ENABLE = 1,
}
impl From<BAT_A> for bool {
    #[inline(always)]
    fn from(variant: BAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BAT` reader - Battery measurement enable"]
pub struct BAT_R(crate::FieldReader<bool, BAT_A>);
impl BAT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BAT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BAT_A {
        match self.bits {
            false => BAT_A::DISABLE,
            true => BAT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == BAT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == BAT_A::ENABLE
    }
}
impl core::ops::Deref for BAT_R {
    type Target = crate::FieldReader<bool, BAT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BAT` writer - Battery measurement enable"]
pub struct BAT_W<'a> {
    w: &'a mut W,
}
impl<'a> BAT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BAT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disables battery measurement"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(BAT_A::DISABLE)
    }
    #[doc = "Enables battery measurement"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(BAT_A::ENABLE)
    }
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
impl R {
    #[doc = "Bit 0 - Asynchronous start-of-conversion. Needs to rise and be held high for each conversion"]
    #[inline(always)]
    pub fn soc(&self) -> SOC_R {
        SOC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel Selection. 0 = CH0, 1 = CH1"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Battery measurement enable"]
    #[inline(always)]
    pub fn bat(&self) -> BAT_R {
        BAT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Asynchronous start-of-conversion. Needs to rise and be held high for each conversion"]
    #[inline(always)]
    pub fn soc(&mut self) -> SOC_W {
        SOC_W { w: self }
    }
    #[doc = "Bit 1 - Channel Selection. 0 = CH0, 1 = CH1"]
    #[inline(always)]
    pub fn sel(&mut self) -> SEL_W {
        SEL_W { w: self }
    }
    #[doc = "Bit 2 - Battery measurement enable"]
    #[inline(always)]
    pub fn bat(&mut self) -> BAT_W {
        BAT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_control](index.html) module"]
pub struct ADC_CONTROL_SPEC;
impl crate::RegisterSpec for ADC_CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_control::R](R) reader structure"]
impl crate::Readable for ADC_CONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_control::W](W) writer structure"]
impl crate::Writable for ADC_CONTROL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADC_Control to value 0"]
impl crate::Resettable for ADC_CONTROL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
