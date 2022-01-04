#[doc = "Register `CLK_CTRL_A_1` reader"]
pub struct R(crate::R<CLK_CTRL_A_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_CTRL_A_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_CTRL_A_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_CTRL_A_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_CTRL_A_1` writer"]
pub struct W(crate::W<CLK_CTRL_A_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_CTRL_A_1_SPEC>;
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
impl From<crate::W<CLK_CTRL_A_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_CTRL_A_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Select the clock source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLOCK_SOURCE_SELECTION_A {
    #[doc = "0: Selects the High speed/Divided clock"]
    WITH_HS_CLK = 0,
    #[doc = "1: Selects the 32KHz oscillator"]
    WITH_32KHZ_CLK = 1,
}
impl From<CLOCK_SOURCE_SELECTION_A> for u8 {
    #[inline(always)]
    fn from(variant: CLOCK_SOURCE_SELECTION_A) -> Self {
        variant as _
    }
}
#[doc = "Field `Clock_Source_Selection` reader - Select the clock source"]
pub struct CLOCK_SOURCE_SELECTION_R(crate::FieldReader<u8, CLOCK_SOURCE_SELECTION_A>);
impl CLOCK_SOURCE_SELECTION_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CLOCK_SOURCE_SELECTION_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLOCK_SOURCE_SELECTION_A> {
        match self.bits {
            0 => Some(CLOCK_SOURCE_SELECTION_A::WITH_HS_CLK),
            1 => Some(CLOCK_SOURCE_SELECTION_A::WITH_32KHZ_CLK),
            _ => None,
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
    type Target = crate::FieldReader<u8, CLOCK_SOURCE_SELECTION_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Clock_Source_Selection` writer - Select the clock source"]
pub struct CLOCK_SOURCE_SELECTION_W<'a> {
    w: &'a mut W,
}
impl<'a> CLOCK_SOURCE_SELECTION_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLOCK_SOURCE_SELECTION_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
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
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Select the clock source"]
    #[inline(always)]
    pub fn clock_source_selection(&self) -> CLOCK_SOURCE_SELECTION_R {
        CLOCK_SOURCE_SELECTION_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Select the clock source"]
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
#[doc = "For Clock 10 (SYNC Up on A0 and AHB Interface of Batching Memory, AUDIO DMA, M4 SRAMs,M4 Bus Matrix and Trace block)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_ctrl_a_1](index.html) module"]
pub struct CLK_CTRL_A_1_SPEC;
impl crate::RegisterSpec for CLK_CTRL_A_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_ctrl_a_1::R](R) reader structure"]
impl crate::Readable for CLK_CTRL_A_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_ctrl_a_1::W](W) writer structure"]
impl crate::Writable for CLK_CTRL_A_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_CTRL_A_1 to value 0"]
impl crate::Resettable for CLK_CTRL_A_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
