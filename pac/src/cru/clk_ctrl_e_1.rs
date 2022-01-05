#[doc = "Register `CLK_CTRL_E_1` reader"]
pub struct R(crate::R<CLK_CTRL_E_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_CTRL_E_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_CTRL_E_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_CTRL_E_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "The selected the clock source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLOCK_SOURCE_SELECTION_A {
    #[doc = "0: Selects the High speed/Divided clock"]
    WITH_HS_CLK = 0,
}
impl From<CLOCK_SOURCE_SELECTION_A> for u8 {
    #[inline(always)]
    fn from(variant: CLOCK_SOURCE_SELECTION_A) -> Self {
        variant as _
    }
}
#[doc = "Field `Clock_Source_Selection` reader - The selected the clock source"]
pub struct CLOCK_SOURCE_SELECTION_R(
    crate::FieldReader<u8, CLOCK_SOURCE_SELECTION_A>,
);
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
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `WITH_HS_CLK`"]
    #[inline(always)]
    pub fn is_with_hs_clk(&self) -> bool {
        **self == CLOCK_SOURCE_SELECTION_A::WITH_HS_CLK
    }
}
impl core::ops::Deref for CLOCK_SOURCE_SELECTION_R {
    type Target = crate::FieldReader<u8, CLOCK_SOURCE_SELECTION_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:1 - The selected the clock source"]
    #[inline(always)]
    pub fn clock_source_selection(&self) -> CLOCK_SOURCE_SELECTION_R {
        CLOCK_SOURCE_SELECTION_R::new((self.bits & 0x03) as u8)
    }
}
#[doc = "For Clock 12\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_ctrl_e_1](index.html) module"]
pub struct CLK_CTRL_E_1_SPEC;
impl crate::RegisterSpec for CLK_CTRL_E_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_ctrl_e_1::R](R) reader structure"]
impl crate::Readable for CLK_CTRL_E_1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CLK_CTRL_E_1 to value 0"]
impl crate::Resettable for CLK_CTRL_E_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
