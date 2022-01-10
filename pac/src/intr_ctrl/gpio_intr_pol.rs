#[doc = "Register `GPIO_INTR_POL` reader"]
pub struct R(crate::R<GPIO_INTR_POL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_INTR_POL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_INTR_POL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_INTR_POL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIO_INTR_POL` writer"]
pub struct W(crate::W<GPIO_INTR_POL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_INTR_POL_SPEC>;
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
impl From<crate::W<GPIO_INTR_POL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_INTR_POL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "GPIO_0 interrupt polarity (depends on interrupt type): type = level => {0:low, 1:hi} --- type = edge => {0:fall, 1:rise}\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO_0_INTR_POL_A {
    #[doc = "0: Depending on the type, if the interrupt is triggered by level, this indicates the interrupt is triggered by a reading low value, if it is an edge trigger, this value indicates that the interrupt polarity is falling edge"]
    LOW_FALL = 0,
    #[doc = "1: Depending on the type, if the interrupt is triggered by level, this indicates the interrupt is triggered by a reading high value, if it is an edge trigger, this value indicates that the interrupt polarity is rising edge"]
    HIGH_RISE = 1,
}
impl From<GPIO_0_INTR_POL_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO_0_INTR_POL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO_0_INTR_POL` reader - GPIO_0 interrupt polarity (depends on interrupt type): type = level => {0:low, 1:hi} --- type = edge => {0:fall, 1:rise}"]
pub struct GPIO_0_INTR_POL_R(crate::FieldReader<bool, GPIO_0_INTR_POL_A>);
impl GPIO_0_INTR_POL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO_0_INTR_POL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO_0_INTR_POL_A {
        match self.bits {
            false => GPIO_0_INTR_POL_A::LOW_FALL,
            true => GPIO_0_INTR_POL_A::HIGH_RISE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_FALL`"]
    #[inline(always)]
    pub fn is_low_fall(&self) -> bool {
        **self == GPIO_0_INTR_POL_A::LOW_FALL
    }
    #[doc = "Checks if the value of the field is `HIGH_RISE`"]
    #[inline(always)]
    pub fn is_high_rise(&self) -> bool {
        **self == GPIO_0_INTR_POL_A::HIGH_RISE
    }
}
impl core::ops::Deref for GPIO_0_INTR_POL_R {
    type Target = crate::FieldReader<bool, GPIO_0_INTR_POL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO_0_INTR_POL` writer - GPIO_0 interrupt polarity (depends on interrupt type): type = level => {0:low, 1:hi} --- type = edge => {0:fall, 1:rise}"]
pub struct GPIO_0_INTR_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_0_INTR_POL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO_0_INTR_POL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Depending on the type, if the interrupt is triggered by level, this indicates the interrupt is triggered by a reading low value, if it is an edge trigger, this value indicates that the interrupt polarity is falling edge"]
    #[inline(always)]
    pub fn low_fall(self) -> &'a mut W {
        self.variant(GPIO_0_INTR_POL_A::LOW_FALL)
    }
    #[doc = "Depending on the type, if the interrupt is triggered by level, this indicates the interrupt is triggered by a reading high value, if it is an edge trigger, this value indicates that the interrupt polarity is rising edge"]
    #[inline(always)]
    pub fn high_rise(self) -> &'a mut W {
        self.variant(GPIO_0_INTR_POL_A::HIGH_RISE)
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
#[doc = "GPIO_1 interrupt polarity (depends on interrupt type): type = level => {0:low, 1:hi} --- type = edge => {0:fall, 1:rise}"]
pub type GPIO_1_INTR_POL_A = GPIO_0_INTR_POL_A;
#[doc = "Field `GPIO_1_INTR_POL` reader - GPIO_1 interrupt polarity (depends on interrupt type): type = level => {0:low, 1:hi} --- type = edge => {0:fall, 1:rise}"]
pub type GPIO_1_INTR_POL_R = GPIO_0_INTR_POL_R;
#[doc = "Field `GPIO_1_INTR_POL` writer - GPIO_1 interrupt polarity (depends on interrupt type): type = level => {0:low, 1:hi} --- type = edge => {0:fall, 1:rise}"]
pub struct GPIO_1_INTR_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_1_INTR_POL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO_1_INTR_POL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Depending on the type, if the interrupt is triggered by level, this indicates the interrupt is triggered by a reading low value, if it is an edge trigger, this value indicates that the interrupt polarity is falling edge"]
    #[inline(always)]
    pub fn low_fall(self) -> &'a mut W {
        self.variant(GPIO_1_INTR_POL_A::LOW_FALL)
    }
    #[doc = "Depending on the type, if the interrupt is triggered by level, this indicates the interrupt is triggered by a reading high value, if it is an edge trigger, this value indicates that the interrupt polarity is rising edge"]
    #[inline(always)]
    pub fn high_rise(self) -> &'a mut W {
        self.variant(GPIO_1_INTR_POL_A::HIGH_RISE)
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
#[doc = "GPIO_2 interrupt polarity (depends on interrupt type): type = level => {0:low, 1:hi} --- type = edge => {0:fall, 1:rise}"]
pub type GPIO_2_INTR_POL_A = GPIO_0_INTR_POL_A;
#[doc = "Field `GPIO_2_INTR_POL` reader - GPIO_2 interrupt polarity (depends on interrupt type): type = level => {0:low, 1:hi} --- type = edge => {0:fall, 1:rise}"]
pub type GPIO_2_INTR_POL_R = GPIO_0_INTR_POL_R;
#[doc = "Field `GPIO_2_INTR_POL` writer - GPIO_2 interrupt polarity (depends on interrupt type): type = level => {0:low, 1:hi} --- type = edge => {0:fall, 1:rise}"]
pub struct GPIO_2_INTR_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_2_INTR_POL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO_2_INTR_POL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Depending on the type, if the interrupt is triggered by level, this indicates the interrupt is triggered by a reading low value, if it is an edge trigger, this value indicates that the interrupt polarity is falling edge"]
    #[inline(always)]
    pub fn low_fall(self) -> &'a mut W {
        self.variant(GPIO_2_INTR_POL_A::LOW_FALL)
    }
    #[doc = "Depending on the type, if the interrupt is triggered by level, this indicates the interrupt is triggered by a reading high value, if it is an edge trigger, this value indicates that the interrupt polarity is rising edge"]
    #[inline(always)]
    pub fn high_rise(self) -> &'a mut W {
        self.variant(GPIO_2_INTR_POL_A::HIGH_RISE)
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
#[doc = "GPIO_3 interrupt polarity (depends on interrupt type): type = level => {0:low, 1:hi} --- type = edge => {0:fall, 1:rise}"]
pub type GPIO_3_INTR_POL_A = GPIO_0_INTR_POL_A;
#[doc = "Field `GPIO_3_INTR_POL` reader - GPIO_3 interrupt polarity (depends on interrupt type): type = level => {0:low, 1:hi} --- type = edge => {0:fall, 1:rise}"]
pub type GPIO_3_INTR_POL_R = GPIO_0_INTR_POL_R;
#[doc = "Field `GPIO_3_INTR_POL` writer - GPIO_3 interrupt polarity (depends on interrupt type): type = level => {0:low, 1:hi} --- type = edge => {0:fall, 1:rise}"]
pub struct GPIO_3_INTR_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_3_INTR_POL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO_3_INTR_POL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Depending on the type, if the interrupt is triggered by level, this indicates the interrupt is triggered by a reading low value, if it is an edge trigger, this value indicates that the interrupt polarity is falling edge"]
    #[inline(always)]
    pub fn low_fall(self) -> &'a mut W {
        self.variant(GPIO_3_INTR_POL_A::LOW_FALL)
    }
    #[doc = "Depending on the type, if the interrupt is triggered by level, this indicates the interrupt is triggered by a reading high value, if it is an edge trigger, this value indicates that the interrupt polarity is rising edge"]
    #[inline(always)]
    pub fn high_rise(self) -> &'a mut W {
        self.variant(GPIO_3_INTR_POL_A::HIGH_RISE)
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
            (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "GPIO_4 interrupt polarity (depends on interrupt type): type = level => {0:low, 1:hi} --- type = edge => {0:fall, 1:rise}"]
