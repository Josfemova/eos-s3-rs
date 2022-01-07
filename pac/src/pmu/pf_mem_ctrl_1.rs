#[doc = "Register `PF_MEM_CTRL_1` reader"]
pub struct R(crate::R<PF_MEM_CTRL_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PF_MEM_CTRL_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PF_MEM_CTRL_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PF_MEM_CTRL_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PF_MEM_CTRL_1` writer"]
pub struct W(crate::W<PF_MEM_CTRL_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PF_MEM_CTRL_1_SPEC>;
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
impl From<crate::W<PF_MEM_CTRL_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PF_MEM_CTRL_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Control SD Pin of FIFO_2 instances on PF subsystem\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTRL_PF_SRAM_SD_0_A {
    #[doc = "0: Disable the shut down function of SRAM Macro."]
    DISABLE_SHUT_DOWN = 0,
    #[doc = "1: Enable the shut down function of SRAM Macro, Memory content will be lost."]
    ENABLE_SHUT_DOWN = 1,
}
impl From<CTRL_PF_SRAM_SD_0_A> for bool {
    #[inline(always)]
    fn from(variant: CTRL_PF_SRAM_SD_0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTRL_PF_SRAM_SD_0` reader - Control SD Pin of FIFO_2 instances on PF subsystem"]
pub struct CTRL_PF_SRAM_SD_0_R(crate::FieldReader<bool, CTRL_PF_SRAM_SD_0_A>);
impl CTRL_PF_SRAM_SD_0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CTRL_PF_SRAM_SD_0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTRL_PF_SRAM_SD_0_A {
        match self.bits {
            false => CTRL_PF_SRAM_SD_0_A::DISABLE_SHUT_DOWN,
            true => CTRL_PF_SRAM_SD_0_A::ENABLE_SHUT_DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE_SHUT_DOWN`"]
    #[inline(always)]
    pub fn is_disable_shut_down(&self) -> bool {
        **self == CTRL_PF_SRAM_SD_0_A::DISABLE_SHUT_DOWN
    }
    #[doc = "Checks if the value of the field is `ENABLE_SHUT_DOWN`"]
    #[inline(always)]
    pub fn is_enable_shut_down(&self) -> bool {
        **self == CTRL_PF_SRAM_SD_0_A::ENABLE_SHUT_DOWN
    }
}
impl core::ops::Deref for CTRL_PF_SRAM_SD_0_R {
    type Target = crate::FieldReader<bool, CTRL_PF_SRAM_SD_0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTRL_PF_SRAM_SD_0` writer - Control SD Pin of FIFO_2 instances on PF subsystem"]
pub struct CTRL_PF_SRAM_SD_0_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRL_PF_SRAM_SD_0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTRL_PF_SRAM_SD_0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the shut down function of SRAM Macro."]
    #[inline(always)]
    pub fn disable_shut_down(self) -> &'a mut W {
        self.variant(CTRL_PF_SRAM_SD_0_A::DISABLE_SHUT_DOWN)
    }
    #[doc = "Enable the shut down function of SRAM Macro, Memory content will be lost."]
    #[inline(always)]
    pub fn enable_shut_down(self) -> &'a mut W {
        self.variant(CTRL_PF_SRAM_SD_0_A::ENABLE_SHUT_DOWN)
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
#[doc = "Control SD Pin of FIFO_8k instances on PF subsystem"]
pub type CTRL_PF_SRAM_SD_8K_A = CTRL_PF_SRAM_SD_0_A;
#[doc = "Field `CTRL_PF_SRAM_SD_8k` reader - Control SD Pin of FIFO_8k instances on PF subsystem"]
pub type CTRL_PF_SRAM_SD_8K_R = CTRL_PF_SRAM_SD_0_R;
#[doc = "Field `CTRL_PF_SRAM_SD_8k` writer - Control SD Pin of FIFO_8k instances on PF subsystem"]
pub struct CTRL_PF_SRAM_SD_8K_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRL_PF_SRAM_SD_8K_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTRL_PF_SRAM_SD_8K_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the shut down function of SRAM Macro."]
    #[inline(always)]
    pub fn disable_shut_down(self) -> &'a mut W {
        self.variant(CTRL_PF_SRAM_SD_8K_A::DISABLE_SHUT_DOWN)
    }
    #[doc = "Enable the shut down function of SRAM Macro, Memory content will be lost."]
    #[inline(always)]
    pub fn enable_shut_down(self) -> &'a mut W {
        self.variant(CTRL_PF_SRAM_SD_8K_A::ENABLE_SHUT_DOWN)
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
impl R {
    #[doc = "Bit 2 - Control SD Pin of FIFO_2 instances on PF subsystem"]
    #[inline(always)]
    pub fn ctrl_pf_sram_sd_0(&self) -> CTRL_PF_SRAM_SD_0_R {
        CTRL_PF_SRAM_SD_0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Control SD Pin of FIFO_8k instances on PF subsystem"]
    #[inline(always)]
    pub fn ctrl_pf_sram_sd_8k(&self) -> CTRL_PF_SRAM_SD_8K_R {
        CTRL_PF_SRAM_SD_8K_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Control SD Pin of FIFO_2 instances on PF subsystem"]
    #[inline(always)]
    pub fn ctrl_pf_sram_sd_0(&mut self) -> CTRL_PF_SRAM_SD_0_W {
        CTRL_PF_SRAM_SD_0_W { w: self }
    }
    #[doc = "Bit 3 - Control SD Pin of FIFO_8k instances on PF subsystem"]
    #[inline(always)]
    pub fn ctrl_pf_sram_sd_8k(&mut self) -> CTRL_PF_SRAM_SD_8K_W {
        CTRL_PF_SRAM_SD_8K_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control SD pin of varios FIFOs intances in the PF subsystem. For each one: 1'b1 : Enable the Shut Down function of SRAM Macro, Memory content will be lost\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pf_mem_ctrl_1](index.html) module"]
pub struct PF_MEM_CTRL_1_SPEC;
impl crate::RegisterSpec for PF_MEM_CTRL_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pf_mem_ctrl_1::R](R) reader structure"]
impl crate::Readable for PF_MEM_CTRL_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pf_mem_ctrl_1::W](W) writer structure"]
impl crate::Writable for PF_MEM_CTRL_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PF_MEM_CTRL_1 to value 0"]
impl crate::Resettable for PF_MEM_CTRL_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
