#[doc = "Register `C30_C31_CLK_GATE` reader"]
pub struct R(crate::R<C30_C31_CLK_GATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C30_C31_CLK_GATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C30_C31_CLK_GATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C30_C31_CLK_GATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C30_C31_CLK_GATE` writer"]
pub struct W(crate::W<C30_C31_CLK_GATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C30_C31_CLK_GATE_SPEC>;
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
impl From<crate::W<C30_C31_CLK_GATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C30_C31_CLK_GATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "To PDM LEFT CLK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum C30_PATH_0_GATING_CONTROL_A {
    #[doc = "0: Clock is stop"]
    STOP = 0,
    #[doc = "1: Clock is runnig"]
    RUN = 1,
}
impl From<C30_PATH_0_GATING_CONTROL_A> for bool {
    #[inline(always)]
    fn from(variant: C30_PATH_0_GATING_CONTROL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `C30_Path_0_Gating_Control` reader - To PDM LEFT CLK"]
pub struct C30_PATH_0_GATING_CONTROL_R(crate::FieldReader<bool, C30_PATH_0_GATING_CONTROL_A>);
impl C30_PATH_0_GATING_CONTROL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        C30_PATH_0_GATING_CONTROL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> C30_PATH_0_GATING_CONTROL_A {
        match self.bits {
            false => C30_PATH_0_GATING_CONTROL_A::STOP,
            true => C30_PATH_0_GATING_CONTROL_A::RUN,
        }
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        **self == C30_PATH_0_GATING_CONTROL_A::STOP
    }
    #[doc = "Checks if the value of the field is `RUN`"]
    #[inline(always)]
    pub fn is_run(&self) -> bool {
        **self == C30_PATH_0_GATING_CONTROL_A::RUN
    }
}
impl core::ops::Deref for C30_PATH_0_GATING_CONTROL_R {
    type Target = crate::FieldReader<bool, C30_PATH_0_GATING_CONTROL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `C30_Path_0_Gating_Control` writer - To PDM LEFT CLK"]
pub struct C30_PATH_0_GATING_CONTROL_W<'a> {
    w: &'a mut W,
}
impl<'a> C30_PATH_0_GATING_CONTROL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: C30_PATH_0_GATING_CONTROL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock is stop"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(C30_PATH_0_GATING_CONTROL_A::STOP)
    }
    #[doc = "Clock is runnig"]
    #[inline(always)]
    pub fn run(self) -> &'a mut W {
        self.variant(C30_PATH_0_GATING_CONTROL_A::RUN)
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
#[doc = "To PDM RIGHT CLK"]
pub type C30_PATH_1_GATING_CONTROL_A = C30_PATH_0_GATING_CONTROL_A;
#[doc = "Field `C30_Path_1_Gating_Control` reader - To PDM RIGHT CLK"]
pub type C30_PATH_1_GATING_CONTROL_R = C30_PATH_0_GATING_CONTROL_R;
#[doc = "Field `C30_Path_1_Gating_Control` writer - To PDM RIGHT CLK"]
pub struct C30_PATH_1_GATING_CONTROL_W<'a> {
    w: &'a mut W,
}
impl<'a> C30_PATH_1_GATING_CONTROL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: C30_PATH_1_GATING_CONTROL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock is stop"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(C30_PATH_1_GATING_CONTROL_A::STOP)
    }
    #[doc = "Clock is runnig"]
    #[inline(always)]
    pub fn run(self) -> &'a mut W {
        self.variant(C30_PATH_1_GATING_CONTROL_A::RUN)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "To I2S Master Clk"]
pub type C30_PATH_2_GATING_CONTROL_A = C30_PATH_0_GATING_CONTROL_A;
#[doc = "Field `C30_Path_2_Gating_Control` reader - To I2S Master Clk"]
pub type C30_PATH_2_GATING_CONTROL_R = C30_PATH_0_GATING_CONTROL_R;
#[doc = "Field `C30_Path_2_Gating_Control` writer - To I2S Master Clk"]
pub struct C30_PATH_2_GATING_CONTROL_W<'a> {
    w: &'a mut W,
}
impl<'a> C30_PATH_2_GATING_CONTROL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: C30_PATH_2_GATING_CONTROL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock is stop"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(C30_PATH_2_GATING_CONTROL_A::STOP)
    }
    #[doc = "Clock is runnig"]
    #[inline(always)]
    pub fn run(self) -> &'a mut W {
        self.variant(C30_PATH_2_GATING_CONTROL_A::RUN)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "To LPSD clk"]
pub type C31_PATH_0_GATING_CONTROL_A = C30_PATH_0_GATING_CONTROL_A;
#[doc = "Field `C31_Path_0_Gating_Control` reader - To LPSD clk"]
pub type C31_PATH_0_GATING_CONTROL_R = C30_PATH_0_GATING_CONTROL_R;
#[doc = "Field `C31_Path_0_Gating_Control` writer - To LPSD clk"]
pub struct C31_PATH_0_GATING_CONTROL_W<'a> {
    w: &'a mut W,
}
impl<'a> C31_PATH_0_GATING_CONTROL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: C31_PATH_0_GATING_CONTROL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock is stop"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(C31_PATH_0_GATING_CONTROL_A::STOP)
    }
    #[doc = "Clock is runnig"]
    #[inline(always)]
    pub fn run(self) -> &'a mut W {
        self.variant(C31_PATH_0_GATING_CONTROL_A::RUN)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - To PDM LEFT CLK"]
    #[inline(always)]
    pub fn c30_path_0_gating_control(&self) -> C30_PATH_0_GATING_CONTROL_R {
        C30_PATH_0_GATING_CONTROL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - To PDM RIGHT CLK"]
    #[inline(always)]
    pub fn c30_path_1_gating_control(&self) -> C30_PATH_1_GATING_CONTROL_R {
        C30_PATH_1_GATING_CONTROL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - To I2S Master Clk"]
    #[inline(always)]
    pub fn c30_path_2_gating_control(&self) -> C30_PATH_2_GATING_CONTROL_R {
        C30_PATH_2_GATING_CONTROL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - To LPSD clk"]
    #[inline(always)]
    pub fn c31_path_0_gating_control(&self) -> C31_PATH_0_GATING_CONTROL_R {
        C31_PATH_0_GATING_CONTROL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - To PDM LEFT CLK"]
    #[inline(always)]
    pub fn c30_path_0_gating_control(&mut self) -> C30_PATH_0_GATING_CONTROL_W {
        C30_PATH_0_GATING_CONTROL_W { w: self }
    }
    #[doc = "Bit 1 - To PDM RIGHT CLK"]
    #[inline(always)]
    pub fn c30_path_1_gating_control(&mut self) -> C30_PATH_1_GATING_CONTROL_W {
        C30_PATH_1_GATING_CONTROL_W { w: self }
    }
    #[doc = "Bit 2 - To I2S Master Clk"]
    #[inline(always)]
    pub fn c30_path_2_gating_control(&mut self) -> C30_PATH_2_GATING_CONTROL_W {
        C30_PATH_2_GATING_CONTROL_W { w: self }
    }
    #[doc = "Bit 3 - To LPSD clk"]
    #[inline(always)]
    pub fn c31_path_0_gating_control(&mut self) -> C31_PATH_0_GATING_CONTROL_W {
        C31_PATH_0_GATING_CONTROL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Gating control for clocks 30-31\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c30_c31_clk_gate](index.html) module"]
pub struct C30_C31_CLK_GATE_SPEC;
impl crate::RegisterSpec for C30_C31_CLK_GATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c30_c31_clk_gate::R](R) reader structure"]
impl crate::Readable for C30_C31_CLK_GATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c30_c31_clk_gate::W](W) writer structure"]
impl crate::Writable for C30_C31_CLK_GATE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets C30_C31_CLK_GATE to value 0"]
impl crate::Resettable for C30_C31_CLK_GATE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