pub type GPIO_4_INTR_POL_A = GPIO_0_INTR_POL_A;
#[doc = "Field `GPIO_4_INTR_POL` reader - GPIO_4 interrupt polarity (depends on interrupt type): type = level => {0:low, 1:hi} --- type = edge => {0:fall, 1:rise}"]
pub type GPIO_4_INTR_POL_R = GPIO_0_INTR_POL_R;
#[doc = "Field `GPIO_4_INTR_POL` writer - GPIO_4 interrupt polarity (depends on interrupt type): type = level => {0:low, 1:hi} --- type = edge => {0:fall, 1:rise}"]
pub struct GPIO_4_INTR_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_4_INTR_POL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO_4_INTR_POL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Depending on the type, if the interrupt is triggered by level, this indicates the interrupt is triggered by a reading low value, if it is an edge trigger, this value indicates that the interrupt polarity is falling edge"]
    #[inline(always)]
    pub fn low_fall(self) -> &'a mut W {
        self.variant(GPIO_4_INTR_POL_A::LOW_FALL)
    }
    #[doc = "Depending on the type, if the interrupt is triggered by level, this indicates the interrupt is triggered by a reading high value, if it is an edge trigger, this value indicates that the interrupt polarity is rising edge"]
    #[inline(always)]
    pub fn high_rise(self) -> &'a mut W {
        self.variant(GPIO_4_INTR_POL_A::HIGH_RISE)
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
            (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "GPIO_5 interrupt polarity (depends on interrupt type): type = level => {0:low, 1:hi} --- type = edge => {0:fall, 1:rise}"]
