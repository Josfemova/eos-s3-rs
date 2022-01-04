#[doc = "Register `C19_CLK_GATE` reader"]
pub struct R(crate::R<C19_CLK_GATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C19_CLK_GATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C19_CLK_GATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C19_CLK_GATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C19_CLK_GATE` writer"]
pub struct W(crate::W<C19_CLK_GATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C19_CLK_GATE_SPEC>;
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
impl From<crate::W<C19_CLK_GATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C19_CLK_GATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "To ADC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PATH_0_GATING_CONTROL_A {
    #[doc = "0: Clock is stop"]
    STOP = 0,
    #[doc = "1: Clock is runnig"]
    RUN = 1,
}
impl From<PATH_0_GATING_CONTROL_A> for bool {
    #[inline(always)]
    fn from(variant: PATH_0_GATING_CONTROL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Path_0_Gating_Control` reader - To ADC"]
pub struct PATH_0_GATING_CONTROL_R(crate::FieldReader<bool, PATH_0_GATING_CONTROL_A>);
impl PATH_0_GATING_CONTROL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PATH_0_GATING_CONTROL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PATH_0_GATING_CONTROL_A {
        match self.bits {
            false => PATH_0_GATING_CONTROL_A::STOP,
            true => PATH_0_GATING_CONTROL_A::RUN,
        }
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        **self == PATH_0_GATING_CONTROL_A::STOP
    }
    #[doc = "Checks if the value of the field is `RUN`"]
    #[inline(always)]
    pub fn is_run(&self) -> bool {
        **self == PATH_0_GATING_CONTROL_A::RUN
    }
}
impl core::ops::Deref for PATH_0_GATING_CONTROL_R {
    type Target = crate::FieldReader<bool, PATH_0_GATING_CONTROL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Path_0_Gating_Control` writer - To ADC"]
pub struct PATH_0_GATING_CONTROL_W<'a> {
    w: &'a mut W,
}
impl<'a> PATH_0_GATING_CONTROL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PATH_0_GATING_CONTROL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock is stop"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(PATH_0_GATING_CONTROL_A::STOP)
    }
    #[doc = "Clock is runnig"]
    #[inline(always)]
    pub fn run(self) -> &'a mut W {
        self.variant(PATH_0_GATING_CONTROL_A::RUN)
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
    #[doc = "Bit 0 - To ADC"]
    #[inline(always)]
    pub fn path_0_gating_control(&self) -> PATH_0_GATING_CONTROL_R {
        PATH_0_GATING_CONTROL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - To ADC"]
    #[inline(always)]
    pub fn path_0_gating_control(&mut self) -> PATH_0_GATING_CONTROL_W {
        PATH_0_GATING_CONTROL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Gating control for ADC clock 19\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c19_clk_gate](index.html) module"]
pub struct C19_CLK_GATE_SPEC;
impl crate::RegisterSpec for C19_CLK_GATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c19_clk_gate::R](R) reader structure"]
impl crate::Readable for C19_CLK_GATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c19_clk_gate::W](W) writer structure"]
impl crate::Writable for C19_CLK_GATE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets C19_CLK_GATE to value 0"]
impl crate::Resettable for C19_CLK_GATE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
