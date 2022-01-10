#[doc = "Register `PKFB_PF1DATA` reader"]
pub struct R(crate::R<PKFB_PF1DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PKFB_PF1DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PKFB_PF1DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PKFB_PF1DATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PKFB_PF1DATA` writer"]
pub struct W(crate::W<PKFB_PF1DATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PKFB_PF1DATA_SPEC>;
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
impl From<crate::W<PKFB_PF1DATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PKFB_PF1DATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pf1_data_reg` reader - FIFO 1 Push/POP Data Register"]
pub struct PF1_DATA_REG_R(crate::FieldReader<u32, u32>);
impl PF1_DATA_REG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        PF1_DATA_REG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PF1_DATA_REG_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pf1_data_reg` writer - FIFO 1 Push/POP Data Register"]
pub struct PF1_DATA_REG_W<'a> {
    w: &'a mut W,
}
impl<'a> PF1_DATA_REG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - FIFO 1 Push/POP Data Register"]
    #[inline(always)]
    pub fn pf1_data_reg(&self) -> PF1_DATA_REG_R {
        PF1_DATA_REG_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - FIFO 1 Push/POP Data Register"]
    #[inline(always)]
    pub fn pf1_data_reg(&mut self) -> PF1_DATA_REG_W {
        PF1_DATA_REG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FIFO 1 Push/POP Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pkfb_pf1data](index.html) module"]
pub struct PKFB_PF1DATA_SPEC;
impl crate::RegisterSpec for PKFB_PF1DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pkfb_pf1data::R](R) reader structure"]
impl crate::Readable for PKFB_PF1DATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pkfb_pf1data::W](W) writer structure"]
impl crate::Writable for PKFB_PF1DATA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PKFB_PF1DATA to value 0"]
impl crate::Resettable for PKFB_PF1DATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