pub type GPIO_5_INTR_POL_A = GPIO_0_INTR_POL_A;
#[doc = "Field `GPIO_5_INTR_POL` reader - GPIO_5 interrupt polarity (depends on interrupt type): type = level => {0:low, 1:hi} --- type = edge => {0:fall, 1:rise}"]
pub type GPIO_5_INTR_POL_R = GPIO_0_INTR_POL_R;
#[doc = "Field `GPIO_5_INTR_POL` writer - GPIO_5 interrupt polarity (depends on interrupt type): type = level => {0:low, 1:hi} --- type = edge => {0:fall, 1:rise}"]
pub struct GPIO_5_INTR_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_5_INTR_POL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO_5_INTR_POL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Depending on the type, if the interrupt is triggered by level, this indicates the interrupt is triggered by a reading low value, if it is an edge trigger, this value indicates that the interrupt polarity is falling edge"]
    #[inline(always)]
    pub fn low_fall(self) -> &'a mut W {
        self.variant(GPIO_5_INTR_POL_A::LOW_FALL)
    }
    #[doc = "Depending on the type, if the interrupt is triggered by level, this indicates the interrupt is triggered by a reading high value, if it is an edge trigger, this value indicates that the interrupt polarity is rising edge"]
    #[inline(always)]
    pub fn high_rise(self) -> &'a mut W {
        self.variant(GPIO_5_INTR_POL_A::HIGH_RISE)
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
            (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "GPIO_6 interrupt polarity (depends on interrupt type): type = level => {0:low, 1:hi} --- type = edge => {0:fall, 1:rise}"]
pub type GPIO_6_INTR_POL_A = GPIO_0_INTR_POL_A;
#[doc = "Field `GPIO_6_INTR_POL` reader - GPIO_6 interrupt polarity (depends on interrupt type): type = level => {0:low, 1:hi} --- type = edge => {0:fall, 1:rise}"]
pub type GPIO_6_INTR_POL_R = GPIO_0_INTR_POL_R;
#[doc = "Field `GPIO_6_INTR_POL` writer - GPIO_6 interrupt polarity (depends on interrupt type): type = level => {0:low, 1:hi} --- type = edge => {0:fall, 1:rise}"]
pub struct GPIO_6_INTR_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_6_INTR_POL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO_6_INTR_POL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Depending on the type, if the interrupt is triggered by level, this indicates the interrupt is triggered by a reading low value, if it is an edge trigger, this value indicates that the interrupt polarity is falling edge"]
    #[inline(always)]
    pub fn low_fall(self) -> &'a mut W {
        self.variant(GPIO_6_INTR_POL_A::LOW_FALL)
    }
    #[doc = "Depending on the type, if the interrupt is triggered by level, this indicates the interrupt is triggered by a reading high value, if it is an edge trigger, this value indicates that the interrupt polarity is rising edge"]
    #[inline(always)]
    pub fn high_rise(self) -> &'a mut W {
        self.variant(GPIO_6_INTR_POL_A::HIGH_RISE)
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
            (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "GPIO_7 interrupt polarity (depends on interrupt type): type = level => {0:low, 1:hi} --- type = edge => {0:fall, 1:rise}"]
