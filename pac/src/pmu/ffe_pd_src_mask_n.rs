#[doc = "Register `FFE_PD_SRC_MASK_N` reader"]
pub struct R(crate::R<FFE_PD_SRC_MASK_N_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FFE_PD_SRC_MASK_N_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FFE_PD_SRC_MASK_N_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FFE_PD_SRC_MASK_N_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FFE_PD_SRC_MASK_N` writer"]
pub struct W(crate::W<FFE_PD_SRC_MASK_N_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FFE_PD_SRC_MASK_N_SPEC>;
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
impl From<crate::W<FFE_PD_SRC_MASK_N_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FFE_PD_SRC_MASK_N_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "FFEs Busy Signal (FFE 0, SM 0 and SM1 Busy Signals ) Mask_N\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FFE_PD_EVENT_MASK_A {
    #[doc = "0: Masks the busy signal sources"]
    MASK = 0,
    #[doc = "1: Will unmask the busy signal sources"]
    UNMASK = 1,
}
impl From<FFE_PD_EVENT_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: FFE_PD_EVENT_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FFE_PD_Event_Mask` reader - FFEs Busy Signal (FFE 0, SM 0 and SM1 Busy Signals ) Mask_N"]
pub struct FFE_PD_EVENT_MASK_R(crate::FieldReader<bool, FFE_PD_EVENT_MASK_A>);
impl FFE_PD_EVENT_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FFE_PD_EVENT_MASK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FFE_PD_EVENT_MASK_A {
        match self.bits {
            false => FFE_PD_EVENT_MASK_A::MASK,
            true => FFE_PD_EVENT_MASK_A::UNMASK,
        }
    }
    #[doc = "Checks if the value of the field is `MASK`"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        **self == FFE_PD_EVENT_MASK_A::MASK
    }
    #[doc = "Checks if the value of the field is `UNMASK`"]
    #[inline(always)]
    pub fn is_unmask(&self) -> bool {
        **self == FFE_PD_EVENT_MASK_A::UNMASK
    }
}
impl core::ops::Deref for FFE_PD_EVENT_MASK_R {
    type Target = crate::FieldReader<bool, FFE_PD_EVENT_MASK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FFE_PD_Event_Mask` writer - FFEs Busy Signal (FFE 0, SM 0 and SM1 Busy Signals ) Mask_N"]
pub struct FFE_PD_EVENT_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> FFE_PD_EVENT_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FFE_PD_EVENT_MASK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Masks the busy signal sources"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(FFE_PD_EVENT_MASK_A::MASK)
    }
    #[doc = "Will unmask the busy signal sources"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut W {
        self.variant(FFE_PD_EVENT_MASK_A::UNMASK)
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
    #[doc = "Bit 0 - FFEs Busy Signal (FFE 0, SM 0 and SM1 Busy Signals ) Mask_N"]
    #[inline(always)]
    pub fn ffe_pd_event_mask(&self) -> FFE_PD_EVENT_MASK_R {
        FFE_PD_EVENT_MASK_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FFEs Busy Signal (FFE 0, SM 0 and SM1 Busy Signals ) Mask_N"]
    #[inline(always)]
    pub fn ffe_pd_event_mask(&mut self) -> FFE_PD_EVENT_MASK_W {
        FFE_PD_EVENT_MASK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control masking of busy signal. The falling edge of any of the above signals (non-mask) will put the FFE into Power saving mode base on the Power Mode Cfg. Note: These signals used to indicate the BUSY status, so they must be level signals.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ffe_pd_src_mask_n](index.html) module"]
pub struct FFE_PD_SRC_MASK_N_SPEC;
impl crate::RegisterSpec for FFE_PD_SRC_MASK_N_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ffe_pd_src_mask_n::R](R) reader structure"]
impl crate::Readable for FFE_PD_SRC_MASK_N_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ffe_pd_src_mask_n::W](W) writer structure"]
impl crate::Writable for FFE_PD_SRC_MASK_N_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FFE_PD_SRC_MASK_N to value 0"]
impl crate::Resettable for FFE_PD_SRC_MASK_N_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
