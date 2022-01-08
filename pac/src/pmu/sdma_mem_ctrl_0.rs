#[doc = "Register `SDMA_MEM_CTRL_0` reader"]
pub struct R(crate::R<SDMA_MEM_CTRL_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDMA_MEM_CTRL_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDMA_MEM_CTRL_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDMA_MEM_CTRL_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDMA_MEM_CTRL_0` writer"]
pub struct W(crate::W<SDMA_MEM_CTRL_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDMA_MEM_CTRL_0_SPEC>;
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
impl From<crate::W<SDMA_MEM_CTRL_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDMA_MEM_CTRL_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Used to control the Deep Sleep function of SRAM Macro, Memory content will be kept.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDMA_SRAM_DS_A {
    #[doc = "0: Disable the Deep Sleep function of SRAM Macro, Memory content will be kept."]
    DISABLE_DEEP_SLEEP = 0,
    #[doc = "1: Enable the Deep Sleep function of SRAM Macro, Memory content will be kept."]
    ENABLE_DEEP_SLEEP = 1,
}
impl From<SDMA_SRAM_DS_A> for bool {
    #[inline(always)]
    fn from(variant: SDMA_SRAM_DS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDMA_SRAM_DS` reader - Used to control the Deep Sleep function of SRAM Macro, Memory content will be kept."]
pub struct SDMA_SRAM_DS_R(crate::FieldReader<bool, SDMA_SRAM_DS_A>);
impl SDMA_SRAM_DS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SDMA_SRAM_DS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDMA_SRAM_DS_A {
        match self.bits {
            false => SDMA_SRAM_DS_A::DISABLE_DEEP_SLEEP,
            true => SDMA_SRAM_DS_A::ENABLE_DEEP_SLEEP,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE_DEEP_SLEEP`"]
    #[inline(always)]
    pub fn is_disable_deep_sleep(&self) -> bool {
        **self == SDMA_SRAM_DS_A::DISABLE_DEEP_SLEEP
    }
    #[doc = "Checks if the value of the field is `ENABLE_DEEP_SLEEP`"]
    #[inline(always)]
    pub fn is_enable_deep_sleep(&self) -> bool {
        **self == SDMA_SRAM_DS_A::ENABLE_DEEP_SLEEP
    }
}
impl core::ops::Deref for SDMA_SRAM_DS_R {
    type Target = crate::FieldReader<bool, SDMA_SRAM_DS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDMA_SRAM_DS` writer - Used to control the Deep Sleep function of SRAM Macro, Memory content will be kept."]
pub struct SDMA_SRAM_DS_W<'a> {
    w: &'a mut W,
}
impl<'a> SDMA_SRAM_DS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDMA_SRAM_DS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the Deep Sleep function of SRAM Macro, Memory content will be kept."]
    #[inline(always)]
    pub fn disable_deep_sleep(self) -> &'a mut W {
        self.variant(SDMA_SRAM_DS_A::DISABLE_DEEP_SLEEP)
    }
    #[doc = "Enable the Deep Sleep function of SRAM Macro, Memory content will be kept."]
    #[inline(always)]
    pub fn enable_deep_sleep(self) -> &'a mut W {
        self.variant(SDMA_SRAM_DS_A::ENABLE_DEEP_SLEEP)
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
    #[doc = "Bit 0 - Used to control the Deep Sleep function of SRAM Macro, Memory content will be kept."]
    #[inline(always)]
    pub fn sdma_sram_ds(&self) -> SDMA_SRAM_DS_R {
        SDMA_SRAM_DS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Used to control the Deep Sleep function of SRAM Macro, Memory content will be kept."]
    #[inline(always)]
    pub fn sdma_sram_ds(&mut self) -> SDMA_SRAM_DS_W {
        SDMA_SRAM_DS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control the Deep Sleep function of SRAM Macro for the SDMA power domain\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdma_mem_ctrl_0](index.html) module"]
pub struct SDMA_MEM_CTRL_0_SPEC;
impl crate::RegisterSpec for SDMA_MEM_CTRL_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdma_mem_ctrl_0::R](R) reader structure"]
impl crate::Readable for SDMA_MEM_CTRL_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdma_mem_ctrl_0::W](W) writer structure"]
impl crate::Writable for SDMA_MEM_CTRL_0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SDMA_MEM_CTRL_0 to value 0"]
impl crate::Resettable for SDMA_MEM_CTRL_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
