#[doc = "Register `GPIO_INTR_TYPE` reader"]
pub struct R(crate::R<GPIO_INTR_TYPE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_INTR_TYPE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_INTR_TYPE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_INTR_TYPE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIO_INTR_TYPE` writer"]
pub struct W(crate::W<GPIO_INTR_TYPE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_INTR_TYPE_SPEC>;
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
impl From<crate::W<GPIO_INTR_TYPE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_INTR_TYPE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "GPIO_0 interrupt type {0: level, 1: edge}\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO_O_INTR_TYPE_A {
    #[doc = "0: Interrupt is triggered by a certain level (High or Low)"]
    LEVEL = 0,
    #[doc = "1: Interrupt is triggered by a signal edge (Falling or Rising)"]
    EDGE = 1,
}
impl From<GPIO_O_INTR_TYPE_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO_O_INTR_TYPE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO_O_INTR_TYPE` reader - GPIO_0 interrupt type {0: level, 1: edge}"]
pub struct GPIO_O_INTR_TYPE_R(crate::FieldReader<bool, GPIO_O_INTR_TYPE_A>);
impl GPIO_O_INTR_TYPE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO_O_INTR_TYPE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO_O_INTR_TYPE_A {
        match self.bits {
            false => GPIO_O_INTR_TYPE_A::LEVEL,
            true => GPIO_O_INTR_TYPE_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        **self == GPIO_O_INTR_TYPE_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        **self == GPIO_O_INTR_TYPE_A::EDGE
    }
}
impl core::ops::Deref for GPIO_O_INTR_TYPE_R {
    type Target = crate::FieldReader<bool, GPIO_O_INTR_TYPE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO_O_INTR_TYPE` writer - GPIO_0 interrupt type {0: level, 1: edge}"]
pub struct GPIO_O_INTR_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_O_INTR_TYPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO_O_INTR_TYPE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt is triggered by a certain level (High or Low)"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(GPIO_O_INTR_TYPE_A::LEVEL)
    }
    #[doc = "Interrupt is triggered by a signal edge (Falling or Rising)"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(GPIO_O_INTR_TYPE_A::EDGE)
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
#[doc = "GPIO_1 interrupt type {0: level, 1: edge}"]
pub type GPIO_1_INTR_TYPE_A = GPIO_O_INTR_TYPE_A;
#[doc = "Field `GPIO_1_INTR_TYPE` reader - GPIO_1 interrupt type {0: level, 1: edge}"]
pub type GPIO_1_INTR_TYPE_R = GPIO_O_INTR_TYPE_R;
#[doc = "Field `GPIO_1_INTR_TYPE` writer - GPIO_1 interrupt type {0: level, 1: edge}"]
pub struct GPIO_1_INTR_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_1_INTR_TYPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO_1_INTR_TYPE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt is triggered by a certain level (High or Low)"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(GPIO_1_INTR_TYPE_A::LEVEL)
    }
    #[doc = "Interrupt is triggered by a signal edge (Falling or Rising)"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(GPIO_1_INTR_TYPE_A::EDGE)
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
#[doc = "GPIO_2 interrupt type {0: level, 1: edge}"]
pub type GPIO_2_INTR_TYPE_A = GPIO_O_INTR_TYPE_A;
#[doc = "Field `GPIO_2_INTR_TYPE` reader - GPIO_2 interrupt type {0: level, 1: edge}"]
pub type GPIO_2_INTR_TYPE_R = GPIO_O_INTR_TYPE_R;
#[doc = "Field `GPIO_2_INTR_TYPE` writer - GPIO_2 interrupt type {0: level, 1: edge}"]
pub struct GPIO_2_INTR_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_2_INTR_TYPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO_2_INTR_TYPE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt is triggered by a certain level (High or Low)"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(GPIO_2_INTR_TYPE_A::LEVEL)
    }
    #[doc = "Interrupt is triggered by a signal edge (Falling or Rising)"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(GPIO_2_INTR_TYPE_A::EDGE)
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
#[doc = "GPIO_3 interrupt type {0: level, 1: edge}"]
pub type GPIO_3_INTR_TYPE_A = GPIO_O_INTR_TYPE_A;
#[doc = "Field `GPIO_3_INTR_TYPE` reader - GPIO_3 interrupt type {0: level, 1: edge}"]
pub type GPIO_3_INTR_TYPE_R = GPIO_O_INTR_TYPE_R;
#[doc = "Field `GPIO_3_INTR_TYPE` writer - GPIO_3 interrupt type {0: level, 1: edge}"]
pub struct GPIO_3_INTR_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_3_INTR_TYPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO_3_INTR_TYPE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt is triggered by a certain level (High or Low)"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(GPIO_3_INTR_TYPE_A::LEVEL)
    }
    #[doc = "Interrupt is triggered by a signal edge (Falling or Rising)"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(GPIO_3_INTR_TYPE_A::EDGE)
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
#[doc = "GPIO_4 interrupt type {0: level, 1: edge}"]
pub type GPIO_4_INTR_TYPE_A = GPIO_O_INTR_TYPE_A;
#[doc = "Field `GPIO_4_INTR_TYPE` reader - GPIO_4 interrupt type {0: level, 1: edge}"]
pub type GPIO_4_INTR_TYPE_R = GPIO_O_INTR_TYPE_R;
#[doc = "Field `GPIO_4_INTR_TYPE` writer - GPIO_4 interrupt type {0: level, 1: edge}"]
pub struct GPIO_4_INTR_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_4_INTR_TYPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO_4_INTR_TYPE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt is triggered by a certain level (High or Low)"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(GPIO_4_INTR_TYPE_A::LEVEL)
    }
    #[doc = "Interrupt is triggered by a signal edge (Falling or Rising)"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(GPIO_4_INTR_TYPE_A::EDGE)
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
#[doc = "GPIO_5 interrupt type {0: level, 1: edge}"]
pub type GPIO_5_INTR_TYPE_A = GPIO_O_INTR_TYPE_A;
#[doc = "Field `GPIO_5_INTR_TYPE` reader - GPIO_5 interrupt type {0: level, 1: edge}"]
pub type GPIO_5_INTR_TYPE_R = GPIO_O_INTR_TYPE_R;
#[doc = "Field `GPIO_5_INTR_TYPE` writer - GPIO_5 interrupt type {0: level, 1: edge}"]
pub struct GPIO_5_INTR_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_5_INTR_TYPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO_5_INTR_TYPE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt is triggered by a certain level (High or Low)"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(GPIO_5_INTR_TYPE_A::LEVEL)
    }
    #[doc = "Interrupt is triggered by a signal edge (Falling or Rising)"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(GPIO_5_INTR_TYPE_A::EDGE)
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
#[doc = "GPIO_6 interrupt type {0: level, 1: edge}"]
pub type GPIO_6_INTR_TYPE_A = GPIO_O_INTR_TYPE_A;
#[doc = "Field `GPIO_6_INTR_TYPE` reader - GPIO_6 interrupt type {0: level, 1: edge}"]
pub type GPIO_6_INTR_TYPE_R = GPIO_O_INTR_TYPE_R;
#[doc = "Field `GPIO_6_INTR_TYPE` writer - GPIO_6 interrupt type {0: level, 1: edge}"]
pub struct GPIO_6_INTR_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_6_INTR_TYPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO_6_INTR_TYPE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt is triggered by a certain level (High or Low)"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(GPIO_6_INTR_TYPE_A::LEVEL)
    }
    #[doc = "Interrupt is triggered by a signal edge (Falling or Rising)"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(GPIO_6_INTR_TYPE_A::EDGE)
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
#[doc = "GPIO_7 interrupt type {0: level, 1: edge}"]
pub type GPIO_7_INTR_TYPE_A = GPIO_O_INTR_TYPE_A;
#[doc = "Field `GPIO_7_INTR_TYPE` reader - GPIO_7 interrupt type {0: level, 1: edge}"]
pub type GPIO_7_INTR_TYPE_R = GPIO_O_INTR_TYPE_R;
#[doc = "Field `GPIO_7_INTR_TYPE` writer - GPIO_7 interrupt type {0: level, 1: edge}"]
pub struct GPIO_7_INTR_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_7_INTR_TYPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO_7_INTR_TYPE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt is triggered by a certain level (High or Low)"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(GPIO_7_INTR_TYPE_A::LEVEL)
    }
    #[doc = "Interrupt is triggered by a signal edge (Falling or Rising)"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(GPIO_7_INTR_TYPE_A::EDGE)
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
    #[doc = "Bit 0 - GPIO_0 interrupt type {0: level, 1: edge}"]
    #[inline(always)]
    pub fn gpio_o_intr_type(&self) -> GPIO_O_INTR_TYPE_R {
        GPIO_O_INTR_TYPE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - GPIO_1 interrupt type {0: level, 1: edge}"]
    #[inline(always)]
    pub fn gpio_1_intr_type(&self) -> GPIO_1_INTR_TYPE_R {
        GPIO_1_INTR_TYPE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - GPIO_2 interrupt type {0: level, 1: edge}"]
    #[inline(always)]
    pub fn gpio_2_intr_type(&self) -> GPIO_2_INTR_TYPE_R {
        GPIO_2_INTR_TYPE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - GPIO_3 interrupt type {0: level, 1: edge}"]
    #[inline(always)]
    pub fn gpio_3_intr_type(&self) -> GPIO_3_INTR_TYPE_R {
        GPIO_3_INTR_TYPE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - GPIO_4 interrupt type {0: level, 1: edge}"]
    #[inline(always)]
    pub fn gpio_4_intr_type(&self) -> GPIO_4_INTR_TYPE_R {
        GPIO_4_INTR_TYPE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - GPIO_5 interrupt type {0: level, 1: edge}"]
    #[inline(always)]
    pub fn gpio_5_intr_type(&self) -> GPIO_5_INTR_TYPE_R {
        GPIO_5_INTR_TYPE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - GPIO_6 interrupt type {0: level, 1: edge}"]
    #[inline(always)]
    pub fn gpio_6_intr_type(&self) -> GPIO_6_INTR_TYPE_R {
        GPIO_6_INTR_TYPE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - GPIO_7 interrupt type {0: level, 1: edge}"]
    #[inline(always)]
    pub fn gpio_7_intr_type(&self) -> GPIO_7_INTR_TYPE_R {
        GPIO_7_INTR_TYPE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIO_0 interrupt type {0: level, 1: edge}"]
    #[inline(always)]
    pub fn gpio_o_intr_type(&mut self) -> GPIO_O_INTR_TYPE_W {
        GPIO_O_INTR_TYPE_W { w: self }
    }
    #[doc = "Bit 1 - GPIO_1 interrupt type {0: level, 1: edge}"]
    #[inline(always)]
    pub fn gpio_1_intr_type(&mut self) -> GPIO_1_INTR_TYPE_W {
        GPIO_1_INTR_TYPE_W { w: self }
    }
    #[doc = "Bit 2 - GPIO_2 interrupt type {0: level, 1: edge}"]
    #[inline(always)]
    pub fn gpio_2_intr_type(&mut self) -> GPIO_2_INTR_TYPE_W {
        GPIO_2_INTR_TYPE_W { w: self }
    }
    #[doc = "Bit 3 - GPIO_3 interrupt type {0: level, 1: edge}"]
    #[inline(always)]
    pub fn gpio_3_intr_type(&mut self) -> GPIO_3_INTR_TYPE_W {
        GPIO_3_INTR_TYPE_W { w: self }
    }
    #[doc = "Bit 4 - GPIO_4 interrupt type {0: level, 1: edge}"]
    #[inline(always)]
    pub fn gpio_4_intr_type(&mut self) -> GPIO_4_INTR_TYPE_W {
        GPIO_4_INTR_TYPE_W { w: self }
    }
    #[doc = "Bit 5 - GPIO_5 interrupt type {0: level, 1: edge}"]
    #[inline(always)]
    pub fn gpio_5_intr_type(&mut self) -> GPIO_5_INTR_TYPE_W {
        GPIO_5_INTR_TYPE_W { w: self }
    }
    #[doc = "Bit 6 - GPIO_6 interrupt type {0: level, 1: edge}"]
    #[inline(always)]
    pub fn gpio_6_intr_type(&mut self) -> GPIO_6_INTR_TYPE_W {
        GPIO_6_INTR_TYPE_W { w: self }
    }
    #[doc = "Bit 7 - GPIO_7 interrupt type {0: level, 1: edge}"]
    #[inline(always)]
    pub fn gpio_7_intr_type(&mut self) -> GPIO_7_INTR_TYPE_W {
        GPIO_7_INTR_TYPE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Indicators of interrupt trigger types\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_intr_type](index.html) module"]
pub struct GPIO_INTR_TYPE_SPEC;
impl crate::RegisterSpec for GPIO_INTR_TYPE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_intr_type::R](R) reader structure"]
impl crate::Readable for GPIO_INTR_TYPE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_intr_type::W](W) writer structure"]
impl crate::Writable for GPIO_INTR_TYPE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPIO_INTR_TYPE to value 0"]
impl crate::Resettable for GPIO_INTR_TYPE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
