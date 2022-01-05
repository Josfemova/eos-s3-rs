#[doc = "Register `CLK_SWITCH_FOR_D` reader"]
pub struct R(crate::R<CLK_SWITCH_FOR_D_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_SWITCH_FOR_D_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_SWITCH_FOR_D_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_SWITCH_FOR_D_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_SWITCH_FOR_D` writer"]
pub struct W(crate::W<CLK_SWITCH_FOR_D_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_SWITCH_FOR_D_SPEC>;
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
impl From<crate::W<CLK_SWITCH_FOR_D_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_SWITCH_FOR_D_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Choose between High Speed Clock or 32Khz oscillator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLOCK_SOURCE_SELECTION_A {
    #[doc = "0: Selects the High speed/Divided clock"]
    WITH_HS_CLK = 0,
    #[doc = "1: Selects the 32KHz oscillator"]
    WITH_32KHZ_CLK = 1,
}
impl From<CLOCK_SOURCE_SELECTION_A> for bool {
    #[inline(always)]
    fn from(variant: CLOCK_SOURCE_SELECTION_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Clock_Source_Selection` reader - Choose between High Speed Clock or 32Khz oscillator"]
pub struct CLOCK_SOURCE_SELECTION_R(
    crate::FieldReader<bool, CLOCK_SOURCE_SELECTION_A>,
);
impl CLOCK_SOURCE_SELECTION_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLOCK_SOURCE_SELECTION_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLOCK_SOURCE_SELECTION_A {
        match self.bits {
            false => CLOCK_SOURCE_SELECTION_A::WITH_HS_CLK,
            true => CLOCK_SOURCE_SELECTION_A::WITH_32KHZ_CLK,
        }
    }
    #[doc = "Checks if the value of the field is `WITH_HS_CLK`"]
    #[inline(always)]
    pub fn is_with_hs_clk(&self) -> bool {
        **self == CLOCK_SOURCE_SELECTION_A::WITH_HS_CLK
    }
    #[doc = "Checks if the value of the field is `WITH_32KHZ_CLK`"]
    #[inline(always)]
    pub fn is_with_32khz_clk(&self) -> bool {
        **self == CLOCK_SOURCE_SELECTION_A::WITH_32KHZ_CLK
    }
}
impl core::ops::Deref for CLOCK_SOURCE_SELECTION_R {
    type Target = crate::FieldReader<bool, CLOCK_SOURCE_SELECTION_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Clock_Source_Selection` writer - Choose between High Speed Clock or 32Khz oscillator"]
pub struct CLOCK_SOURCE_SELECTION_W<'a> {
    w: &'a mut W,
}
impl<'a> CLOCK_SOURCE_SELECTION_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLOCK_SOURCE_SELECTION_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Selects the High speed/Divided clock"]
    #[inline(always)]
    pub fn with_hs_clk(self) -> &'a mut W {
        self.variant(CLOCK_SOURCE_SELECTION_A::WITH_HS_CLK)
    }
    #[doc = "Selects the 32KHz oscillator"]
    #[inline(always)]
    pub fn with_32khz_clk(self) -> &'a mut W {
        self.variant(CLOCK_SOURCE_SELECTION_A::WITH_32KHZ_CLK)
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
impl R {
    #[doc = "Bit 0 - Choose between High Speed Clock or 32Khz oscillator"]
    #[inline(always)]
    pub fn clock_source_selection(&self) -> CLOCK_SOURCE_SELECTION_R {
        CLOCK_SOURCE_SELECTION_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Choose between High Speed Clock or 32Khz oscillator"]
    #[inline(always)]
    pub fn clock_source_selection(&mut self) -> CLOCK_SOURCE_SELECTION_W {
        CLOCK_SOURCE_SELECTION_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "For Clock 11 (To M4 peripherals - AHB/APB bridge, UART, WDT and TIMER)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_switch_for_d](index.html) module"]
pub struct CLK_SWITCH_FOR_D_SPEC;
impl crate::RegisterSpec for CLK_SWITCH_FOR_D_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_switch_for_d::R](R) reader structure"]
impl crate::Readable for CLK_SWITCH_FOR_D_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_switch_for_d::W](W) writer structure"]
impl crate::Writable for CLK_SWITCH_FOR_D_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_SWITCH_FOR_D to value 0"]
impl crate::Resettable for CLK_SWITCH_FOR_D_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
