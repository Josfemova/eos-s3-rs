#[doc = "Register `SDMA_MEM_CTRL_1` reader"]
pub struct R(crate::R<SDMA_MEM_CTRL_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDMA_MEM_CTRL_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDMA_MEM_CTRL_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDMA_MEM_CTRL_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDMA_MEM_CTRL_1` writer"]
pub struct W(crate::W<SDMA_MEM_CTRL_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDMA_MEM_CTRL_1_SPEC>;
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
impl From<crate::W<SDMA_MEM_CTRL_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDMA_MEM_CTRL_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Used to control the Shut Down function of SRAM Macro, Memory content will be lost\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDMA_SRAM_SD_A {
    #[doc = "0: Disable the shut down function of SRAM Macro."]
    DISABLE_SHUT_DOWN = 0,
    #[doc = "1: Enable the shut down function of SRAM Macro, Memory content will be lost."]
    ENABLE_SHUT_DOWN = 1,
}
impl From<SDMA_SRAM_SD_A> for bool {
    #[inline(always)]
    fn from(variant: SDMA_SRAM_SD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDMA_SRAM_SD` reader - Used to control the Shut Down function of SRAM Macro, Memory content will be lost"]
pub struct SDMA_SRAM_SD_R(crate::FieldReader<bool, SDMA_SRAM_SD_A>);
impl SDMA_SRAM_SD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SDMA_SRAM_SD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDMA_SRAM_SD_A {
        match self.bits {
            false => SDMA_SRAM_SD_A::DISABLE_SHUT_DOWN,
            true => SDMA_SRAM_SD_A::ENABLE_SHUT_DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE_SHUT_DOWN`"]
    #[inline(always)]
    pub fn is_disable_shut_down(&self) -> bool {
        **self == SDMA_SRAM_SD_A::DISABLE_SHUT_DOWN
    }
    #[doc = "Checks if the value of the field is `ENABLE_SHUT_DOWN`"]
    #[inline(always)]
    pub fn is_enable_shut_down(&self) -> bool {
        **self == SDMA_SRAM_SD_A::ENABLE_SHUT_DOWN
    }
}
impl core::ops::Deref for SDMA_SRAM_SD_R {
    type Target = crate::FieldReader<bool, SDMA_SRAM_SD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDMA_SRAM_SD` writer - Used to control the Shut Down function of SRAM Macro, Memory content will be lost"]
pub struct SDMA_SRAM_SD_W<'a> {
    w: &'a mut W,
}
impl<'a> SDMA_SRAM_SD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDMA_SRAM_SD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the shut down function of SRAM Macro."]
    #[inline(always)]
    pub fn disable_shut_down(self) -> &'a mut W {
        self.variant(SDMA_SRAM_SD_A::DISABLE_SHUT_DOWN)
    }
    #[doc = "Enable the shut down function of SRAM Macro, Memory content will be lost."]
    #[inline(always)]
    pub fn enable_shut_down(self) -> &'a mut W {
        self.variant(SDMA_SRAM_SD_A::ENABLE_SHUT_DOWN)
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
    #[doc = "Bit 0 - Used to control the Shut Down function of SRAM Macro, Memory content will be lost"]
    #[inline(always)]
    pub fn sdma_sram_sd(&self) -> SDMA_SRAM_SD_R {
        SDMA_SRAM_SD_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Used to control the Shut Down function of SRAM Macro, Memory content will be lost"]
    #[inline(always)]
    pub fn sdma_sram_sd(&mut self) -> SDMA_SRAM_SD_W {
        SDMA_SRAM_SD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control the Shut Down function of SRAM Macro for the SDMA power domain\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdma_mem_ctrl_1](index.html) module"]
pub struct SDMA_MEM_CTRL_1_SPEC;
impl crate::RegisterSpec for SDMA_MEM_CTRL_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdma_mem_ctrl_1::R](R) reader structure"]
impl crate::Readable for SDMA_MEM_CTRL_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdma_mem_ctrl_1::W](W) writer structure"]
impl crate::Writable for SDMA_MEM_CTRL_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SDMA_MEM_CTRL_1 to value 0"]
impl crate::Resettable for SDMA_MEM_CTRL_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
