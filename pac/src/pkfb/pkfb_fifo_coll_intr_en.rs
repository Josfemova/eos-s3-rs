#[doc = "Register `PKFB_FIFO_COLL_INTR_EN` reader"]
pub struct R(crate::R<PKFB_FIFO_COLL_INTR_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PKFB_FIFO_COLL_INTR_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PKFB_FIFO_COLL_INTR_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PKFB_FIFO_COLL_INTR_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PKFB_FIFO_COLL_INTR_EN` writer"]
pub struct W(crate::W<PKFB_FIFO_COLL_INTR_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PKFB_FIFO_COLL_INTR_EN_SPEC>;
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
impl From<crate::W<PKFB_FIFO_COLL_INTR_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PKFB_FIFO_COLL_INTR_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Set bit to enable interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PF0_COLL_INTR_EN_A {
    #[doc = "0: Mask the interrupt event"]
    MASK = 0,
    #[doc = "1: Unmask the interrupt event"]
    UNMASK = 1,
}
impl From<PF0_COLL_INTR_EN_A> for bool {
    #[inline(always)]
    fn from(variant: PF0_COLL_INTR_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pf0_coll_intr_en` reader - Set bit to enable interrupt"]
pub struct PF0_COLL_INTR_EN_R(crate::FieldReader<bool, PF0_COLL_INTR_EN_A>);
impl PF0_COLL_INTR_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PF0_COLL_INTR_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PF0_COLL_INTR_EN_A {
        match self.bits {
            false => PF0_COLL_INTR_EN_A::MASK,
            true => PF0_COLL_INTR_EN_A::UNMASK,
        }
    }
    #[doc = "Checks if the value of the field is `MASK`"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        **self == PF0_COLL_INTR_EN_A::MASK
    }
    #[doc = "Checks if the value of the field is `UNMASK`"]
    #[inline(always)]
    pub fn is_unmask(&self) -> bool {
        **self == PF0_COLL_INTR_EN_A::UNMASK
    }
}
impl core::ops::Deref for PF0_COLL_INTR_EN_R {
    type Target = crate::FieldReader<bool, PF0_COLL_INTR_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pf0_coll_intr_en` writer - Set bit to enable interrupt"]
pub struct PF0_COLL_INTR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PF0_COLL_INTR_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PF0_COLL_INTR_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Mask the interrupt event"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(PF0_COLL_INTR_EN_A::MASK)
    }
    #[doc = "Unmask the interrupt event"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut W {
        self.variant(PF0_COLL_INTR_EN_A::UNMASK)
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
#[doc = "Set bit to enable interrupt"]
pub type PF1_COLL_INTR_EN_A = PF0_COLL_INTR_EN_A;
#[doc = "Field `pf1_coll_intr_en` reader - Set bit to enable interrupt"]
pub type PF1_COLL_INTR_EN_R = PF0_COLL_INTR_EN_R;
#[doc = "Field `pf1_coll_intr_en` writer - Set bit to enable interrupt"]
pub struct PF1_COLL_INTR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PF1_COLL_INTR_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PF1_COLL_INTR_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Mask the interrupt event"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(PF1_COLL_INTR_EN_A::MASK)
    }
    #[doc = "Unmask the interrupt event"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut W {
        self.variant(PF1_COLL_INTR_EN_A::UNMASK)
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
#[doc = "Set bit to enable interrupt"]
pub type PF2_COLL_INTR_EN_A = PF0_COLL_INTR_EN_A;
#[doc = "Field `pf2_coll_intr_en` reader - Set bit to enable interrupt"]
pub type PF2_COLL_INTR_EN_R = PF0_COLL_INTR_EN_R;
#[doc = "Field `pf2_coll_intr_en` writer - Set bit to enable interrupt"]
pub struct PF2_COLL_INTR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PF2_COLL_INTR_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PF2_COLL_INTR_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Mask the interrupt event"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(PF2_COLL_INTR_EN_A::MASK)
    }
    #[doc = "Unmask the interrupt event"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut W {
        self.variant(PF2_COLL_INTR_EN_A::UNMASK)
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
#[doc = "Set bit to enable interrupt"]
pub type PF8K_COLL_INTR_EN_A = PF0_COLL_INTR_EN_A;
#[doc = "Field `pf8k_coll_intr_en` reader - Set bit to enable interrupt"]
pub type PF8K_COLL_INTR_EN_R = PF0_COLL_INTR_EN_R;
#[doc = "Field `pf8k_coll_intr_en` writer - Set bit to enable interrupt"]
pub struct PF8K_COLL_INTR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PF8K_COLL_INTR_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PF8K_COLL_INTR_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Mask the interrupt event"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(PF8K_COLL_INTR_EN_A::MASK)
    }
    #[doc = "Unmask the interrupt event"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut W {
        self.variant(PF8K_COLL_INTR_EN_A::UNMASK)
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
    #[doc = "Bit 0 - Set bit to enable interrupt"]
    #[inline(always)]
    pub fn pf0_coll_intr_en(&self) -> PF0_COLL_INTR_EN_R {
        PF0_COLL_INTR_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Set bit to enable interrupt"]
    #[inline(always)]
    pub fn pf1_coll_intr_en(&self) -> PF1_COLL_INTR_EN_R {
        PF1_COLL_INTR_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Set bit to enable interrupt"]
    #[inline(always)]
    pub fn pf2_coll_intr_en(&self) -> PF2_COLL_INTR_EN_R {
        PF2_COLL_INTR_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Set bit to enable interrupt"]
    #[inline(always)]
    pub fn pf8k_coll_intr_en(&self) -> PF8K_COLL_INTR_EN_R {
        PF8K_COLL_INTR_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set bit to enable interrupt"]
    #[inline(always)]
    pub fn pf0_coll_intr_en(&mut self) -> PF0_COLL_INTR_EN_W {
        PF0_COLL_INTR_EN_W { w: self }
    }
    #[doc = "Bit 1 - Set bit to enable interrupt"]
    #[inline(always)]
    pub fn pf1_coll_intr_en(&mut self) -> PF1_COLL_INTR_EN_W {
        PF1_COLL_INTR_EN_W { w: self }
    }
    #[doc = "Bit 2 - Set bit to enable interrupt"]
    #[inline(always)]
    pub fn pf2_coll_intr_en(&mut self) -> PF2_COLL_INTR_EN_W {
        PF2_COLL_INTR_EN_W { w: self }
    }
    #[doc = "Bit 3 - Set bit to enable interrupt"]
    #[inline(always)]
    pub fn pf8k_coll_intr_en(&mut self) -> PF8K_COLL_INTR_EN_W {
        PF8K_COLL_INTR_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control register for enabling or masking the collisione interrupts\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pkfb_fifo_coll_intr_en](index.html) module"]
pub struct PKFB_FIFO_COLL_INTR_EN_SPEC;
impl crate::RegisterSpec for PKFB_FIFO_COLL_INTR_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pkfb_fifo_coll_intr_en::R](R) reader structure"]
impl crate::Readable for PKFB_FIFO_COLL_INTR_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pkfb_fifo_coll_intr_en::W](W) writer structure"]
impl crate::Writable for PKFB_FIFO_COLL_INTR_EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PKFB_FIFO_COLL_INTR_EN to value 0"]
impl crate::Resettable for PKFB_FIFO_COLL_INTR_EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
