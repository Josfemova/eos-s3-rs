#[doc = "Register `SDMA_PD_SRC_MASK_N` reader"]
pub struct R(crate::R<SDMA_PD_SRC_MASK_N_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDMA_PD_SRC_MASK_N_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDMA_PD_SRC_MASK_N_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDMA_PD_SRC_MASK_N_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDMA_PD_SRC_MASK_N` writer"]
pub struct W(crate::W<SDMA_PD_SRC_MASK_N_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDMA_PD_SRC_MASK_N_SPEC>;
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
impl From<crate::W<SDMA_PD_SRC_MASK_N_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDMA_PD_SRC_MASK_N_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "1'b0 : SDMA HW Power Down Event will be masked.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDMA_PD_EVENT_A {
    #[doc = "0: SDMA HW Power Down Event will be masked."]
    MASK = 0,
    #[doc = "1: Undefined Behaviour. Probably unmasks the Power Down event."]
    UNDEFINED = 1,
}
impl From<SDMA_PD_EVENT_A> for bool {
    #[inline(always)]
    fn from(variant: SDMA_PD_EVENT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDMA_PD_EVENT` reader - 1'b0 : SDMA HW Power Down Event will be masked."]
pub struct SDMA_PD_EVENT_R(crate::FieldReader<bool, SDMA_PD_EVENT_A>);
impl SDMA_PD_EVENT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SDMA_PD_EVENT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDMA_PD_EVENT_A {
        match self.bits {
            false => SDMA_PD_EVENT_A::MASK,
            true => SDMA_PD_EVENT_A::UNDEFINED,
        }
    }
    #[doc = "Checks if the value of the field is `MASK`"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        **self == SDMA_PD_EVENT_A::MASK
    }
    #[doc = "Checks if the value of the field is `UNDEFINED`"]
    #[inline(always)]
    pub fn is_undefined(&self) -> bool {
        **self == SDMA_PD_EVENT_A::UNDEFINED
    }
}
impl core::ops::Deref for SDMA_PD_EVENT_R {
    type Target = crate::FieldReader<bool, SDMA_PD_EVENT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDMA_PD_EVENT` writer - 1'b0 : SDMA HW Power Down Event will be masked."]
pub struct SDMA_PD_EVENT_W<'a> {
    w: &'a mut W,
}
impl<'a> SDMA_PD_EVENT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDMA_PD_EVENT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "SDMA HW Power Down Event will be masked."]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(SDMA_PD_EVENT_A::MASK)
    }
    #[doc = "Undefined Behaviour. Probably unmasks the Power Down event."]
    #[inline(always)]
    pub fn undefined(self) -> &'a mut W {
        self.variant(SDMA_PD_EVENT_A::UNDEFINED)
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
    #[doc = "Bit 0 - 1'b0 : SDMA HW Power Down Event will be masked."]
    #[inline(always)]
    pub fn sdma_pd_event(&self) -> SDMA_PD_EVENT_R {
        SDMA_PD_EVENT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1'b0 : SDMA HW Power Down Event will be masked."]
    #[inline(always)]
    pub fn sdma_pd_event(&mut self) -> SDMA_PD_EVENT_W {
        SDMA_PD_EVENT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Register for controlling if power down event will be masked\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdma_pd_src_mask_n](index.html) module"]
pub struct SDMA_PD_SRC_MASK_N_SPEC;
impl crate::RegisterSpec for SDMA_PD_SRC_MASK_N_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdma_pd_src_mask_n::R](R) reader structure"]
impl crate::Readable for SDMA_PD_SRC_MASK_N_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdma_pd_src_mask_n::W](W) writer structure"]
impl crate::Writable for SDMA_PD_SRC_MASK_N_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SDMA_PD_SRC_MASK_N to value 0"]
impl crate::Resettable for SDMA_PD_SRC_MASK_N_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
