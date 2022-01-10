#[doc = "Register `PKFB_FIFO_COLL_INTR` reader"]
pub struct R(crate::R<PKFB_FIFO_COLL_INTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PKFB_FIFO_COLL_INTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PKFB_FIFO_COLL_INTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PKFB_FIFO_COLL_INTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PKFB_FIFO_COLL_INTR` writer"]
pub struct W(crate::W<PKFB_FIFO_COLL_INTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PKFB_FIFO_COLL_INTR_SPEC>;
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
impl From<crate::W<PKFB_FIFO_COLL_INTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PKFB_FIFO_COLL_INTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pf0_coll_intr` reader - Interrupt is triggered when bot FFE0 and FFE1 try to push into packet FIFO 0"]
pub struct PF0_COLL_INTR_R(crate::FieldReader<bool, bool>);
impl PF0_COLL_INTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PF0_COLL_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PF0_COLL_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pf0_coll_intr` writer - Interrupt is triggered when bot FFE0 and FFE1 try to push into packet FIFO 0"]
pub struct PF0_COLL_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> PF0_COLL_INTR_W<'a> {
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
#[doc = "Field `pf1_coll_intr` reader - Interrupt is triggered when bot FFE0 and FFE1 try to push into packet FIFO 1"]
pub struct PF1_COLL_INTR_R(crate::FieldReader<bool, bool>);
impl PF1_COLL_INTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PF1_COLL_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PF1_COLL_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pf1_coll_intr` writer - Interrupt is triggered when bot FFE0 and FFE1 try to push into packet FIFO 1"]
pub struct PF1_COLL_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> PF1_COLL_INTR_W<'a> {
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
#[doc = "Field `pf2_coll_intr` reader - Interrupt is triggered when bot FFE0 and FFE1 try to push into packet FIFO 0"]
pub struct PF2_COLL_INTR_R(crate::FieldReader<bool, bool>);
impl PF2_COLL_INTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PF2_COLL_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PF2_COLL_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pf2_coll_intr` writer - Interrupt is triggered when bot FFE0 and FFE1 try to push into packet FIFO 0"]
pub struct PF2_COLL_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> PF2_COLL_INTR_W<'a> {
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
#[doc = "Field `pf8k_coll_intr` reader - Interrupt is triggered when bot FFE0 and FFE1 try to push into packet FIFO 8k"]
pub struct PF8K_COLL_INTR_R(crate::FieldReader<bool, bool>);
impl PF8K_COLL_INTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PF8K_COLL_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PF8K_COLL_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pf8k_coll_intr` writer - Interrupt is triggered when bot FFE0 and FFE1 try to push into packet FIFO 8k"]
pub struct PF8K_COLL_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> PF8K_COLL_INTR_W<'a> {
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
    #[doc = "Bit 0 - Interrupt is triggered when bot FFE0 and FFE1 try to push into packet FIFO 0"]
    #[inline(always)]
    pub fn pf0_coll_intr(&self) -> PF0_COLL_INTR_R {
        PF0_COLL_INTR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Interrupt is triggered when bot FFE0 and FFE1 try to push into packet FIFO 1"]
    #[inline(always)]
    pub fn pf1_coll_intr(&self) -> PF1_COLL_INTR_R {
        PF1_COLL_INTR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Interrupt is triggered when bot FFE0 and FFE1 try to push into packet FIFO 0"]
    #[inline(always)]
    pub fn pf2_coll_intr(&self) -> PF2_COLL_INTR_R {
        PF2_COLL_INTR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Interrupt is triggered when bot FFE0 and FFE1 try to push into packet FIFO 8k"]
    #[inline(always)]
    pub fn pf8k_coll_intr(&self) -> PF8K_COLL_INTR_R {
        PF8K_COLL_INTR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt is triggered when bot FFE0 and FFE1 try to push into packet FIFO 0"]
    #[inline(always)]
    pub fn pf0_coll_intr(&mut self) -> PF0_COLL_INTR_W {
        PF0_COLL_INTR_W { w: self }
    }
    #[doc = "Bit 1 - Interrupt is triggered when bot FFE0 and FFE1 try to push into packet FIFO 1"]
    #[inline(always)]
    pub fn pf1_coll_intr(&mut self) -> PF1_COLL_INTR_W {
        PF1_COLL_INTR_W { w: self }
    }
    #[doc = "Bit 2 - Interrupt is triggered when bot FFE0 and FFE1 try to push into packet FIFO 0"]
    #[inline(always)]
    pub fn pf2_coll_intr(&mut self) -> PF2_COLL_INTR_W {
        PF2_COLL_INTR_W { w: self }
    }
    #[doc = "Bit 3 - Interrupt is triggered when bot FFE0 and FFE1 try to push into packet FIFO 8k"]
    #[inline(always)]
    pub fn pf8k_coll_intr(&mut self) -> PF8K_COLL_INTR_W {
        PF8K_COLL_INTR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control for collision interrupts\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pkfb_fifo_coll_intr](index.html) module"]
pub struct PKFB_FIFO_COLL_INTR_SPEC;
impl crate::RegisterSpec for PKFB_FIFO_COLL_INTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pkfb_fifo_coll_intr::R](R) reader structure"]
impl crate::Readable for PKFB_FIFO_COLL_INTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pkfb_fifo_coll_intr::W](W) writer structure"]
impl crate::Writable for PKFB_FIFO_COLL_INTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PKFB_FIFO_COLL_INTR to value 0"]
impl crate::Resettable for PKFB_FIFO_COLL_INTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
