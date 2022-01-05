#[doc = "Register `C10_FCLK_GATE` reader"]
pub struct R(crate::R<C10_FCLK_GATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C10_FCLK_GATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C10_FCLK_GATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C10_FCLK_GATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C10_FCLK_GATE` writer"]
pub struct W(crate::W<C10_FCLK_GATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C10_FCLK_GATE_SPEC>;
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
impl From<crate::W<C10_FCLK_GATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C10_FCLK_GATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "To M4 Bus Matrix and Trace block\n\nValue on reset: 1"]
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
#[doc = "Field `Path_0_Gating_Control` reader - To M4 Bus Matrix and Trace block"]
pub struct PATH_0_GATING_CONTROL_R(
    crate::FieldReader<bool, PATH_0_GATING_CONTROL_A>,
);
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
#[doc = "RWHC: To M4 SRAM Instance, M4S0~M4S3. This bit will be set if any of the Memories (M4S0~M4S3) been wakeup by Hardware."]
pub type PATH_1_GATING_CONTROL_A = PATH_0_GATING_CONTROL_A;
#[doc = "Field `Path_1_Gating_Control` reader - RWHC: To M4 SRAM Instance, M4S0~M4S3. This bit will be set if any of the Memories (M4S0~M4S3) been wakeup by Hardware."]
pub type PATH_1_GATING_CONTROL_R = PATH_0_GATING_CONTROL_R;
#[doc = "Field `Path_1_Gating_Control` writer - RWHC: To M4 SRAM Instance, M4S0~M4S3. This bit will be set if any of the Memories (M4S0~M4S3) been wakeup by Hardware."]
pub struct PATH_1_GATING_CONTROL_W<'a> {
    w: &'a mut W,
}
impl<'a> PATH_1_GATING_CONTROL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PATH_1_GATING_CONTROL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock is stop"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(PATH_1_GATING_CONTROL_A::STOP)
    }
    #[doc = "Clock is runnig"]
    #[inline(always)]
    pub fn run(self) -> &'a mut W {
        self.variant(PATH_1_GATING_CONTROL_A::RUN)
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
#[doc = "RWHC: To M4 SRAM Instance, M4S4~M4S7. This bit will be set if any of the Memories (M4S4~M4S7) been wakeup by Hardware."]
pub type PATH_2_GATING_CONTROL_A = PATH_0_GATING_CONTROL_A;
#[doc = "Field `Path_2_Gating_Control` reader - RWHC: To M4 SRAM Instance, M4S4~M4S7. This bit will be set if any of the Memories (M4S4~M4S7) been wakeup by Hardware."]
pub type PATH_2_GATING_CONTROL_R = PATH_0_GATING_CONTROL_R;
#[doc = "Field `Path_2_Gating_Control` writer - RWHC: To M4 SRAM Instance, M4S4~M4S7. This bit will be set if any of the Memories (M4S4~M4S7) been wakeup by Hardware."]
pub struct PATH_2_GATING_CONTROL_W<'a> {
    w: &'a mut W,
}
impl<'a> PATH_2_GATING_CONTROL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PATH_2_GATING_CONTROL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock is stop"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(PATH_2_GATING_CONTROL_A::STOP)
    }
    #[doc = "Clock is runnig"]
    #[inline(always)]
    pub fn run(self) -> &'a mut W {
        self.variant(PATH_2_GATING_CONTROL_A::RUN)
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
#[doc = "RWHC: To M4 SRAM Instance, M4S8~M4S11. This bit will be set if any of the Memories (M4S8~M4S11) been wakeup by Hardware."]
pub type PATH_3_GATING_CONTROL_A = PATH_0_GATING_CONTROL_A;
#[doc = "Field `Path_3_Gating_Control` reader - RWHC: To M4 SRAM Instance, M4S8~M4S11. This bit will be set if any of the Memories (M4S8~M4S11) been wakeup by Hardware."]
pub type PATH_3_GATING_CONTROL_R = PATH_0_GATING_CONTROL_R;
#[doc = "Field `Path_3_Gating_Control` writer - RWHC: To M4 SRAM Instance, M4S8~M4S11. This bit will be set if any of the Memories (M4S8~M4S11) been wakeup by Hardware."]
pub struct PATH_3_GATING_CONTROL_W<'a> {
    w: &'a mut W,
}
impl<'a> PATH_3_GATING_CONTROL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PATH_3_GATING_CONTROL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock is stop"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(PATH_3_GATING_CONTROL_A::STOP)
    }
    #[doc = "Clock is runnig"]
    #[inline(always)]
    pub fn run(self) -> &'a mut W {
        self.variant(PATH_3_GATING_CONTROL_A::RUN)
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
#[doc = "RWHC: To M4 SRAM Instance, M4S12~M4S15. This bit will be set if any of the Memories (M4S12~M4S15) been wakeup by Hardware."]
pub type PATH_4_GATING_CONTROL_A = PATH_0_GATING_CONTROL_A;
#[doc = "Field `Path_4_Gating_Control` reader - RWHC: To M4 SRAM Instance, M4S12~M4S15. This bit will be set if any of the Memories (M4S12~M4S15) been wakeup by Hardware."]
pub type PATH_4_GATING_CONTROL_R = PATH_0_GATING_CONTROL_R;
#[doc = "Field `Path_4_Gating_Control` writer - RWHC: To M4 SRAM Instance, M4S12~M4S15. This bit will be set if any of the Memories (M4S12~M4S15) been wakeup by Hardware."]
pub struct PATH_4_GATING_CONTROL_W<'a> {
    w: &'a mut W,
}
impl<'a> PATH_4_GATING_CONTROL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PATH_4_GATING_CONTROL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock is stop"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(PATH_4_GATING_CONTROL_A::STOP)
    }
    #[doc = "Clock is runnig"]
    #[inline(always)]
    pub fn run(self) -> &'a mut W {
        self.variant(PATH_4_GATING_CONTROL_A::RUN)
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
#[doc = "To AUDIO DMA"]
pub type PATH_5_GATING_CONTROL_A = PATH_0_GATING_CONTROL_A;
#[doc = "Field `Path_5_Gating_Control` reader - To AUDIO DMA"]
pub type PATH_5_GATING_CONTROL_R = PATH_0_GATING_CONTROL_R;
#[doc = "Field `Path_5_Gating_Control` writer - To AUDIO DMA"]
pub struct PATH_5_GATING_CONTROL_W<'a> {
    w: &'a mut W,
}
impl<'a> PATH_5_GATING_CONTROL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PATH_5_GATING_CONTROL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock is stop"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(PATH_5_GATING_CONTROL_A::STOP)
    }
    #[doc = "Clock is runnig"]
    #[inline(always)]
    pub fn run(self) -> &'a mut W {
        self.variant(PATH_5_GATING_CONTROL_A::RUN)
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
#[doc = "To the SYNC Up on A0 and AHB Interface of Batching Memory"]
pub type PATH_6_GATING_CONTROL_A = PATH_0_GATING_CONTROL_A;
#[doc = "Field `Path_6_Gating_Control` reader - To the SYNC Up on A0 and AHB Interface of Batching Memory"]
pub type PATH_6_GATING_CONTROL_R = PATH_0_GATING_CONTROL_R;
#[doc = "Field `Path_6_Gating_Control` writer - To the SYNC Up on A0 and AHB Interface of Batching Memory"]
pub struct PATH_6_GATING_CONTROL_W<'a> {
    w: &'a mut W,
}
impl<'a> PATH_6_GATING_CONTROL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PATH_6_GATING_CONTROL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock is stop"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(PATH_6_GATING_CONTROL_A::STOP)
    }
    #[doc = "Clock is runnig"]
    #[inline(always)]
    pub fn run(self) -> &'a mut W {
        self.variant(PATH_6_GATING_CONTROL_A::RUN)
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
impl R {
    #[doc = "Bit 0 - To M4 Bus Matrix and Trace block"]
    #[inline(always)]
    pub fn path_0_gating_control(&self) -> PATH_0_GATING_CONTROL_R {
        PATH_0_GATING_CONTROL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - RWHC: To M4 SRAM Instance, M4S0~M4S3. This bit will be set if any of the Memories (M4S0~M4S3) been wakeup by Hardware."]
    #[inline(always)]
    pub fn path_1_gating_control(&self) -> PATH_1_GATING_CONTROL_R {
        PATH_1_GATING_CONTROL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RWHC: To M4 SRAM Instance, M4S4~M4S7. This bit will be set if any of the Memories (M4S4~M4S7) been wakeup by Hardware."]
    #[inline(always)]
    pub fn path_2_gating_control(&self) -> PATH_2_GATING_CONTROL_R {
        PATH_2_GATING_CONTROL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RWHC: To M4 SRAM Instance, M4S8~M4S11. This bit will be set if any of the Memories (M4S8~M4S11) been wakeup by Hardware."]
    #[inline(always)]
    pub fn path_3_gating_control(&self) -> PATH_3_GATING_CONTROL_R {
        PATH_3_GATING_CONTROL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - RWHC: To M4 SRAM Instance, M4S12~M4S15. This bit will be set if any of the Memories (M4S12~M4S15) been wakeup by Hardware."]
    #[inline(always)]
    pub fn path_4_gating_control(&self) -> PATH_4_GATING_CONTROL_R {
        PATH_4_GATING_CONTROL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - To AUDIO DMA"]
    #[inline(always)]
    pub fn path_5_gating_control(&self) -> PATH_5_GATING_CONTROL_R {
        PATH_5_GATING_CONTROL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - To the SYNC Up on A0 and AHB Interface of Batching Memory"]
    #[inline(always)]
    pub fn path_6_gating_control(&self) -> PATH_6_GATING_CONTROL_R {
        PATH_6_GATING_CONTROL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - RWHC: To M4 SRAM Instance, M4S0~M4S3. This bit will be set if any of the Memories (M4S0~M4S3) been wakeup by Hardware."]
    #[inline(always)]
    pub fn path_1_gating_control(&mut self) -> PATH_1_GATING_CONTROL_W {
        PATH_1_GATING_CONTROL_W { w: self }
    }
    #[doc = "Bit 2 - RWHC: To M4 SRAM Instance, M4S4~M4S7. This bit will be set if any of the Memories (M4S4~M4S7) been wakeup by Hardware."]
    #[inline(always)]
    pub fn path_2_gating_control(&mut self) -> PATH_2_GATING_CONTROL_W {
        PATH_2_GATING_CONTROL_W { w: self }
    }
    #[doc = "Bit 3 - RWHC: To M4 SRAM Instance, M4S8~M4S11. This bit will be set if any of the Memories (M4S8~M4S11) been wakeup by Hardware."]
    #[inline(always)]
    pub fn path_3_gating_control(&mut self) -> PATH_3_GATING_CONTROL_W {
        PATH_3_GATING_CONTROL_W { w: self }
    }
    #[doc = "Bit 4 - RWHC: To M4 SRAM Instance, M4S12~M4S15. This bit will be set if any of the Memories (M4S12~M4S15) been wakeup by Hardware."]
    #[inline(always)]
    pub fn path_4_gating_control(&mut self) -> PATH_4_GATING_CONTROL_W {
        PATH_4_GATING_CONTROL_W { w: self }
    }
    #[doc = "Bit 5 - To AUDIO DMA"]
    #[inline(always)]
    pub fn path_5_gating_control(&mut self) -> PATH_5_GATING_CONTROL_W {
        PATH_5_GATING_CONTROL_W { w: self }
    }
    #[doc = "Bit 6 - To the SYNC Up on A0 and AHB Interface of Batching Memory"]
    #[inline(always)]
    pub fn path_6_gating_control(&mut self) -> PATH_6_GATING_CONTROL_W {
        PATH_6_GATING_CONTROL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Gating control for Clock 10\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c10_fclk_gate](index.html) module"]
pub struct C10_FCLK_GATE_SPEC;
impl crate::RegisterSpec for C10_FCLK_GATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c10_fclk_gate::R](R) reader structure"]
impl crate::Readable for C10_FCLK_GATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c10_fclk_gate::W](W) writer structure"]
impl crate::Writable for C10_FCLK_GATE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets C10_FCLK_GATE to value 0x43"]
impl crate::Resettable for C10_FCLK_GATE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x43
    }
}
