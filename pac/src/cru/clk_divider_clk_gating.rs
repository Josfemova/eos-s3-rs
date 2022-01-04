#[doc = "Register `CLK_DIVIDER_CLK_GATING` reader"]
pub struct R(crate::R<CLK_DIVIDER_CLK_GATING_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_DIVIDER_CLK_GATING_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_DIVIDER_CLK_GATING_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_DIVIDER_CLK_GATING_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_DIVIDER_CLK_GATING` writer"]
pub struct W(crate::W<CLK_DIVIDER_CLK_GATING_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_DIVIDER_CLK_GATING_SPEC>;
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
impl From<crate::W<CLK_DIVIDER_CLK_GATING_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_DIVIDER_CLK_GATING_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "To C10,C01,C09 (SYNC Up on A0, AHB Interface of Batching Memory, AUDIO DMA, M4 SRAMs,M4 Bus Matrix , M4 Trace block, Debug controller, SDMA,I2S module Inside A1, AHB2APB Bridge /CFG DMA Bridge inside A1 , FFE, Packet FIFO,SDMA,A0, Voice APB interface, PIF, FB). Note: Firmware Should NOT program this bit to 0.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLK_DIVIDER_A_CG_A {
    #[doc = "0: Clock is stop"]
    STOP = 0,
    #[doc = "1: Clock is runnig"]
    RUN = 1,
}
impl From<CLK_DIVIDER_A_CG_A> for bool {
    #[inline(always)]
    fn from(variant: CLK_DIVIDER_A_CG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLK_DIVIDER_A_CG` reader - To C10,C01,C09 (SYNC Up on A0, AHB Interface of Batching Memory, AUDIO DMA, M4 SRAMs,M4 Bus Matrix , M4 Trace block, Debug controller, SDMA,I2S module Inside A1, AHB2APB Bridge /CFG DMA Bridge inside A1 , FFE, Packet FIFO,SDMA,A0, Voice APB interface, PIF, FB). Note: Firmware Should NOT program this bit to 0."]
pub struct CLK_DIVIDER_A_CG_R(crate::FieldReader<bool, CLK_DIVIDER_A_CG_A>);
impl CLK_DIVIDER_A_CG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLK_DIVIDER_A_CG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLK_DIVIDER_A_CG_A {
        match self.bits {
            false => CLK_DIVIDER_A_CG_A::STOP,
            true => CLK_DIVIDER_A_CG_A::RUN,
        }
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        **self == CLK_DIVIDER_A_CG_A::STOP
    }
    #[doc = "Checks if the value of the field is `RUN`"]
    #[inline(always)]
    pub fn is_run(&self) -> bool {
        **self == CLK_DIVIDER_A_CG_A::RUN
    }
}
impl core::ops::Deref for CLK_DIVIDER_A_CG_R {
    type Target = crate::FieldReader<bool, CLK_DIVIDER_A_CG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLK_DIVIDER_A_CG` writer - To C10,C01,C09 (SYNC Up on A0, AHB Interface of Batching Memory, AUDIO DMA, M4 SRAMs,M4 Bus Matrix , M4 Trace block, Debug controller, SDMA,I2S module Inside A1, AHB2APB Bridge /CFG DMA Bridge inside A1 , FFE, Packet FIFO,SDMA,A0, Voice APB interface, PIF, FB). Note: Firmware Should NOT program this bit to 0."]
pub struct CLK_DIVIDER_A_CG_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_DIVIDER_A_CG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLK_DIVIDER_A_CG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock is stop"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(CLK_DIVIDER_A_CG_A::STOP)
    }
    #[doc = "Clock is runnig"]
    #[inline(always)]
    pub fn run(self) -> &'a mut W {
        self.variant(CLK_DIVIDER_A_CG_A::RUN)
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
#[doc = "To C02 (FB, A1 (Including CFGSM))"]
pub type CLK_DIVIDER_B_CG_A = CLK_DIVIDER_A_CG_A;
#[doc = "Field `CLK_DIVIDER_B_CG` reader - To C02 (FB, A1 (Including CFGSM))"]
pub type CLK_DIVIDER_B_CG_R = CLK_DIVIDER_A_CG_R;
#[doc = "Field `CLK_DIVIDER_B_CG` writer - To C02 (FB, A1 (Including CFGSM))"]
pub struct CLK_DIVIDER_B_CG_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_DIVIDER_B_CG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLK_DIVIDER_B_CG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock is stop"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(CLK_DIVIDER_B_CG_A::STOP)
    }
    #[doc = "Clock is runnig"]
    #[inline(always)]
    pub fn run(self) -> &'a mut W {
        self.variant(CLK_DIVIDER_B_CG_A::RUN)
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
#[doc = "To C08 (FFE X4, X1)"]
pub type CLK_DIVIDER_C_CG_A = CLK_DIVIDER_A_CG_A;
#[doc = "Field `CLK_DIVIDER_C_CG` reader - To C08 (FFE X4, X1)"]
pub type CLK_DIVIDER_C_CG_R = CLK_DIVIDER_A_CG_R;
#[doc = "Field `CLK_DIVIDER_C_CG` writer - To C08 (FFE X4, X1)"]
pub struct CLK_DIVIDER_C_CG_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_DIVIDER_C_CG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLK_DIVIDER_C_CG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock is stop"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(CLK_DIVIDER_C_CG_A::STOP)
    }
    #[doc = "Clock is runnig"]
    #[inline(always)]
    pub fn run(self) -> &'a mut W {
        self.variant(CLK_DIVIDER_C_CG_A::RUN)
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
#[doc = "To C11 (M4 peripherals - AHB/APB bridge, UART, WDT and TIMER)"]
pub type CLK_DIVIDER_D_CG_A = CLK_DIVIDER_A_CG_A;
#[doc = "Field `CLK_DIVIDER_D_CG` reader - To C11 (M4 peripherals - AHB/APB bridge, UART, WDT and TIMER)"]
pub type CLK_DIVIDER_D_CG_R = CLK_DIVIDER_A_CG_R;
#[doc = "Field `CLK_DIVIDER_D_CG` writer - To C11 (M4 peripherals - AHB/APB bridge, UART, WDT and TIMER)"]
pub struct CLK_DIVIDER_D_CG_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_DIVIDER_D_CG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLK_DIVIDER_D_CG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock is stop"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(CLK_DIVIDER_D_CG_A::STOP)
    }
    #[doc = "Clock is runnig"]
    #[inline(always)]
    pub fn run(self) -> &'a mut W {
        self.variant(CLK_DIVIDER_D_CG_A::RUN)
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
#[doc = "To C16 (FB)"]
pub type CLK_DIVIDER_F_CG_A = CLK_DIVIDER_A_CG_A;
#[doc = "Field `CLK_DIVIDER_F_CG` reader - To C16 (FB)"]
pub type CLK_DIVIDER_F_CG_R = CLK_DIVIDER_A_CG_R;
#[doc = "Field `CLK_DIVIDER_F_CG` writer - To C16 (FB)"]
pub struct CLK_DIVIDER_F_CG_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_DIVIDER_F_CG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLK_DIVIDER_F_CG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock is stop"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(CLK_DIVIDER_F_CG_A::STOP)
    }
    #[doc = "Clock is runnig"]
    #[inline(always)]
    pub fn run(self) -> &'a mut W {
        self.variant(CLK_DIVIDER_F_CG_A::RUN)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "To C30, C31 (PDM LEFT/RIGHT Clk, I2S_MASTER clk, LPSD CLK)"]
pub type CLK_DIVIDER_G_CG_A = CLK_DIVIDER_A_CG_A;
#[doc = "Field `CLK_DIVIDER_G_CG` reader - To C30, C31 (PDM LEFT/RIGHT Clk, I2S_MASTER clk, LPSD CLK)"]
pub type CLK_DIVIDER_G_CG_R = CLK_DIVIDER_A_CG_R;
#[doc = "Field `CLK_DIVIDER_G_CG` writer - To C30, C31 (PDM LEFT/RIGHT Clk, I2S_MASTER clk, LPSD CLK)"]
pub struct CLK_DIVIDER_G_CG_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_DIVIDER_G_CG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLK_DIVIDER_G_CG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock is stop"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(CLK_DIVIDER_G_CG_A::STOP)
    }
    #[doc = "Clock is runnig"]
    #[inline(always)]
    pub fn run(self) -> &'a mut W {
        self.variant(CLK_DIVIDER_G_CG_A::RUN)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "To C19 (ADC)<F11>"]
pub type CLK_DIVIDER_H_CG_A = CLK_DIVIDER_A_CG_A;
#[doc = "Field `CLK_DIVIDER_H_CG` reader - To C19 (ADC)<F11>"]
pub type CLK_DIVIDER_H_CG_R = CLK_DIVIDER_A_CG_R;
#[doc = "Field `CLK_DIVIDER_H_CG` writer - To C19 (ADC)<F11>"]
pub struct CLK_DIVIDER_H_CG_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_DIVIDER_H_CG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLK_DIVIDER_H_CG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock is stop"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(CLK_DIVIDER_H_CG_A::STOP)
    }
    #[doc = "Clock is runnig"]
    #[inline(always)]
    pub fn run(self) -> &'a mut W {
        self.variant(CLK_DIVIDER_H_CG_A::RUN)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "To C21 (FB)"]
pub type CLK_DIVIDER_I_CG_A = CLK_DIVIDER_A_CG_A;
#[doc = "Field `CLK_DIVIDER_I_CG` reader - To C21 (FB)"]
pub type CLK_DIVIDER_I_CG_R = CLK_DIVIDER_A_CG_R;
#[doc = "Field `CLK_DIVIDER_I_CG` writer - To C21 (FB)"]
pub struct CLK_DIVIDER_I_CG_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_DIVIDER_I_CG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLK_DIVIDER_I_CG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock is stop"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(CLK_DIVIDER_I_CG_A::STOP)
    }
    #[doc = "Clock is runnig"]
    #[inline(always)]
    pub fn run(self) -> &'a mut W {
        self.variant(CLK_DIVIDER_I_CG_A::RUN)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "To C23 (PMU clk gating control)"]
pub type CLK_DIVIDER_J_CG_A = CLK_DIVIDER_A_CG_A;
#[doc = "Field `CLK_DIVIDER_J_CG` reader - To C23 (PMU clk gating control)"]
pub type CLK_DIVIDER_J_CG_R = CLK_DIVIDER_A_CG_R;
#[doc = "Field `CLK_DIVIDER_J_CG` writer - To C23 (PMU clk gating control)"]
pub struct CLK_DIVIDER_J_CG_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_DIVIDER_J_CG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLK_DIVIDER_J_CG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock is stop"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(CLK_DIVIDER_J_CG_A::STOP)
    }
    #[doc = "Clock is runnig"]
    #[inline(always)]
    pub fn run(self) -> &'a mut W {
        self.variant(CLK_DIVIDER_J_CG_A::RUN)
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
    #[doc = "Bit 0 - To C10,C01,C09 (SYNC Up on A0, AHB Interface of Batching Memory, AUDIO DMA, M4 SRAMs,M4 Bus Matrix , M4 Trace block, Debug controller, SDMA,I2S module Inside A1, AHB2APB Bridge /CFG DMA Bridge inside A1 , FFE, Packet FIFO,SDMA,A0, Voice APB interface, PIF, FB). Note: Firmware Should NOT program this bit to 0."]
    #[inline(always)]
    pub fn clk_divider_a_cg(&self) -> CLK_DIVIDER_A_CG_R {
        CLK_DIVIDER_A_CG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - To C02 (FB, A1 (Including CFGSM))"]
    #[inline(always)]
    pub fn clk_divider_b_cg(&self) -> CLK_DIVIDER_B_CG_R {
        CLK_DIVIDER_B_CG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - To C08 (FFE X4, X1)"]
    #[inline(always)]
    pub fn clk_divider_c_cg(&self) -> CLK_DIVIDER_C_CG_R {
        CLK_DIVIDER_C_CG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - To C11 (M4 peripherals - AHB/APB bridge, UART, WDT and TIMER)"]
    #[inline(always)]
    pub fn clk_divider_d_cg(&self) -> CLK_DIVIDER_D_CG_R {
        CLK_DIVIDER_D_CG_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - To C16 (FB)"]
    #[inline(always)]
    pub fn clk_divider_f_cg(&self) -> CLK_DIVIDER_F_CG_R {
        CLK_DIVIDER_F_CG_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - To C30, C31 (PDM LEFT/RIGHT Clk, I2S_MASTER clk, LPSD CLK)"]
    #[inline(always)]
    pub fn clk_divider_g_cg(&self) -> CLK_DIVIDER_G_CG_R {
        CLK_DIVIDER_G_CG_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - To C19 (ADC)<F11>"]
    #[inline(always)]
    pub fn clk_divider_h_cg(&self) -> CLK_DIVIDER_H_CG_R {
        CLK_DIVIDER_H_CG_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - To C21 (FB)"]
    #[inline(always)]
    pub fn clk_divider_i_cg(&self) -> CLK_DIVIDER_I_CG_R {
        CLK_DIVIDER_I_CG_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - To C23 (PMU clk gating control)"]
    #[inline(always)]
    pub fn clk_divider_j_cg(&self) -> CLK_DIVIDER_J_CG_R {
        CLK_DIVIDER_J_CG_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - To C10,C01,C09 (SYNC Up on A0, AHB Interface of Batching Memory, AUDIO DMA, M4 SRAMs,M4 Bus Matrix , M4 Trace block, Debug controller, SDMA,I2S module Inside A1, AHB2APB Bridge /CFG DMA Bridge inside A1 , FFE, Packet FIFO,SDMA,A0, Voice APB interface, PIF, FB). Note: Firmware Should NOT program this bit to 0."]
    #[inline(always)]
    pub fn clk_divider_a_cg(&mut self) -> CLK_DIVIDER_A_CG_W {
        CLK_DIVIDER_A_CG_W { w: self }
    }
    #[doc = "Bit 1 - To C02 (FB, A1 (Including CFGSM))"]
    #[inline(always)]
    pub fn clk_divider_b_cg(&mut self) -> CLK_DIVIDER_B_CG_W {
        CLK_DIVIDER_B_CG_W { w: self }
    }
    #[doc = "Bit 2 - To C08 (FFE X4, X1)"]
    #[inline(always)]
    pub fn clk_divider_c_cg(&mut self) -> CLK_DIVIDER_C_CG_W {
        CLK_DIVIDER_C_CG_W { w: self }
    }
    #[doc = "Bit 3 - To C11 (M4 peripherals - AHB/APB bridge, UART, WDT and TIMER)"]
    #[inline(always)]
    pub fn clk_divider_d_cg(&mut self) -> CLK_DIVIDER_D_CG_W {
        CLK_DIVIDER_D_CG_W { w: self }
    }
    #[doc = "Bit 5 - To C16 (FB)"]
    #[inline(always)]
    pub fn clk_divider_f_cg(&mut self) -> CLK_DIVIDER_F_CG_W {
        CLK_DIVIDER_F_CG_W { w: self }
    }
    #[doc = "Bit 6 - To C30, C31 (PDM LEFT/RIGHT Clk, I2S_MASTER clk, LPSD CLK)"]
    #[inline(always)]
    pub fn clk_divider_g_cg(&mut self) -> CLK_DIVIDER_G_CG_W {
        CLK_DIVIDER_G_CG_W { w: self }
    }
    #[doc = "Bit 7 - To C19 (ADC)<F11>"]
    #[inline(always)]
    pub fn clk_divider_h_cg(&mut self) -> CLK_DIVIDER_H_CG_W {
        CLK_DIVIDER_H_CG_W { w: self }
    }
    #[doc = "Bit 8 - To C21 (FB)"]
    #[inline(always)]
    pub fn clk_divider_i_cg(&mut self) -> CLK_DIVIDER_I_CG_W {
        CLK_DIVIDER_I_CG_W { w: self }
    }
    #[doc = "Bit 9 - To C23 (PMU clk gating control)"]
    #[inline(always)]
    pub fn clk_divider_j_cg(&mut self) -> CLK_DIVIDER_J_CG_W {
        CLK_DIVIDER_J_CG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control for divider gates in different clocks\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_divider_clk_gating](index.html) module"]
pub struct CLK_DIVIDER_CLK_GATING_SPEC;
impl crate::RegisterSpec for CLK_DIVIDER_CLK_GATING_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_divider_clk_gating::R](R) reader structure"]
impl crate::Readable for CLK_DIVIDER_CLK_GATING_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_divider_clk_gating::W](W) writer structure"]
impl crate::Writable for CLK_DIVIDER_CLK_GATING_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_DIVIDER_CLK_GATING to value 0x03ff"]
impl crate::Resettable for CLK_DIVIDER_CLK_GATING_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x03ff
    }
}
