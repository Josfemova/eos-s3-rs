#[doc = "Register `CLK_CTRL_PMU` reader"]
pub struct R(crate::R<CLK_CTRL_PMU_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_CTRL_PMU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_CTRL_PMU_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_CTRL_PMU_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_CTRL_PMU` writer"]
pub struct W(crate::W<CLK_CTRL_PMU_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_CTRL_PMU_SPEC>;
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
impl From<crate::W<CLK_CTRL_PMU_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_CTRL_PMU_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `Clock_Divider_Ratio` reader - High Speed Clock Divider Ratio. Ratio is reg value + 2. default div is 64"]
pub struct CLOCK_DIVIDER_RATIO_R(crate::FieldReader<u16, u16>);
impl CLOCK_DIVIDER_RATIO_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        CLOCK_DIVIDER_RATIO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLOCK_DIVIDER_RATIO_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Clock_Divider_Ratio` writer - High Speed Clock Divider Ratio. Ratio is reg value + 2. default div is 64"]
pub struct CLOCK_DIVIDER_RATIO_W<'a> {
    w: &'a mut W,
}
impl<'a> CLOCK_DIVIDER_RATIO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | (value as u32 & 0x01ff);
        self.w
    }
}
#[doc = "1'b1 Clock Divider is ON. 1'b0 Clock Divider is OFF, Output the Source Clock Directly\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLE_CLOCK_DIVIDER_A {
    #[doc = "0: Clock divider is OFF, Output Source clock directly"]
    DISABLE = 0,
    #[doc = "1: Clock divider is ON"]
    ENABLE = 1,
}
impl From<ENABLE_CLOCK_DIVIDER_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE_CLOCK_DIVIDER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Enable_Clock_Divider` reader - 1'b1 Clock Divider is ON. 1'b0 Clock Divider is OFF, Output the Source Clock Directly"]
pub struct ENABLE_CLOCK_DIVIDER_R(crate::FieldReader<bool, ENABLE_CLOCK_DIVIDER_A>);
impl ENABLE_CLOCK_DIVIDER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENABLE_CLOCK_DIVIDER_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE_CLOCK_DIVIDER_A {
        match self.bits {
            false => ENABLE_CLOCK_DIVIDER_A::DISABLE,
            true => ENABLE_CLOCK_DIVIDER_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == ENABLE_CLOCK_DIVIDER_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == ENABLE_CLOCK_DIVIDER_A::ENABLE
    }
}
impl core::ops::Deref for ENABLE_CLOCK_DIVIDER_R {
    type Target = crate::FieldReader<bool, ENABLE_CLOCK_DIVIDER_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Enable_Clock_Divider` writer - 1'b1 Clock Divider is ON. 1'b0 Clock Divider is OFF, Output the Source Clock Directly"]
pub struct ENABLE_CLOCK_DIVIDER_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_CLOCK_DIVIDER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENABLE_CLOCK_DIVIDER_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock divider is OFF, Output Source clock directly"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ENABLE_CLOCK_DIVIDER_A::DISABLE)
    }
    #[doc = "Clock divider is ON"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ENABLE_CLOCK_DIVIDER_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - High Speed Clock Divider Ratio. Ratio is reg value + 2. default div is 64"]
    #[inline(always)]
    pub fn clock_divider_ratio(&self) -> CLOCK_DIVIDER_RATIO_R {
        CLOCK_DIVIDER_RATIO_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 9 - 1'b1 Clock Divider is ON. 1'b0 Clock Divider is OFF, Output the Source Clock Directly"]
    #[inline(always)]
    pub fn enable_clock_divider(&self) -> ENABLE_CLOCK_DIVIDER_R {
        ENABLE_CLOCK_DIVIDER_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - High Speed Clock Divider Ratio. Ratio is reg value + 2. default div is 64"]
    #[inline(always)]
    pub fn clock_divider_ratio(&mut self) -> CLOCK_DIVIDER_RATIO_W {
        CLOCK_DIVIDER_RATIO_W { w: self }
    }
    #[doc = "Bit 9 - 1'b1 Clock Divider is ON. 1'b0 Clock Divider is OFF, Output the Source Clock Directly"]
    #[inline(always)]
    pub fn enable_clock_divider(&mut self) -> ENABLE_CLOCK_DIVIDER_W {
        ENABLE_CLOCK_DIVIDER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This Clock is used to delay the Clock gating control signals from PMU. The PMU will monitor the feedback/delayed Clock Gating Control signals to ensure the clocks are OFF before jump to next state. The Firmware needs to Configure this divider to ensure there delay time is longer enough. C23 Clock needs to be 2/3 of the lowest clock frequency of other clocks. For Example, if the Lowest clock frequency of other clocks are 5, then C23 should be lower than 3.33MHz (Or the clock frequency of other clocks should be at least 1.5 times faster than C23.)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_ctrl_pmu](index.html) module"]
pub struct CLK_CTRL_PMU_SPEC;
impl crate::RegisterSpec for CLK_CTRL_PMU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_ctrl_pmu::R](R) reader structure"]
impl crate::Readable for CLK_CTRL_PMU_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_ctrl_pmu::W](W) writer structure"]
impl crate::Writable for CLK_CTRL_PMU_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_CTRL_PMU to value 0x023e"]
impl crate::Resettable for CLK_CTRL_PMU_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x023e
    }
}
