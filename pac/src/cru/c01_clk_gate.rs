#[doc = "Register `C01_CLK_GATE` reader"]
pub struct R(crate::R<C01_CLK_GATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C01_CLK_GATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C01_CLK_GATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C01_CLK_GATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C01_CLK_GATE` writer"]
pub struct W(crate::W<C01_CLK_GATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C01_CLK_GATE_SPEC>;
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
impl From<crate::W<C01_CLK_GATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C01_CLK_GATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "To A0\n\nValue on reset: 1"]
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
#[doc = "Field `Path_0_Gating_Control` reader - To A0"]
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
#[doc = "To SDMA SRAM"]
pub type PATH_1_GATING_CONTROL_A = PATH_0_GATING_CONTROL_A;
#[doc = "Field `Path_1_Gating_Control` reader - To SDMA SRAM"]
pub type PATH_1_GATING_CONTROL_R = PATH_0_GATING_CONTROL_R;
#[doc = "Field `Path_1_Gating_Control` writer - To SDMA SRAM"]
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
#[doc = "To packet FIFO"]
pub type PATH_2_GATING_CONTROL_A = PATH_0_GATING_CONTROL_A;
#[doc = "Field `Path_2_Gating_Control` reader - To packet FIFO"]
pub type PATH_2_GATING_CONTROL_R = PATH_0_GATING_CONTROL_R;
#[doc = "Field `Path_2_Gating_Control` writer - To packet FIFO"]
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
#[doc = "To FFE"]
pub type PATH_3_GATING_CONTROL_A = PATH_0_GATING_CONTROL_A;
#[doc = "Field `Path_3_Gating_Control` reader - To FFE"]
pub type PATH_3_GATING_CONTROL_R = PATH_0_GATING_CONTROL_R;
#[doc = "Field `Path_3_Gating_Control` writer - To FFE"]
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
#[doc = "To AHB2APB Bridge /CFG DMA Bridge inside A1 , Allow M4 to configure SPI Master to load the code"]
pub type PATH_4_GATING_CONTROL_A = PATH_0_GATING_CONTROL_A;
#[doc = "Field `Path_4_Gating_Control` reader - To AHB2APB Bridge /CFG DMA Bridge inside A1 , Allow M4 to configure SPI Master to load the code"]
pub type PATH_4_GATING_CONTROL_R = PATH_0_GATING_CONTROL_R;
#[doc = "Field `Path_4_Gating_Control` writer - To AHB2APB Bridge /CFG DMA Bridge inside A1 , Allow M4 to configure SPI Master to load the code"]
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
#[doc = "To I2S module inside A1"]
pub type PATH_5_GATING_CONTROL_A = PATH_0_GATING_CONTROL_A;
#[doc = "Field `Path_5_Gating_Control` reader - To I2S module inside A1"]
pub type PATH_5_GATING_CONTROL_R = PATH_0_GATING_CONTROL_R;
#[doc = "Field `Path_5_Gating_Control` writer - To I2S module inside A1"]
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
#[doc = "To SDMA"]
pub type PATH_6_GATING_CONTROL_A = PATH_0_GATING_CONTROL_A;
#[doc = "Field `Path_6_Gating_Control` reader - To SDMA"]
pub type PATH_6_GATING_CONTROL_R = PATH_0_GATING_CONTROL_R;
#[doc = "Field `Path_6_Gating_Control` writer - To SDMA"]
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
#[doc = "Not specified"]
pub type PATH_7_GATING_CONTROL_A = PATH_0_GATING_CONTROL_A;
#[doc = "Field `Path_7_Gating_Control` reader - Not specified"]
pub type PATH_7_GATING_CONTROL_R = PATH_0_GATING_CONTROL_R;
#[doc = "Field `Path_7_Gating_Control` writer - Not specified"]
pub struct PATH_7_GATING_CONTROL_W<'a> {
    w: &'a mut W,
}
impl<'a> PATH_7_GATING_CONTROL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PATH_7_GATING_CONTROL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock is stop"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(PATH_7_GATING_CONTROL_A::STOP)
    }
    #[doc = "Clock is runnig"]
    #[inline(always)]
    pub fn run(self) -> &'a mut W {
        self.variant(PATH_7_GATING_CONTROL_A::RUN)
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
#[doc = "For SPT"]
pub type PATH_9_GATING_CONTROL_A = PATH_0_GATING_CONTROL_A;
#[doc = "Field `Path_9_Gating_Control` reader - For SPT"]
pub type PATH_9_GATING_CONTROL_R = PATH_0_GATING_CONTROL_R;
#[doc = "Field `Path_9_Gating_Control` writer - For SPT"]
pub struct PATH_9_GATING_CONTROL_W<'a> {
    w: &'a mut W,
}
impl<'a> PATH_9_GATING_CONTROL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PATH_9_GATING_CONTROL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock is stop"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(PATH_9_GATING_CONTROL_A::STOP)
    }
    #[doc = "Clock is runnig"]
    #[inline(always)]
    pub fn run(self) -> &'a mut W {
        self.variant(PATH_9_GATING_CONTROL_A::RUN)
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
            (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - To A0"]
    #[inline(always)]
    pub fn path_0_gating_control(&self) -> PATH_0_GATING_CONTROL_R {
        PATH_0_GATING_CONTROL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - To SDMA SRAM"]
    #[inline(always)]
    pub fn path_1_gating_control(&self) -> PATH_1_GATING_CONTROL_R {
        PATH_1_GATING_CONTROL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - To packet FIFO"]
    #[inline(always)]
    pub fn path_2_gating_control(&self) -> PATH_2_GATING_CONTROL_R {
        PATH_2_GATING_CONTROL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - To FFE"]
    #[inline(always)]
    pub fn path_3_gating_control(&self) -> PATH_3_GATING_CONTROL_R {
        PATH_3_GATING_CONTROL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - To AHB2APB Bridge /CFG DMA Bridge inside A1 , Allow M4 to configure SPI Master to load the code"]
    #[inline(always)]
    pub fn path_4_gating_control(&self) -> PATH_4_GATING_CONTROL_R {
        PATH_4_GATING_CONTROL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - To I2S module inside A1"]
    #[inline(always)]
    pub fn path_5_gating_control(&self) -> PATH_5_GATING_CONTROL_R {
        PATH_5_GATING_CONTROL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - To SDMA"]
    #[inline(always)]
    pub fn path_6_gating_control(&self) -> PATH_6_GATING_CONTROL_R {
        PATH_6_GATING_CONTROL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Not specified"]
    #[inline(always)]
    pub fn path_7_gating_control(&self) -> PATH_7_GATING_CONTROL_R {
        PATH_7_GATING_CONTROL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 9 - For SPT"]
    #[inline(always)]
    pub fn path_9_gating_control(&self) -> PATH_9_GATING_CONTROL_R {
        PATH_9_GATING_CONTROL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - To SDMA SRAM"]
    #[inline(always)]
    pub fn path_1_gating_control(&mut self) -> PATH_1_GATING_CONTROL_W {
        PATH_1_GATING_CONTROL_W { w: self }
    }
    #[doc = "Bit 2 - To packet FIFO"]
    #[inline(always)]
    pub fn path_2_gating_control(&mut self) -> PATH_2_GATING_CONTROL_W {
        PATH_2_GATING_CONTROL_W { w: self }
    }
    #[doc = "Bit 3 - To FFE"]
    #[inline(always)]
    pub fn path_3_gating_control(&mut self) -> PATH_3_GATING_CONTROL_W {
        PATH_3_GATING_CONTROL_W { w: self }
    }
    #[doc = "Bit 4 - To AHB2APB Bridge /CFG DMA Bridge inside A1 , Allow M4 to configure SPI Master to load the code"]
    #[inline(always)]
    pub fn path_4_gating_control(&mut self) -> PATH_4_GATING_CONTROL_W {
        PATH_4_GATING_CONTROL_W { w: self }
    }
    #[doc = "Bit 5 - To I2S module inside A1"]
    #[inline(always)]
    pub fn path_5_gating_control(&mut self) -> PATH_5_GATING_CONTROL_W {
        PATH_5_GATING_CONTROL_W { w: self }
    }
    #[doc = "Bit 6 - To SDMA"]
    #[inline(always)]
    pub fn path_6_gating_control(&mut self) -> PATH_6_GATING_CONTROL_W {
        PATH_6_GATING_CONTROL_W { w: self }
    }
    #[doc = "Bit 7 - Not specified"]
    #[inline(always)]
    pub fn path_7_gating_control(&mut self) -> PATH_7_GATING_CONTROL_W {
        PATH_7_GATING_CONTROL_W { w: self }
    }
    #[doc = "Bit 9 - For SPT"]
    #[inline(always)]
    pub fn path_9_gating_control(&mut self) -> PATH_9_GATING_CONTROL_W {
        PATH_9_GATING_CONTROL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Gating control for Clock 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c01_clk_gate](index.html) module"]
pub struct C01_CLK_GATE_SPEC;
impl crate::RegisterSpec for C01_CLK_GATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c01_clk_gate::R](R) reader structure"]
impl crate::Readable for C01_CLK_GATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c01_clk_gate::W](W) writer structure"]
impl crate::Writable for C01_CLK_GATE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets C01_CLK_GATE to value 0x0291"]
impl crate::Resettable for C01_CLK_GATE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0291
    }
}
