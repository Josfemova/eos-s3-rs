#[doc = "Register `AUDIO_WU_SRC_MASK_N` reader"]
pub struct R(crate::R<AUDIO_WU_SRC_MASK_N_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AUDIO_WU_SRC_MASK_N_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AUDIO_WU_SRC_MASK_N_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AUDIO_WU_SRC_MASK_N_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AUDIO_WU_SRC_MASK_N` writer"]
pub struct W(crate::W<AUDIO_WU_SRC_MASK_N_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AUDIO_WU_SRC_MASK_N_SPEC>;
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
impl From<crate::W<AUDIO_WU_SRC_MASK_N_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AUDIO_WU_SRC_MASK_N_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Allow external interrupt coming from pads 9 or 30 to serve as wake-up event trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AD0_WU_EVENT_MASK_N_A {
    #[doc = "0: Mask the external interrupts from pads 9 or 30 as wake-up event triggers"]
    MASK = 0,
    #[doc = "1: Unmask the external interrupts from pads 9 or 30 as wake-up event triggers"]
    UNMASK = 1,
}
impl From<AD0_WU_EVENT_MASK_N_A> for bool {
    #[inline(always)]
    fn from(variant: AD0_WU_EVENT_MASK_N_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AD0_WU_Event_Mask_N` reader - Allow external interrupt coming from pads 9 or 30 to serve as wake-up event trigger"]
pub struct AD0_WU_EVENT_MASK_N_R(
    crate::FieldReader<bool, AD0_WU_EVENT_MASK_N_A>,
);
impl AD0_WU_EVENT_MASK_N_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AD0_WU_EVENT_MASK_N_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AD0_WU_EVENT_MASK_N_A {
        match self.bits {
            false => AD0_WU_EVENT_MASK_N_A::MASK,
            true => AD0_WU_EVENT_MASK_N_A::UNMASK,
        }
    }
    #[doc = "Checks if the value of the field is `MASK`"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        **self == AD0_WU_EVENT_MASK_N_A::MASK
    }
    #[doc = "Checks if the value of the field is `UNMASK`"]
    #[inline(always)]
    pub fn is_unmask(&self) -> bool {
        **self == AD0_WU_EVENT_MASK_N_A::UNMASK
    }
}
impl core::ops::Deref for AD0_WU_EVENT_MASK_N_R {
    type Target = crate::FieldReader<bool, AD0_WU_EVENT_MASK_N_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AD0_WU_Event_Mask_N` writer - Allow external interrupt coming from pads 9 or 30 to serve as wake-up event trigger"]
pub struct AD0_WU_EVENT_MASK_N_W<'a> {
    w: &'a mut W,
}
impl<'a> AD0_WU_EVENT_MASK_N_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AD0_WU_EVENT_MASK_N_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Mask the external interrupts from pads 9 or 30 as wake-up event triggers"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(AD0_WU_EVENT_MASK_N_A::MASK)
    }
    #[doc = "Unmask the external interrupts from pads 9 or 30 as wake-up event triggers"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut W {
        self.variant(AD0_WU_EVENT_MASK_N_A::UNMASK)
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
#[doc = "Allow external interrupt coming from pads 9 or 30 to serve as wake-up event trigger"]
pub type AD1_WU_EVENT_MASK_N_A = AD0_WU_EVENT_MASK_N_A;
#[doc = "Field `AD1_WU_Event_Mask_N` reader - Allow external interrupt coming from pads 9 or 30 to serve as wake-up event trigger"]
pub type AD1_WU_EVENT_MASK_N_R = AD0_WU_EVENT_MASK_N_R;
#[doc = "Field `AD1_WU_Event_Mask_N` writer - Allow external interrupt coming from pads 9 or 30 to serve as wake-up event trigger"]
pub struct AD1_WU_EVENT_MASK_N_W<'a> {
    w: &'a mut W,
}
impl<'a> AD1_WU_EVENT_MASK_N_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AD1_WU_EVENT_MASK_N_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Mask the external interrupts from pads 9 or 30 as wake-up event triggers"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(AD1_WU_EVENT_MASK_N_A::MASK)
    }
    #[doc = "Unmask the external interrupts from pads 9 or 30 as wake-up event triggers"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut W {
        self.variant(AD1_WU_EVENT_MASK_N_A::UNMASK)
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
#[doc = "Allow external interrupt coming from pads 9 or 30 to serve as wake-up event trigger"]
pub type AD2_WU_EVENT_MASK_N_A = AD0_WU_EVENT_MASK_N_A;
#[doc = "Field `AD2_WU_Event_Mask_N` reader - Allow external interrupt coming from pads 9 or 30 to serve as wake-up event trigger"]
pub type AD2_WU_EVENT_MASK_N_R = AD0_WU_EVENT_MASK_N_R;
#[doc = "Field `AD2_WU_Event_Mask_N` writer - Allow external interrupt coming from pads 9 or 30 to serve as wake-up event trigger"]
pub struct AD2_WU_EVENT_MASK_N_W<'a> {
    w: &'a mut W,
}
impl<'a> AD2_WU_EVENT_MASK_N_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AD2_WU_EVENT_MASK_N_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Mask the external interrupts from pads 9 or 30 as wake-up event triggers"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(AD2_WU_EVENT_MASK_N_A::MASK)
    }
    #[doc = "Unmask the external interrupts from pads 9 or 30 as wake-up event triggers"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut W {
        self.variant(AD2_WU_EVENT_MASK_N_A::UNMASK)
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
#[doc = "Allow external interrupt coming from pads 9 or 30 to serve as wake-up event trigger"]
pub type AD3_WU_EVENT_MASK_N_A = AD0_WU_EVENT_MASK_N_A;
#[doc = "Field `AD3_WU_Event_Mask_N` reader - Allow external interrupt coming from pads 9 or 30 to serve as wake-up event trigger"]
pub type AD3_WU_EVENT_MASK_N_R = AD0_WU_EVENT_MASK_N_R;
#[doc = "Field `AD3_WU_Event_Mask_N` writer - Allow external interrupt coming from pads 9 or 30 to serve as wake-up event trigger"]
pub struct AD3_WU_EVENT_MASK_N_W<'a> {
    w: &'a mut W,
}
impl<'a> AD3_WU_EVENT_MASK_N_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AD3_WU_EVENT_MASK_N_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Mask the external interrupts from pads 9 or 30 as wake-up event triggers"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(AD3_WU_EVENT_MASK_N_A::MASK)
    }
    #[doc = "Unmask the external interrupts from pads 9 or 30 as wake-up event triggers"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut W {
        self.variant(AD3_WU_EVENT_MASK_N_A::UNMASK)
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
#[doc = "Allow external interrupt coming from pads 9 or 30 to serve as wake-up event trigger"]
pub type AD4_WU_EVENT_MASK_N_A = AD0_WU_EVENT_MASK_N_A;
#[doc = "Field `AD4_WU_Event_Mask_N` reader - Allow external interrupt coming from pads 9 or 30 to serve as wake-up event trigger"]
pub type AD4_WU_EVENT_MASK_N_R = AD0_WU_EVENT_MASK_N_R;
#[doc = "Field `AD4_WU_Event_Mask_N` writer - Allow external interrupt coming from pads 9 or 30 to serve as wake-up event trigger"]
pub struct AD4_WU_EVENT_MASK_N_W<'a> {
    w: &'a mut W,
}
impl<'a> AD4_WU_EVENT_MASK_N_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AD4_WU_EVENT_MASK_N_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Mask the external interrupts from pads 9 or 30 as wake-up event triggers"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(AD4_WU_EVENT_MASK_N_A::MASK)
    }
    #[doc = "Unmask the external interrupts from pads 9 or 30 as wake-up event triggers"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut W {
        self.variant(AD4_WU_EVENT_MASK_N_A::UNMASK)
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
#[doc = "Allow external interrupt coming from pads 9 or 30 to serve as wake-up event trigger"]
pub type AD5_WU_EVENT_MASK_N_A = AD0_WU_EVENT_MASK_N_A;
#[doc = "Field `AD5_WU_Event_Mask_N` reader - Allow external interrupt coming from pads 9 or 30 to serve as wake-up event trigger"]
pub type AD5_WU_EVENT_MASK_N_R = AD0_WU_EVENT_MASK_N_R;
#[doc = "Field `AD5_WU_Event_Mask_N` writer - Allow external interrupt coming from pads 9 or 30 to serve as wake-up event trigger"]
pub struct AD5_WU_EVENT_MASK_N_W<'a> {
    w: &'a mut W,
}
impl<'a> AD5_WU_EVENT_MASK_N_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AD5_WU_EVENT_MASK_N_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Mask the external interrupts from pads 9 or 30 as wake-up event triggers"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(AD5_WU_EVENT_MASK_N_A::MASK)
    }
    #[doc = "Unmask the external interrupts from pads 9 or 30 as wake-up event triggers"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut W {
        self.variant(AD5_WU_EVENT_MASK_N_A::UNMASK)
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
impl R {
    #[doc = "Bit 0 - Allow external interrupt coming from pads 9 or 30 to serve as wake-up event trigger"]
    #[inline(always)]
    pub fn ad0_wu_event_mask_n(&self) -> AD0_WU_EVENT_MASK_N_R {
        AD0_WU_EVENT_MASK_N_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Allow external interrupt coming from pads 9 or 30 to serve as wake-up event trigger"]
    #[inline(always)]
    pub fn ad1_wu_event_mask_n(&self) -> AD1_WU_EVENT_MASK_N_R {
        AD1_WU_EVENT_MASK_N_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Allow external interrupt coming from pads 9 or 30 to serve as wake-up event trigger"]
    #[inline(always)]
    pub fn ad2_wu_event_mask_n(&self) -> AD2_WU_EVENT_MASK_N_R {
        AD2_WU_EVENT_MASK_N_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Allow external interrupt coming from pads 9 or 30 to serve as wake-up event trigger"]
    #[inline(always)]
    pub fn ad3_wu_event_mask_n(&self) -> AD3_WU_EVENT_MASK_N_R {
        AD3_WU_EVENT_MASK_N_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Allow external interrupt coming from pads 9 or 30 to serve as wake-up event trigger"]
    #[inline(always)]
    pub fn ad4_wu_event_mask_n(&self) -> AD4_WU_EVENT_MASK_N_R {
        AD4_WU_EVENT_MASK_N_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Allow external interrupt coming from pads 9 or 30 to serve as wake-up event trigger"]
    #[inline(always)]
    pub fn ad5_wu_event_mask_n(&self) -> AD5_WU_EVENT_MASK_N_R {
        AD5_WU_EVENT_MASK_N_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Allow external interrupt coming from pads 9 or 30 to serve as wake-up event trigger"]
    #[inline(always)]
    pub fn ad0_wu_event_mask_n(&mut self) -> AD0_WU_EVENT_MASK_N_W {
        AD0_WU_EVENT_MASK_N_W { w: self }
    }
    #[doc = "Bit 1 - Allow external interrupt coming from pads 9 or 30 to serve as wake-up event trigger"]
    #[inline(always)]
    pub fn ad1_wu_event_mask_n(&mut self) -> AD1_WU_EVENT_MASK_N_W {
        AD1_WU_EVENT_MASK_N_W { w: self }
    }
    #[doc = "Bit 2 - Allow external interrupt coming from pads 9 or 30 to serve as wake-up event trigger"]
    #[inline(always)]
    pub fn ad2_wu_event_mask_n(&mut self) -> AD2_WU_EVENT_MASK_N_W {
        AD2_WU_EVENT_MASK_N_W { w: self }
    }
    #[doc = "Bit 3 - Allow external interrupt coming from pads 9 or 30 to serve as wake-up event trigger"]
    #[inline(always)]
    pub fn ad3_wu_event_mask_n(&mut self) -> AD3_WU_EVENT_MASK_N_W {
        AD3_WU_EVENT_MASK_N_W { w: self }
    }
    #[doc = "Bit 4 - Allow external interrupt coming from pads 9 or 30 to serve as wake-up event trigger"]
    #[inline(always)]
    pub fn ad4_wu_event_mask_n(&mut self) -> AD4_WU_EVENT_MASK_N_W {
        AD4_WU_EVENT_MASK_N_W { w: self }
    }
    #[doc = "Bit 5 - Allow external interrupt coming from pads 9 or 30 to serve as wake-up event trigger"]
    #[inline(always)]
    pub fn ad5_wu_event_mask_n(&mut self) -> AD5_WU_EVENT_MASK_N_W {
        AD5_WU_EVENT_MASK_N_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control masking of wake-up event triggers for the Audio domains\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [audio_wu_src_mask_n](index.html) module"]
pub struct AUDIO_WU_SRC_MASK_N_SPEC;
impl crate::RegisterSpec for AUDIO_WU_SRC_MASK_N_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [audio_wu_src_mask_n::R](R) reader structure"]
impl crate::Readable for AUDIO_WU_SRC_MASK_N_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [audio_wu_src_mask_n::W](W) writer structure"]
impl crate::Writable for AUDIO_WU_SRC_MASK_N_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AUDIO_WU_SRC_MASK_N to value 0"]
impl crate::Resettable for AUDIO_WU_SRC_MASK_N_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
