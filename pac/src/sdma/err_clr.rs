#[doc = "Register `ERR_CLR` reader"]
pub struct R(crate::R<ERR_CLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ERR_CLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ERR_CLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ERR_CLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ERR_CLR` writer"]
pub struct W(crate::W<ERR_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ERR_CLR_SPEC>;
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
impl From<crate::W<ERR_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ERR_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `err_clr` reader - Returns the status of dma_err, or sets the signal LOW. Read as:\n \n 0 = dma_err is LOW \n \n 1 = dma_err is HIGH. \n \n Write as: \n \n 0 = No effect, status of dma_err is unchanged. \n \n 1 = Sets dma_err LOW. \n \n For test purposes, use the err_set register to set dma_err HIGH."]
pub struct ERR_CLR_R(crate::FieldReader<bool, bool>);
impl ERR_CLR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERR_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERR_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `err_clr` writer - Returns the status of dma_err, or sets the signal LOW. Read as:\n \n 0 = dma_err is LOW \n \n 1 = dma_err is HIGH. \n \n Write as: \n \n 0 = No effect, status of dma_err is unchanged. \n \n 1 = Sets dma_err LOW. \n \n For test purposes, use the err_set register to set dma_err HIGH."]
pub struct ERR_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR_CLR_W<'a> {
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
    #[doc = "Bit 0 - Returns the status of dma_err, or sets the signal LOW. Read as:\n \n 0 = dma_err is LOW \n \n 1 = dma_err is HIGH. \n \n Write as: \n \n 0 = No effect, status of dma_err is unchanged. \n \n 1 = Sets dma_err LOW. \n \n For test purposes, use the err_set register to set dma_err HIGH."]
    #[inline(always)]
    pub fn err_clr(&self) -> ERR_CLR_R {
        ERR_CLR_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Returns the status of dma_err, or sets the signal LOW. Read as:\n \n 0 = dma_err is LOW \n \n 1 = dma_err is HIGH. \n \n Write as: \n \n 0 = No effect, status of dma_err is unchanged. \n \n 1 = Sets dma_err LOW. \n \n For test purposes, use the err_set register to set dma_err HIGH."]
    #[inline(always)]
    pub fn err_clr(&mut self) -> ERR_CLR_W {
        ERR_CLR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Returns the status of dma_err, or sets the signal LOW.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [err_clr](index.html) module"]
pub struct ERR_CLR_SPEC;
impl crate::RegisterSpec for ERR_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [err_clr::R](R) reader structure"]
impl crate::Readable for ERR_CLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [err_clr::W](W) writer structure"]
impl crate::Writable for ERR_CLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ERR_CLR to value 0"]
impl crate::Resettable for ERR_CLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
