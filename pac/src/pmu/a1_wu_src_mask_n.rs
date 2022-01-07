#[doc = "Register `A1_WU_SRC_MASK_N` reader"]
pub struct R(crate::R<A1_WU_SRC_MASK_N_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<A1_WU_SRC_MASK_N_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<A1_WU_SRC_MASK_N_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<A1_WU_SRC_MASK_N_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `A1_WU_SRC_MASK_N` writer"]
pub struct W(crate::W<A1_WU_SRC_MASK_N_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<A1_WU_SRC_MASK_N_SPEC>;
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
impl From<crate::W<A1_WU_SRC_MASK_N_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<A1_WU_SRC_MASK_N_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "If Boot Code Reloading after M4 wake up is needed, this bit should be set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum A1_WU_EVENT_MASK_M_A {
    #[doc = "0: Mask the M4 Wakeup event as the trigger for the A1 wake-up event"]
    MASK = 0,
    #[doc = "1: Unmask M4 wake-up event as the trigger for the A1 wake-up"]
    UNMASK = 1,
}
impl From<A1_WU_EVENT_MASK_M_A> for bool {
    #[inline(always)]
    fn from(variant: A1_WU_EVENT_MASK_M_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `A1_WU_EVENT_MASK_M` reader - If Boot Code Reloading after M4 wake up is needed, this bit should be set to 1."]
pub struct A1_WU_EVENT_MASK_M_R(crate::FieldReader<bool, A1_WU_EVENT_MASK_M_A>);
impl A1_WU_EVENT_MASK_M_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        A1_WU_EVENT_MASK_M_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> A1_WU_EVENT_MASK_M_A {
        match self.bits {
            false => A1_WU_EVENT_MASK_M_A::MASK,
            true => A1_WU_EVENT_MASK_M_A::UNMASK,
        }
    }
    #[doc = "Checks if the value of the field is `MASK`"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        **self == A1_WU_EVENT_MASK_M_A::MASK
    }
    #[doc = "Checks if the value of the field is `UNMASK`"]
    #[inline(always)]
    pub fn is_unmask(&self) -> bool {
        **self == A1_WU_EVENT_MASK_M_A::UNMASK
    }
}
impl core::ops::Deref for A1_WU_EVENT_MASK_M_R {
    type Target = crate::FieldReader<bool, A1_WU_EVENT_MASK_M_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `A1_WU_EVENT_MASK_M` writer - If Boot Code Reloading after M4 wake up is needed, this bit should be set to 1."]
pub struct A1_WU_EVENT_MASK_M_W<'a> {
    w: &'a mut W,
}
impl<'a> A1_WU_EVENT_MASK_M_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: A1_WU_EVENT_MASK_M_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Mask the M4 Wakeup event as the trigger for the A1 wake-up event"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(A1_WU_EVENT_MASK_M_A::MASK)
    }
    #[doc = "Unmask M4 wake-up event as the trigger for the A1 wake-up"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut W {
        self.variant(A1_WU_EVENT_MASK_M_A::UNMASK)
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
    #[doc = "Bit 0 - If Boot Code Reloading after M4 wake up is needed, this bit should be set to 1."]
    #[inline(always)]
    pub fn a1_wu_event_mask_m(&self) -> A1_WU_EVENT_MASK_M_R {
        A1_WU_EVENT_MASK_M_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - If Boot Code Reloading after M4 wake up is needed, this bit should be set to 1."]
    #[inline(always)]
    pub fn a1_wu_event_mask_m(&mut self) -> A1_WU_EVENT_MASK_M_W {
        A1_WU_EVENT_MASK_M_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control masking of wake-up event triggers for the A1 domain\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [a1_wu_src_mask_n](index.html) module"]
pub struct A1_WU_SRC_MASK_N_SPEC;
impl crate::RegisterSpec for A1_WU_SRC_MASK_N_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [a1_wu_src_mask_n::R](R) reader structure"]
impl crate::Readable for A1_WU_SRC_MASK_N_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [a1_wu_src_mask_n::W](W) writer structure"]
impl crate::Writable for A1_WU_SRC_MASK_N_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets A1_WU_SRC_MASK_N to value 0"]
impl crate::Resettable for A1_WU_SRC_MASK_N_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