pub type GPIO_7_INTR_POL_A = GPIO_0_INTR_POL_A;
#[doc = "Field `GPIO_7_INTR_POL` reader - GPIO_7 interrupt polarity (depends on interrupt type): type = level => {0:low, 1:hi} --- type = edge => {0:fall, 1:rise}"]
pub type GPIO_7_INTR_POL_R = GPIO_0_INTR_POL_R;
#[doc = "Field `GPIO_7_INTR_POL` writer - GPIO_7 interrupt polarity (depends on interrupt type): type = level => {0:low, 1:hi} --- type = edge => {0:fall, 1:rise}"]
pub struct GPIO_7_INTR_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_7_INTR_POL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO_7_INTR_POL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Depending on the type, if the interrupt is triggered by level, this indicates the interrupt is triggered by a reading low value, if it is an edge trigger, this value indicates that the interrupt polarity is falling edge"]
    #[inline(always)]
    pub fn low_fall(self) -> &'a mut W {
        self.variant(GPIO_7_INTR_POL_A::LOW_FALL)
    }
    #[doc = "Depending on the type, if the interrupt is triggered by level, this indicates the interrupt is triggered by a reading high value, if it is an edge trigger, this value indicates that the interrupt polarity is rising edge"]
    #[inline(always)]
    pub fn high_rise(self) -> &'a mut W {
        self.variant(GPIO_7_INTR_POL_A::HIGH_RISE)
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
            (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - GPIO_0 interrupt polarity (depends on interrupt type): type = level => {0:low, 1:hi} --- type = edge => {0:fall, 1:rise}"]
    #[inline(always)]
    pub fn gpio_0_intr_pol(&self) -> GPIO_0_INTR_POL_R {
        GPIO_0_INTR_POL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - GPIO_1 interrupt polarity (depends on interrupt type): type = level => {0:low, 1:hi} --- type = edge => {0:fall, 1:rise}"]
    #[inline(always)]
    pub fn gpio_1_intr_pol(&self) -> GPIO_1_INTR_POL_R {
        GPIO_1_INTR_POL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - GPIO_2 interrupt polarity (depends on interrupt type): type = level => {0:low, 1:hi} --- type = edge => {0:fall, 1:rise}"]
    #[inline(always)]
    pub fn gpio_2_intr_pol(&self) -> GPIO_2_INTR_POL_R {
        GPIO_2_INTR_POL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - GPIO_3 interrupt polarity (depends on interrupt type): type = level => {0:low, 1:hi} --- type = edge => {0:fall, 1:rise}"]
    #[inline(always)]
    pub fn gpio_3_intr_pol(&self) -> GPIO_3_INTR_POL_R {
        GPIO_3_INTR_POL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - GPIO_4 interrupt polarity (depends on interrupt type): type = level => {0:low, 1:hi} --- type = edge => {0:fall, 1:rise}"]
    #[inline(always)]
    pub fn gpio_4_intr_pol(&self) -> GPIO_4_INTR_POL_R {
        GPIO_4_INTR_POL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - GPIO_5 interrupt polarity (depends on interrupt type): type = level => {0:low, 1:hi} --- type = edge => {0:fall, 1:rise}"]
    #[inline(always)]
    pub fn gpio_5_intr_pol(&self) -> GPIO_5_INTR_POL_R {
        GPIO_5_INTR_POL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - GPIO_6 interrupt polarity (depends on interrupt type): type = level => {0:low, 1:hi} --- type = edge => {0:fall, 1:rise}"]
    #[inline(always)]
    pub fn gpio_6_intr_pol(&self) -> GPIO_6_INTR_POL_R {
        GPIO_6_INTR_POL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - GPIO_7 interrupt polarity (depends on interrupt type): type = level => {0:low, 1:hi} --- type = edge => {0:fall, 1:rise}"]
    #[inline(always)]
    pub fn gpio_7_intr_pol(&self) -> GPIO_7_INTR_POL_R {
        GPIO_7_INTR_POL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIO_0 interrupt polarity (depends on interrupt type): type = level => {0:low, 1:hi} --- type = edge => {0:fall, 1:rise}"]
    #[inline(always)]
    pub fn gpio_0_intr_pol(&mut self) -> GPIO_0_INTR_POL_W {
        GPIO_0_INTR_POL_W { w: self }
    }
    #[doc = "Bit 1 - GPIO_1 interrupt polarity (depends on interrupt type): type = level => {0:low, 1:hi} --- type = edge => {0:fall, 1:rise}"]
    #[inline(always)]
    pub fn gpio_1_intr_pol(&mut self) -> GPIO_1_INTR_POL_W {
        GPIO_1_INTR_POL_W { w: self }
    }
    #[doc = "Bit 2 - GPIO_2 interrupt polarity (depends on interrupt type): type = level => {0:low, 1:hi} --- type = edge => {0:fall, 1:rise}"]
    #[inline(always)]
    pub fn gpio_2_intr_pol(&mut self) -> GPIO_2_INTR_POL_W {
        GPIO_2_INTR_POL_W { w: self }
    }
    #[doc = "Bit 3 - GPIO_3 interrupt polarity (depends on interrupt type): type = level => {0:low, 1:hi} --- type = edge => {0:fall, 1:rise}"]
    #[inline(always)]
    pub fn gpio_3_intr_pol(&mut self) -> GPIO_3_INTR_POL_W {
        GPIO_3_INTR_POL_W { w: self }
    }
    #[doc = "Bit 4 - GPIO_4 interrupt polarity (depends on interrupt type): type = level => {0:low, 1:hi} --- type = edge => {0:fall, 1:rise}"]
    #[inline(always)]
    pub fn gpio_4_intr_pol(&mut self) -> GPIO_4_INTR_POL_W {
        GPIO_4_INTR_POL_W { w: self }
    }
    #[doc = "Bit 5 - GPIO_5 interrupt polarity (depends on interrupt type): type = level => {0:low, 1:hi} --- type = edge => {0:fall, 1:rise}"]
    #[inline(always)]
    pub fn gpio_5_intr_pol(&mut self) -> GPIO_5_INTR_POL_W {
        GPIO_5_INTR_POL_W { w: self }
    }
    #[doc = "Bit 6 - GPIO_6 interrupt polarity (depends on interrupt type): type = level => {0:low, 1:hi} --- type = edge => {0:fall, 1:rise}"]
    #[inline(always)]
    pub fn gpio_6_intr_pol(&mut self) -> GPIO_6_INTR_POL_W {
        GPIO_6_INTR_POL_W { w: self }
    }
    #[doc = "Bit 7 - GPIO_7 interrupt polarity (depends on interrupt type): type = level => {0:low, 1:hi} --- type = edge => {0:fall, 1:rise}"]
    #[inline(always)]
    pub fn gpio_7_intr_pol(&mut self) -> GPIO_7_INTR_POL_W {
        GPIO_7_INTR_POL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Indicators of interrupt trigger polarities (will depend on the type of interrupt)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_intr_pol](index.html) module"]
pub struct GPIO_INTR_POL_SPEC;
impl crate::RegisterSpec for GPIO_INTR_POL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_intr_pol::R](R) reader structure"]
impl crate::Readable for GPIO_INTR_POL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_intr_pol::W](W) writer structure"]
impl crate::Writable for GPIO_INTR_POL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPIO_INTR_POL to value 0"]
impl crate::Resettable for GPIO_INTR_POL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
