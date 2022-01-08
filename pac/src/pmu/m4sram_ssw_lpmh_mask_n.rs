#[doc = "Register `M4SRAM_SSW_LPMH_MASK_N` reader"]
pub struct R(crate::R<M4SRAM_SSW_LPMH_MASK_N_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<M4SRAM_SSW_LPMH_MASK_N_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<M4SRAM_SSW_LPMH_MASK_N_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<M4SRAM_SSW_LPMH_MASK_N_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `M4SRAM_SSW_LPMH_MASK_N` writer"]
pub struct W(crate::W<M4SRAM_SSW_LPMH_MASK_N_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<M4SRAM_SSW_LPMH_MASK_N_SPEC>;
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
impl From<crate::W<M4SRAM_SSW_LPMH_MASK_N_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<M4SRAM_SSW_LPMH_MASK_N_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "if M4 SRAM power is removing Or SRAM is auto waking up from Deep Sleep Mode, the corresponding bit will be clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum M4SRAM_LPMH_MASK_N_A {
    #[doc = "0: M4 SRAM's LPMH will be de-asserted"]
    LPMH_DE_ASSERT = 0,
    #[doc = "1: M4_MEM_CTRL_0 will control the corresponding M4 SRAM's LPMH"]
    LET_M4_MEM_CTRL_0_CONTROL = 1,
}
impl From<M4SRAM_LPMH_MASK_N_A> for u16 {
    #[inline(always)]
    fn from(variant: M4SRAM_LPMH_MASK_N_A) -> Self {
        variant as _
    }
}
#[doc = "Field `M4SRAM_LPMH_MASK_N` reader - if M4 SRAM power is removing Or SRAM is auto waking up from Deep Sleep Mode, the corresponding bit will be clear."]
pub struct M4SRAM_LPMH_MASK_N_R(crate::FieldReader<u16, M4SRAM_LPMH_MASK_N_A>);
impl M4SRAM_LPMH_MASK_N_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        M4SRAM_LPMH_MASK_N_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<M4SRAM_LPMH_MASK_N_A> {
        match self.bits {
            0 => Some(M4SRAM_LPMH_MASK_N_A::LPMH_DE_ASSERT),
            1 => Some(M4SRAM_LPMH_MASK_N_A::LET_M4_MEM_CTRL_0_CONTROL),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LPMH_DE_ASSERT`"]
    #[inline(always)]
    pub fn is_lpmh_de_assert(&self) -> bool {
        **self == M4SRAM_LPMH_MASK_N_A::LPMH_DE_ASSERT
    }
    #[doc = "Checks if the value of the field is `LET_M4_MEM_CTRL_0_CONTROL`"]
    #[inline(always)]
    pub fn is_let_m4_mem_ctrl_0_control(&self) -> bool {
        **self == M4SRAM_LPMH_MASK_N_A::LET_M4_MEM_CTRL_0_CONTROL
    }
}
impl core::ops::Deref for M4SRAM_LPMH_MASK_N_R {
    type Target = crate::FieldReader<u16, M4SRAM_LPMH_MASK_N_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M4SRAM_LPMH_MASK_N` writer - if M4 SRAM power is removing Or SRAM is auto waking up from Deep Sleep Mode, the corresponding bit will be clear."]
pub struct M4SRAM_LPMH_MASK_N_W<'a> {
    w: &'a mut W,
}
impl<'a> M4SRAM_LPMH_MASK_N_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: M4SRAM_LPMH_MASK_N_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "M4 SRAM's LPMH will be de-asserted"]
    #[inline(always)]
    pub fn lpmh_de_assert(self) -> &'a mut W {
        self.variant(M4SRAM_LPMH_MASK_N_A::LPMH_DE_ASSERT)
    }
    #[doc = "M4_MEM_CTRL_0 will control the corresponding M4 SRAM's LPMH"]
    #[inline(always)]
    pub fn let_m4_mem_ctrl_0_control(self) -> &'a mut W {
        self.variant(M4SRAM_LPMH_MASK_N_A::LET_M4_MEM_CTRL_0_CONTROL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - if M4 SRAM power is removing Or SRAM is auto waking up from Deep Sleep Mode, the corresponding bit will be clear."]
    #[inline(always)]
    pub fn m4sram_lpmh_mask_n(&self) -> M4SRAM_LPMH_MASK_N_R {
        M4SRAM_LPMH_MASK_N_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - if M4 SRAM power is removing Or SRAM is auto waking up from Deep Sleep Mode, the corresponding bit will be clear."]
    #[inline(always)]
    pub fn m4sram_lpmh_mask_n(&mut self) -> M4SRAM_LPMH_MASK_N_W {
        M4SRAM_LPMH_MASK_N_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control masking for the LPMH (Low Power Mode header - deep sleep circuit)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m4sram_ssw_lpmh_mask_n](index.html) module"]
pub struct M4SRAM_SSW_LPMH_MASK_N_SPEC;
impl crate::RegisterSpec for M4SRAM_SSW_LPMH_MASK_N_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [m4sram_ssw_lpmh_mask_n::R](R) reader structure"]
impl crate::Readable for M4SRAM_SSW_LPMH_MASK_N_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [m4sram_ssw_lpmh_mask_n::W](W) writer structure"]
impl crate::Writable for M4SRAM_SSW_LPMH_MASK_N_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets M4SRAM_SSW_LPMH_MASK_N to value 0"]
impl crate::Resettable for M4SRAM_SSW_LPMH_MASK_N_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
