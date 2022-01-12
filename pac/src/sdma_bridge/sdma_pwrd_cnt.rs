#[doc = "Register `SDMA_PWRD_CNT` reader"]
pub struct R(crate::R<SDMA_PWRD_CNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDMA_PWRD_CNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDMA_PWRD_CNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDMA_PWRD_CNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDMA_PWRD_CNT` writer"]
pub struct W(crate::W<SDMA_PWRD_CNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDMA_PWRD_CNT_SPEC>;
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
impl From<crate::W<SDMA_PWRD_CNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDMA_PWRD_CNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `sdma_pwrdn_cnt` reader - sdma power down event threshold. If sdma stays in idle cycles longer than the threshold, sdma will be automaticlly put into power down to save power."]
pub struct SDMA_PWRDN_CNT_R(crate::FieldReader<u16, u16>);
impl SDMA_PWRDN_CNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        SDMA_PWRDN_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDMA_PWRDN_CNT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sdma_pwrdn_cnt` writer - sdma power down event threshold. If sdma stays in idle cycles longer than the threshold, sdma will be automaticlly put into power down to save power."]
pub struct SDMA_PWRDN_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> SDMA_PWRDN_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | (value as u32 & 0x3fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:13 - sdma power down event threshold. If sdma stays in idle cycles longer than the threshold, sdma will be automaticlly put into power down to save power."]
    #[inline(always)]
    pub fn sdma_pwrdn_cnt(&self) -> SDMA_PWRDN_CNT_R {
        SDMA_PWRDN_CNT_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - sdma power down event threshold. If sdma stays in idle cycles longer than the threshold, sdma will be automaticlly put into power down to save power."]
    #[inline(always)]
    pub fn sdma_pwrdn_cnt(&mut self) -> SDMA_PWRDN_CNT_W {
        SDMA_PWRDN_CNT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "sdma power down event threshold. If sdma stays in idle cycles longer than the threshold, sdma will be automaticlly put into power down to save power.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdma_pwrd_cnt](index.html) module"]
pub struct SDMA_PWRD_CNT_SPEC;
impl crate::RegisterSpec for SDMA_PWRD_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdma_pwrd_cnt::R](R) reader structure"]
impl crate::Readable for SDMA_PWRD_CNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdma_pwrd_cnt::W](W) writer structure"]
impl crate::Writable for SDMA_PWRD_CNT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SDMA_PWRD_CNT to value 0"]
impl crate::Resettable for SDMA_PWRD_CNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
