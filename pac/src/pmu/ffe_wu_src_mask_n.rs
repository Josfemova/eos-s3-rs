#[doc = "Register `FFE_WU_SRC_MASK_N` reader"]
pub struct R(crate::R<FFE_WU_SRC_MASK_N_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FFE_WU_SRC_MASK_N_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FFE_WU_SRC_MASK_N_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FFE_WU_SRC_MASK_N_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FFE_WU_SRC_MASK_N` writer"]
pub struct W(crate::W<FFE_WU_SRC_MASK_N_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FFE_WU_SRC_MASK_N_SPEC>;
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
impl From<crate::W<FFE_WU_SRC_MASK_N_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FFE_WU_SRC_MASK_N_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "If unmasked, kick off timer time-out event will wake up FFE.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KICKOFF_TIMER_TIME_OUT_A {
    #[doc = "0: Mask the interrupt as a wake-up event source"]
    MASK = 0,
    #[doc = "1: Unmask the interrupt as a wake-up event source"]
    UNMASK = 1,
}
impl From<KICKOFF_TIMER_TIME_OUT_A> for bool {
    #[inline(always)]
    fn from(variant: KICKOFF_TIMER_TIME_OUT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `KickOff_Timer_Time_Out` reader - If unmasked, kick off timer time-out event will wake up FFE."]
pub struct KICKOFF_TIMER_TIME_OUT_R(
    crate::FieldReader<bool, KICKOFF_TIMER_TIME_OUT_A>,
);
impl KICKOFF_TIMER_TIME_OUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        KICKOFF_TIMER_TIME_OUT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> KICKOFF_TIMER_TIME_OUT_A {
        match self.bits {
            false => KICKOFF_TIMER_TIME_OUT_A::MASK,
            true => KICKOFF_TIMER_TIME_OUT_A::UNMASK,
        }
    }
    #[doc = "Checks if the value of the field is `MASK`"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        **self == KICKOFF_TIMER_TIME_OUT_A::MASK
    }
    #[doc = "Checks if the value of the field is `UNMASK`"]
    #[inline(always)]
    pub fn is_unmask(&self) -> bool {
        **self == KICKOFF_TIMER_TIME_OUT_A::UNMASK
    }
}
impl core::ops::Deref for KICKOFF_TIMER_TIME_OUT_R {
    type Target = crate::FieldReader<bool, KICKOFF_TIMER_TIME_OUT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KickOff_Timer_Time_Out` writer - If unmasked, kick off timer time-out event will wake up FFE."]
pub struct KICKOFF_TIMER_TIME_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> KICKOFF_TIMER_TIME_OUT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: KICKOFF_TIMER_TIME_OUT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Mask the interrupt as a wake-up event source"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(KICKOFF_TIMER_TIME_OUT_A::MASK)
    }
    #[doc = "Unmask the interrupt as a wake-up event source"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut W {
        self.variant(KICKOFF_TIMER_TIME_OUT_A::UNMASK)
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
#[doc = "If unmasked, GPIO0 interrupt will wake up FFE."]
pub type SENSOR_GPIO_0_INT_A = KICKOFF_TIMER_TIME_OUT_A;
#[doc = "Field `Sensor_GPIO_0_INT` reader - If unmasked, GPIO0 interrupt will wake up FFE."]
pub type SENSOR_GPIO_0_INT_R = KICKOFF_TIMER_TIME_OUT_R;
#[doc = "Field `Sensor_GPIO_0_INT` writer - If unmasked, GPIO0 interrupt will wake up FFE."]
pub struct SENSOR_GPIO_0_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSOR_GPIO_0_INT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SENSOR_GPIO_0_INT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Mask the interrupt as a wake-up event source"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(SENSOR_GPIO_0_INT_A::MASK)
    }
    #[doc = "Unmask the interrupt as a wake-up event source"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut W {
        self.variant(SENSOR_GPIO_0_INT_A::UNMASK)
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
#[doc = "If unmasked, GPIO1 interrupt will wake up FFE."]
pub type SENSOR_GPIO_1_INT_A = KICKOFF_TIMER_TIME_OUT_A;
#[doc = "Field `Sensor_GPIO_1_INT` reader - If unmasked, GPIO1 interrupt will wake up FFE."]
pub type SENSOR_GPIO_1_INT_R = KICKOFF_TIMER_TIME_OUT_R;
#[doc = "Field `Sensor_GPIO_1_INT` writer - If unmasked, GPIO1 interrupt will wake up FFE."]
pub struct SENSOR_GPIO_1_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSOR_GPIO_1_INT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SENSOR_GPIO_1_INT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Mask the interrupt as a wake-up event source"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(SENSOR_GPIO_1_INT_A::MASK)
    }
    #[doc = "Unmask the interrupt as a wake-up event source"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut W {
        self.variant(SENSOR_GPIO_1_INT_A::UNMASK)
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
#[doc = "If unmasked, GPIO2 interrupt will wake up FFE."]
pub type SENSOR_GPIO_2_INT_A = KICKOFF_TIMER_TIME_OUT_A;
#[doc = "Field `Sensor_GPIO_2_INT` reader - If unmasked, GPIO2 interrupt will wake up FFE."]
pub type SENSOR_GPIO_2_INT_R = KICKOFF_TIMER_TIME_OUT_R;
#[doc = "Field `Sensor_GPIO_2_INT` writer - If unmasked, GPIO2 interrupt will wake up FFE."]
pub struct SENSOR_GPIO_2_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSOR_GPIO_2_INT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SENSOR_GPIO_2_INT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Mask the interrupt as a wake-up event source"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(SENSOR_GPIO_2_INT_A::MASK)
    }
    #[doc = "Unmask the interrupt as a wake-up event source"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut W {
        self.variant(SENSOR_GPIO_2_INT_A::UNMASK)
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
#[doc = "If unmasked, GPIO3 interrupt will wake up FFE."]
pub type SENSOR_GPIO_3_INT_A = KICKOFF_TIMER_TIME_OUT_A;
#[doc = "Field `Sensor_GPIO_3_INT` reader - If unmasked, GPIO3 interrupt will wake up FFE."]
pub type SENSOR_GPIO_3_INT_R = KICKOFF_TIMER_TIME_OUT_R;
#[doc = "Field `Sensor_GPIO_3_INT` writer - If unmasked, GPIO3 interrupt will wake up FFE."]
pub struct SENSOR_GPIO_3_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSOR_GPIO_3_INT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SENSOR_GPIO_3_INT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Mask the interrupt as a wake-up event source"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(SENSOR_GPIO_3_INT_A::MASK)
    }
    #[doc = "Unmask the interrupt as a wake-up event source"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut W {
        self.variant(SENSOR_GPIO_3_INT_A::UNMASK)
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
#[doc = "If unmasked, GPIO4 interrupt will wake up FFE."]
pub type SENSOR_GPIO_4_INT_A = KICKOFF_TIMER_TIME_OUT_A;
#[doc = "Field `Sensor_GPIO_4_INT` reader - If unmasked, GPIO4 interrupt will wake up FFE."]
pub type SENSOR_GPIO_4_INT_R = KICKOFF_TIMER_TIME_OUT_R;
#[doc = "Field `Sensor_GPIO_4_INT` writer - If unmasked, GPIO4 interrupt will wake up FFE."]
pub struct SENSOR_GPIO_4_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSOR_GPIO_4_INT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SENSOR_GPIO_4_INT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Mask the interrupt as a wake-up event source"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(SENSOR_GPIO_4_INT_A::MASK)
    }
    #[doc = "Unmask the interrupt as a wake-up event source"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut W {
        self.variant(SENSOR_GPIO_4_INT_A::UNMASK)
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
#[doc = "If unmasked, GPIO5 interrupt will wake up FFE."]
pub type SENSOR_GPIO_5_INT_A = KICKOFF_TIMER_TIME_OUT_A;
#[doc = "Field `Sensor_GPIO_5_INT` reader - If unmasked, GPIO5 interrupt will wake up FFE."]
pub type SENSOR_GPIO_5_INT_R = KICKOFF_TIMER_TIME_OUT_R;
#[doc = "Field `Sensor_GPIO_5_INT` writer - If unmasked, GPIO5 interrupt will wake up FFE."]
pub struct SENSOR_GPIO_5_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSOR_GPIO_5_INT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SENSOR_GPIO_5_INT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Mask the interrupt as a wake-up event source"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(SENSOR_GPIO_5_INT_A::MASK)
    }
    #[doc = "Unmask the interrupt as a wake-up event source"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut W {
        self.variant(SENSOR_GPIO_5_INT_A::UNMASK)
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
#[doc = "If unmasked, GPIO6 interrupt will wake up FFE."]
pub type SENSOR_GPIO_6_INT_A = KICKOFF_TIMER_TIME_OUT_A;
#[doc = "Field `Sensor_GPIO_6_INT` reader - If unmasked, GPIO6 interrupt will wake up FFE."]
pub type SENSOR_GPIO_6_INT_R = KICKOFF_TIMER_TIME_OUT_R;
#[doc = "Field `Sensor_GPIO_6_INT` writer - If unmasked, GPIO6 interrupt will wake up FFE."]
pub struct SENSOR_GPIO_6_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSOR_GPIO_6_INT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SENSOR_GPIO_6_INT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Mask the interrupt as a wake-up event source"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(SENSOR_GPIO_6_INT_A::MASK)
    }
    #[doc = "Unmask the interrupt as a wake-up event source"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut W {
        self.variant(SENSOR_GPIO_6_INT_A::UNMASK)
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
#[doc = "If unmasked, GPIO7 interrupt will wake up FFE."]
pub type SENSOR_GPIO_7_INT_A = KICKOFF_TIMER_TIME_OUT_A;
#[doc = "Field `Sensor_GPIO_7_INT` reader - If unmasked, GPIO7 interrupt will wake up FFE."]
pub type SENSOR_GPIO_7_INT_R = KICKOFF_TIMER_TIME_OUT_R;
#[doc = "Field `Sensor_GPIO_7_INT` writer - If unmasked, GPIO7 interrupt will wake up FFE."]
pub struct SENSOR_GPIO_7_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSOR_GPIO_7_INT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SENSOR_GPIO_7_INT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Mask the interrupt as a wake-up event source"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(SENSOR_GPIO_7_INT_A::MASK)
    }
    #[doc = "Unmask the interrupt as a wake-up event source"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut W {
        self.variant(SENSOR_GPIO_7_INT_A::UNMASK)
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
            (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - If unmasked, kick off timer time-out event will wake up FFE."]
    #[inline(always)]
    pub fn kick_off_timer_time_out(&self) -> KICKOFF_TIMER_TIME_OUT_R {
        KICKOFF_TIMER_TIME_OUT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - If unmasked, GPIO0 interrupt will wake up FFE."]
    #[inline(always)]
    pub fn sensor_gpio_0_int(&self) -> SENSOR_GPIO_0_INT_R {
        SENSOR_GPIO_0_INT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - If unmasked, GPIO1 interrupt will wake up FFE."]
    #[inline(always)]
    pub fn sensor_gpio_1_int(&self) -> SENSOR_GPIO_1_INT_R {
        SENSOR_GPIO_1_INT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - If unmasked, GPIO2 interrupt will wake up FFE."]
    #[inline(always)]
    pub fn sensor_gpio_2_int(&self) -> SENSOR_GPIO_2_INT_R {
        SENSOR_GPIO_2_INT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - If unmasked, GPIO3 interrupt will wake up FFE."]
    #[inline(always)]
    pub fn sensor_gpio_3_int(&self) -> SENSOR_GPIO_3_INT_R {
        SENSOR_GPIO_3_INT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - If unmasked, GPIO4 interrupt will wake up FFE."]
    #[inline(always)]
    pub fn sensor_gpio_4_int(&self) -> SENSOR_GPIO_4_INT_R {
        SENSOR_GPIO_4_INT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - If unmasked, GPIO5 interrupt will wake up FFE."]
    #[inline(always)]
    pub fn sensor_gpio_5_int(&self) -> SENSOR_GPIO_5_INT_R {
        SENSOR_GPIO_5_INT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - If unmasked, GPIO6 interrupt will wake up FFE."]
    #[inline(always)]
    pub fn sensor_gpio_6_int(&self) -> SENSOR_GPIO_6_INT_R {
        SENSOR_GPIO_6_INT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - If unmasked, GPIO7 interrupt will wake up FFE."]
    #[inline(always)]
    pub fn sensor_gpio_7_int(&self) -> SENSOR_GPIO_7_INT_R {
        SENSOR_GPIO_7_INT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - If unmasked, kick off timer time-out event will wake up FFE."]
    #[inline(always)]
    pub fn kick_off_timer_time_out(&mut self) -> KICKOFF_TIMER_TIME_OUT_W {
        KICKOFF_TIMER_TIME_OUT_W { w: self }
    }
    #[doc = "Bit 1 - If unmasked, GPIO0 interrupt will wake up FFE."]
    #[inline(always)]
    pub fn sensor_gpio_0_int(&mut self) -> SENSOR_GPIO_0_INT_W {
        SENSOR_GPIO_0_INT_W { w: self }
    }
    #[doc = "Bit 2 - If unmasked, GPIO1 interrupt will wake up FFE."]
    #[inline(always)]
    pub fn sensor_gpio_1_int(&mut self) -> SENSOR_GPIO_1_INT_W {
        SENSOR_GPIO_1_INT_W { w: self }
    }
    #[doc = "Bit 3 - If unmasked, GPIO2 interrupt will wake up FFE."]
    #[inline(always)]
    pub fn sensor_gpio_2_int(&mut self) -> SENSOR_GPIO_2_INT_W {
        SENSOR_GPIO_2_INT_W { w: self }
    }
    #[doc = "Bit 4 - If unmasked, GPIO3 interrupt will wake up FFE."]
    #[inline(always)]
    pub fn sensor_gpio_3_int(&mut self) -> SENSOR_GPIO_3_INT_W {
        SENSOR_GPIO_3_INT_W { w: self }
    }
    #[doc = "Bit 5 - If unmasked, GPIO4 interrupt will wake up FFE."]
    #[inline(always)]
    pub fn sensor_gpio_4_int(&mut self) -> SENSOR_GPIO_4_INT_W {
        SENSOR_GPIO_4_INT_W { w: self }
    }
    #[doc = "Bit 6 - If unmasked, GPIO5 interrupt will wake up FFE."]
    #[inline(always)]
    pub fn sensor_gpio_5_int(&mut self) -> SENSOR_GPIO_5_INT_W {
        SENSOR_GPIO_5_INT_W { w: self }
    }
    #[doc = "Bit 7 - If unmasked, GPIO6 interrupt will wake up FFE."]
    #[inline(always)]
    pub fn sensor_gpio_6_int(&mut self) -> SENSOR_GPIO_6_INT_W {
        SENSOR_GPIO_6_INT_W { w: self }
    }
    #[doc = "Bit 8 - If unmasked, GPIO7 interrupt will wake up FFE."]
    #[inline(always)]
    pub fn sensor_gpio_7_int(&mut self) -> SENSOR_GPIO_7_INT_W {
        SENSOR_GPIO_7_INT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control the masking of the Flexible Fusion Engine wake-up event triggers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ffe_wu_src_mask_n](index.html) module"]
pub struct FFE_WU_SRC_MASK_N_SPEC;
impl crate::RegisterSpec for FFE_WU_SRC_MASK_N_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ffe_wu_src_mask_n::R](R) reader structure"]
impl crate::Readable for FFE_WU_SRC_MASK_N_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ffe_wu_src_mask_n::W](W) writer structure"]
impl crate::Writable for FFE_WU_SRC_MASK_N_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FFE_WU_SRC_MASK_N to value 0"]
impl crate::Resettable for FFE_WU_SRC_MASK_N_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
